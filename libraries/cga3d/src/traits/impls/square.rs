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
//   Median:        12      14       0
//  Average:        56      62       0
//  Maximum:       610     637       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        12      16       0
//  Average:        72      79       0
//  Maximum:       912     962       0
impl Square for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       85       96        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       89      101        0
    //  no simd      101      116        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-(Simd32x4::from(self[e41]) * Simd32x4::from([self[e45], self[e12], self[e31], self[e15]]))
                - (Simd32x4::from(self[e43]) * Simd32x4::from([self[e31], self[e23], self[e45], self[e35]]))
                - (Simd32x4::from(self[e42]) * Simd32x4::from([self[e12], self[e45], self[e23], self[e25]]))
                + Simd32x4::from([
                    ((self[scalar] * self[e41]) + (self[e12] * self[e42]) + (self[e43] * self[e31]) + (self[e41] * self[e45]) + (self[e41] * self[scalar])),
                    ((self[scalar] * self[e42]) + (self[e23] * self[e43]) + (self[e42] * self[scalar]) + (self[e41] * self[e12]) + (self[e42] * self[e45])),
                    ((self[scalar] * self[e43]) + (self[e31] * self[e41]) + (self[e43] * self[scalar]) + (self[e43] * self[e45]) + (self[e42] * self[e23])),
                    (f32::powi(self[scalar], 2) + f32::powi(self[e45], 2)
                        - f32::powi(self[e12], 2)
                        - f32::powi(self[e31], 2)
                        - f32::powi(self[e23], 2)
                        - (self[e43] * self[e35])
                        - (self[e41] * self[e15])
                        - (self[e42] * self[e25])),
                ])),
            // e23, e31, e12, e45
            (Simd32x4::from(2.0) * (Simd32x4::from(self[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]))
                + Simd32x4::from([
                    ((self[e35] * self[e42]) - (self[e25] * self[e43]) + (self[e12] * self[e31]) - (self[e31] * self[e12]) - (self[e42] * self[e35]) + (self[e43] * self[e25])),
                    (-(self[e35] * self[e41]) + (self[e15] * self[e43]) - (self[e12] * self[e23]) + (self[e23] * self[e12]) + (self[e41] * self[e35]) - (self[e43] * self[e15])),
                    ((self[e25] * self[e41]) - (self[e15] * self[e42]) + (self[e31] * self[e23]) - (self[e23] * self[e31]) - (self[e41] * self[e25]) + (self[e42] * self[e15])),
                    (-(self[e35] * self[e43]) - (self[e25] * self[e42]) - (self[e15] * self[e41]) + (self[e43] * self[e35]) + (self[e41] * self[e15]) + (self[e42] * self[e25])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self[scalar] * self[e15]) + (self[e35] * self[e31]) - (self[e25] * self[e12]) + (self[e15] * self[scalar]) - (self[e15] * self[e45]) + (self[e45] * self[e15])
                    - (self[e31] * self[e35])
                    + (self[e12] * self[e25])),
                ((self[scalar] * self[e25]) - (self[e35] * self[e23]) + (self[e25] * self[scalar]) - (self[e25] * self[e45])
                    + (self[e15] * self[e12])
                    + (self[e45] * self[e25])
                    + (self[e23] * self[e35])
                    - (self[e12] * self[e15])),
                ((self[scalar] * self[e35]) + (self[e35] * self[scalar]) - (self[e35] * self[e45]) + (self[e25] * self[e23]) - (self[e15] * self[e31]) + (self[e45] * self[e35])
                    - (self[e23] * self[e25])
                    + (self[e31] * self[e15])),
                (-(self[e12] * self[e43]) - (self[e31] * self[e42]) - (self[e23] * self[e41]) - (self[e43] * self[e12]) - (self[e41] * self[e23]) - (self[e42] * self[e31])),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                ((self[e35] * self[e42]) - (self[e25] * self[e43]) + (self[e45] * self[e23]) + (self[e23] * self[e45]) + (self[e42] * self[e35]) - (self[e43] * self[e25])),
                (-(self[e35] * self[e41]) + (self[e15] * self[e43]) + (self[e45] * self[e31]) + (self[e31] * self[e45]) - (self[e41] * self[e35]) + (self[e43] * self[e15])),
                ((self[e25] * self[e41]) - (self[e15] * self[e42]) + (self[e45] * self[e12]) + (self[e12] * self[e45]) + (self[e41] * self[e25]) - (self[e42] * self[e15])),
                (-(self[e35] * self[e12]) - (self[e25] * self[e31]) - (self[e15] * self[e23]) - (self[e12] * self[e35]) - (self[e23] * self[e15]) - (self[e31] * self[e25])),
            ]),
        );
    }
}
impl Square for AntiDipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      141      150        0
    //    simd4       15       15        0
    // Totals...
    // yes simd      156      165        0
    //  no simd      201      210        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x4::from(self[e412]) * Simd32x4::from([self[e2], self[e415], self[e321], self[e125]]))
                - (Simd32x4::from(self[e4]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]))
                - (Simd32x4::from(self[e4]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e5]]))
                + (Simd32x4::from(self[e423]) * Simd32x4::from([self[e321], self[e3], self[e425], self[e235]]))
                + (Simd32x4::from(self[e431]) * Simd32x4::from([self[e435], self[e321], self[e1], self[e315]]))
                + Simd32x4::from([
                    (-(self[e3] * self[e431]) + (self[e4] * self[e1]) - (self[e321] * self[e423]) - (self[e435] * self[e431]) + (self[e425] * self[e412])
                        - (self[e415] * self[e4])
                        + (self[e412] * self[e2])
                        - (self[e412] * self[e425])
                        - (self[e431] * self[e3])),
                    (-(self[e1] * self[e412]) + (self[e4] * self[e2]) - (self[e321] * self[e431]) + (self[e435] * self[e423])
                        - (self[e425] * self[e4])
                        - (self[e415] * self[e412])
                        - (self[e412] * self[e1])
                        - (self[e423] * self[e435])
                        + (self[e423] * self[e3])),
                    (-(self[e2] * self[e423]) + (self[e4] * self[e3]) - (self[e321] * self[e412]) - (self[e435] * self[e4]) - (self[e425] * self[e423])
                        + (self[e415] * self[e431])
                        + (self[e431] * self[e1])
                        - (self[e431] * self[e415])
                        - (self[e423] * self[e2])),
                    (f32::powi(self[e3], 2) + f32::powi(self[e2], 2) + f32::powi(self[e1], 2) - f32::powi(self[e321], 2)
                        + f32::powi(self[e435], 2)
                        + f32::powi(self[e425], 2)
                        + f32::powi(self[e415], 2)
                        + (self[e412] * self[e125])
                        + (self[e423] * self[e235])
                        + (self[e431] * self[e315])),
                ])),
            // e23, e31, e12, e45
            (-(Simd32x4::from(self[e5]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
                - (Simd32x4::from(self[e2]) * Simd32x4::from([self[e3], self[e321], self[e1], self[e425]]))
                - (Simd32x4::from(self[e235]) * Simd32x4::from([self[e4], self[e412], self[e431], self[e423]]))
                - (Simd32x4::from(self[e125]) * Simd32x4::from([self[e431], self[e423], self[e4], self[e412]]))
                - (Simd32x4::from(self[e315]) * Simd32x4::from([self[e412], self[e4], self[e423], self[e431]]))
                + Simd32x4::from([
                    ((self[e2] * self[e3]) - (self[e1] * self[e321]) + (self[e315] * self[e412]) - (self[e235] * self[e4]) - (self[e321] * self[e1]) - (self[e435] * self[e425])
                        + (self[e425] * self[e435])
                        - (self[e423] * self[e5])
                        + (self[e431] * self[e125])),
                    ((self[e3] * self[e1]) - (self[e1] * self[e3]) + (self[e125] * self[e423]) - (self[e315] * self[e4]) - (self[e321] * self[e2]) + (self[e435] * self[e415])
                        - (self[e415] * self[e435])
                        + (self[e412] * self[e235])
                        - (self[e431] * self[e5])),
                    (-(self[e3] * self[e321]) + (self[e1] * self[e2]) - (self[e125] * self[e4]) + (self[e235] * self[e431]) - (self[e321] * self[e3]) - (self[e425] * self[e415])
                        + (self[e415] * self[e425])
                        - (self[e412] * self[e5])
                        + (self[e423] * self[e315])),
                    (-(self[e3] * self[e435]) - (self[e1] * self[e415])
                        + (self[e4] * self[e5])
                        + (self[e125] * self[e412])
                        + (self[e315] * self[e431])
                        + (self[e235] * self[e423])
                        - (self[e435] * self[e3])
                        - (self[e425] * self[e2])
                        - (self[e415] * self[e1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self[e5] * self[e1]) - (self[e5] * self[e415]) + (self[e3] * self[e315]) - (self[e2] * self[e125]) + (self[e1] * self[e5])
                    - (self[e125] * self[e2])
                    - (self[e125] * self[e425])
                    + (self[e315] * self[e3])
                    + (self[e315] * self[e435])
                    - (self[e435] * self[e315])
                    - (self[e415] * self[e5])
                    + (self[e425] * self[e125])),
                (-(self[e5] * self[e2]) - (self[e5] * self[e425]) - (self[e3] * self[e235])
                    + (self[e2] * self[e5])
                    + (self[e1] * self[e125])
                    + (self[e125] * self[e1])
                    + (self[e125] * self[e415])
                    - (self[e235] * self[e3])
                    - (self[e235] * self[e435])
                    + (self[e435] * self[e235])
                    - (self[e415] * self[e125])
                    - (self[e425] * self[e5])),
                (-(self[e5] * self[e3]) - (self[e5] * self[e435]) + (self[e3] * self[e5]) + (self[e2] * self[e235])
                    - (self[e1] * self[e315])
                    - (self[e315] * self[e1])
                    - (self[e315] * self[e415])
                    + (self[e235] * self[e2])
                    + (self[e235] * self[e425])
                    - (self[e435] * self[e5])
                    + (self[e415] * self[e315])
                    - (self[e425] * self[e235])),
                ((self[e3] * self[e412]) + (self[e2] * self[e431]) + (self[e1] * self[e423]) + (self[e435] * self[e412]) + (self[e425] * self[e431]) + (self[e415] * self[e423])
                    - (self[e412] * self[e3])
                    + (self[e412] * self[e435])
                    - (self[e431] * self[e2])
                    + (self[e431] * self[e425])
                    + (self[e423] * self[e415])
                    - (self[e423] * self[e1])),
            ]),
            // e4235, e4315, e4125, e3215
            (-(Simd32x4::from(self[e5]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]))
                + (Simd32x4::from(self[e425]) * Simd32x4::from([self[e3], self[e321], self[e1], self[e315]]))
                + (Simd32x4::from(self[e321]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e5]]))
                + (Simd32x4::from(self[e435]) * Simd32x4::from([self[e2], self[e1], self[e321], self[e125]]))
                + (Simd32x4::from(self[e415]) * Simd32x4::from([self[e321], self[e3], self[e2], self[e235]]))
                + Simd32x4::from([
                    (-(self[e2] * self[e435]) + (self[e4] * self[e235]) - (self[e125] * self[e431]) + (self[e315] * self[e412])
                        - (self[e235] * self[e4])
                        - (self[e425] * self[e3])
                        + (self[e412] * self[e315])
                        + (self[e423] * self[e5])
                        - (self[e431] * self[e125])),
                    (-(self[e3] * self[e415]) + (self[e4] * self[e315]) + (self[e125] * self[e423])
                        - (self[e315] * self[e4])
                        - (self[e235] * self[e412])
                        - (self[e435] * self[e1])
                        - (self[e412] * self[e235])
                        + (self[e423] * self[e125])
                        + (self[e431] * self[e5])),
                    (-(self[e1] * self[e425]) + (self[e4] * self[e125]) - (self[e125] * self[e4]) - (self[e315] * self[e423]) + (self[e235] * self[e431])
                        - (self[e415] * self[e2])
                        + (self[e412] * self[e5])
                        - (self[e423] * self[e315])
                        + (self[e431] * self[e235])),
                    (-(self[e3] * self[e125]) - (self[e2] * self[e315]) - (self[e1] * self[e235])
                        + (self[e125] * self[e3])
                        + (self[e315] * self[e2])
                        + (self[e235] * self[e1])
                        + (self[e435] * self[e125])
                        + (self[e415] * self[e235])
                        + (self[e425] * self[e315])),
                ])),
        );
    }
}
impl Square for AntiDualNum321 {
    type Output = AntiDualNum321;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        2        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiDualNum321::from_groups(/* e45, scalar */ Simd32x2::from([
            ((self[e45] * self[scalar]) + (self[scalar] * self[e45])),
            (f32::powi(self[e45], 2) + f32::powi(self[scalar], 2)),
        ]));
    }
}
impl Square for AntiDualNum4 {
    type Output = AntiDualNum4;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiDualNum4::from_groups(
            // e1234, scalar
            Simd32x2::from([((self[e1234] * self[scalar]) + (self[scalar] * self[e1234])), f32::powi(self[scalar], 2)]),
        );
    }
}
impl Square for AntiDualNum5 {
    type Output = AntiDualNum5;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiDualNum5::from_groups(
            // e3215, scalar
            Simd32x2::from([((self[e3215] * self[scalar]) + (self[scalar] * self[e3215])), f32::powi(self[scalar], 2)]),
        );
    }
}
impl Square for AntiFlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (f32::powi(self[e321], 2) * -1.0)]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                (-(self[e235] * self[e321]) + (self[e321] * self[e235])),
                (-(self[e315] * self[e321]) + (self[e321] * self[e315])),
                (-(self[e125] * self[e321]) + (self[e321] * self[e125])),
                0.0,
            ]),
        );
    }
}
impl Square for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       36        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (-(self[e3] * self[e2]) + (self[e2] * self[e3]) - (self[e321] * self[e1]) - (self[e1] * self[e321])),
                ((self[e3] * self[e1]) - (self[e2] * self[e321]) - (self[e321] * self[e2]) - (self[e1] * self[e3])),
                (-(self[e3] * self[e321]) - (self[e2] * self[e1]) - (self[e321] * self[e3]) + (self[e1] * self[e2])),
                (f32::powi(self[e3], 2) + f32::powi(self[e2], 2) - f32::powi(self[e321], 2) + f32::powi(self[e1], 2)),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                ((self[e3] * self[e315]) - (self[e2] * self[e125]) + (self[e321] * self[e235]) - (self[e125] * self[e2]) - (self[e235] * self[e321]) + (self[e315] * self[e3])),
                (-(self[e3] * self[e235]) + (self[e1] * self[e125]) + (self[e321] * self[e315]) + (self[e125] * self[e1]) - (self[e235] * self[e3]) - (self[e315] * self[e321])),
                ((self[e2] * self[e235]) - (self[e1] * self[e315]) + (self[e321] * self[e125]) - (self[e125] * self[e321]) + (self[e235] * self[e2]) - (self[e315] * self[e1])),
                (-(self[e3] * self[e125]) - (self[e2] * self[e315]) - (self[e1] * self[e235]) + (self[e125] * self[e3]) + (self[e235] * self[e1]) + (self[e315] * self[e2])),
            ]),
        );
    }
}
impl Square for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       19       24        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (-(self[e31] * self[e12]) + (self[e12] * self[e31])),
                ((self[e23] * self[e12]) - (self[e12] * self[e23])),
                (-(self[e23] * self[e31]) + (self[e31] * self[e23])),
                (-f32::powi(self[e12], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2)),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                ((self[e35] * self[e31]) - (self[e25] * self[e12]) - (self[e31] * self[e35]) + (self[e12] * self[e25])),
                (-(self[e35] * self[e23]) + (self[e15] * self[e12]) + (self[e23] * self[e35]) - (self[e12] * self[e15])),
                ((self[e25] * self[e23]) - (self[e15] * self[e31]) - (self[e23] * self[e25]) + (self[e31] * self[e15])),
                (-(self[e35] * self[e12]) - (self[e25] * self[e31]) - (self[e15] * self[e23]) - (self[e12] * self[e35]) - (self[e23] * self[e15]) - (self[e31] * self[e25])),
            ]),
        );
    }
}
impl Square for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       36        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       34       38        0
    //  no simd       40       44        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((self[scalar] * self[e23]) + (self[e12] * self[e31]) + (self[e23] * self[scalar]) - (self[e31] * self[e12])),
                ((self[scalar] * self[e31]) - (self[e12] * self[e23]) + (self[e23] * self[e12]) + (self[e31] * self[scalar])),
                ((self[scalar] * self[e12]) + (self[e12] * self[scalar]) - (self[e23] * self[e31]) + (self[e31] * self[e23])),
                (f32::powi(self[scalar], 2) - f32::powi(self[e12], 2) - f32::powi(self[e23], 2) - f32::powi(self[e31], 2)),
            ]),
            // e15, e25, e35, e3215
            ((Simd32x4::from(self[e3215]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
                + (Simd32x4::from(self[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
                + Simd32x4::from([
                    ((self[e35] * self[e31]) - (self[e25] * self[e12]) + (self[scalar] * self[e15]) + (self[e12] * self[e25]) + (self[e23] * self[e3215])
                        - (self[e31] * self[e35])),
                    (-(self[e35] * self[e23]) + (self[e15] * self[e12]) + (self[scalar] * self[e25]) - (self[e12] * self[e15])
                        + (self[e23] * self[e35])
                        + (self[e31] * self[e3215])),
                    ((self[e25] * self[e23]) - (self[e15] * self[e31]) + (self[scalar] * self[e35]) + (self[e12] * self[e3215]) - (self[e23] * self[e25])
                        + (self[e31] * self[e15])),
                    (-(self[e35] * self[e12]) - (self[e25] * self[e31]) - (self[e15] * self[e23]) - (self[e12] * self[e35]) - (self[e23] * self[e15]) - (self[e31] * self[e25])),
                ])),
        );
    }
}
impl Square for AntiPlane {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((self[e2] * self[e3]) - (self[e3] * self[e2])),
                (-(self[e1] * self[e3]) + (self[e3] * self[e1])),
                ((self[e1] * self[e2]) - (self[e2] * self[e1])),
                (f32::powi(self[e3], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2)),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                ((self[e1] * self[e5]) - (self[e5] * self[e1])),
                ((self[e2] * self[e5]) - (self[e5] * self[e2])),
                ((self[e3] * self[e5]) - (self[e5] * self[e3])),
                0.0,
            ]),
        );
    }
}
impl Square for AntiQuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       14        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            ((self[scalar] * self[e1234]) - (self[e45] * self[e1234]) + (self[e1234] * self[e45]) + (self[e1234] * self[scalar])),
            ((self[scalar] * self[e3215]) + (self[e45] * self[e3215]) - (self[e3215] * self[e45]) + (self[e3215] * self[scalar])),
            ((self[scalar] * self[e45]) + (self[e45] * self[scalar]) - (self[e1234] * self[e3215]) + (self[e3215] * self[e1234])),
            (f32::powi(self[scalar], 2) + f32::powi(self[e45], 2) + (self[e1234] * self[e3215]) + (self[e3215] * self[e1234])),
        ]));
    }
}
impl Square for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (f32::powi(self[e12345], 2) * -1.0));
    }
}
impl Square for AntiTripleNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        8        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            ((self[e1234] * self[scalar]) + (self[scalar] * self[e1234])),
            ((self[e3215] * self[scalar]) + (self[scalar] * self[e3215])),
            (-(self[e1234] * self[e3215]) + (self[e3215] * self[e1234])),
            (f32::powi(self[scalar], 2) + (self[e1234] * self[e3215]) + (self[e3215] * self[e1234])),
        ]));
    }
}
impl Square for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       84        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       75       87        0
    //  no simd       84       96        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x4::from(self[e412]) * Simd32x4::from([self[e425], self[e415], self[e321], self[e125]]))
                + (Simd32x4::from(self[e423]) * Simd32x4::from([self[e321], self[e435], self[e425], self[e235]]))
                + (Simd32x4::from(self[e431]) * Simd32x4::from([self[e435], self[e321], self[e415], self[e315]]))
                + Simd32x4::from([
                    (-(self[e321] * self[e423]) - (self[e435] * self[e431]) - (self[e412] * self[e425])),
                    (-(self[e321] * self[e431]) - (self[e415] * self[e412]) - (self[e423] * self[e435])),
                    (-(self[e321] * self[e412]) - (self[e425] * self[e423]) - (self[e431] * self[e415])),
                    (-f32::powi(self[e321], 2)
                        + f32::powi(self[e435], 2)
                        + f32::powi(self[e425], 2)
                        + f32::powi(self[e415], 2)
                        + (self[e412] * self[e125])
                        + (self[e423] * self[e235])
                        + (self[e431] * self[e315])),
                ])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-(self[e125] * self[e431]) + (self[e315] * self[e412]) - (self[e435] * self[e425]) + (self[e425] * self[e435]) + (self[e431] * self[e125])
                    - (self[e412] * self[e315])),
                ((self[e125] * self[e423]) - (self[e235] * self[e412]) + (self[e435] * self[e415]) - (self[e415] * self[e435]) - (self[e423] * self[e125])
                    + (self[e412] * self[e235])),
                (-(self[e315] * self[e423]) + (self[e235] * self[e431]) - (self[e425] * self[e415]) + (self[e415] * self[e425]) + (self[e423] * self[e315])
                    - (self[e431] * self[e235])),
                ((self[e125] * self[e412]) + (self[e315] * self[e431]) + (self[e235] * self[e423])
                    - (self[e412] * self[e125])
                    - (self[e423] * self[e235])
                    - (self[e431] * self[e315])),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self[e125] * self[e425]) + (self[e315] * self[e435]) - (self[e235] * self[e321]) + (self[e321] * self[e235]) + (self[e425] * self[e125])
                    - (self[e435] * self[e315])),
                ((self[e125] * self[e415]) - (self[e315] * self[e321]) - (self[e235] * self[e435]) + (self[e321] * self[e315]) - (self[e415] * self[e125])
                    + (self[e435] * self[e235])),
                (-(self[e125] * self[e321]) - (self[e315] * self[e415]) + (self[e235] * self[e425]) + (self[e321] * self[e125]) + (self[e415] * self[e315])
                    - (self[e425] * self[e235])),
                ((self[e435] * self[e412])
                    + (self[e425] * self[e431])
                    + (self[e415] * self[e423])
                    + (self[e412] * self[e435])
                    + (self[e423] * self[e415])
                    + (self[e431] * self[e425])),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-(self[e125] * self[e431]) + (self[e315] * self[e412]) + (self[e321] * self[e415]) + (self[e415] * self[e321]) - (self[e431] * self[e125])
                    + (self[e412] * self[e315])),
                ((self[e125] * self[e423]) - (self[e235] * self[e412]) + (self[e321] * self[e425]) + (self[e425] * self[e321]) + (self[e423] * self[e125])
                    - (self[e412] * self[e235])),
                (-(self[e315] * self[e423]) + (self[e235] * self[e431]) + (self[e321] * self[e435]) + (self[e435] * self[e321]) - (self[e423] * self[e315])
                    + (self[e431] * self[e235])),
                ((self[e125] * self[e435])
                    + (self[e315] * self[e425])
                    + (self[e235] * self[e415])
                    + (self[e435] * self[e125])
                    + (self[e415] * self[e235])
                    + (self[e425] * self[e315])),
            ]),
        );
    }
}
impl Square for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       93      104        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       96      107        0
    //  no simd      105      116        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x4::from(self[e412]) * Simd32x4::from([self[e425], self[e415], self[e321], self[e125]]))
                + (Simd32x4::from(self[e431]) * Simd32x4::from([self[e435], self[e321], self[e415], self[e315]]))
                + (Simd32x4::from(self[e423]) * Simd32x4::from([self[e321], self[e435], self[e425], self[e235]]))
                + Simd32x4::from([
                    (-(self[e12345] * self[e423]) - (self[e321] * self[e423]) - (self[e435] * self[e431]) - (self[e412] * self[e425]) - (self[e423] * self[e12345])),
                    (-(self[e12345] * self[e431]) - (self[e321] * self[e431]) - (self[e415] * self[e412]) - (self[e431] * self[e12345]) - (self[e423] * self[e435])),
                    (-(self[e12345] * self[e412]) - (self[e321] * self[e412]) - (self[e425] * self[e423]) - (self[e412] * self[e12345]) - (self[e431] * self[e415])),
                    (-f32::powi(self[e12345], 2) - f32::powi(self[e321], 2)
                        + f32::powi(self[e435], 2)
                        + f32::powi(self[e425], 2)
                        + f32::powi(self[e415], 2)
                        + (self[e412] * self[e125])
                        + (self[e423] * self[e235])
                        + (self[e431] * self[e315])),
                ])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-(self[e12345] * self[e415]) - (self[e125] * self[e431]) + (self[e315] * self[e412]) - (self[e435] * self[e425]) + (self[e425] * self[e435])
                    - (self[e415] * self[e12345])
                    + (self[e431] * self[e125])
                    - (self[e412] * self[e315])),
                (-(self[e12345] * self[e425]) + (self[e125] * self[e423]) - (self[e235] * self[e412]) + (self[e435] * self[e415])
                    - (self[e425] * self[e12345])
                    - (self[e415] * self[e435])
                    - (self[e423] * self[e125])
                    + (self[e412] * self[e235])),
                (-(self[e12345] * self[e435]) - (self[e315] * self[e423]) + (self[e235] * self[e431]) - (self[e435] * self[e12345]) - (self[e425] * self[e415])
                    + (self[e415] * self[e425])
                    + (self[e423] * self[e315])
                    - (self[e431] * self[e235])),
                ((self[e12345] * self[e321]) + (self[e125] * self[e412]) + (self[e315] * self[e431]) + (self[e235] * self[e423]) + (self[e321] * self[e12345])
                    - (self[e412] * self[e125])
                    - (self[e423] * self[e235])
                    - (self[e431] * self[e315])),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self[e12345] * self[e235]) - (self[e125] * self[e425]) + (self[e315] * self[e435]) - (self[e235] * self[e12345]) - (self[e235] * self[e321])
                    + (self[e321] * self[e235])
                    + (self[e425] * self[e125])
                    - (self[e435] * self[e315])),
                (-(self[e12345] * self[e315]) + (self[e125] * self[e415]) - (self[e315] * self[e12345]) - (self[e315] * self[e321]) - (self[e235] * self[e435])
                    + (self[e321] * self[e315])
                    - (self[e415] * self[e125])
                    + (self[e435] * self[e235])),
                (-(self[e12345] * self[e125]) - (self[e125] * self[e12345]) - (self[e125] * self[e321]) - (self[e315] * self[e415])
                    + (self[e235] * self[e425])
                    + (self[e321] * self[e125])
                    + (self[e415] * self[e315])
                    - (self[e425] * self[e235])),
                ((self[e435] * self[e412])
                    + (self[e425] * self[e431])
                    + (self[e415] * self[e423])
                    + (self[e412] * self[e435])
                    + (self[e423] * self[e415])
                    + (self[e431] * self[e425])),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (-(self[e125] * self[e431]) + (self[e315] * self[e412]) + (self[e321] * self[e415]) + (self[e415] * self[e321]) - (self[e431] * self[e125])
                    + (self[e412] * self[e315])),
                ((self[e125] * self[e423]) - (self[e235] * self[e412]) + (self[e321] * self[e425]) + (self[e425] * self[e321]) + (self[e423] * self[e125])
                    - (self[e412] * self[e235])),
                (-(self[e315] * self[e423]) + (self[e235] * self[e431]) + (self[e321] * self[e435]) + (self[e435] * self[e321]) - (self[e423] * self[e315])
                    + (self[e431] * self[e235])),
                ((self[e125] * self[e435])
                    + (self[e315] * self[e425])
                    + (self[e235] * self[e415])
                    + (self[e435] * self[e125])
                    + (self[e415] * self[e235])
                    + (self[e425] * self[e315])),
            ]),
        );
    }
}
impl Square for Dipole {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       84        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       75       87        0
    //  no simd       84       96        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-(Simd32x4::from(self[e41]) * Simd32x4::from([self[e45], self[e12], self[e31], self[e15]]))
                - (Simd32x4::from(self[e43]) * Simd32x4::from([self[e31], self[e23], self[e45], self[e35]]))
                - (Simd32x4::from(self[e42]) * Simd32x4::from([self[e12], self[e45], self[e23], self[e25]]))
                + Simd32x4::from([
                    ((self[e12] * self[e42]) + (self[e43] * self[e31]) + (self[e41] * self[e45])),
                    ((self[e23] * self[e43]) + (self[e41] * self[e12]) + (self[e42] * self[e45])),
                    ((self[e31] * self[e41]) + (self[e43] * self[e45]) + (self[e42] * self[e23])),
                    (f32::powi(self[e45], 2)
                        - f32::powi(self[e12], 2)
                        - f32::powi(self[e31], 2)
                        - f32::powi(self[e23], 2)
                        - (self[e43] * self[e35])
                        - (self[e41] * self[e15])
                        - (self[e42] * self[e25])),
                ])),
            // e23, e31, e12, e45
            Simd32x4::from([
                ((self[e35] * self[e42]) - (self[e25] * self[e43]) + (self[e12] * self[e31]) - (self[e31] * self[e12]) - (self[e42] * self[e35]) + (self[e43] * self[e25])),
                (-(self[e35] * self[e41]) + (self[e15] * self[e43]) - (self[e12] * self[e23]) + (self[e23] * self[e12]) + (self[e41] * self[e35]) - (self[e43] * self[e15])),
                ((self[e25] * self[e41]) - (self[e15] * self[e42]) + (self[e31] * self[e23]) - (self[e23] * self[e31]) - (self[e41] * self[e25]) + (self[e42] * self[e15])),
                (-(self[e35] * self[e43]) - (self[e25] * self[e42]) - (self[e15] * self[e41]) + (self[e43] * self[e35]) + (self[e41] * self[e15]) + (self[e42] * self[e25])),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self[e35] * self[e31]) - (self[e25] * self[e12]) - (self[e15] * self[e45]) + (self[e45] * self[e15]) - (self[e31] * self[e35]) + (self[e12] * self[e25])),
                (-(self[e35] * self[e23]) - (self[e25] * self[e45]) + (self[e15] * self[e12]) + (self[e45] * self[e25]) + (self[e23] * self[e35]) - (self[e12] * self[e15])),
                (-(self[e35] * self[e45]) + (self[e25] * self[e23]) - (self[e15] * self[e31]) + (self[e45] * self[e35]) - (self[e23] * self[e25]) + (self[e31] * self[e15])),
                (-(self[e12] * self[e43]) - (self[e31] * self[e42]) - (self[e23] * self[e41]) - (self[e43] * self[e12]) - (self[e41] * self[e23]) - (self[e42] * self[e31])),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                ((self[e35] * self[e42]) - (self[e25] * self[e43]) + (self[e45] * self[e23]) + (self[e23] * self[e45]) + (self[e42] * self[e35]) - (self[e43] * self[e25])),
                (-(self[e35] * self[e41]) + (self[e15] * self[e43]) + (self[e45] * self[e31]) + (self[e31] * self[e45]) - (self[e41] * self[e35]) + (self[e43] * self[e15])),
                ((self[e25] * self[e41]) - (self[e15] * self[e42]) + (self[e45] * self[e12]) + (self[e12] * self[e45]) + (self[e41] * self[e25]) - (self[e42] * self[e15])),
                (-(self[e35] * self[e12]) - (self[e25] * self[e31]) - (self[e15] * self[e23]) - (self[e12] * self[e35]) - (self[e23] * self[e15]) - (self[e31] * self[e25])),
            ]),
        );
    }
}
impl Square for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      157      166        0
    //    simd4        9        9        0
    // Totals...
    // yes simd      166      175        0
    //  no simd      193      202        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-(Simd32x4::from(self[e42]) * Simd32x4::from([self[e4125], self[e45], self[e23], self[e25]]))
                + (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]))
                + (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e3215]]))
                - (Simd32x4::from(self[e41]) * Simd32x4::from([self[e45], self[e12], self[e4315], self[e15]]))
                - (Simd32x4::from(self[e43]) * Simd32x4::from([self[e31], self[e4235], self[e45], self[e35]]))
                + Simd32x4::from([
                    ((self[e4315] * self[e43]) - (self[e4235] * self[e1234])
                        + (self[e12] * self[e42])
                        + (self[e23] * self[e1234])
                        + (self[e43] * self[e4315])
                        + (self[e43] * self[e31])
                        - (self[e42] * self[e4125])
                        + (self[e41] * self[e45])
                        - (self[e42] * self[e12])),
                    ((self[e4125] * self[e41]) - (self[e4315] * self[e1234]) + (self[e31] * self[e1234]) + (self[e23] * self[e43])
                        - (self[e43] * self[e4235])
                        - (self[e43] * self[e23])
                        + (self[e42] * self[e45])
                        + (self[e41] * self[e12])
                        + (self[e41] * self[e4125])),
                    (-(self[e4125] * self[e1234])
                        + (self[e4235] * self[e42])
                        + (self[e12] * self[e1234])
                        + (self[e31] * self[e41])
                        + (self[e43] * self[e45])
                        + (self[e42] * self[e4235])
                        + (self[e42] * self[e23])
                        - (self[e41] * self[e31])
                        - (self[e41] * self[e4315])),
                    (-f32::powi(self[e4125], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4235], 2) + f32::powi(self[e45], 2)
                        - f32::powi(self[e12], 2)
                        - f32::powi(self[e31], 2)
                        - f32::powi(self[e23], 2)
                        - (self[e43] * self[e35])
                        - (self[e41] * self[e15])
                        - (self[e42] * self[e25])),
                ])),
            // e23, e31, e12, e45
            ((Simd32x4::from(self[e3215]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]))
                + (Simd32x4::from(self[e15]) * Simd32x4::from([self[e1234], self[e43], self[e42], self[e41]]))
                + (Simd32x4::from(self[e35]) * Simd32x4::from([self[e42], self[e41], self[e1234], self[e43]]))
                + (Simd32x4::from(self[e25]) * Simd32x4::from([self[e43], self[e1234], self[e41], self[e42]]))
                + Simd32x4::from([
                    ((self[e4125] * self[e4315]) - (self[e4315] * self[e4125]) + (self[e4235] * self[e45]) - (self[e25] * self[e43])
                        + (self[e15] * self[e1234])
                        + (self[e45] * self[e4235])
                        + (self[e12] * self[e31])
                        - (self[e31] * self[e12])
                        + (self[e41] * self[e3215])
                        - (self[e42] * self[e35])),
                    (-(self[e4125] * self[e4235]) + (self[e4315] * self[e45]) + (self[e4235] * self[e4125]) - (self[e35] * self[e41])
                        + (self[e25] * self[e1234])
                        + (self[e45] * self[e4315])
                        - (self[e12] * self[e23])
                        + (self[e23] * self[e12])
                        - (self[e43] * self[e15])
                        + (self[e42] * self[e3215])),
                    ((self[e4125] * self[e45]) + (self[e4315] * self[e4235]) - (self[e4235] * self[e4315]) + (self[e35] * self[e1234]) - (self[e15] * self[e42])
                        + (self[e45] * self[e4125])
                        + (self[e31] * self[e23])
                        - (self[e23] * self[e31])
                        + (self[e43] * self[e3215])
                        - (self[e41] * self[e25])),
                    (-(self[e4125] * self[e12])
                        - (self[e4315] * self[e31])
                        - (self[e4235] * self[e23])
                        - (self[e1234] * self[e3215])
                        - (self[e35] * self[e43])
                        - (self[e25] * self[e42])
                        - (self[e15] * self[e41])
                        - (self[e12] * self[e4125])
                        - (self[e31] * self[e4315])
                        - (self[e23] * self[e4235])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self[e3215] * self[e4235]) + (self[e3215] * self[e23]) + (self[e4125] * self[e25]) - (self[e4315] * self[e35]) + (self[e4235] * self[e3215])
                    - (self[e35] * self[e4315])
                    + (self[e35] * self[e31])
                    + (self[e25] * self[e4125])
                    - (self[e25] * self[e12])
                    + (self[e12] * self[e25])
                    + (self[e23] * self[e3215])
                    - (self[e31] * self[e35])),
                (-(self[e3215] * self[e4315]) + (self[e3215] * self[e31]) - (self[e4125] * self[e15])
                    + (self[e4315] * self[e3215])
                    + (self[e4235] * self[e35])
                    + (self[e35] * self[e4235])
                    - (self[e35] * self[e23])
                    - (self[e15] * self[e4125])
                    + (self[e15] * self[e12])
                    - (self[e12] * self[e15])
                    + (self[e23] * self[e35])
                    + (self[e31] * self[e3215])),
                (-(self[e3215] * self[e4125]) + (self[e3215] * self[e12]) + (self[e4125] * self[e3215]) + (self[e4315] * self[e15])
                    - (self[e4235] * self[e25])
                    - (self[e25] * self[e4235])
                    + (self[e25] * self[e23])
                    + (self[e15] * self[e4315])
                    - (self[e15] * self[e31])
                    + (self[e12] * self[e3215])
                    - (self[e23] * self[e25])
                    + (self[e31] * self[e15])),
                ((self[e4125] * self[e43]) + (self[e4315] * self[e42]) + (self[e4235] * self[e41])
                    - (self[e12] * self[e43])
                    - (self[e31] * self[e42])
                    - (self[e23] * self[e41])
                    - (self[e43] * self[e4125])
                    - (self[e43] * self[e12])
                    - (self[e42] * self[e4315])
                    - (self[e42] * self[e31])
                    - (self[e41] * self[e23])
                    - (self[e41] * self[e4235])),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                ((self[e4125] * self[e31]) - (self[e4315] * self[e12]) - (self[e1234] * self[e15]) + (self[e35] * self[e42]) - (self[e25] * self[e43])
                    + (self[e15] * self[e1234])
                    + (self[e45] * self[e23])
                    + (self[e12] * self[e4315])
                    - (self[e31] * self[e4125])
                    + (self[e23] * self[e45])
                    - (self[e43] * self[e25])
                    + (self[e42] * self[e35])),
                (-(self[e4125] * self[e23]) + (self[e4235] * self[e12]) - (self[e1234] * self[e25]) - (self[e35] * self[e41])
                    + (self[e25] * self[e1234])
                    + (self[e15] * self[e43])
                    + (self[e45] * self[e31])
                    - (self[e12] * self[e4235])
                    + (self[e31] * self[e45])
                    + (self[e23] * self[e4125])
                    + (self[e43] * self[e15])
                    - (self[e41] * self[e35])),
                ((self[e4315] * self[e23]) - (self[e4235] * self[e31]) - (self[e1234] * self[e35]) + (self[e35] * self[e1234]) + (self[e25] * self[e41]) - (self[e15] * self[e42])
                    + (self[e45] * self[e12])
                    + (self[e12] * self[e45])
                    + (self[e31] * self[e4235])
                    - (self[e23] * self[e4315])
                    + (self[e41] * self[e25])
                    - (self[e42] * self[e15])),
                (-(self[e4125] * self[e35]) - (self[e4315] * self[e25]) - (self[e4235] * self[e15]) + (self[e35] * self[e4125]) - (self[e35] * self[e12])
                    + (self[e25] * self[e4315])
                    - (self[e25] * self[e31])
                    + (self[e15] * self[e4235])
                    - (self[e15] * self[e23])
                    - (self[e12] * self[e35])
                    - (self[e23] * self[e15])
                    - (self[e31] * self[e25])),
            ]),
        );
    }
}
impl Square for DualNum321 {
    type Output = AntiDualNum321;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        2        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiDualNum321::from_groups(/* e45, scalar */ Simd32x2::from([
            ((self[e321] * self[e12345]) + (self[e12345] * self[e321])),
            (-f32::powi(self[e321], 2) - f32::powi(self[e12345], 2)),
        ]));
    }
}
impl Square for DualNum4 {
    type Output = AntiDualNum4;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiDualNum4::from_groups(
            // e1234, scalar
            Simd32x2::from([(-(self[e4] * self[e12345]) - (self[e12345] * self[e4])), (f32::powi(self[e12345], 2) * -1.0)]),
        );
    }
}
impl Square for DualNum5 {
    type Output = AntiDualNum5;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiDualNum5::from_groups(
            // e3215, scalar
            Simd32x2::from([(-(self[e5] * self[e12345]) - (self[e12345] * self[e5])), (f32::powi(self[e12345], 2) * -1.0)]),
        );
    }
}
impl Square for FlatPoint {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, f32::powi(self[e45], 2)]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                (-(self[e15] * self[e45]) + (self[e45] * self[e15])),
                (-(self[e25] * self[e45]) + (self[e45] * self[e25])),
                (-(self[e35] * self[e45]) + (self[e45] * self[e35])),
                0.0,
            ]),
        );
    }
}
impl Square for Flector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       36        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((self[e4125] * self[e4315]) - (self[e4315] * self[e4125]) + (self[e45] * self[e4235]) + (self[e4235] * self[e45])),
                (-(self[e4125] * self[e4235]) + (self[e4315] * self[e45]) + (self[e45] * self[e4315]) + (self[e4235] * self[e4125])),
                ((self[e4125] * self[e45]) + (self[e4315] * self[e4235]) + (self[e45] * self[e4125]) - (self[e4235] * self[e4315])),
                (-f32::powi(self[e4125], 2) - f32::powi(self[e4315], 2) + f32::powi(self[e45], 2) - f32::powi(self[e4235], 2)),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                ((self[e4125] * self[e25]) - (self[e4315] * self[e35]) + (self[e45] * self[e15]) - (self[e35] * self[e4315]) - (self[e15] * self[e45]) + (self[e25] * self[e4125])),
                (-(self[e4125] * self[e15]) + (self[e4235] * self[e35]) + (self[e45] * self[e25]) + (self[e35] * self[e4235])
                    - (self[e15] * self[e4125])
                    - (self[e25] * self[e45])),
                ((self[e4315] * self[e15]) - (self[e4235] * self[e25]) + (self[e45] * self[e35]) - (self[e35] * self[e45]) + (self[e15] * self[e4315]) - (self[e25] * self[e4235])),
                (-(self[e4125] * self[e35]) - (self[e4315] * self[e25]) - (self[e4235] * self[e15])
                    + (self[e35] * self[e4125])
                    + (self[e15] * self[e4235])
                    + (self[e25] * self[e4315])),
            ]),
        );
    }
}
impl Square for Line {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       19       24        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((self[e425] * self[e435]) - (self[e435] * self[e425])),
                (-(self[e415] * self[e435]) + (self[e435] * self[e415])),
                ((self[e415] * self[e425]) - (self[e425] * self[e415])),
                (f32::powi(self[e435], 2) + f32::powi(self[e415], 2) + f32::powi(self[e425], 2)),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                (-(self[e125] * self[e425]) + (self[e315] * self[e435]) + (self[e425] * self[e125]) - (self[e435] * self[e315])),
                ((self[e125] * self[e415]) - (self[e235] * self[e435]) - (self[e415] * self[e125]) + (self[e435] * self[e235])),
                (-(self[e315] * self[e415]) + (self[e235] * self[e425]) + (self[e415] * self[e315]) - (self[e425] * self[e235])),
                ((self[e125] * self[e435])
                    + (self[e315] * self[e425])
                    + (self[e235] * self[e415])
                    + (self[e435] * self[e125])
                    + (self[e415] * self[e235])
                    + (self[e425] * self[e315])),
            ]),
        );
    }
}
impl Square for Motor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       36        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       34       38        0
    //  no simd       40       44        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (-(self[e12345] * self[e415]) - (self[e435] * self[e425]) - (self[e415] * self[e12345]) + (self[e425] * self[e435])),
                (-(self[e12345] * self[e425]) + (self[e435] * self[e415]) - (self[e415] * self[e435]) - (self[e425] * self[e12345])),
                (-(self[e12345] * self[e435]) - (self[e435] * self[e12345]) + (self[e415] * self[e425]) - (self[e425] * self[e415])),
                (-f32::powi(self[e12345], 2) + f32::powi(self[e435], 2) + f32::powi(self[e415], 2) + f32::powi(self[e425], 2)),
            ]),
            // e15, e25, e35, e3215
            (-(Simd32x4::from(self[e5]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]))
                - (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]))
                + Simd32x4::from([
                    (-(self[e125] * self[e425]) + (self[e315] * self[e435]) - (self[e12345] * self[e235]) - (self[e435] * self[e315]) - (self[e415] * self[e5])
                        + (self[e425] * self[e125])),
                    ((self[e125] * self[e415]) - (self[e235] * self[e435]) - (self[e12345] * self[e315]) + (self[e435] * self[e235])
                        - (self[e415] * self[e125])
                        - (self[e425] * self[e5])),
                    (-(self[e315] * self[e415]) + (self[e235] * self[e425]) - (self[e12345] * self[e125]) - (self[e435] * self[e5]) + (self[e415] * self[e315])
                        - (self[e425] * self[e235])),
                    ((self[e125] * self[e435])
                        + (self[e315] * self[e425])
                        + (self[e235] * self[e415])
                        + (self[e435] * self[e125])
                        + (self[e415] * self[e235])
                        + (self[e425] * self[e315])),
                ])),
        );
    }
}
impl Square for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      481      497        0
    //    simd2        8        8        0
    //    simd3       69       79        0
    //    simd4       52       53        0
    // Totals...
    // yes simd      610      637        0
    //  no simd      912      962        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            ((Simd32x2::from(self[e1234]) * Simd32x2::from([self[e3215], self[e5]])) + (Simd32x2::from(self[e3215]) * Simd32x2::from([self[e1234], self[e4]]))
                - (Simd32x2::from(self[e43]) * Simd32x2::from([self[e35], self[e125]]))
                - (Simd32x2::from(self[e42]) * Simd32x2::from([self[e25], self[e315]]))
                - (Simd32x2::from(self[e41]) * Simd32x2::from([self[e15], self[e235]]))
                - (Simd32x2::from(self[e35]) * Simd32x2::from([self[e43], self[e412]]))
                - (Simd32x2::from(self[e25]) * Simd32x2::from([self[e42], self[e431]]))
                - (Simd32x2::from(self[e15]) * Simd32x2::from([self[e41], self[e423]]))
                + Simd32x2::from([
                    (-f32::powi(self[e4125], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4235], 2)
                        + (self[e125] * self[e412])
                        + (self[e315] * self[e431])
                        + (self[e235] * self[e423])
                        + (self[e412] * self[e125])
                        + (self[e431] * self[e315])
                        + (self[e423] * self[e235])
                        - f32::powi(self[e321], 2)
                        + f32::powi(self[e435], 2)
                        + f32::powi(self[e425], 2)
                        + f32::powi(self[e415], 2)
                        - f32::powi(self[e12], 2)
                        - f32::powi(self[e31], 2)
                        - f32::powi(self[e23], 2)
                        + f32::powi(self[e45], 2)
                        - (self[e5] * self[e4])
                        - (self[e4] * self[e5])
                        + f32::powi(self[e3], 2)
                        + f32::powi(self[e2], 2)
                        + f32::powi(self[e1], 2)
                        + f32::powi(self[scalar], 2)
                        - f32::powi(self[e12345], 2)),
                    ((self[e4125] * self[e3]) + (self[e4315] * self[e2]) + (self[e4235] * self[e1])
                        - (self[e321] * self[e45])
                        - (self[e435] * self[e12])
                        - (self[e425] * self[e31])
                        - (self[e415] * self[e23])
                        - (self[e12] * self[e435])
                        - (self[e31] * self[e425])
                        - (self[e23] * self[e415])
                        - (self[e43] * self[e125])
                        - (self[e42] * self[e315])
                        - (self[e41] * self[e235])
                        - (self[e45] * self[e321])
                        - (self[e35] * self[e412])
                        - (self[e25] * self[e431])
                        - (self[e15] * self[e423])
                        + (self[e5] * self[e1234])
                        + (self[e4] * self[e3215])
                        + (self[e3] * self[e4125])
                        + (self[e2] * self[e4315])
                        + (self[e1] * self[e4235])
                        + (self[scalar] * self[e12345])
                        + (self[e12345] * self[scalar])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self[e1234]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e12345]]))
                - (Simd32x4::from(self[e425]) * Simd32x4::from([self[e4125], self[e45], self[e4235], self[e42]]))
                - (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e321]]))
                + (Simd32x4::from(self[e321]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e1234]]))
                - (Simd32x4::from(self[e4315]) * Simd32x4::from([self[e435], self[e12345], self[e415], self[e431]]))
                - (Simd32x4::from(self[e45]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e4]]))
                + (Simd32x4::from(self[e2]) * Simd32x4::from([self[e12], self[scalar], self[e23], self[e42]]))
                + (Simd32x4::from(self[e4]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + (Simd32x4::from(self[e3]) * Simd32x4::from([self[e31], self[e23], self[scalar], self[e43]]))
                + (Simd32x4::from(self[e1]) * Simd32x4::from([self[scalar], self[e12], self[e31], self[e41]]))
                + (Simd32x4::from(self[scalar]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]))
                + Simd32x4::from([
                    (-(self[e3215] * self[e423]) + (self[e4315] * self[e435]) - (self[e4235] * self[e12345]) - (self[e125] * self[e42])
                        + (self[e315] * self[e43])
                        + (self[e412] * self[e25])
                        - (self[e431] * self[e35])
                        + (self[e423] * self[e3215])
                        + (self[e425] * self[e4125])
                        - (self[e31] * self[e3])
                        + (self[e23] * self[e321])
                        + (self[e43] * self[e315])
                        - (self[e42] * self[e125])
                        + (self[e41] * self[e5])
                        - (self[e45] * self[e415])
                        - (self[e35] * self[e431])
                        + (self[e25] * self[e412])
                        - (self[e15] * self[e4])
                        - (self[e5] * self[e41])
                        - (self[e2] * self[e12])
                        - (self[e12345] * self[e4235])),
                    (-(self[e3215] * self[e431]) + (self[e4125] * self[e415]) - (self[e4235] * self[e435]) + (self[e125] * self[e41])
                        - (self[e235] * self[e43])
                        - (self[e412] * self[e15])
                        + (self[e431] * self[e3215])
                        + (self[e423] * self[e35])
                        + (self[e435] * self[e4235])
                        - (self[e415] * self[e4125])
                        - (self[e12] * self[e1])
                        + (self[e31] * self[e321])
                        - (self[e43] * self[e235])
                        + (self[e42] * self[e5])
                        + (self[e41] * self[e125])
                        + (self[e35] * self[e423])
                        - (self[e25] * self[e4])
                        - (self[e15] * self[e412])
                        - (self[e5] * self[e42])
                        - (self[e3] * self[e23])
                        - (self[e12345] * self[e4315])),
                    (-(self[e3215] * self[e412]) - (self[e4125] * self[e12345]) + (self[e4235] * self[e425]) - (self[e315] * self[e41])
                        + (self[e235] * self[e42])
                        + (self[e412] * self[e3215])
                        + (self[e431] * self[e15])
                        - (self[e423] * self[e25])
                        + (self[e415] * self[e4315])
                        + (self[e12] * self[e321])
                        - (self[e23] * self[e2])
                        + (self[e43] * self[e5])
                        + (self[e42] * self[e235])
                        - (self[e41] * self[e315])
                        - (self[e45] * self[e435])
                        - (self[e35] * self[e4])
                        - (self[e25] * self[e423])
                        + (self[e15] * self[e431])
                        - (self[e5] * self[e43])
                        - (self[e1] * self[e31])
                        - (self[e12345] * self[e4125])),
                    ((self[e4125] * self[e412]) + (self[e4315] * self[e431]) + (self[e4235] * self[e423])
                        - (self[e412] * self[e4125])
                        - (self[e412] * self[e12])
                        - (self[e431] * self[e31])
                        - (self[e423] * self[e4235])
                        - (self[e423] * self[e23])
                        - (self[e435] * self[e43])
                        - (self[e415] * self[e41])
                        - (self[e12] * self[e412])
                        - (self[e31] * self[e431])
                        - (self[e23] * self[e423])
                        - (self[e43] * self[e435])
                        - (self[e42] * self[e425])
                        - (self[e41] * self[e415])
                        - (self[e3] * self[e43])
                        - (self[e2] * self[e42])
                        - (self[e1] * self[e41])
                        + (self[scalar] * self[e4])
                        + (self[e12345] * self[e1234])),
                ])),
            // e5
            ((self[e3215] * self[e321]) + (self[e3215] * self[e12345]) - (self[e4125] * self[e125]) - (self[e4315] * self[e315]) - (self[e4235] * self[e235])
                + (self[e125] * self[e4125])
                - (self[e125] * self[e12])
                + (self[e315] * self[e4315])
                - (self[e315] * self[e31])
                + (self[e235] * self[e4235])
                - (self[e235] * self[e23])
                - (self[e321] * self[e3215])
                - (self[e435] * self[e35])
                - (self[e425] * self[e25])
                - (self[e415] * self[e15])
                - (self[e12] * self[e125])
                - (self[e31] * self[e315])
                - (self[e23] * self[e235])
                + (self[e45] * self[e5])
                - (self[e35] * self[e435])
                - (self[e35] * self[e3])
                - (self[e25] * self[e425])
                - (self[e25] * self[e2])
                - (self[e15] * self[e415])
                - (self[e15] * self[e1])
                - (self[e5] * self[e45])
                + (self[e5] * self[scalar])
                + (self[e3] * self[e35])
                + (self[e2] * self[e25])
                + (self[e1] * self[e15])
                + (self[scalar] * self[e5])
                + (self[e12345] * self[e3215])),
            // e15, e25, e35, e45
            (-(Simd32x4::from(self[e3215]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e1234]]))
                + (Simd32x4::from(self[e3215]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e1234]]))
                + (Simd32x4::from(self[e25]) * Simd32x4::from([self[e4125], self[e45], self[e23], self[e42]]))
                - (Simd32x4::from(self[e35]) * Simd32x4::from([self[e4315], self[e23], self[e45], self[e43]]))
                - (Simd32x4::from(self[e425]) * Simd32x4::from([self[e125], self[e5], self[e235], self[e2]]))
                - (Simd32x4::from(self[e125]) * Simd32x4::from([self[e2], self[e415], self[e321], self[e412]]))
                + (Simd32x4::from(self[e235]) * Simd32x4::from([self[e321], self[e435], self[e425], self[e423]]))
                + (Simd32x4::from(self[e125]) * Simd32x4::from([self[e425], self[e415], self[e321], self[e412]]))
                - (Simd32x4::from(self[e5]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e4]]))
                + (Simd32x4::from(self[e15]) * Simd32x4::from([self[e45], self[e12], self[e4315], self[e41]]))
                + (Simd32x4::from(self[e35]) * Simd32x4::from([self[e31], self[e4235], self[e45], self[e43]]))
                - (Simd32x4::from(self[e25]) * Simd32x4::from([self[e12], self[e45], self[e4235], self[e42]]))
                - (Simd32x4::from(self[e15]) * Simd32x4::from([self[e45], self[e4125], self[e31], self[e41]]))
                + Simd32x4::from(2.0) * (Simd32x4::from(self[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + (Simd32x4::from(self[e5]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]))
                + Simd32x4::from([
                    ((self[e4235] * self[e3215]) + (self[e315] * self[e3]) - (self[e235] * self[e321]) - (self[e235] * self[e12345]) + (self[e12] * self[e25])
                        - (self[e31] * self[e35])
                        + (self[e23] * self[e3215])
                        - (self[e35] * self[e4315])
                        + (self[e25] * self[e4125])
                        - (self[e5] * self[e415])
                        - (self[e5] * self[e1])
                        + (self[e3] * self[e315])
                        - (self[e2] * self[e125])
                        - (self[e12345] * self[e235])),
                    ((self[e4315] * self[e3215]) + (self[e125] * self[e1])
                        - (self[e315] * self[e12345])
                        - (self[e235] * self[e435])
                        - (self[e235] * self[e3])
                        - (self[e12] * self[e15])
                        + (self[e31] * self[e3215])
                        + (self[e23] * self[e35])
                        + (self[e35] * self[e4235])
                        - (self[e15] * self[e4125])
                        - (self[e5] * self[e2])
                        - (self[e3] * self[e235])
                        + (self[e1] * self[e125])
                        - (self[e12345] * self[e315])),
                    ((self[e4125] * self[e3215]) - (self[e125] * self[e12345]) - (self[e315] * self[e1])
                        + (self[e235] * self[e2])
                        + (self[e12] * self[e3215])
                        + (self[e31] * self[e15])
                        - (self[e23] * self[e25])
                        - (self[e25] * self[e4235])
                        + (self[e15] * self[e4315])
                        - (self[e5] * self[e435])
                        - (self[e5] * self[e3])
                        + (self[e2] * self[e235])
                        - (self[e1] * self[e315])
                        - (self[e12345] * self[e125])),
                    (-(self[e4125] * self[e12]) - (self[e4315] * self[e31]) - (self[e4235] * self[e23]) - (self[e423] * self[e235]) + (self[e321] * self[e12345])
                        - (self[e435] * self[e3])
                        - (self[e415] * self[e1])
                        - (self[e12] * self[e4125])
                        - (self[e31] * self[e4315])
                        - (self[e23] * self[e4235])
                        - (self[e3] * self[e435])
                        - (self[e2] * self[e425])
                        - (self[e1] * self[e415])
                        + (self[e12345] * self[e321])),
                ])),
            // e41, e42, e43
            (Simd32x3::from(2.0) * (Simd32x3::from(self[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]))
                - (Simd32x3::from(self[e42]) * Simd32x3::from([self[e4125], self[e45], self[e23]]))
                + (Simd32x3::from(self[e43]) * Simd32x3::from([self[e4315], self[e23], self[e45]]))
                - (Simd32x3::from(self[e412]) * Simd32x3::from([self[e425], self[e1], self[e12345]]))
                + (Simd32x3::from(self[e412]) * Simd32x3::from([self[e2], self[e415], self[e321]]))
                + (Simd32x3::from(self[e431]) * Simd32x3::from([self[e435], self[e321], self[e1]]))
                - (Simd32x3::from(self[e431]) * Simd32x3::from([self[e3], self[e12345], self[e415]]))
                + (Simd32x3::from(self[e423]) * Simd32x3::from([self[e321], self[e3], self[e425]]))
                - (Simd32x3::from(self[e423]) * Simd32x3::from([self[e12345], self[e435], self[e2]]))
                - (Simd32x3::from(self[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * Simd32x3::from([self[e415], self[e425], self[e435]]))
                + (Simd32x3::from(self[e42]) * Simd32x3::from([self[e12], self[e45], self[e4235]]))
                - (Simd32x3::from(self[e43]) * Simd32x3::from([self[e31], self[e4235], self[e45]]))
                + (Simd32x3::from(self[e41]) * Simd32x3::from([self[e45], self[e4125], self[e31]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]))
                - (Simd32x3::from(self[e41]) * Simd32x3::from([self[e45], self[e12], self[e4315]]))
                - (Simd32x3::from(self[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                + Simd32x3::from([
                    (-(self[e435] * self[e431]) + (self[e425] * self[e412]) + (self[e43] * self[e4315]) + (self[e43] * self[e31])
                        - (self[e42] * self[e4125])
                        - (self[e42] * self[e12])
                        - (self[e3] * self[e431])
                        + (self[e2] * self[e412])),
                    ((self[e435] * self[e423]) - (self[e415] * self[e412]) - (self[e43] * self[e4235]) - (self[e43] * self[e23])
                        + (self[e41] * self[e4125])
                        + (self[e41] * self[e12])
                        + (self[e3] * self[e423])
                        - (self[e1] * self[e412])),
                    (-(self[e425] * self[e423]) + (self[e415] * self[e431]) + (self[e42] * self[e4235]) + (self[e42] * self[e23])
                        - (self[e41] * self[e4315])
                        - (self[e41] * self[e31])
                        - (self[e2] * self[e423])
                        + (self[e1] * self[e431])),
                ])),
            // e23, e31, e12
            ((Simd32x3::from(self[e1234]) * Simd32x3::from([self[e15], self[e25], self[e35]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e3215]) * Simd32x3::from([self[e41], self[e42], self[e43]]))
                + (Simd32x3::from(self[e4315]) * Simd32x3::from([self[e4125], self[e45], self[e4235]]))
                + (Simd32x3::from(self[e45]) * Simd32x3::from([self[e4235], self[e4315], self[e4125]]))
                - (Simd32x3::from(self[e431]) * Simd32x3::from([self[e125], self[e5], self[e235]]))
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * Simd32x3::from([self[e235], self[e315], self[e125]]))
                - (Simd32x3::from(self[e412]) * Simd32x3::from([self[e315], self[e235], self[e5]]))
                - (Simd32x3::from(self[e423]) * Simd32x3::from([self[e5], self[e125], self[e315]]))
                - (Simd32x3::from(self[e321]) * Simd32x3::from([self[e1], self[e2], self[e3]]))
                - (Simd32x3::from(self[e425]) * Simd32x3::from([self[e435], self[e12345], self[e415]]))
                - (Simd32x3::from(self[e12345]) * Simd32x3::from([self[e415], self[e425], self[e435]]))
                + (Simd32x3::from(self[e31]) * Simd32x3::from([self[e12], self[scalar], self[e23]]))
                + (Simd32x3::from(self[scalar]) * Simd32x3::from([self[e23], self[e31], self[e12]]))
                + (Simd32x3::from(self[e25]) * Simd32x3::from([self[e43], self[e1234], self[e41]]))
                + (Simd32x3::from(self[e35]) * Simd32x3::from([self[e42], self[e41], self[e1234]]))
                + (Simd32x3::from(self[e15]) * Simd32x3::from([self[e1234], self[e43], self[e42]]))
                - (Simd32x3::from(self[e5]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                - (Simd32x3::from(self[e2]) * Simd32x3::from([self[e3], self[e321], self[e1]]))
                + Simd32x3::from([
                    (-(self[e4315] * self[e4125]) + (self[e315] * self[e412]) + (self[e431] * self[e125]) + (self[e425] * self[e435])
                        - (self[e31] * self[e12])
                        - (self[e42] * self[e35])
                        + (self[e45] * self[e4235])
                        - (self[e25] * self[e43])
                        + (self[e2] * self[e3])
                        - (self[e1] * self[e321])
                        + (self[scalar] * self[e23])
                        - (self[e12345] * self[e415])),
                    (-(self[e4125] * self[e4235]) + (self[e4235] * self[e4125]) + (self[e125] * self[e423]) + (self[e412] * self[e235]) + (self[e435] * self[e415])
                        - (self[e415] * self[e435])
                        - (self[e12] * self[e23])
                        + (self[e23] * self[e12])
                        - (self[e43] * self[e15])
                        - (self[e35] * self[e41])
                        + (self[e3] * self[e1])
                        - (self[e1] * self[e3])),
                    (-(self[e4235] * self[e4315]) + (self[e235] * self[e431]) + (self[e423] * self[e315]) + (self[e415] * self[e425])
                        - (self[e23] * self[e31])
                        - (self[e41] * self[e25])
                        + (self[e45] * self[e4125])
                        - (self[e15] * self[e42])
                        - (self[e3] * self[e321])
                        + (self[e1] * self[e2])
                        + (self[scalar] * self[e12])
                        - (self[e12345] * self[e435])),
                ])),
            // e415, e425, e435, e321
            ((Simd32x4::from(self[e1234]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]))
                + (Simd32x4::from(self[e3215]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
                - (Simd32x4::from(self[e2]) * Simd32x4::from([self[e4125], self[e45], self[e4235], self[e31]]))
                + (Simd32x4::from(self[e42]) * Simd32x4::from([self[e125], self[e5], self[e235], self[e315]]))
                + (Simd32x4::from(self[e25]) * Simd32x4::from([self[e412], self[e4], self[e423], self[e431]]))
                + (Simd32x4::from(self[e435]) * Simd32x4::from([self[e31], self[e23], self[scalar], self[e4125]]))
                + (Simd32x4::from(self[scalar]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]))
                + (Simd32x4::from(self[e425]) * Simd32x4::from([self[e12], self[scalar], self[e23], self[e4315]]))
                + (Simd32x4::from(self[e43]) * Simd32x4::from([self[e315], self[e235], self[e5], self[e125]]))
                + (Simd32x4::from(self[e41]) * Simd32x4::from([self[e5], self[e125], self[e315], self[e235]]))
                - (Simd32x4::from(self[e45]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e12345]]))
                + (Simd32x4::from(self[e35]) * Simd32x4::from([self[e431], self[e423], self[e4], self[e412]]))
                + (Simd32x4::from(self[e15]) * Simd32x4::from([self[e4], self[e412], self[e431], self[e423]]))
                - (Simd32x4::from(self[e3]) * Simd32x4::from([self[e4315], self[e4235], self[e45], self[e12]]))
                - (Simd32x4::from(self[e1]) * Simd32x4::from([self[e45], self[e4125], self[e4315], self[e23]]))
                + (Simd32x4::from(self[e415]) * Simd32x4::from([self[scalar], self[e12], self[e31], self[e4235]]))
                + Simd32x4::from([
                    ((self[e4315] * self[e3]) - (self[e4235] * self[e321]) - (self[e315] * self[e43]) + (self[e235] * self[e1234]) - (self[e431] * self[e35])
                        + (self[e423] * self[e3215])
                        - (self[e321] * self[e4235])
                        - (self[e425] * self[e12])
                        - (self[e31] * self[e435])
                        + (self[e23] * self[e12345])
                        - (self[e42] * self[e125])
                        - (self[e25] * self[e412])
                        + (self[e5] * self[e41])
                        + (self[e4] * self[e15])
                        + (self[e2] * self[e4125])
                        + (self[e12345] * self[e23])),
                    ((self[e4125] * self[e1]) - (self[e4315] * self[e321]) - (self[e125] * self[e41]) + (self[e315] * self[e1234]) - (self[e412] * self[e15])
                        + (self[e431] * self[e3215])
                        - (self[e321] * self[e4315])
                        - (self[e435] * self[e23])
                        - (self[e12] * self[e415])
                        + (self[e31] * self[e12345])
                        - (self[e43] * self[e235])
                        - (self[e35] * self[e423])
                        + (self[e5] * self[e42])
                        + (self[e4] * self[e25])
                        + (self[e3] * self[e4235])
                        + (self[e12345] * self[e31])),
                    (-(self[e4125] * self[e321]) + (self[e4235] * self[e2]) + (self[e125] * self[e1234]) - (self[e235] * self[e42]) + (self[e412] * self[e3215])
                        - (self[e423] * self[e25])
                        - (self[e321] * self[e4125])
                        - (self[e415] * self[e31])
                        + (self[e12] * self[e12345])
                        - (self[e23] * self[e425])
                        - (self[e41] * self[e315])
                        - (self[e15] * self[e431])
                        + (self[e5] * self[e43])
                        + (self[e4] * self[e35])
                        + (self[e1] * self[e4315])
                        + (self[e12345] * self[e12])),
                    (-(self[e3215] * self[e4]) - (self[e412] * self[e35]) - (self[e431] * self[e25]) - (self[e423] * self[e15])
                        + (self[e435] * self[e4125])
                        + (self[e425] * self[e4315])
                        + (self[e415] * self[e4235])
                        - (self[e43] * self[e125])
                        - (self[e42] * self[e315])
                        - (self[e41] * self[e235])
                        - (self[e5] * self[e1234])
                        - (self[e3] * self[e12])
                        - (self[e2] * self[e31])
                        - (self[e1] * self[e23])
                        + (self[scalar] * self[e321])
                        - (self[e12345] * self[e45])),
                ])),
            // e423, e431, e412
            (Simd32x3::from(2.0) * (Simd32x3::from(self[e1234]) * Simd32x3::from([self[e415], self[e425], self[e435]]))
                - (Simd32x3::from(self[e431]) * Simd32x3::from([self[e4125], self[e45], self[e23]]))
                + (Simd32x3::from(self[e412]) * Simd32x3::from([self[e4315], self[e23], self[e45]]))
                - (Simd32x3::from(self[e4]) * Simd32x3::from([self[e4235], self[e4315], self[e4125]]))
                + (Simd32x3::from(self[e31]) * Simd32x3::from([self[e412], self[e4], self[e423]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                + (Simd32x3::from(self[e321]) * Simd32x3::from([self[e41], self[e42], self[e43]]))
                + (Simd32x3::from(self[e42]) * Simd32x3::from([self[e435], self[e12345], self[e415]]))
                - (Simd32x3::from(self[e43]) * Simd32x3::from([self[e425], self[e415], self[e321]]))
                + (Simd32x3::from(self[e431]) * Simd32x3::from([self[e12], self[e45], self[e4235]]))
                - (Simd32x3::from(self[e412]) * Simd32x3::from([self[e31], self[e4235], self[e45]]))
                + (Simd32x3::from(self[e4]) * Simd32x3::from([self[e23], self[e4315], self[e12]]))
                + (Simd32x3::from(self[e43]) * Simd32x3::from([self[e425], self[e415], self[e12345]]))
                - (Simd32x3::from(self[e42]) * Simd32x3::from([self[e435], self[e321], self[e415]]))
                - (Simd32x3::from(self[e41]) * Simd32x3::from([self[e321], self[e435], self[e425]]))
                + (Simd32x3::from(self[e41]) * Simd32x3::from([self[e12345], self[e435], self[e425]]))
                - (Simd32x3::from(self[e423]) * Simd32x3::from([self[e45], self[e12], self[e4315]]))
                + (Simd32x3::from(self[e4]) * Simd32x3::from([self[e4235], self[e31], self[e4125]]))
                + (Simd32x3::from(self[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]))
                + Simd32x3::from([
                    ((self[e412] * self[e4315]) - (self[e431] * self[e4125]) - (self[e431] * self[e12]) + (self[e423] * self[e45]) - (self[e43] * self[e2])
                        + (self[e42] * self[e3])
                        + (self[e4] * self[e23])
                        + (self[e3] * self[e42])
                        - (self[e2] * self[e43])),
                    ((self[e4125] * self[e423]) - (self[e412] * self[e4235]) - (self[e412] * self[e23])
                        + (self[e423] * self[e4125])
                        + (self[e423] * self[e12])
                        + (self[e43] * self[e1])
                        - (self[e41] * self[e3])
                        - (self[e3] * self[e41])
                        + (self[e1] * self[e43])),
                    ((self[e431] * self[e4235]) + (self[e431] * self[e23]) - (self[e423] * self[e4315]) - (self[e423] * self[e31]) - (self[e42] * self[e1])
                        + (self[e41] * self[e2])
                        + (self[e4] * self[e12])
                        + (self[e2] * self[e41])
                        - (self[e1] * self[e42])),
                ])),
            // e235, e315, e125
            (Simd32x3::from(2.0) * (Simd32x3::from(self[e3215]) * Simd32x3::from([self[e415], self[e425], self[e435]]))
                + (Simd32x3::from(self[e315]) * Simd32x3::from([self[e4125], self[scalar], self[e23]]))
                - (Simd32x3::from(self[e125]) * Simd32x3::from([self[e4315], self[e23], self[e45]]))
                + (Simd32x3::from(self[e125]) * Simd32x3::from([self[e31], self[e4235], self[scalar]]))
                - (Simd32x3::from(self[e315]) * Simd32x3::from([self[e12], self[e45], self[e4235]]))
                - (Simd32x3::from(self[e235]) * Simd32x3::from([self[e45], self[e4125], self[e31]]))
                + (Simd32x3::from(self[e235]) * Simd32x3::from([self[scalar], self[e12], self[e4315]]))
                - (Simd32x3::from(self[e321]) * Simd32x3::from([self[e15], self[e25], self[e35]]))
                + (Simd32x3::from(self[e25]) * Simd32x3::from([self[e435], self[e321], self[e415]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e5]) * Simd32x3::from([self[e23], self[e31], self[e12]]))
                + (Simd32x3::from(self[e45]) * Simd32x3::from([self[e235], self[e315], self[e125]]))
                + (Simd32x3::from(self[e35]) * Simd32x3::from([self[e425], self[e415], self[e321]]))
                + (Simd32x3::from(self[e15]) * Simd32x3::from([self[e321], self[e435], self[e425]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * Simd32x3::from([self[e15], self[e25], self[e35]]))
                + (Simd32x3::from(self[scalar]) * Simd32x3::from([self[e235], self[e315], self[e125]]))
                + Simd32x3::from([
                    (-(self[e125] * self[e4315]) + (self[e315] * self[e4125]) - (self[e425] * self[e35]) + (self[e12] * self[e315]) - (self[e31] * self[e125])
                        + (self[e35] * self[e2])
                        - (self[e25] * self[e435])
                        - (self[e25] * self[e3])
                        - (self[e3] * self[e25])
                        + (self[e2] * self[e35])),
                    ((self[e125] * self[e4235]) - (self[e235] * self[e4125]) - (self[e435] * self[e15]) - (self[e12] * self[e235]) + (self[e23] * self[e125])
                        - (self[e35] * self[e415])
                        - (self[e35] * self[e1])
                        + (self[e15] * self[e3])
                        + (self[e3] * self[e15])
                        - (self[e1] * self[e35])),
                    (-(self[e315] * self[e4235]) + (self[e235] * self[e4315]) - (self[e415] * self[e25]) + (self[e31] * self[e235]) - (self[e23] * self[e315])
                        + (self[e25] * self[e1])
                        - (self[e15] * self[e425])
                        - (self[e15] * self[e2])
                        - (self[e2] * self[e15])
                        + (self[e1] * self[e25])),
                ])),
            // e4235, e4315, e4125, e3215
            ((Simd32x4::from(self[e3215]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]))
                + (Simd32x4::from(self[e4125]) * Simd32x4::from([self[e31], self[e23], self[scalar], self[e35]]))
                + (Simd32x4::from(self[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]))
                + (Simd32x4::from(self[e415]) * Simd32x4::from([self[e321], self[e3], self[e2], self[e235]]))
                + (Simd32x4::from(self[e435]) * Simd32x4::from([self[e2], self[e1], self[e321], self[e125]]))
                + (Simd32x4::from(self[e4315]) * Simd32x4::from([self[e12], self[scalar], self[e23], self[e25]]))
                + (Simd32x4::from(self[e45]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e3215]]))
                - (Simd32x4::from(self[e3215]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e45]]))
                + (Simd32x4::from(self[e425]) * Simd32x4::from([self[e3], self[e321], self[e1], self[e315]]))
                + (Simd32x4::from(self[e4235]) * Simd32x4::from([self[scalar], self[e12], self[e31], self[e15]]))
                + Simd32x4::from([
                    (-(self[e1234] * self[e15]) - (self[e4315] * self[e12]) - (self[e125] * self[e431]) + (self[e315] * self[e412]) - (self[e235] * self[e4])
                        + (self[e412] * self[e315])
                        - (self[e431] * self[e125])
                        - (self[e425] * self[e3])
                        + (self[e415] * self[e321])
                        - (self[e31] * self[e4125])
                        - (self[e43] * self[e25])
                        + (self[e42] * self[e35])
                        + (self[e45] * self[e23])
                        + (self[e35] * self[e42])
                        - (self[e25] * self[e43])
                        + (self[e15] * self[e1234])
                        + (self[e4] * self[e235])
                        - (self[e2] * self[e435])
                        + (self[e1] * self[e12345])
                        + (self[e12345] * self[e1])),
                    (-(self[e1234] * self[e25]) - (self[e4125] * self[e23]) + (self[e125] * self[e423])
                        - (self[e315] * self[e4])
                        - (self[e235] * self[e412])
                        - (self[e412] * self[e235])
                        + (self[e423] * self[e125])
                        - (self[e435] * self[e1])
                        + (self[e425] * self[e321])
                        - (self[e12] * self[e4235])
                        + (self[e43] * self[e15])
                        - (self[e41] * self[e35])
                        + (self[e45] * self[e31])
                        - (self[e35] * self[e41])
                        + (self[e25] * self[e1234])
                        + (self[e15] * self[e43])
                        + (self[e4] * self[e315])
                        - (self[e3] * self[e415])
                        + (self[e2] * self[e12345])
                        + (self[e12345] * self[e2])),
                    (-(self[e1234] * self[e35]) - (self[e4235] * self[e31]) - (self[e125] * self[e4]) - (self[e315] * self[e423])
                        + (self[e235] * self[e431])
                        + (self[e431] * self[e235])
                        - (self[e423] * self[e315])
                        + (self[e435] * self[e321])
                        - (self[e415] * self[e2])
                        - (self[e23] * self[e4315])
                        - (self[e42] * self[e15])
                        + (self[e41] * self[e25])
                        + (self[e45] * self[e12])
                        + (self[e35] * self[e1234])
                        + (self[e25] * self[e41])
                        - (self[e15] * self[e42])
                        + (self[e4] * self[e125])
                        + (self[e3] * self[e12345])
                        - (self[e1] * self[e425])
                        + (self[e12345] * self[e3])),
                    (-(self[e4125] * self[e35]) - (self[e4315] * self[e25]) - (self[e4235] * self[e15])
                        + (self[e125] * self[e3])
                        + (self[e315] * self[e2])
                        + (self[e235] * self[e1])
                        + (self[e435] * self[e125])
                        + (self[e425] * self[e315])
                        + (self[e415] * self[e235])
                        - (self[e12] * self[e35])
                        - (self[e31] * self[e25])
                        - (self[e23] * self[e15])
                        - (self[e35] * self[e12])
                        - (self[e25] * self[e31])
                        - (self[e15] * self[e23])
                        - (self[e5] * self[e12345])
                        - (self[e3] * self[e125])
                        - (self[e2] * self[e315])
                        - (self[e1] * self[e235])
                        - (self[e12345] * self[e5])),
                ])),
            // e1234
            ((self[e1234] * self[e45])
                + (self[e1234] * self[scalar])
                + (self[e4125] * self[e43])
                + (self[e4315] * self[e42])
                + (self[e4235] * self[e41])
                + (self[e412] * self[e435])
                - (self[e412] * self[e3])
                + (self[e431] * self[e425])
                - (self[e431] * self[e2])
                + (self[e423] * self[e415])
                - (self[e423] * self[e1])
                - (self[e321] * self[e4])
                + (self[e435] * self[e412])
                + (self[e425] * self[e431])
                + (self[e415] * self[e423])
                - (self[e12] * self[e43])
                - (self[e31] * self[e42])
                - (self[e23] * self[e41])
                - (self[e43] * self[e4125])
                - (self[e43] * self[e12])
                - (self[e42] * self[e4315])
                - (self[e42] * self[e31])
                - (self[e41] * self[e4235])
                - (self[e41] * self[e23])
                - (self[e45] * self[e1234])
                + (self[e4] * self[e321])
                - (self[e4] * self[e12345])
                + (self[e3] * self[e412])
                + (self[e2] * self[e431])
                + (self[e1] * self[e423])
                + (self[scalar] * self[e1234])
                - (self[e12345] * self[e4])),
        );
    }
}
impl Square for Plane {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (-(self[e4315] * self[e4125]) + (self[e4125] * self[e4315])),
                ((self[e4235] * self[e4125]) - (self[e4125] * self[e4235])),
                (-(self[e4235] * self[e4315]) + (self[e4315] * self[e4235])),
                (-f32::powi(self[e4125], 2) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2)),
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                ((self[e4235] * self[e3215]) - (self[e3215] * self[e4235])),
                ((self[e4315] * self[e3215]) - (self[e3215] * self[e4315])),
                ((self[e4125] * self[e3215]) - (self[e3215] * self[e4125])),
                0.0,
            ]),
        );
    }
}
impl Square for QuadNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       14        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            (-(self[e12345] * self[e4]) - (self[e321] * self[e4]) + (self[e4] * self[e321]) - (self[e4] * self[e12345])),
            (-(self[e12345] * self[e5]) + (self[e321] * self[e5]) - (self[e5] * self[e321]) - (self[e5] * self[e12345])),
            ((self[e12345] * self[e321]) + (self[e321] * self[e12345]) + (self[e4] * self[e5]) - (self[e5] * self[e4])),
            (-f32::powi(self[e12345], 2) - f32::powi(self[e321], 2) - (self[e4] * self[e5]) - (self[e5] * self[e4])),
        ]));
    }
}
impl Square for RoundPoint {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(-0.0),
            // e23, e31, e12, e45
            Simd32x4::from([
                ((self[e2] * self[e3]) - (self[e3] * self[e2])),
                (-(self[e1] * self[e3]) + (self[e3] * self[e1])),
                ((self[e1] * self[e2]) - (self[e2] * self[e1])),
                ((self[e4] * self[e5]) - (self[e5] * self[e4])),
            ]),
            // e15, e25, e35, scalar
            (-(Simd32x4::from(self[e5]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]))
                + Simd32x4::from([
                    (self[e1] * self[e5]),
                    (self[e2] * self[e5]),
                    (self[e3] * self[e5]),
                    (-(self[e4] * self[e5]) + f32::powi(self[e3], 2) + f32::powi(self[e1], 2) + f32::powi(self[e2], 2)),
                ])),
        );
    }
}
impl Square for Scalar {
    type Output = Scalar;
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ f32::powi(self[scalar], 2));
    }
}
impl Square for Sphere {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       15        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       16        0
    //  no simd       11       19        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(-0.0),
            // e23, e31, e12, e45
            Simd32x4::from([
                (-(self[e4315] * self[e4125]) + (self[e4125] * self[e4315])),
                ((self[e4235] * self[e4125]) - (self[e4125] * self[e4235])),
                (-(self[e4235] * self[e4315]) + (self[e4315] * self[e4235])),
                ((self[e3215] * self[e1234]) - (self[e1234] * self[e3215])),
            ]),
            // e15, e25, e35, scalar
            ((Simd32x4::from(self[e3215]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e1234]]))
                + Simd32x4::from([
                    ((self[e3215] * self[e4235]) * -1.0),
                    ((self[e3215] * self[e4315]) * -1.0),
                    ((self[e3215] * self[e4125]) * -1.0),
                    ((self[e3215] * self[e1234]) - f32::powi(self[e4125], 2) - f32::powi(self[e4235], 2) - f32::powi(self[e4315], 2)),
                ])),
        );
    }
}
impl Square for TripleNum {
    type Output = AntiQuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        8        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from([
            (-(self[e4] * self[e12345]) - (self[e12345] * self[e4])),
            (-(self[e5] * self[e12345]) - (self[e12345] * self[e5])),
            ((self[e4] * self[e5]) - (self[e5] * self[e4])),
            (-f32::powi(self[e12345], 2) - (self[e4] * self[e5]) - (self[e5] * self[e4])),
        ]));
    }
}
impl Square for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      164      172        0
    //    simd4       16       17        0
    // Totals...
    // yes simd      180      189        0
    //  no simd      228      240        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-(Simd32x4::from(self[e4]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e5]]))
                + (Simd32x4::from(self[e412]) * Simd32x4::from([self[e2], self[e415], self[e321], self[e125]]))
                - (Simd32x4::from(self[e4]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]))
                + (Simd32x4::from(self[e431]) * Simd32x4::from([self[e435], self[e321], self[e1], self[e315]]))
                + (Simd32x4::from(self[e423]) * Simd32x4::from([self[e321], self[e3], self[e425], self[e235]]))
                + Simd32x4::from([
                    ((self[e4] * self[e1]) - (self[e3] * self[e431]) - (self[e321] * self[e423]) - (self[e435] * self[e431]) + (self[e425] * self[e412])
                        - (self[e415] * self[e4])
                        - (self[e12345] * self[e423])
                        + (self[e412] * self[e2])
                        - (self[e412] * self[e425])
                        - (self[e431] * self[e3])
                        - (self[e423] * self[e12345])),
                    ((self[e4] * self[e2]) - (self[e1] * self[e412]) - (self[e321] * self[e431]) + (self[e435] * self[e423])
                        - (self[e425] * self[e4])
                        - (self[e415] * self[e412])
                        - (self[e12345] * self[e431])
                        - (self[e412] * self[e1])
                        - (self[e431] * self[e12345])
                        - (self[e423] * self[e435])
                        + (self[e423] * self[e3])),
                    ((self[e4] * self[e3]) - (self[e2] * self[e423]) - (self[e321] * self[e412]) - (self[e435] * self[e4]) - (self[e425] * self[e423]) + (self[e415] * self[e431])
                        - (self[e12345] * self[e412])
                        - (self[e412] * self[e12345])
                        + (self[e431] * self[e1])
                        - (self[e431] * self[e415])
                        - (self[e423] * self[e2])),
                    (f32::powi(self[e3], 2) + f32::powi(self[e2], 2) + f32::powi(self[e1], 2) - f32::powi(self[e321], 2)
                        + f32::powi(self[e435], 2)
                        + f32::powi(self[e425], 2)
                        + f32::powi(self[e415], 2)
                        - f32::powi(self[e12345], 2)
                        + (self[e412] * self[e125])
                        + (self[e423] * self[e235])
                        + (self[e431] * self[e315])),
                ])),
            // e23, e31, e12, e45
            (-(Simd32x4::from(self[e4]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]))
                - (Simd32x4::from(self[e2]) * Simd32x4::from([self[e3], self[e321], self[e1], self[e425]]))
                - (Simd32x4::from(self[e423]) * Simd32x4::from([self[e5], self[e125], self[e315], self[e235]]))
                - (Simd32x4::from(self[e431]) * Simd32x4::from([self[e125], self[e5], self[e235], self[e315]]))
                - (Simd32x4::from(self[e425]) * Simd32x4::from([self[e435], self[e12345], self[e415], self[e2]]))
                - (Simd32x4::from(self[e412]) * Simd32x4::from([self[e315], self[e235], self[e5], self[e125]]))
                + Simd32x4::from([
                    ((self[e2] * self[e3]) - (self[e1] * self[e321]) + (self[e315] * self[e412]) - (self[e235] * self[e4]) - (self[e321] * self[e1]) + (self[e425] * self[e435])
                        - (self[e415] * self[e12345])
                        - (self[e12345] * self[e415])
                        - (self[e423] * self[e5])
                        + (self[e431] * self[e125])),
                    ((self[e3] * self[e1]) - (self[e1] * self[e3]) + (self[e125] * self[e423]) - (self[e315] * self[e4]) - (self[e321] * self[e2]) + (self[e435] * self[e415])
                        - (self[e415] * self[e435])
                        - (self[e12345] * self[e425])
                        + (self[e412] * self[e235])
                        - (self[e431] * self[e5])),
                    (-(self[e3] * self[e321]) + (self[e1] * self[e2]) - (self[e125] * self[e4]) + (self[e235] * self[e431])
                        - (self[e321] * self[e3])
                        - (self[e435] * self[e12345])
                        + (self[e415] * self[e425])
                        - (self[e12345] * self[e435])
                        - (self[e412] * self[e5])
                        + (self[e423] * self[e315])),
                    ((self[e4] * self[e5]) - (self[e3] * self[e435]) - (self[e1] * self[e415])
                        + (self[e125] * self[e412])
                        + (self[e315] * self[e431])
                        + (self[e235] * self[e423])
                        + (self[e321] * self[e12345])
                        - (self[e435] * self[e3])
                        - (self[e415] * self[e1])
                        + (self[e12345] * self[e321])),
                ])),
            // e15, e25, e35, e1234
            (Simd32x4::from(-2.0) * (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e4]]))
                + Simd32x4::from([
                    ((self[e3] * self[e315]) - (self[e2] * self[e125]) + (self[e1] * self[e5])
                        - (self[e5] * self[e1])
                        - (self[e5] * self[e415])
                        - (self[e125] * self[e2])
                        - (self[e125] * self[e425])
                        + (self[e315] * self[e3])
                        + (self[e315] * self[e435])
                        - (self[e435] * self[e315])
                        + (self[e425] * self[e125])
                        - (self[e415] * self[e5])),
                    (-(self[e3] * self[e235]) + (self[e2] * self[e5]) + (self[e1] * self[e125]) - (self[e5] * self[e2]) - (self[e5] * self[e425])
                        + (self[e125] * self[e1])
                        + (self[e125] * self[e415])
                        - (self[e235] * self[e3])
                        - (self[e235] * self[e435])
                        + (self[e435] * self[e235])
                        - (self[e425] * self[e5])
                        - (self[e415] * self[e125])),
                    ((self[e3] * self[e5]) + (self[e2] * self[e235])
                        - (self[e1] * self[e315])
                        - (self[e5] * self[e3])
                        - (self[e5] * self[e435])
                        - (self[e315] * self[e1])
                        - (self[e315] * self[e415])
                        + (self[e235] * self[e2])
                        + (self[e235] * self[e425])
                        - (self[e435] * self[e5])
                        - (self[e425] * self[e235])
                        + (self[e415] * self[e315])),
                    ((self[e3] * self[e412])
                        + (self[e2] * self[e431])
                        + (self[e1] * self[e423])
                        + (self[e435] * self[e412])
                        + (self[e425] * self[e431])
                        + (self[e415] * self[e423])
                        - (self[e412] * self[e3])
                        + (self[e412] * self[e435])
                        - (self[e431] * self[e2])
                        + (self[e431] * self[e425])
                        + (self[e423] * self[e415])
                        - (self[e423] * self[e1])),
                ])),
            // e4235, e4315, e4125, e3215
            ((Simd32x4::from(self[e425]) * Simd32x4::from([self[e3], self[e321], self[e1], self[e315]]))
                - (Simd32x4::from(self[e5]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]))
                + (Simd32x4::from(self[e321]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e5]]))
                + (Simd32x4::from(self[e2]) * Simd32x4::from([self[e435], self[e12345], self[e415], self[e315]]))
                + Simd32x4::from([
                    ((self[e4] * self[e235]) - (self[e2] * self[e435]) + (self[e1] * self[e12345]) - (self[e125] * self[e431]) + (self[e315] * self[e412])
                        - (self[e235] * self[e4])
                        - (self[e425] * self[e3])
                        + (self[e415] * self[e321])
                        + (self[e12345] * self[e1])
                        + (self[e412] * self[e315])
                        + (self[e423] * self[e5])
                        - (self[e431] * self[e125])),
                    ((self[e4] * self[e315]) - (self[e3] * self[e415]) + (self[e1] * self[e435]) + (self[e125] * self[e423])
                        - (self[e315] * self[e4])
                        - (self[e235] * self[e412])
                        - (self[e435] * self[e1])
                        + (self[e415] * self[e3])
                        + (self[e12345] * self[e2])
                        - (self[e412] * self[e235])
                        + (self[e423] * self[e125])
                        + (self[e431] * self[e5])),
                    ((self[e4] * self[e125]) + (self[e3] * self[e12345]) - (self[e1] * self[e425]) - (self[e125] * self[e4]) - (self[e315] * self[e423])
                        + (self[e235] * self[e431])
                        + (self[e435] * self[e321])
                        - (self[e415] * self[e2])
                        + (self[e12345] * self[e3])
                        + (self[e412] * self[e5])
                        - (self[e423] * self[e315])
                        + (self[e431] * self[e235])),
                    (-(self[e3] * self[e125]) - (self[e2] * self[e315]) - (self[e1] * self[e235]) - (self[e5] * self[e12345])
                        + (self[e125] * self[e3])
                        + (self[e125] * self[e435])
                        + (self[e235] * self[e1])
                        + (self[e235] * self[e415])
                        + (self[e435] * self[e125])
                        + (self[e425] * self[e315])
                        - (self[e12345] * self[e5])
                        + (self[e415] * self[e235])),
                ])),
        );
    }
}
impl Square for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      152      160        0
    //    simd4       18       20        0
    // Totals...
    // yes simd      170      180        0
    //  no simd      224      240        0
    fn square(self) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-(Simd32x4::from(self[e42]) * Simd32x4::from([self[e4125], self[e45], self[e23], self[e25]]))
                + (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]))
                + (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e3215]]))
                - (Simd32x4::from(self[e41]) * Simd32x4::from([self[e45], self[e12], self[e4315], self[e15]]))
                - (Simd32x4::from(self[e43]) * Simd32x4::from([self[e31], self[e4235], self[e45], self[e35]]))
                + Simd32x4::from([
                    ((self[e4315] * self[e43]) - (self[e4235] * self[e1234])
                        + (self[e12] * self[e42])
                        + (self[e23] * self[e1234])
                        + (self[scalar] * self[e41])
                        + (self[e43] * self[e4315])
                        + (self[e43] * self[e31])
                        - (self[e42] * self[e4125])
                        - (self[e42] * self[e12])
                        + (self[e41] * self[scalar])
                        + (self[e41] * self[e45])),
                    ((self[e4125] * self[e41]) - (self[e4315] * self[e1234]) + (self[e31] * self[e1234]) + (self[e23] * self[e43]) + (self[scalar] * self[e42])
                        - (self[e43] * self[e4235])
                        - (self[e43] * self[e23])
                        + (self[e42] * self[e45])
                        + (self[e42] * self[scalar])
                        + (self[e41] * self[e12])
                        + (self[e41] * self[e4125])),
                    (-(self[e4125] * self[e1234])
                        + (self[e4235] * self[e42])
                        + (self[e12] * self[e1234])
                        + (self[e31] * self[e41])
                        + (self[scalar] * self[e43])
                        + (self[e43] * self[e45])
                        + (self[e43] * self[scalar])
                        + (self[e42] * self[e4235])
                        + (self[e42] * self[e23])
                        - (self[e41] * self[e31])
                        - (self[e41] * self[e4315])),
                    (-f32::powi(self[e4125], 2) - f32::powi(self[e4315], 2) - f32::powi(self[e4235], 2) + f32::powi(self[e45], 2)
                        - f32::powi(self[e12], 2)
                        - f32::powi(self[e31], 2)
                        - f32::powi(self[e23], 2)
                        + f32::powi(self[scalar], 2)
                        - (self[e43] * self[e35])
                        - (self[e41] * self[e15])
                        - (self[e42] * self[e25])),
                ])),
            // e23, e31, e12, e45
            ((Simd32x4::from(self[e3215]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]))
                + Simd32x4::from(2.0) * (Simd32x4::from(self[e45]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[scalar]]))
                + (Simd32x4::from(self[e15]) * Simd32x4::from([self[e1234], self[e43], self[e42], self[e41]]))
                + (Simd32x4::from(self[e35]) * Simd32x4::from([self[e42], self[e41], self[e1234], self[e43]]))
                + (Simd32x4::from(self[e25]) * Simd32x4::from([self[e43], self[e1234], self[e41], self[e42]]))
                + Simd32x4::from([
                    ((self[e4125] * self[e4315]) - (self[e4315] * self[e4125]) - (self[e25] * self[e43]) + (self[e15] * self[e1234]) + (self[e12] * self[e31])
                        - (self[e31] * self[e12])
                        + (self[e23] * self[scalar])
                        + (self[scalar] * self[e23])
                        + (self[e41] * self[e3215])
                        - (self[e42] * self[e35])),
                    (-(self[e4125] * self[e4235]) + (self[e4235] * self[e4125]) - (self[e35] * self[e41]) + (self[e25] * self[e1234]) - (self[e12] * self[e23])
                        + (self[e31] * self[scalar])
                        + (self[e23] * self[e12])
                        + (self[scalar] * self[e31])
                        - (self[e43] * self[e15])
                        + (self[e42] * self[e3215])),
                    ((self[e4315] * self[e4235]) - (self[e4235] * self[e4315]) + (self[e35] * self[e1234]) - (self[e15] * self[e42])
                        + (self[e12] * self[scalar])
                        + (self[e31] * self[e23])
                        - (self[e23] * self[e31])
                        + (self[scalar] * self[e12])
                        + (self[e43] * self[e3215])
                        - (self[e41] * self[e25])),
                    (-(self[e4125] * self[e12])
                        - (self[e4315] * self[e31])
                        - (self[e4235] * self[e23])
                        - (self[e1234] * self[e3215])
                        - (self[e35] * self[e43])
                        - (self[e25] * self[e42])
                        - (self[e15] * self[e41])
                        - (self[e12] * self[e4125])
                        - (self[e31] * self[e4315])
                        - (self[e23] * self[e4235])),
                ])),
            // e15, e25, e35, e1234
            (Simd32x4::from(2.0) * (Simd32x4::from(self[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e1234]]))
                + Simd32x4::from([
                    (-(self[e3215] * self[e4235]) + (self[e3215] * self[e23]) + (self[e4125] * self[e25]) - (self[e4315] * self[e35]) + (self[e4235] * self[e3215])
                        - (self[e35] * self[e4315])
                        + (self[e35] * self[e31])
                        + (self[e25] * self[e4125])
                        - (self[e25] * self[e12])
                        + (self[e12] * self[e25])
                        - (self[e31] * self[e35])
                        + (self[e23] * self[e3215])),
                    (-(self[e3215] * self[e4315]) + (self[e3215] * self[e31]) - (self[e4125] * self[e15])
                        + (self[e4315] * self[e3215])
                        + (self[e4235] * self[e35])
                        + (self[e35] * self[e4235])
                        - (self[e35] * self[e23])
                        - (self[e15] * self[e4125])
                        + (self[e15] * self[e12])
                        - (self[e12] * self[e15])
                        + (self[e31] * self[e3215])
                        + (self[e23] * self[e35])),
                    (-(self[e3215] * self[e4125]) + (self[e3215] * self[e12]) + (self[e4125] * self[e3215]) + (self[e4315] * self[e15])
                        - (self[e4235] * self[e25])
                        - (self[e25] * self[e4235])
                        + (self[e25] * self[e23])
                        + (self[e15] * self[e4315])
                        - (self[e15] * self[e31])
                        + (self[e12] * self[e3215])
                        + (self[e31] * self[e15])
                        - (self[e23] * self[e25])),
                    ((self[e4125] * self[e43]) + (self[e4315] * self[e42]) + (self[e4235] * self[e41])
                        - (self[e12] * self[e43])
                        - (self[e31] * self[e42])
                        - (self[e23] * self[e41])
                        - (self[e43] * self[e4125])
                        - (self[e43] * self[e12])
                        - (self[e42] * self[e4315])
                        - (self[e42] * self[e31])
                        - (self[e41] * self[e23])
                        - (self[e41] * self[e4235])),
                ])),
            // e4235, e4315, e4125, e3215
            ((Simd32x4::from(self[e3215]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]))
                + (Simd32x4::from(self[e4125]) * Simd32x4::from([self[e31], self[e23], self[scalar], self[e35]]))
                + (Simd32x4::from(self[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]))
                + (Simd32x4::from(self[e45]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e3215]]))
                + (Simd32x4::from(self[e4315]) * Simd32x4::from([self[e12], self[scalar], self[e23], self[e25]]))
                + (Simd32x4::from(self[e4235]) * Simd32x4::from([self[scalar], self[e12], self[e31], self[e15]]))
                - (Simd32x4::from(self[e3215]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e45]]))
                + Simd32x4::from([
                    (-(self[e4315] * self[e12]) - (self[e1234] * self[e15]) + (self[e35] * self[e42]) - (self[e25] * self[e43]) + (self[e15] * self[e1234])
                        - (self[e31] * self[e4125])
                        + (self[e23] * self[e45])
                        - (self[e43] * self[e25])
                        + (self[e42] * self[e35])),
                    (-(self[e4125] * self[e23]) - (self[e1234] * self[e25]) - (self[e35] * self[e41]) + (self[e25] * self[e1234]) + (self[e15] * self[e43])
                        - (self[e12] * self[e4235])
                        + (self[e31] * self[e45])
                        + (self[e43] * self[e15])
                        - (self[e41] * self[e35])),
                    (-(self[e4235] * self[e31]) - (self[e1234] * self[e35]) + (self[e35] * self[e1234]) + (self[e25] * self[e41]) - (self[e15] * self[e42])
                        + (self[e12] * self[e45])
                        - (self[e23] * self[e4315])
                        + (self[e41] * self[e25])
                        - (self[e42] * self[e15])),
                    (-(self[e4125] * self[e35])
                        - (self[e4315] * self[e25])
                        - (self[e4235] * self[e15])
                        - (self[e35] * self[e12])
                        - (self[e25] * self[e31])
                        - (self[e15] * self[e23])
                        - (self[e12] * self[e35])
                        - (self[e31] * self[e25])
                        - (self[e23] * self[e15])),
                ])),
        );
    }
}
