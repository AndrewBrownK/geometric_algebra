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
//  Average:        53      58       0
//  Maximum:       599     627       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:        12      16       0
//  Average:        71      77       0
//  Maximum:       863     916       0
impl AntiSquare for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       81       92        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       87       98        0
    //  no simd      105      116        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x4::from(self[e42]) * Simd32x4::from([self[e12], self[e45], self[e23], self[e25]]))
                + (Simd32x4::from(self[e43]) * Simd32x4::from([self[e31], self[e23], self[e45], self[e35]]))
                + (Simd32x4::from(self[e41]) * Simd32x4::from([self[e45], self[e12], self[e31], self[e15]]))
                + Simd32x4::from([
                    (-(self[scalar] * self[e41]) - (self[e45] * self[e41]) - (self[e31] * self[e43]) - (self[e42] * self[e12]) - (self[e41] * self[scalar])),
                    (-(self[scalar] * self[e42]) - (self[e45] * self[e42]) - (self[e12] * self[e41]) - (self[e43] * self[e23]) - (self[e42] * self[scalar])),
                    (-(self[scalar] * self[e43]) - (self[e45] * self[e43]) - (self[e23] * self[e42]) - (self[e43] * self[scalar]) - (self[e41] * self[e31])),
                    (-f32::powi(self[scalar], 2) - f32::powi(self[e45], 2)
                        + f32::powi(self[e12], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e23], 2)
                        + (self[e43] * self[e35])
                        + (self[e41] * self[e15])
                        + (self[e42] * self[e25])),
                ])),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self[scalar] * self[e23]) + (self[e35] * self[e42]) - (self[e25] * self[e43]) + (self[e12] * self[e31])
                    - (self[e31] * self[e12])
                    - (self[e23] * self[scalar])
                    - (self[e42] * self[e35])
                    + (self[e43] * self[e25])),
                (-(self[scalar] * self[e31]) - (self[e35] * self[e41]) + (self[e15] * self[e43]) - (self[e12] * self[e23]) - (self[e31] * self[scalar])
                    + (self[e23] * self[e12])
                    + (self[e41] * self[e35])
                    - (self[e43] * self[e15])),
                (-(self[scalar] * self[e12]) + (self[e25] * self[e41]) - (self[e15] * self[e42]) - (self[e12] * self[scalar]) + (self[e31] * self[e23])
                    - (self[e23] * self[e31])
                    - (self[e41] * self[e25])
                    + (self[e42] * self[e15])),
                ((self[scalar] * self[e45]) + (self[e35] * self[e43]) + (self[e25] * self[e42]) + (self[e15] * self[e41]) + (self[e45] * self[scalar])
                    - (self[e43] * self[e35])
                    - (self[e41] * self[e15])
                    - (self[e42] * self[e25])),
            ]),
            // e235, e315, e125, e5
            ((Simd32x4::from(self[e35]) * Simd32x4::from([self[e31], self[e23], self[e45], self[e12]]))
                + (Simd32x4::from(self[e15]) * Simd32x4::from([self[e45], self[e12], self[e31], self[e23]]))
                + (Simd32x4::from(self[e25]) * Simd32x4::from([self[e12], self[e45], self[e23], self[e31]]))
                + Simd32x4::from([
                    (-(self[scalar] * self[e15]) - (self[e25] * self[e12]) - (self[e15] * self[scalar]) - (self[e15] * self[e45]) - (self[e31] * self[e35])),
                    (-(self[scalar] * self[e25]) - (self[e35] * self[e23]) - (self[e25] * self[scalar]) - (self[e25] * self[e45]) - (self[e12] * self[e15])),
                    (-(self[scalar] * self[e35]) - (self[e35] * self[scalar]) - (self[e35] * self[e45]) - (self[e15] * self[e31]) - (self[e23] * self[e25])),
                    ((self[e12] * self[e35]) + (self[e23] * self[e15]) + (self[e31] * self[e25])),
                ])),
            // e1, e2, e3, e4
            Simd32x4::from([
                ((self[e35] * self[e42]) - (self[e25] * self[e43]) + (self[e45] * self[e23]) + (self[e23] * self[e45]) + (self[e42] * self[e35]) - (self[e43] * self[e25])),
                (-(self[e35] * self[e41]) + (self[e15] * self[e43]) + (self[e45] * self[e31]) + (self[e31] * self[e45]) - (self[e41] * self[e35]) + (self[e43] * self[e15])),
                ((self[e25] * self[e41]) - (self[e15] * self[e42]) + (self[e45] * self[e12]) + (self[e12] * self[e45]) + (self[e41] * self[e25]) - (self[e42] * self[e15])),
                ((self[e12] * self[e43]) + (self[e31] * self[e42]) + (self[e23] * self[e41]) + (self[e43] * self[e12]) + (self[e41] * self[e23]) + (self[e42] * self[e31])),
            ]),
        );
    }
}
impl AntiSquare for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      133      142        0
    //    simd4       17       17        0
    // Totals...
    // yes simd      150      159        0
    //  no simd      201      210        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (-(Simd32x4::from(self[e412]) * Simd32x4::from([self[e2], self[e415], self[e321], self[e125]]))
                + (Simd32x4::from(self[e4]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]))
                + (Simd32x4::from(self[e4]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e5]]))
                - (Simd32x4::from(self[e423]) * Simd32x4::from([self[e321], self[e3], self[e425], self[e235]]))
                - (Simd32x4::from(self[e431]) * Simd32x4::from([self[e435], self[e321], self[e1], self[e315]]))
                + Simd32x4::from([
                    ((self[e3] * self[e431]) - (self[e1] * self[e4]) + (self[e425] * self[e412]) + (self[e415] * self[e4]) - (self[e412] * self[e2]) - (self[e412] * self[e425])
                        + (self[e431] * self[e3])
                        + (self[e423] * self[e321])
                        + (self[e431] * self[e435])),
                    (-(self[e2] * self[e4])
                        + (self[e1] * self[e412])
                        + (self[e435] * self[e423])
                        + (self[e425] * self[e4])
                        + (self[e412] * self[e1])
                        + (self[e412] * self[e415])
                        + (self[e431] * self[e321])
                        - (self[e423] * self[e435])
                        - (self[e423] * self[e3])),
                    (-(self[e3] * self[e4]) + (self[e2] * self[e423]) + (self[e435] * self[e4]) + (self[e415] * self[e431]) + (self[e412] * self[e321])
                        - (self[e431] * self[e1])
                        - (self[e431] * self[e415])
                        + (self[e423] * self[e425])
                        + (self[e423] * self[e2])),
                    (-f32::powi(self[e3], 2) - f32::powi(self[e2], 2) - f32::powi(self[e1], 2) + f32::powi(self[e321], 2)
                        - f32::powi(self[e435], 2)
                        - f32::powi(self[e425], 2)
                        - f32::powi(self[e415], 2)
                        - (self[e412] * self[e125])
                        - (self[e423] * self[e235])
                        - (self[e431] * self[e315])),
                ])),
            // e415, e425, e435, e321
            ((Simd32x4::from(self[e5]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
                + (Simd32x4::from(self[e235]) * Simd32x4::from([self[e4], self[e412], self[e431], self[e423]]))
                + (Simd32x4::from(self[e315]) * Simd32x4::from([self[e412], self[e4], self[e423], self[e431]]))
                + (Simd32x4::from(self[e125]) * Simd32x4::from([self[e431], self[e423], self[e4], self[e412]]))
                + Simd32x4::from([
                    (-(self[e3] * self[e2]) + (self[e2] * self[e3]) + (self[e1] * self[e321]) - (self[e125] * self[e431]) + (self[e235] * self[e4]) + (self[e321] * self[e1])
                        - (self[e435] * self[e425])
                        + (self[e425] * self[e435])
                        - (self[e412] * self[e315])
                        + (self[e423] * self[e5])),
                    ((self[e3] * self[e1]) + (self[e2] * self[e321]) - (self[e1] * self[e3]) + (self[e315] * self[e4]) - (self[e235] * self[e412])
                        + (self[e321] * self[e2])
                        + (self[e435] * self[e415])
                        - (self[e415] * self[e435])
                        - (self[e423] * self[e125])
                        + (self[e431] * self[e5])),
                    ((self[e3] * self[e321]) - (self[e2] * self[e1]) + (self[e1] * self[e2]) + (self[e125] * self[e4]) - (self[e315] * self[e423]) + (self[e321] * self[e3])
                        - (self[e425] * self[e415])
                        + (self[e415] * self[e425])
                        + (self[e412] * self[e5])
                        - (self[e431] * self[e235])),
                    (-(self[e3] * self[e435])
                        - (self[e2] * self[e425])
                        - (self[e1] * self[e415])
                        - (self[e4] * self[e5])
                        - (self[e125] * self[e412])
                        - (self[e315] * self[e431])
                        - (self[e235] * self[e423])
                        - (self[e435] * self[e3])
                        - (self[e425] * self[e2])
                        - (self[e415] * self[e1])),
                ])),
            // e235, e315, e125, e5
            (-(Simd32x4::from(self[e5]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e321]]))
                + (Simd32x4::from(self[e5]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]))
                - (Simd32x4::from(self[e315]) * Simd32x4::from([self[e3], self[e321], self[e415], self[e2]]))
                + (Simd32x4::from(self[e125]) * Simd32x4::from([self[e2], self[e415], self[e321], self[e3]]))
                - (Simd32x4::from(self[e125]) * Simd32x4::from([self[e425], self[e1], self[e321], self[e3]]))
                + (Simd32x4::from(self[e315]) * Simd32x4::from([self[e435], self[e321], self[e1], self[e2]]))
                - (Simd32x4::from(self[e235]) * Simd32x4::from([self[e321], self[e435], self[e2], self[e1]]))
                + (Simd32x4::from(self[e235]) * Simd32x4::from([self[e321], self[e3], self[e425], self[e1]]))
                + Simd32x4::from([
                    ((self[e1] * self[e5]) + (self[e125] * self[e2]) - (self[e315] * self[e3]) - (self[e435] * self[e315]) + (self[e415] * self[e5]) + (self[e425] * self[e125])),
                    ((self[e2] * self[e5]) - (self[e125] * self[e1]) + (self[e235] * self[e3]) + (self[e435] * self[e235]) - (self[e415] * self[e125]) + (self[e425] * self[e5])),
                    ((self[e3] * self[e5]) + (self[e315] * self[e1]) - (self[e235] * self[e2]) + (self[e435] * self[e5]) + (self[e415] * self[e315]) - (self[e425] * self[e235])),
                    (-(self[e125] * self[e435])
                        - (self[e315] * self[e425])
                        - (self[e235] * self[e415])
                        - (self[e435] * self[e125])
                        - (self[e415] * self[e235])
                        - (self[e425] * self[e315])),
                ])),
            // e1, e2, e3, e4
            Simd32x4::from([
                ((self[e5] * self[e423]) - (self[e3] * self[e425]) + (self[e2] * self[e435]) - (self[e125] * self[e431]) + (self[e315] * self[e412]) + (self[e321] * self[e415])
                    - (self[e435] * self[e2])
                    + (self[e425] * self[e3])
                    + (self[e415] * self[e321])
                    + (self[e412] * self[e315])
                    - (self[e423] * self[e5])
                    - (self[e431] * self[e125])),
                ((self[e5] * self[e431]) + (self[e3] * self[e415]) - (self[e1] * self[e435]) + (self[e125] * self[e423]) - (self[e235] * self[e412])
                    + (self[e321] * self[e425])
                    + (self[e435] * self[e1])
                    + (self[e425] * self[e321])
                    - (self[e415] * self[e3])
                    - (self[e412] * self[e235])
                    + (self[e423] * self[e125])
                    - (self[e431] * self[e5])),
                ((self[e5] * self[e412]) - (self[e2] * self[e415]) + (self[e1] * self[e425]) - (self[e315] * self[e423])
                    + (self[e235] * self[e431])
                    + (self[e321] * self[e435])
                    + (self[e435] * self[e321])
                    - (self[e425] * self[e1])
                    + (self[e415] * self[e2])
                    - (self[e412] * self[e5])
                    - (self[e423] * self[e315])
                    + (self[e431] * self[e235])),
                ((self[e3] * self[e412]) + (self[e2] * self[e431]) + (self[e1] * self[e423])
                    - (self[e435] * self[e412])
                    - (self[e425] * self[e431])
                    - (self[e415] * self[e423])
                    - (self[e412] * self[e3])
                    - (self[e412] * self[e435])
                    - (self[e431] * self[e2])
                    - (self[e431] * self[e425])
                    - (self[e423] * self[e415])
                    - (self[e423] * self[e1])),
            ]),
        );
    }
}
impl AntiSquare for AntiDualNum321 {
    type Output = DualNum321;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        2        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum321::from_groups(/* e321, e12345 */ Simd32x2::from([
            ((self[e45] * self[scalar]) + (self[scalar] * self[e45])),
            (-f32::powi(self[e45], 2) - f32::powi(self[scalar], 2)),
        ]));
    }
}
impl AntiSquare for AntiDualNum4 {
    type Output = DualNum4;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum4::from_groups(
            // e4, e12345
            Simd32x2::from([(-(self[e1234] * self[scalar]) - (self[scalar] * self[e1234])), (f32::powi(self[scalar], 2) * -1.0)]),
        );
    }
}
impl AntiSquare for AntiDualNum5 {
    type Output = DualNum5;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum5::from_groups(
            // e5, e12345
            Simd32x2::from([(-(self[e3215] * self[scalar]) - (self[scalar] * self[e3215])), (f32::powi(self[scalar], 2) * -1.0)]),
        );
    }
}
impl AntiSquare for AntiFlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        6        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, f32::powi(self[e321], 2)]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-(self[e235] * self[e321]) + (self[e321] * self[e235])),
                (-(self[e315] * self[e321]) + (self[e321] * self[e315])),
                (-(self[e125] * self[e321]) + (self[e321] * self[e125])),
                0.0,
            ]),
        );
    }
}
impl AntiSquare for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       36        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (-(self[e3] * self[e2]) + (self[e2] * self[e3]) + (self[e321] * self[e1]) + (self[e1] * self[e321])),
                ((self[e3] * self[e1]) + (self[e2] * self[e321]) + (self[e321] * self[e2]) - (self[e1] * self[e3])),
                ((self[e3] * self[e321]) - (self[e2] * self[e1]) + (self[e321] * self[e3]) + (self[e1] * self[e2])),
                (-f32::powi(self[e3], 2) - f32::powi(self[e2], 2) + f32::powi(self[e321], 2) - f32::powi(self[e1], 2)),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-(self[e3] * self[e315]) + (self[e2] * self[e125]) + (self[e321] * self[e235]) + (self[e125] * self[e2]) - (self[e235] * self[e321]) - (self[e315] * self[e3])),
                ((self[e3] * self[e235]) - (self[e1] * self[e125]) + (self[e321] * self[e315]) - (self[e125] * self[e1]) + (self[e235] * self[e3]) - (self[e315] * self[e321])),
                (-(self[e2] * self[e235]) + (self[e1] * self[e315]) + (self[e321] * self[e125]) - (self[e125] * self[e321]) - (self[e235] * self[e2]) + (self[e315] * self[e1])),
                (-(self[e3] * self[e125]) - (self[e2] * self[e315]) - (self[e1] * self[e235]) + (self[e125] * self[e3]) + (self[e235] * self[e1]) + (self[e315] * self[e2])),
            ]),
        );
    }
}
impl AntiSquare for AntiLine {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       19       24        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (-(self[e31] * self[e12]) + (self[e12] * self[e31])),
                ((self[e23] * self[e12]) - (self[e12] * self[e23])),
                (-(self[e23] * self[e31]) + (self[e31] * self[e23])),
                (f32::powi(self[e12], 2) + f32::powi(self[e23], 2) + f32::powi(self[e31], 2)),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                ((self[e35] * self[e31]) - (self[e25] * self[e12]) - (self[e31] * self[e35]) + (self[e12] * self[e25])),
                (-(self[e35] * self[e23]) + (self[e15] * self[e12]) + (self[e23] * self[e35]) - (self[e12] * self[e15])),
                ((self[e25] * self[e23]) - (self[e15] * self[e31]) - (self[e23] * self[e25]) + (self[e31] * self[e15])),
                ((self[e35] * self[e12]) + (self[e25] * self[e31]) + (self[e15] * self[e23]) + (self[e12] * self[e35]) + (self[e23] * self[e15]) + (self[e31] * self[e25])),
            ]),
        );
    }
}
impl AntiSquare for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       36        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       34       38        0
    //  no simd       40       44        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (-(self[scalar] * self[e23]) + (self[e12] * self[e31]) - (self[e23] * self[scalar]) - (self[e31] * self[e12])),
                (-(self[scalar] * self[e31]) - (self[e12] * self[e23]) + (self[e23] * self[e12]) - (self[e31] * self[scalar])),
                (-(self[scalar] * self[e12]) - (self[e12] * self[scalar]) - (self[e23] * self[e31]) + (self[e31] * self[e23])),
                (-f32::powi(self[scalar], 2) + f32::powi(self[e12], 2) + f32::powi(self[e23], 2) + f32::powi(self[e31], 2)),
            ]),
            // e235, e315, e125, e5
            (-(Simd32x4::from(self[e3215]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
                - (Simd32x4::from(self[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
                + Simd32x4::from([
                    ((self[e35] * self[e31]) - (self[e25] * self[e12]) - (self[scalar] * self[e15]) + (self[e12] * self[e25])
                        - (self[e23] * self[e3215])
                        - (self[e31] * self[e35])),
                    (-(self[e35] * self[e23]) + (self[e15] * self[e12]) - (self[scalar] * self[e25]) - (self[e12] * self[e15]) + (self[e23] * self[e35])
                        - (self[e31] * self[e3215])),
                    ((self[e25] * self[e23]) - (self[e15] * self[e31]) - (self[scalar] * self[e35]) - (self[e12] * self[e3215]) - (self[e23] * self[e25])
                        + (self[e31] * self[e15])),
                    ((self[e35] * self[e12]) + (self[e25] * self[e31]) + (self[e15] * self[e23]) + (self[e12] * self[e35]) + (self[e23] * self[e15]) + (self[e31] * self[e25])),
                ])),
        );
    }
}
impl AntiSquare for AntiPlane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                ((self[e2] * self[e3]) - (self[e3] * self[e2])),
                (-(self[e1] * self[e3]) + (self[e3] * self[e1])),
                ((self[e1] * self[e2]) - (self[e2] * self[e1])),
                (-f32::powi(self[e3], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2)),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                ((self[e1] * self[e5]) - (self[e5] * self[e1])),
                ((self[e2] * self[e5]) - (self[e5] * self[e2])),
                ((self[e3] * self[e5]) - (self[e5] * self[e3])),
                0.0,
            ]),
        );
    }
}
impl AntiSquare for AntiQuadNum {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       14        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return QuadNum::from_groups(/* e4, e5, e321, e12345 */ Simd32x4::from([
            (-(self[scalar] * self[e1234]) - (self[e45] * self[e1234]) + (self[e1234] * self[e45]) - (self[e1234] * self[scalar])),
            (-(self[scalar] * self[e3215]) + (self[e45] * self[e3215]) - (self[e3215] * self[e45]) - (self[e3215] * self[scalar])),
            ((self[scalar] * self[e45]) + (self[e45] * self[scalar]) + (self[e1234] * self[e3215]) - (self[e3215] * self[e1234])),
            (-f32::powi(self[scalar], 2) - f32::powi(self[e45], 2) - (self[e1234] * self[e3215]) - (self[e3215] * self[e1234])),
        ]));
    }
}
impl AntiSquare for AntiScalar {
    type Output = AntiScalar;
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e12345], 2));
    }
}
impl AntiSquare for AntiTripleNum {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        8        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return QuadNum::from_groups(/* e4, e5, e321, e12345 */ Simd32x4::from([
            (-(self[e1234] * self[scalar]) - (self[scalar] * self[e1234])),
            (-(self[e3215] * self[scalar]) - (self[scalar] * self[e3215])),
            ((self[e1234] * self[e3215]) - (self[e3215] * self[e1234])),
            (-f32::powi(self[scalar], 2) - (self[e1234] * self[e3215]) - (self[e3215] * self[e1234])),
        ]));
    }
}
impl AntiSquare for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       72        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       66       78        0
    //  no simd       84       96        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (-(Simd32x4::from(self[e423]) * Simd32x4::from([self[e321], self[e435], self[e425], self[e235]]))
                - (Simd32x4::from(self[e431]) * Simd32x4::from([self[e435], self[e321], self[e415], self[e315]]))
                - (Simd32x4::from(self[e412]) * Simd32x4::from([self[e425], self[e415], self[e321], self[e125]]))
                + Simd32x4::from([
                    ((self[e425] * self[e412]) + (self[e423] * self[e321]) + (self[e431] * self[e435])),
                    ((self[e435] * self[e423]) + (self[e412] * self[e415]) + (self[e431] * self[e321])),
                    ((self[e415] * self[e431]) + (self[e412] * self[e321]) + (self[e423] * self[e425])),
                    (f32::powi(self[e321], 2)
                        - f32::powi(self[e435], 2)
                        - f32::powi(self[e425], 2)
                        - f32::powi(self[e415], 2)
                        - (self[e412] * self[e125])
                        - (self[e423] * self[e235])
                        - (self[e431] * self[e315])),
                ])),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self[e125] * self[e431]) + (self[e315] * self[e412]) - (self[e435] * self[e425]) + (self[e425] * self[e435]) + (self[e431] * self[e125])
                    - (self[e412] * self[e315])),
                ((self[e125] * self[e423]) - (self[e235] * self[e412]) + (self[e435] * self[e415]) - (self[e415] * self[e435]) - (self[e423] * self[e125])
                    + (self[e412] * self[e235])),
                (-(self[e315] * self[e423]) + (self[e235] * self[e431]) - (self[e425] * self[e415]) + (self[e415] * self[e425]) + (self[e423] * self[e315])
                    - (self[e431] * self[e235])),
                (-(self[e125] * self[e412]) - (self[e315] * self[e431]) - (self[e235] * self[e423])
                    + (self[e412] * self[e125])
                    + (self[e423] * self[e235])
                    + (self[e431] * self[e315])),
            ]),
            // e235, e315, e125, e5
            (-(Simd32x4::from(self[e125]) * Simd32x4::from([self[e425], self[e415], self[e321], self[e435]]))
                - (Simd32x4::from(self[e235]) * Simd32x4::from([self[e321], self[e435], self[e425], self[e415]]))
                - (Simd32x4::from(self[e315]) * Simd32x4::from([self[e435], self[e321], self[e415], self[e425]]))
                + Simd32x4::from([
                    ((self[e315] * self[e435]) + (self[e321] * self[e235]) + (self[e425] * self[e125])),
                    ((self[e125] * self[e415]) + (self[e321] * self[e315]) + (self[e435] * self[e235])),
                    ((self[e235] * self[e425]) + (self[e321] * self[e125]) + (self[e415] * self[e315])),
                    (-(self[e435] * self[e125]) - (self[e415] * self[e235]) - (self[e425] * self[e315])),
                ])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(self[e125] * self[e431]) + (self[e315] * self[e412]) + (self[e321] * self[e415]) + (self[e415] * self[e321]) - (self[e431] * self[e125])
                    + (self[e412] * self[e315])),
                ((self[e125] * self[e423]) - (self[e235] * self[e412]) + (self[e321] * self[e425]) + (self[e425] * self[e321]) + (self[e423] * self[e125])
                    - (self[e412] * self[e235])),
                (-(self[e315] * self[e423]) + (self[e235] * self[e431]) + (self[e321] * self[e435]) + (self[e435] * self[e321]) - (self[e423] * self[e315])
                    + (self[e431] * self[e235])),
                (-(self[e435] * self[e412])
                    - (self[e425] * self[e431])
                    - (self[e415] * self[e423])
                    - (self[e412] * self[e435])
                    - (self[e423] * self[e415])
                    - (self[e431] * self[e425])),
            ]),
        );
    }
}
impl AntiSquare for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       73       84        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       80       92        0
    //  no simd      101      116        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (-(Simd32x4::from(self[e423]) * Simd32x4::from([self[e321], self[e435], self[e425], self[e235]]))
                - (Simd32x4::from(self[e431]) * Simd32x4::from([self[e435], self[e321], self[e415], self[e315]]))
                - (Simd32x4::from(self[e412]) * Simd32x4::from([self[e425], self[e415], self[e321], self[e125]]))
                + Simd32x4::from([
                    ((self[e12345] * self[e423]) + (self[e425] * self[e412]) + (self[e431] * self[e435]) + (self[e423] * self[e321]) + (self[e423] * self[e12345])),
                    ((self[e12345] * self[e431]) + (self[e435] * self[e423]) + (self[e412] * self[e415]) + (self[e431] * self[e12345]) + (self[e431] * self[e321])),
                    ((self[e12345] * self[e412]) + (self[e415] * self[e431]) + (self[e412] * self[e12345]) + (self[e412] * self[e321]) + (self[e423] * self[e425])),
                    (f32::powi(self[e12345], 2) + f32::powi(self[e321], 2)
                        - f32::powi(self[e435], 2)
                        - f32::powi(self[e425], 2)
                        - f32::powi(self[e415], 2)
                        - (self[e412] * self[e125])
                        - (self[e423] * self[e235])
                        - (self[e431] * self[e315])),
                ])),
            // e415, e425, e435, e321
            (Simd32x4::from(2.0) * (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]))
                + Simd32x4::from([
                    (-(self[e125] * self[e431]) + (self[e315] * self[e412]) - (self[e435] * self[e425]) + (self[e425] * self[e435]) + (self[e431] * self[e125])
                        - (self[e412] * self[e315])),
                    ((self[e125] * self[e423]) - (self[e235] * self[e412]) + (self[e435] * self[e415]) - (self[e415] * self[e435]) - (self[e423] * self[e125])
                        + (self[e412] * self[e235])),
                    (-(self[e315] * self[e423]) + (self[e235] * self[e431]) - (self[e425] * self[e415]) + (self[e415] * self[e425]) + (self[e423] * self[e315])
                        - (self[e431] * self[e235])),
                    (-(self[e125] * self[e412]) - (self[e315] * self[e431]) - (self[e235] * self[e423])
                        + (self[e412] * self[e125])
                        + (self[e423] * self[e235])
                        + (self[e431] * self[e315])),
                ])),
            // e235, e315, e125, e5
            (-(Simd32x4::from(self[e125]) * Simd32x4::from([self[e425], self[e415], self[e321], self[e435]]))
                - (Simd32x4::from(self[e235]) * Simd32x4::from([self[e321], self[e435], self[e425], self[e415]]))
                - (Simd32x4::from(self[e315]) * Simd32x4::from([self[e435], self[e321], self[e415], self[e425]]))
                + Simd32x4::from([
                    ((self[e12345] * self[e235]) + (self[e315] * self[e435]) + (self[e235] * self[e12345]) + (self[e321] * self[e235]) + (self[e425] * self[e125])),
                    ((self[e12345] * self[e315]) + (self[e125] * self[e415]) + (self[e315] * self[e12345]) + (self[e321] * self[e315]) + (self[e435] * self[e235])),
                    ((self[e12345] * self[e125]) + (self[e125] * self[e12345]) + (self[e235] * self[e425]) + (self[e321] * self[e125]) + (self[e415] * self[e315])),
                    (-(self[e435] * self[e125]) - (self[e415] * self[e235]) - (self[e425] * self[e315])),
                ])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(self[e125] * self[e431]) + (self[e315] * self[e412]) + (self[e321] * self[e415]) + (self[e415] * self[e321]) - (self[e431] * self[e125])
                    + (self[e412] * self[e315])),
                ((self[e125] * self[e423]) - (self[e235] * self[e412]) + (self[e321] * self[e425]) + (self[e425] * self[e321]) + (self[e423] * self[e125])
                    - (self[e412] * self[e235])),
                (-(self[e315] * self[e423]) + (self[e235] * self[e431]) + (self[e321] * self[e435]) + (self[e435] * self[e321]) - (self[e423] * self[e315])
                    + (self[e431] * self[e235])),
                (-(self[e435] * self[e412])
                    - (self[e425] * self[e431])
                    - (self[e415] * self[e423])
                    - (self[e412] * self[e435])
                    - (self[e423] * self[e415])
                    - (self[e431] * self[e425])),
            ]),
        );
    }
}
impl AntiSquare for Dipole {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       72        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       66       78        0
    //  no simd       84       96        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x4::from(self[e42]) * Simd32x4::from([self[e12], self[e45], self[e23], self[e25]]))
                + (Simd32x4::from(self[e43]) * Simd32x4::from([self[e31], self[e23], self[e45], self[e35]]))
                + (Simd32x4::from(self[e41]) * Simd32x4::from([self[e45], self[e12], self[e31], self[e15]]))
                + Simd32x4::from([
                    (-(self[e45] * self[e41]) - (self[e31] * self[e43]) - (self[e42] * self[e12])),
                    (-(self[e45] * self[e42]) - (self[e12] * self[e41]) - (self[e43] * self[e23])),
                    (-(self[e45] * self[e43]) - (self[e23] * self[e42]) - (self[e41] * self[e31])),
                    (-f32::powi(self[e45], 2)
                        + f32::powi(self[e12], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e23], 2)
                        + (self[e43] * self[e35])
                        + (self[e41] * self[e15])
                        + (self[e42] * self[e25])),
                ])),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((self[e35] * self[e42]) - (self[e25] * self[e43]) + (self[e12] * self[e31]) - (self[e31] * self[e12]) - (self[e42] * self[e35]) + (self[e43] * self[e25])),
                (-(self[e35] * self[e41]) + (self[e15] * self[e43]) - (self[e12] * self[e23]) + (self[e23] * self[e12]) + (self[e41] * self[e35]) - (self[e43] * self[e15])),
                ((self[e25] * self[e41]) - (self[e15] * self[e42]) + (self[e31] * self[e23]) - (self[e23] * self[e31]) - (self[e41] * self[e25]) + (self[e42] * self[e15])),
                ((self[e35] * self[e43]) + (self[e25] * self[e42]) + (self[e15] * self[e41]) - (self[e43] * self[e35]) - (self[e41] * self[e15]) - (self[e42] * self[e25])),
            ]),
            // e235, e315, e125, e5
            ((Simd32x4::from(self[e35]) * Simd32x4::from([self[e31], self[e23], self[e45], self[e12]]))
                + (Simd32x4::from(self[e15]) * Simd32x4::from([self[e45], self[e12], self[e31], self[e23]]))
                + (Simd32x4::from(self[e25]) * Simd32x4::from([self[e12], self[e45], self[e23], self[e31]]))
                + Simd32x4::from([
                    (-(self[e25] * self[e12]) - (self[e15] * self[e45]) - (self[e31] * self[e35])),
                    (-(self[e35] * self[e23]) - (self[e25] * self[e45]) - (self[e12] * self[e15])),
                    (-(self[e35] * self[e45]) - (self[e15] * self[e31]) - (self[e23] * self[e25])),
                    ((self[e12] * self[e35]) + (self[e23] * self[e15]) + (self[e31] * self[e25])),
                ])),
            // e1, e2, e3, e4
            Simd32x4::from([
                ((self[e35] * self[e42]) - (self[e25] * self[e43]) + (self[e45] * self[e23]) + (self[e23] * self[e45]) + (self[e42] * self[e35]) - (self[e43] * self[e25])),
                (-(self[e35] * self[e41]) + (self[e15] * self[e43]) + (self[e45] * self[e31]) + (self[e31] * self[e45]) - (self[e41] * self[e35]) + (self[e43] * self[e15])),
                ((self[e25] * self[e41]) - (self[e15] * self[e42]) + (self[e45] * self[e12]) + (self[e12] * self[e45]) + (self[e41] * self[e25]) - (self[e42] * self[e15])),
                ((self[e12] * self[e43]) + (self[e31] * self[e42]) + (self[e23] * self[e41]) + (self[e43] * self[e12]) + (self[e41] * self[e23]) + (self[e42] * self[e31])),
            ]),
        );
    }
}
impl AntiSquare for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      117      126        0
    //    simd4       19       19        0
    // Totals...
    // yes simd      136      145        0
    //  no simd      193      202        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x4::from(self[e42]) * Simd32x4::from([self[e4125], self[e45], self[e23], self[e25]]))
                - (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]))
                - (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e3215]]))
                + (Simd32x4::from(self[e43]) * Simd32x4::from([self[e31], self[e4235], self[e45], self[e35]]))
                + (Simd32x4::from(self[e41]) * Simd32x4::from([self[e45], self[e12], self[e4315], self[e15]]))
                + Simd32x4::from([
                    (-(self[e4315] * self[e43]) + (self[e1234] * self[e4235]) - (self[e45] * self[e41]) + (self[e12] * self[e42])
                        - (self[e31] * self[e43])
                        - (self[e23] * self[e1234])
                        - (self[e43] * self[e4315])
                        + (self[e42] * self[e4125])
                        - (self[e42] * self[e12])),
                    (-(self[e4125] * self[e41]) + (self[e1234] * self[e4315]) - (self[e45] * self[e42]) - (self[e12] * self[e41]) - (self[e31] * self[e1234])
                        + (self[e23] * self[e43])
                        + (self[e43] * self[e4235])
                        - (self[e43] * self[e23])
                        - (self[e41] * self[e4125])),
                    (-(self[e4235] * self[e42]) + (self[e1234] * self[e4125]) - (self[e45] * self[e43]) - (self[e12] * self[e1234]) + (self[e31] * self[e41])
                        - (self[e23] * self[e42])
                        - (self[e42] * self[e4235])
                        - (self[e41] * self[e31])
                        + (self[e41] * self[e4315])),
                    (f32::powi(self[e4125], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4235], 2) - f32::powi(self[e45], 2)
                        + f32::powi(self[e12], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e23], 2)
                        + (self[e43] * self[e35])
                        + (self[e41] * self[e15])
                        + (self[e42] * self[e25])),
                ])),
            // e415, e425, e435, e321
            (-(Simd32x4::from(self[e3215]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]))
                - (Simd32x4::from(self[e4125]) * Simd32x4::from([self[e4315], self[e4235], self[e45], self[e12]]))
                - (Simd32x4::from(self[e15]) * Simd32x4::from([self[e1234], self[e43], self[e42], self[e41]]))
                - (Simd32x4::from(self[e25]) * Simd32x4::from([self[e43], self[e1234], self[e41], self[e42]]))
                - (Simd32x4::from(self[e35]) * Simd32x4::from([self[e42], self[e41], self[e1234], self[e43]]))
                + Simd32x4::from([
                    ((self[e4125] * self[e4315]) - (self[e4235] * self[e45]) + (self[e35] * self[e42]) - (self[e15] * self[e1234]) - (self[e45] * self[e4235])
                        + (self[e12] * self[e31])
                        - (self[e31] * self[e12])
                        + (self[e43] * self[e25])
                        - (self[e41] * self[e3215])),
                    (-(self[e4315] * self[e45]) + (self[e4235] * self[e4125]) - (self[e25] * self[e1234]) + (self[e15] * self[e43])
                        - (self[e45] * self[e4315])
                        - (self[e12] * self[e23])
                        + (self[e23] * self[e12])
                        + (self[e41] * self[e35])
                        - (self[e42] * self[e3215])),
                    ((self[e4315] * self[e4235]) - (self[e4235] * self[e4315]) - (self[e35] * self[e1234]) + (self[e25] * self[e41]) - (self[e45] * self[e4125])
                        + (self[e31] * self[e23])
                        - (self[e23] * self[e31])
                        - (self[e43] * self[e3215])
                        + (self[e42] * self[e15])),
                    (-(self[e4315] * self[e31]) - (self[e4235] * self[e23])
                        + (self[e1234] * self[e3215])
                        + (self[e35] * self[e43])
                        + (self[e25] * self[e42])
                        + (self[e15] * self[e41])
                        - (self[e12] * self[e4125])
                        - (self[e31] * self[e4315])
                        - (self[e23] * self[e4235])),
                ])),
            // e235, e315, e125, e5
            (-(Simd32x4::from(self[e25]) * Simd32x4::from([self[e4125], self[e45], self[e23], self[e4315]]))
                + (Simd32x4::from(self[e35]) * Simd32x4::from([self[e4315], self[e23], self[e45], self[e4125]]))
                - (Simd32x4::from(self[e15]) * Simd32x4::from([self[e45], self[e12], self[e4315], self[e4235]]))
                + (Simd32x4::from(self[e15]) * Simd32x4::from([self[e45], self[e4125], self[e31], self[e4235]]))
                + (Simd32x4::from(self[e25]) * Simd32x4::from([self[e12], self[e45], self[e4235], self[e4315]]))
                - (Simd32x4::from(self[e35]) * Simd32x4::from([self[e31], self[e4235], self[e45], self[e4125]]))
                + Simd32x4::from([
                    (-(self[e3215] * self[e23]) + (self[e35] * self[e4315]) + (self[e35] * self[e31])
                        - (self[e25] * self[e4125])
                        - (self[e25] * self[e12])
                        - (self[e23] * self[e3215])),
                    (-(self[e3215] * self[e31]) - (self[e35] * self[e4235]) - (self[e35] * self[e23]) + (self[e15] * self[e4125]) + (self[e15] * self[e12])
                        - (self[e31] * self[e3215])),
                    (-(self[e3215] * self[e12]) + (self[e25] * self[e4235]) + (self[e25] * self[e23])
                        - (self[e15] * self[e4315])
                        - (self[e15] * self[e31])
                        - (self[e12] * self[e3215])),
                    ((self[e35] * self[e12]) + (self[e25] * self[e31]) + (self[e15] * self[e23]) + (self[e12] * self[e35]) + (self[e23] * self[e15]) + (self[e31] * self[e25])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self[e12]) * Simd32x4::from([self[e4315], self[e4235], self[e45], self[e43]]))
                + (Simd32x4::from(self[e23]) * Simd32x4::from([self[e45], self[e4125], self[e4315], self[e41]]))
                + (Simd32x4::from(self[e31]) * Simd32x4::from([self[e4125], self[e45], self[e4235], self[e42]]))
                + Simd32x4::from([
                    (-(self[e3215] * self[e41]) - (self[e4125] * self[e31]) + (self[e35] * self[e42]) - (self[e25] * self[e43]) - (self[e12] * self[e4315])
                        + (self[e23] * self[e45])
                        - (self[e43] * self[e25])
                        + (self[e41] * self[e3215])
                        + (self[e42] * self[e35])),
                    (-(self[e3215] * self[e42]) - (self[e4235] * self[e12]) - (self[e35] * self[e41]) + (self[e15] * self[e43]) + (self[e31] * self[e45])
                        - (self[e23] * self[e4125])
                        + (self[e43] * self[e15])
                        - (self[e41] * self[e35])
                        + (self[e42] * self[e3215])),
                    (-(self[e3215] * self[e43]) - (self[e4315] * self[e23]) + (self[e25] * self[e41]) - (self[e15] * self[e42]) + (self[e12] * self[e45])
                        - (self[e31] * self[e4235])
                        + (self[e43] * self[e3215])
                        + (self[e41] * self[e25])
                        - (self[e42] * self[e15])),
                    ((self[e4125] * self[e43]) + (self[e4315] * self[e42]) + (self[e4235] * self[e41]) - (self[e43] * self[e4125]) + (self[e43] * self[e12])
                        - (self[e42] * self[e4315])
                        + (self[e42] * self[e31])
                        + (self[e41] * self[e23])
                        - (self[e41] * self[e4235])),
                ])),
        );
    }
}
impl AntiSquare for DualNum321 {
    type Output = DualNum321;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        2        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum321::from_groups(/* e321, e12345 */ Simd32x2::from([
            ((self[e321] * self[e12345]) + (self[e12345] * self[e321])),
            (f32::powi(self[e321], 2) + f32::powi(self[e12345], 2)),
        ]));
    }
}
impl AntiSquare for DualNum4 {
    type Output = DualNum4;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum4::from_groups(
            // e4, e12345
            Simd32x2::from([((self[e4] * self[e12345]) + (self[e12345] * self[e4])), f32::powi(self[e12345], 2)]),
        );
    }
}
impl AntiSquare for DualNum5 {
    type Output = DualNum5;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum5::from_groups(
            // e5, e12345
            Simd32x2::from([((self[e5] * self[e12345]) + (self[e12345] * self[e5])), f32::powi(self[e12345], 2)]),
        );
    }
}
impl AntiSquare for FlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        7        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, (f32::powi(self[e45], 2) * -1.0)]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-(self[e15] * self[e45]) + (self[e45] * self[e15])),
                (-(self[e25] * self[e45]) + (self[e45] * self[e25])),
                (-(self[e35] * self[e45]) + (self[e45] * self[e35])),
                0.0,
            ]),
        );
    }
}
impl AntiSquare for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       36        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                ((self[e4125] * self[e4315]) - (self[e4315] * self[e4125]) - (self[e45] * self[e4235]) - (self[e4235] * self[e45])),
                (-(self[e4125] * self[e4235]) - (self[e4315] * self[e45]) - (self[e45] * self[e4315]) + (self[e4235] * self[e4125])),
                (-(self[e4125] * self[e45]) + (self[e4315] * self[e4235]) - (self[e45] * self[e4125]) - (self[e4235] * self[e4315])),
                (f32::powi(self[e4125], 2) + f32::powi(self[e4315], 2) - f32::powi(self[e45], 2) + f32::powi(self[e4235], 2)),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-(self[e4125] * self[e25]) + (self[e4315] * self[e35]) + (self[e45] * self[e15]) + (self[e35] * self[e4315])
                    - (self[e15] * self[e45])
                    - (self[e25] * self[e4125])),
                ((self[e4125] * self[e15]) - (self[e4235] * self[e35]) + (self[e45] * self[e25]) - (self[e35] * self[e4235]) + (self[e15] * self[e4125]) - (self[e25] * self[e45])),
                (-(self[e4315] * self[e15]) + (self[e4235] * self[e25]) + (self[e45] * self[e35]) - (self[e35] * self[e45]) - (self[e15] * self[e4315])
                    + (self[e25] * self[e4235])),
                (-(self[e4125] * self[e35]) - (self[e4315] * self[e25]) - (self[e4235] * self[e15])
                    + (self[e35] * self[e4125])
                    + (self[e15] * self[e4235])
                    + (self[e25] * self[e4315])),
            ]),
        );
    }
}
impl AntiSquare for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       19       24        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                ((self[e425] * self[e435]) - (self[e435] * self[e425])),
                (-(self[e415] * self[e435]) + (self[e435] * self[e415])),
                ((self[e415] * self[e425]) - (self[e425] * self[e415])),
                (-f32::powi(self[e435], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2)),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-(self[e125] * self[e425]) + (self[e315] * self[e435]) + (self[e425] * self[e125]) - (self[e435] * self[e315])),
                ((self[e125] * self[e415]) - (self[e235] * self[e435]) - (self[e415] * self[e125]) + (self[e435] * self[e235])),
                (-(self[e315] * self[e415]) + (self[e235] * self[e425]) + (self[e415] * self[e315]) - (self[e425] * self[e235])),
                (-(self[e125] * self[e435])
                    - (self[e315] * self[e425])
                    - (self[e235] * self[e415])
                    - (self[e435] * self[e125])
                    - (self[e415] * self[e235])
                    - (self[e425] * self[e315])),
            ]),
        );
    }
}
impl AntiSquare for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       36        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       34       38        0
    //  no simd       40       44        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                ((self[e12345] * self[e415]) - (self[e435] * self[e425]) + (self[e415] * self[e12345]) + (self[e425] * self[e435])),
                ((self[e12345] * self[e425]) + (self[e435] * self[e415]) - (self[e415] * self[e435]) + (self[e425] * self[e12345])),
                ((self[e12345] * self[e435]) + (self[e435] * self[e12345]) + (self[e415] * self[e425]) - (self[e425] * self[e415])),
                (f32::powi(self[e12345], 2) - f32::powi(self[e435], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2)),
            ]),
            // e235, e315, e125, e5
            ((Simd32x4::from(self[e5]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]))
                + (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]))
                + Simd32x4::from([
                    (-(self[e125] * self[e425]) + (self[e315] * self[e435]) + (self[e12345] * self[e235]) - (self[e435] * self[e315])
                        + (self[e415] * self[e5])
                        + (self[e425] * self[e125])),
                    ((self[e125] * self[e415]) - (self[e235] * self[e435]) + (self[e12345] * self[e315]) + (self[e435] * self[e235]) - (self[e415] * self[e125])
                        + (self[e425] * self[e5])),
                    (-(self[e315] * self[e415]) + (self[e235] * self[e425]) + (self[e12345] * self[e125]) + (self[e435] * self[e5]) + (self[e415] * self[e315])
                        - (self[e425] * self[e235])),
                    (-(self[e125] * self[e435])
                        - (self[e315] * self[e425])
                        - (self[e235] * self[e415])
                        - (self[e435] * self[e125])
                        - (self[e415] * self[e235])
                        - (self[e425] * self[e315])),
                ])),
        );
    }
}
impl AntiSquare for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      485      501        0
    //    simd2        8        8        0
    //    simd3       62       73        0
    //    simd4       44       45        0
    // Totals...
    // yes simd      599      627        0
    //  no simd      863      916        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            ((Simd32x2::from(self[e5]) * Simd32x2::from([self[e1234], self[e4]])) + (Simd32x2::from(self[e4]) * Simd32x2::from([self[e3215], self[e5]]))
                - (Simd32x2::from(self[e125]) * Simd32x2::from([self[e43], self[e412]]))
                - (Simd32x2::from(self[e315]) * Simd32x2::from([self[e42], self[e431]]))
                - (Simd32x2::from(self[e235]) * Simd32x2::from([self[e41], self[e423]]))
                - (Simd32x2::from(self[e412]) * Simd32x2::from([self[e35], self[e125]]))
                - (Simd32x2::from(self[e431]) * Simd32x2::from([self[e25], self[e315]]))
                - (Simd32x2::from(self[e423]) * Simd32x2::from([self[e15], self[e235]]))
                + Simd32x2::from([
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
                    (-(self[e1234] * self[e3215]) - (self[e3215] * self[e1234])
                        + f32::powi(self[e4125], 2)
                        + f32::powi(self[e4315], 2)
                        + f32::powi(self[e4235], 2)
                        + f32::powi(self[e321], 2)
                        - f32::powi(self[e435], 2)
                        - f32::powi(self[e425], 2)
                        - f32::powi(self[e415], 2)
                        + f32::powi(self[e12], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e23], 2)
                        + (self[e43] * self[e35])
                        + (self[e42] * self[e25])
                        + (self[e41] * self[e15])
                        - f32::powi(self[e45], 2)
                        + (self[e35] * self[e43])
                        + (self[e25] * self[e42])
                        + (self[e15] * self[e41])
                        - f32::powi(self[e3], 2)
                        - f32::powi(self[e2], 2)
                        - f32::powi(self[e1], 2)
                        - f32::powi(self[scalar], 2)
                        + f32::powi(self[e12345], 2)),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + (Simd32x4::from(self[e4315]) * Simd32x4::from([self[e12], self[scalar], self[e23], self[e42]]))
                + (Simd32x4::from(self[e4235]) * Simd32x4::from([self[scalar], self[e12], self[e31], self[e41]]))
                + (Simd32x4::from(self[e3]) * Simd32x4::from([self[e425], self[e415], self[e12345], self[e412]]))
                + (Simd32x4::from(self[e4125]) * Simd32x4::from([self[e31], self[e23], self[scalar], self[e43]]))
                - (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]))
                + (Simd32x4::from(self[e2]) * Simd32x4::from([self[e435], self[e12345], self[e415], self[e431]]))
                + (Simd32x4::from(self[e1]) * Simd32x4::from([self[e12345], self[e435], self[e425], self[e423]]))
                + (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]))
                + Simd32x4::from([
                    (-(self[e3215] * self[e41]) - (self[e4125] * self[e31]) - (self[e125] * self[e431]) + (self[e315] * self[e412]) + (self[e412] * self[e315])
                        - (self[e431] * self[e125])
                        - (self[e423] * self[e5])
                        + (self[e321] * self[e415])
                        - (self[e435] * self[e2])
                        + (self[e415] * self[e321])
                        - (self[e12] * self[e4315])
                        + (self[e23] * self[e45])
                        - (self[e43] * self[e25])
                        + (self[e42] * self[e35])
                        + (self[e41] * self[e3215])
                        + (self[e45] * self[e23])
                        + (self[e35] * self[e42])
                        - (self[e25] * self[e43])
                        + (self[e5] * self[e423])
                        - (self[e3] * self[e425])
                        + (self[scalar] * self[e4235])),
                    (-(self[e3215] * self[e42]) - (self[e4235] * self[e12]) + (self[e125] * self[e423])
                        - (self[e235] * self[e412])
                        - (self[e412] * self[e235])
                        - (self[e431] * self[e5])
                        + (self[e423] * self[e125])
                        + (self[e321] * self[e425])
                        + (self[e425] * self[e321])
                        - (self[e415] * self[e3])
                        + (self[e31] * self[e45])
                        - (self[e23] * self[e4125])
                        + (self[e43] * self[e15])
                        + (self[e42] * self[e3215])
                        - (self[e41] * self[e35])
                        + (self[e45] * self[e31])
                        - (self[e35] * self[e41])
                        + (self[e15] * self[e43])
                        + (self[e5] * self[e431])
                        - (self[e1] * self[e435])
                        + (self[scalar] * self[e4315])),
                    (-(self[e3215] * self[e43]) - (self[e4315] * self[e23]) - (self[e315] * self[e423]) + (self[e235] * self[e431]) - (self[e412] * self[e5])
                        + (self[e431] * self[e235])
                        - (self[e423] * self[e315])
                        + (self[e321] * self[e435])
                        + (self[e435] * self[e321])
                        - (self[e425] * self[e1])
                        + (self[e12] * self[e45])
                        - (self[e31] * self[e4235])
                        + (self[e43] * self[e3215])
                        - (self[e42] * self[e15])
                        + (self[e41] * self[e25])
                        + (self[e45] * self[e12])
                        + (self[e25] * self[e41])
                        - (self[e15] * self[e42])
                        + (self[e5] * self[e412])
                        - (self[e2] * self[e415])
                        + (self[scalar] * self[e4125])),
                    (-(self[e412] * self[e435])
                        - (self[e412] * self[e3])
                        - (self[e431] * self[e425])
                        - (self[e431] * self[e2])
                        - (self[e423] * self[e415])
                        - (self[e423] * self[e1])
                        - (self[e435] * self[e412])
                        - (self[e425] * self[e431])
                        - (self[e415] * self[e423])
                        + (self[e12] * self[e43])
                        + (self[e31] * self[e42])
                        + (self[e23] * self[e41])
                        - (self[e43] * self[e4125])
                        + (self[e43] * self[e12])
                        - (self[e42] * self[e4315])
                        + (self[e42] * self[e31])
                        - (self[e41] * self[e4235])
                        + (self[e41] * self[e23])
                        - (self[e45] * self[e1234])
                        - (self[scalar] * self[e1234])
                        + (self[e12345] * self[e4])),
                ])),
            // e5
            (-(self[e3215] * self[e45])
                - (self[e3215] * self[scalar])
                - (self[e4125] * self[e35])
                - (self[e4315] * self[e25])
                - (self[e4235] * self[e15])
                - (self[e125] * self[e435])
                + (self[e125] * self[e3])
                - (self[e315] * self[e425])
                + (self[e315] * self[e2])
                - (self[e235] * self[e415])
                + (self[e235] * self[e1])
                + (self[e321] * self[e5])
                - (self[e435] * self[e125])
                - (self[e425] * self[e315])
                - (self[e415] * self[e235])
                + (self[e12] * self[e35])
                + (self[e31] * self[e25])
                + (self[e23] * self[e15])
                + (self[e45] * self[e3215])
                + (self[e35] * self[e4125])
                + (self[e35] * self[e12])
                + (self[e25] * self[e4315])
                + (self[e25] * self[e31])
                + (self[e15] * self[e4235])
                + (self[e15] * self[e23])
                - (self[e5] * self[e321])
                + (self[e5] * self[e12345])
                - (self[e3] * self[e125])
                - (self[e2] * self[e315])
                - (self[e1] * self[e235])
                - (self[scalar] * self[e3215])
                + (self[e12345] * self[e5])),
            // e15, e25, e35, e45
            ((Simd32x4::from(self[e3215]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e4]]))
                - (Simd32x4::from(self[e3215]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]))
                + (Simd32x4::from(self[e315]) * Simd32x4::from([self[e4125], self[e45], self[e23], self[e42]]))
                - (Simd32x4::from(self[e125]) * Simd32x4::from([self[e4315], self[e23], self[e45], self[e43]]))
                - (Simd32x4::from(self[e5]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e1234]]))
                + (Simd32x4::from(self[e12]) * Simd32x4::from([self[e315], self[e235], self[e5], self[e3]]))
                - (Simd32x4::from(self[e315]) * Simd32x4::from([self[e12], self[e45], self[e4235], self[e42]]))
                + (Simd32x4::from(self[e125]) * Simd32x4::from([self[e31], self[e4235], self[e45], self[e43]]))
                + (Simd32x4::from(self[e5]) * Simd32x4::from([self[e23], self[e31], self[e4125], self[e1234]]))
                - (Simd32x4::from(self[e235]) * Simd32x4::from([self[e45], self[e4125], self[e31], self[e41]]))
                + Simd32x4::from(2.0) * (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                + Simd32x4::from([
                    (-(self[e125] * self[e4315]) - (self[e125] * self[e31])
                        + (self[e315] * self[e4125])
                        + (self[e235] * self[e45])
                        + (self[e235] * self[scalar])
                        + (self[e415] * self[e3215])
                        + (self[e35] * self[e2])
                        - (self[e25] * self[e3])
                        + (self[e5] * self[e4235])
                        + (self[e5] * self[e23])
                        - (self[e3] * self[e25])
                        + (self[e2] * self[e35])
                        + (self[e1] * self[e3215])
                        + (self[scalar] * self[e235])),
                    ((self[e125] * self[e4235]) + (self[e125] * self[e23]) + (self[e315] * self[scalar]) - (self[e235] * self[e4125]) - (self[e235] * self[e12])
                        + (self[e425] * self[e3215])
                        - (self[e35] * self[e1])
                        + (self[e15] * self[e3])
                        + (self[e5] * self[e4315])
                        + (self[e5] * self[e31])
                        + (self[e3] * self[e15])
                        + (self[e2] * self[e3215])
                        - (self[e1] * self[e35])
                        + (self[scalar] * self[e315])),
                    ((self[e4315] * self[e235]) + (self[e125] * self[scalar]) - (self[e315] * self[e4235]) - (self[e315] * self[e23])
                        + (self[e235] * self[e4315])
                        + (self[e235] * self[e31])
                        + (self[e435] * self[e3215])
                        + (self[e25] * self[e1])
                        - (self[e15] * self[e2])
                        + (self[e5] * self[e12])
                        + (self[e3] * self[e3215])
                        - (self[e2] * self[e15])
                        + (self[e1] * self[e25])
                        + (self[scalar] * self[e125])),
                    (-(self[e4125] * self[e435]) - (self[e4315] * self[e425]) - (self[e4235] * self[e415]) + (self[e235] * self[e41])
                        - (self[e321] * self[scalar])
                        - (self[e435] * self[e4125])
                        - (self[e425] * self[e4315])
                        - (self[e415] * self[e4235])
                        + (self[e31] * self[e2])
                        + (self[e23] * self[e1])
                        + (self[e3] * self[e12])
                        + (self[e2] * self[e31])
                        + (self[e1] * self[e23])
                        - (self[scalar] * self[e321])),
                ])),
            // e41, e42, e43
            ((Simd32x3::from(self[e1234]) * Simd32x3::from([self[e415], self[e425], self[e435]]))
                - (Simd32x3::from(self[e431]) * Simd32x3::from([self[e4125], self[e45], self[e23]]))
                + (Simd32x3::from(self[e412]) * Simd32x3::from([self[e4315], self[e23], self[scalar]]))
                - (Simd32x3::from(self[e412]) * Simd32x3::from([self[e31], self[e4235], self[e45]]))
                + (Simd32x3::from(self[e431]) * Simd32x3::from([self[e12], self[scalar], self[e4235]]))
                - (Simd32x3::from(self[e423]) * Simd32x3::from([self[e45], self[e12], self[e4315]]))
                + (Simd32x3::from(self[e423]) * Simd32x3::from([self[scalar], self[e4125], self[e31]]))
                + (Simd32x3::from(self[e425]) * Simd32x3::from([self[e43], self[e1234], self[e41]]))
                + (Simd32x3::from(self[e415]) * Simd32x3::from([self[e1234], self[e43], self[e42]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * Simd32x3::from([self[e23], self[e31], self[e12]]))
                + (Simd32x3::from(self[e435]) * Simd32x3::from([self[e42], self[e41], self[e1234]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]))
                + (Simd32x3::from(self[e45]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                + (Simd32x3::from(self[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                + Simd32x3::from([
                    ((self[e412] * self[e4315]) - (self[e431] * self[e4125]) - (self[e435] * self[e42]) - (self[e12] * self[e431]) + (self[e31] * self[e412])
                        - (self[e43] * self[e425])
                        - (self[e43] * self[e2])
                        + (self[e42] * self[e3])
                        + (self[e3] * self[e42])
                        - (self[e2] * self[e43])),
                    (-(self[e412] * self[e4235]) + (self[e423] * self[e4125]) - (self[e415] * self[e43]) + (self[e12] * self[e423]) - (self[e23] * self[e412])
                        + (self[e43] * self[e1])
                        - (self[e41] * self[e435])
                        - (self[e41] * self[e3])
                        - (self[e3] * self[e41])
                        + (self[e1] * self[e43])),
                    ((self[e431] * self[e4235]) - (self[e423] * self[e4315]) - (self[e425] * self[e41]) - (self[e31] * self[e423]) + (self[e23] * self[e431])
                        - (self[e42] * self[e415])
                        - (self[e42] * self[e1])
                        + (self[e41] * self[e2])
                        + (self[e2] * self[e41])
                        - (self[e1] * self[e42])),
                ])),
            // e23, e31, e12
            ((Simd32x3::from(self[e1234]) * Simd32x3::from([self[e235], self[e315], self[e125]]))
                + (Simd32x3::from(self[e3215]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                - (Simd32x3::from(self[e4315]) * Simd32x3::from([self[e3], self[e321], self[e1]]))
                - (Simd32x3::from(self[e321]) * Simd32x3::from([self[e4235], self[e4315], self[e4125]]))
                + (Simd32x3::from(self[e315]) * Simd32x3::from([self[e43], self[e1234], self[e41]]))
                + (Simd32x3::from(self[e235]) * Simd32x3::from([self[e1234], self[e43], self[e42]]))
                + (Simd32x3::from(self[e431]) * Simd32x3::from([self[e35], self[e3215], self[e15]]))
                + (Simd32x3::from(self[e423]) * Simd32x3::from([self[e3215], self[e35], self[e25]]))
                - (Simd32x3::from(self[e4235]) * Simd32x3::from([self[e321], self[e3], self[e2]]))
                + (Simd32x3::from(self[e425]) * Simd32x3::from([self[e12], self[scalar], self[e23]]))
                + (Simd32x3::from(self[e415]) * Simd32x3::from([self[scalar], self[e12], self[e31]]))
                + (Simd32x3::from(self[e435]) * Simd32x3::from([self[e31], self[e23], self[scalar]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]))
                + (Simd32x3::from(self[e125]) * Simd32x3::from([self[e42], self[e41], self[e1234]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e5]) * Simd32x3::from([self[e41], self[e42], self[e43]]))
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e45]) * Simd32x3::from([self[e1], self[e2], self[e3]]))
                + (Simd32x3::from(self[e412]) * Simd32x3::from([self[e25], self[e15], self[e3215]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * Simd32x3::from([self[e15], self[e25], self[e35]]))
                - (Simd32x3::from(self[e4125]) * Simd32x3::from([self[e2], self[e1], self[e321]]))
                + (Simd32x3::from(self[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]))
                + Simd32x3::from([
                    ((self[e4125] * self[e2])
                        - (self[e125] * self[e42])
                        - (self[e412] * self[e25])
                        - (self[e435] * self[e31])
                        - (self[e12] * self[e425])
                        - (self[e43] * self[e315])
                        - (self[e35] * self[e431])
                        + (self[e3] * self[e4315])),
                    ((self[e4235] * self[e3])
                        - (self[e235] * self[e43])
                        - (self[e423] * self[e35])
                        - (self[e415] * self[e12])
                        - (self[e23] * self[e435])
                        - (self[e41] * self[e125])
                        - (self[e15] * self[e412])
                        + (self[e1] * self[e4125])),
                    ((self[e4315] * self[e1])
                        - (self[e315] * self[e41])
                        - (self[e431] * self[e15])
                        - (self[e425] * self[e23])
                        - (self[e31] * self[e415])
                        - (self[e42] * self[e235])
                        - (self[e25] * self[e423])
                        + (self[e2] * self[e4235])),
                ])),
            // e415, e425, e435, e321
            (-(Simd32x4::from(self[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
                - (Simd32x4::from(self[e41]) * Simd32x4::from([self[e3215], self[e35], self[e25], self[e15]]))
                - (Simd32x4::from(self[e4125]) * Simd32x4::from([self[e4315], self[e4235], self[e45], self[e12]]))
                + (Simd32x4::from(self[e315]) * Simd32x4::from([self[e412], self[e4], self[e423], self[e431]]))
                + (Simd32x4::from(self[e235]) * Simd32x4::from([self[e4], self[e412], self[e431], self[e423]]))
                + (Simd32x4::from(self[e125]) * Simd32x4::from([self[e431], self[e423], self[e4], self[e412]]))
                + (Simd32x4::from(self[e5]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]))
                + (Simd32x4::from(self[e321]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e12345]]))
                + (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]))
                - (Simd32x4::from(self[e12]) * Simd32x4::from([self[e31], self[e23], self[scalar], self[e4125]]))
                - (Simd32x4::from(self[e42]) * Simd32x4::from([self[e35], self[e3215], self[e15], self[e25]]))
                - (Simd32x4::from(self[e43]) * Simd32x4::from([self[e25], self[e15], self[e3215], self[e35]]))
                + Simd32x4::from([
                    ((self[e4125] * self[e4315]) - (self[e4235] * self[e45]) - (self[e125] * self[e431]) - (self[e412] * self[e315]) - (self[e435] * self[e425])
                        + (self[e425] * self[e435])
                        + (self[e12] * self[e31])
                        - (self[e23] * self[scalar])
                        + (self[e43] * self[e25])
                        - (self[e41] * self[e3215])
                        - (self[e45] * self[e4235])
                        + (self[e35] * self[e42])
                        - (self[e15] * self[e1234])
                        + (self[e5] * self[e423])
                        + (self[e4] * self[e235])
                        - (self[e3] * self[e2])
                        + (self[e2] * self[e3])
                        + (self[e1] * self[e321])
                        - (self[scalar] * self[e23])
                        + (self[e12345] * self[e415])),
                    (-(self[e4315] * self[e45]) + (self[e4235] * self[e4125]) - (self[e235] * self[e412]) - (self[e423] * self[e125]) + (self[e435] * self[e415])
                        - (self[e415] * self[e435])
                        - (self[e31] * self[scalar])
                        + (self[e23] * self[e12])
                        - (self[e42] * self[e3215])
                        + (self[e41] * self[e35])
                        - (self[e45] * self[e4315])
                        - (self[e25] * self[e1234])
                        + (self[e15] * self[e43])
                        + (self[e5] * self[e431])
                        + (self[e4] * self[e315])
                        + (self[e3] * self[e1])
                        + (self[e2] * self[e321])
                        - (self[e1] * self[e3])
                        - (self[scalar] * self[e31])
                        + (self[e12345] * self[e425])),
                    ((self[e4315] * self[e4235]) - (self[e4235] * self[e4315]) - (self[e315] * self[e423]) - (self[e431] * self[e235]) - (self[e425] * self[e415])
                        + (self[e415] * self[e425])
                        + (self[e31] * self[e23])
                        - (self[e23] * self[e31])
                        - (self[e43] * self[e3215])
                        + (self[e42] * self[e15])
                        - (self[e45] * self[e4125])
                        - (self[e35] * self[e1234])
                        + (self[e25] * self[e41])
                        + (self[e5] * self[e412])
                        + (self[e4] * self[e125])
                        + (self[e3] * self[e321])
                        - (self[e2] * self[e1])
                        + (self[e1] * self[e2])
                        - (self[scalar] * self[e12])
                        + (self[e12345] * self[e435])),
                    ((self[e1234] * self[e3215])
                        - (self[e4315] * self[e31])
                        - (self[e4235] * self[e23])
                        - (self[e125] * self[e412])
                        - (self[e315] * self[e431])
                        - (self[e235] * self[e423])
                        - (self[e435] * self[e3])
                        - (self[e425] * self[e2])
                        - (self[e415] * self[e1])
                        - (self[e31] * self[e4315])
                        - (self[e23] * self[e4235])
                        + (self[e45] * self[scalar])
                        + (self[e35] * self[e43])
                        + (self[e25] * self[e42])
                        + (self[e15] * self[e41])
                        - (self[e4] * self[e5])
                        - (self[e3] * self[e435])
                        - (self[e2] * self[e425])
                        - (self[e1] * self[e415])
                        + (self[scalar] * self[e45])),
                ])),
            // e423, e431, e412
            (-(Simd32x3::from(self[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]))
                + (Simd32x3::from(self[e42]) * Simd32x3::from([self[e4125], self[e45], self[e23]]))
                - (Simd32x3::from(self[e43]) * Simd32x3::from([self[e4315], self[e23], self[scalar]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * Simd32x3::from([self[e415], self[e425], self[e435]]))
                - (Simd32x3::from(self[e31]) * Simd32x3::from([self[e43], self[e1234], self[e41]]))
                + (Simd32x3::from(self[e43]) * Simd32x3::from([self[e31], self[e4235], self[e45]]))
                - (Simd32x3::from(self[e12]) * Simd32x3::from([self[e42], self[e41], self[e1234]]))
                + (Simd32x3::from(self[e41]) * Simd32x3::from([self[e45], self[e12], self[e4315]]))
                - (Simd32x3::from(self[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]))
                - (Simd32x3::from(self[e45]) * Simd32x3::from([self[e41], self[e42], self[e43]]))
                + Simd32x3::from([
                    (-(self[e412] * self[e2]) + (self[e431] * self[e3]) + (self[e12] * self[e42]) - (self[e23] * self[e1234]) - (self[e43] * self[e4315])
                        + (self[e42] * self[e4125])
                        + (self[e3] * self[e431])
                        - (self[e2] * self[e412])
                        - (self[scalar] * self[e41])),
                    (-(self[e4125] * self[e41]) + (self[e412] * self[e1]) - (self[e423] * self[e3]) + (self[e23] * self[e43]) + (self[e43] * self[e4235])
                        - (self[e41] * self[e4125])
                        - (self[e3] * self[e423])
                        + (self[e1] * self[e412])
                        - (self[scalar] * self[e42])),
                    (-(self[e4235] * self[e42]) - (self[e431] * self[e1]) + (self[e423] * self[e2]) + (self[e31] * self[e41])
                        - (self[e23] * self[e42])
                        - (self[e42] * self[e4235])
                        + (self[e41] * self[e4315])
                        + (self[e2] * self[e423])
                        - (self[e1] * self[e431])),
                ])),
            // e235, e315, e125
            (Simd32x3::from(-2.0) * (Simd32x3::from(self[e3215]) * Simd32x3::from([self[e23], self[e31], self[e12]]))
                - (Simd32x3::from(self[e25]) * Simd32x3::from([self[e4125], self[e45], self[e23]]))
                + (Simd32x3::from(self[e35]) * Simd32x3::from([self[e4315], self[e23], self[e45]]))
                - (Simd32x3::from(self[e125]) * Simd32x3::from([self[e425], self[e1], self[e321]]))
                + (Simd32x3::from(self[e125]) * Simd32x3::from([self[e2], self[e415], self[e12345]]))
                + (Simd32x3::from(self[e315]) * Simd32x3::from([self[e435], self[e12345], self[e1]]))
                - (Simd32x3::from(self[e315]) * Simd32x3::from([self[e3], self[e321], self[e415]]))
                - (Simd32x3::from(self[e235]) * Simd32x3::from([self[e321], self[e435], self[e2]]))
                + (Simd32x3::from(self[e235]) * Simd32x3::from([self[e12345], self[e3], self[e425]]))
                + (Simd32x3::from(self[e321]) * Simd32x3::from([self[e235], self[e315], self[e125]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e5]) * Simd32x3::from([self[e415], self[e425], self[e435]]))
                + (Simd32x3::from(self[e25]) * Simd32x3::from([self[e12], self[e45], self[e4235]]))
                - (Simd32x3::from(self[e35]) * Simd32x3::from([self[e31], self[e4235], self[e45]]))
                + (Simd32x3::from(self[e15]) * Simd32x3::from([self[e45], self[e4125], self[e31]]))
                - (Simd32x3::from(self[e15]) * Simd32x3::from([self[e45], self[e12], self[e4315]]))
                - Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]))
                + (Simd32x3::from(self[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]))
                + Simd32x3::from([
                    (-(self[e435] * self[e315]) + (self[e425] * self[e125]) + (self[e35] * self[e4315]) + (self[e35] * self[e31])
                        - (self[e25] * self[e4125])
                        - (self[e25] * self[e12])
                        - (self[e3] * self[e315])
                        + (self[e2] * self[e125])),
                    ((self[e435] * self[e235]) - (self[e415] * self[e125]) - (self[e35] * self[e4235]) - (self[e35] * self[e23])
                        + (self[e15] * self[e4125])
                        + (self[e15] * self[e12])
                        + (self[e3] * self[e235])
                        - (self[e1] * self[e125])),
                    (-(self[e425] * self[e235]) + (self[e415] * self[e315]) + (self[e25] * self[e4235]) + (self[e25] * self[e23])
                        - (self[e15] * self[e4315])
                        - (self[e15] * self[e31])
                        - (self[e2] * self[e235])
                        + (self[e1] * self[e315])),
                ])),
            // e4235, e4315, e4125, e3215
            (-(Simd32x4::from(self[e3215]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e321]]))
                + (Simd32x4::from(self[e4315]) * Simd32x4::from([self[e435], self[e12345], self[e415], self[e315]]))
                + (Simd32x4::from(self[e4235]) * Simd32x4::from([self[e12345], self[e435], self[e425], self[e235]]))
                + (Simd32x4::from(self[e3215]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e12345]]))
                - (Simd32x4::from(self[e23]) * Simd32x4::from([self[e321], self[e3], self[e2], self[e235]]))
                + (Simd32x4::from(self[e4125]) * Simd32x4::from([self[e425], self[e415], self[e12345], self[e125]]))
                + (Simd32x4::from(self[e45]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e5]]))
                - (Simd32x4::from(self[e31]) * Simd32x4::from([self[e3], self[e321], self[e1], self[e315]]))
                + (Simd32x4::from(self[e5]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]))
                - (Simd32x4::from(self[e5]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e45]]))
                - (Simd32x4::from(self[e12]) * Simd32x4::from([self[e2], self[e1], self[e321], self[e125]]))
                + (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]))
                + Simd32x4::from([
                    ((self[e1234] * self[e235]) - (self[e4125] * self[e425]) + (self[e125] * self[e42])
                        - (self[e315] * self[e43])
                        - (self[e235] * self[e1234])
                        - (self[e412] * self[e25])
                        + (self[e431] * self[e35])
                        - (self[e435] * self[e4315])
                        + (self[e12] * self[e2])
                        - (self[e23] * self[e321])
                        - (self[e43] * self[e315])
                        + (self[e42] * self[e125])
                        + (self[e45] * self[e415])
                        + (self[e35] * self[e431])
                        - (self[e25] * self[e412])
                        - (self[e15] * self[e4])
                        + (self[e4] * self[e15])
                        + (self[e3] * self[e31])
                        - (self[e1] * self[scalar])
                        - (self[scalar] * self[e1])),
                    ((self[e1234] * self[e315]) - (self[e4235] * self[e435]) - (self[e125] * self[e41]) - (self[e315] * self[e1234])
                        + (self[e235] * self[e43])
                        + (self[e412] * self[e15])
                        - (self[e423] * self[e35])
                        - (self[e415] * self[e4125])
                        - (self[e31] * self[e321])
                        + (self[e23] * self[e3])
                        + (self[e43] * self[e235])
                        - (self[e41] * self[e125])
                        + (self[e45] * self[e425])
                        - (self[e35] * self[e423])
                        - (self[e25] * self[e4])
                        + (self[e15] * self[e412])
                        + (self[e4] * self[e25])
                        - (self[e2] * self[scalar])
                        + (self[e1] * self[e12])
                        - (self[scalar] * self[e2])),
                    ((self[e1234] * self[e125]) - (self[e4315] * self[e415]) - (self[e125] * self[e1234]) + (self[e315] * self[e41])
                        - (self[e235] * self[e42])
                        - (self[e431] * self[e15])
                        + (self[e423] * self[e25])
                        - (self[e425] * self[e4235])
                        - (self[e12] * self[e321])
                        + (self[e31] * self[e1])
                        - (self[e42] * self[e235])
                        + (self[e41] * self[e315])
                        + (self[e45] * self[e435])
                        - (self[e35] * self[e4])
                        + (self[e25] * self[e423])
                        - (self[e15] * self[e431])
                        + (self[e4] * self[e35])
                        - (self[e3] * self[scalar])
                        + (self[e2] * self[e23])
                        - (self[scalar] * self[e3])),
                    (-(self[e125] * self[e4125]) - (self[e315] * self[e4315]) - (self[e235] * self[e4235]) + (self[e321] * self[e3215])
                        - (self[e435] * self[e35])
                        - (self[e425] * self[e25])
                        - (self[e415] * self[e15])
                        - (self[e12] * self[e125])
                        - (self[e31] * self[e315])
                        - (self[e23] * self[e235])
                        - (self[e35] * self[e435])
                        + (self[e35] * self[e3])
                        - (self[e25] * self[e425])
                        + (self[e25] * self[e2])
                        - (self[e15] * self[e415])
                        + (self[e15] * self[e1])
                        - (self[e3] * self[e35])
                        - (self[e2] * self[e25])
                        - (self[e1] * self[e15])
                        + (self[scalar] * self[e5])),
                ])),
            // e1234
            ((self[e1234] * self[e321]) + (self[e1234] * self[e12345]) - (self[e4125] * self[e412]) - (self[e4315] * self[e431]) - (self[e4235] * self[e423])
                + (self[e412] * self[e4125])
                - (self[e412] * self[e12])
                + (self[e431] * self[e4315])
                - (self[e431] * self[e31])
                + (self[e423] * self[e4235])
                - (self[e423] * self[e23])
                - (self[e321] * self[e1234])
                - (self[e435] * self[e43])
                - (self[e425] * self[e42])
                - (self[e415] * self[e41])
                - (self[e12] * self[e412])
                - (self[e31] * self[e431])
                - (self[e23] * self[e423])
                - (self[e43] * self[e435])
                - (self[e43] * self[e3])
                - (self[e42] * self[e425])
                - (self[e42] * self[e2])
                - (self[e41] * self[e415])
                - (self[e41] * self[e1])
                + (self[e45] * self[e4])
                - (self[e4] * self[e45])
                + (self[e4] * self[scalar])
                + (self[e3] * self[e43])
                + (self[e2] * self[e42])
                + (self[e1] * self[e41])
                + (self[scalar] * self[e4])
                + (self[e12345] * self[e1234])),
        );
    }
}
impl AntiSquare for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       12        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (-(self[e4315] * self[e4125]) + (self[e4125] * self[e4315])),
                ((self[e4235] * self[e4125]) - (self[e4125] * self[e4235])),
                (-(self[e4235] * self[e4315]) + (self[e4315] * self[e4235])),
                (f32::powi(self[e4125], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2)),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                ((self[e4235] * self[e3215]) - (self[e3215] * self[e4235])),
                ((self[e4315] * self[e3215]) - (self[e3215] * self[e4315])),
                ((self[e4125] * self[e3215]) - (self[e3215] * self[e4125])),
                0.0,
            ]),
        );
    }
}
impl AntiSquare for QuadNum {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       14        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return QuadNum::from_groups(/* e4, e5, e321, e12345 */ Simd32x4::from([
            ((self[e12345] * self[e4]) - (self[e321] * self[e4]) + (self[e4] * self[e321]) + (self[e4] * self[e12345])),
            ((self[e12345] * self[e5]) + (self[e321] * self[e5]) - (self[e5] * self[e321]) + (self[e5] * self[e12345])),
            ((self[e12345] * self[e321]) + (self[e321] * self[e12345]) - (self[e4] * self[e5]) + (self[e5] * self[e4])),
            (f32::powi(self[e12345], 2) + f32::powi(self[e321], 2) + (self[e4] * self[e5]) + (self[e5] * self[e4])),
        ]));
    }
}
impl AntiSquare for RoundPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       15        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       16        0
    //  no simd       11       19        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((self[e2] * self[e3]) - (self[e3] * self[e2])),
                (-(self[e1] * self[e3]) + (self[e3] * self[e1])),
                ((self[e1] * self[e2]) - (self[e2] * self[e1])),
                (-(self[e4] * self[e5]) + (self[e5] * self[e4])),
            ]),
            // e235, e315, e125, e12345
            ((Simd32x4::from(self[e5]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]))
                + Simd32x4::from([
                    ((self[e5] * self[e1]) * -1.0),
                    ((self[e5] * self[e2]) * -1.0),
                    ((self[e5] * self[e3]) * -1.0),
                    ((self[e4] * self[e5]) - f32::powi(self[e3], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2)),
                ])),
        );
    }
}
impl AntiSquare for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (f32::powi(self[scalar], 2) * -1.0));
    }
}
impl AntiSquare for Sphere {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self[e4315] * self[e4125]) + (self[e4125] * self[e4315])),
                ((self[e4235] * self[e4125]) - (self[e4125] * self[e4235])),
                (-(self[e4235] * self[e4315]) + (self[e4315] * self[e4235])),
                (-(self[e3215] * self[e1234]) + (self[e1234] * self[e3215])),
            ]),
            // e235, e315, e125, e12345
            (-(Simd32x4::from(self[e3215]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e1234]]))
                + Simd32x4::from([
                    (self[e4235] * self[e3215]),
                    (self[e4315] * self[e3215]),
                    (self[e4125] * self[e3215]),
                    (-(self[e3215] * self[e1234]) + f32::powi(self[e4125], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2)),
                ])),
        );
    }
}
impl AntiSquare for TripleNum {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        8        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return QuadNum::from_groups(/* e4, e5, e321, e12345 */ Simd32x4::from([
            ((self[e4] * self[e12345]) + (self[e12345] * self[e4])),
            ((self[e5] * self[e12345]) + (self[e12345] * self[e5])),
            (-(self[e4] * self[e5]) + (self[e5] * self[e4])),
            (f32::powi(self[e12345], 2) + (self[e4] * self[e5]) + (self[e5] * self[e4])),
        ]));
    }
}
impl AntiSquare for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      132      140        0
    //    simd4       24       25        0
    // Totals...
    // yes simd      156      165        0
    //  no simd      228      240        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x4::from(self[e4]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e5]]))
                + (Simd32x4::from(self[e4]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e5]]))
                - (Simd32x4::from(self[e412]) * Simd32x4::from([self[e2], self[e415], self[e321], self[e125]]))
                - (Simd32x4::from(self[e423]) * Simd32x4::from([self[e321], self[e3], self[e425], self[e235]]))
                - (Simd32x4::from(self[e431]) * Simd32x4::from([self[e435], self[e321], self[e1], self[e315]]))
                + Simd32x4::from([
                    ((self[e3] * self[e431]) - (self[e1] * self[e4]) + (self[e425] * self[e412]) + (self[e415] * self[e4]) + (self[e12345] * self[e423])
                        - (self[e412] * self[e2])
                        - (self[e412] * self[e425])
                        + (self[e431] * self[e3])
                        + (self[e431] * self[e435])
                        + (self[e423] * self[e12345])
                        + (self[e423] * self[e321])),
                    (-(self[e2] * self[e4])
                        + (self[e1] * self[e412])
                        + (self[e435] * self[e423])
                        + (self[e425] * self[e4])
                        + (self[e12345] * self[e431])
                        + (self[e412] * self[e1])
                        + (self[e412] * self[e415])
                        + (self[e431] * self[e321])
                        + (self[e431] * self[e12345])
                        - (self[e423] * self[e435])
                        - (self[e423] * self[e3])),
                    (-(self[e3] * self[e4])
                        + (self[e2] * self[e423])
                        + (self[e435] * self[e4])
                        + (self[e415] * self[e431])
                        + (self[e12345] * self[e412])
                        + (self[e412] * self[e321])
                        + (self[e412] * self[e12345])
                        - (self[e431] * self[e1])
                        - (self[e431] * self[e415])
                        + (self[e423] * self[e425])
                        + (self[e423] * self[e2])),
                    (-f32::powi(self[e3], 2) - f32::powi(self[e2], 2) - f32::powi(self[e1], 2) + f32::powi(self[e321], 2)
                        - f32::powi(self[e435], 2)
                        - f32::powi(self[e425], 2)
                        - f32::powi(self[e415], 2)
                        + f32::powi(self[e12345], 2)
                        - (self[e412] * self[e125])
                        - (self[e423] * self[e235])
                        - (self[e431] * self[e315])),
                ])),
            // e415, e425, e435, e321
            ((Simd32x4::from(self[e4]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]))
                + Simd32x4::from(2.0) * (Simd32x4::from(self[e321]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e12345]]))
                + (Simd32x4::from(self[e423]) * Simd32x4::from([self[e5], self[e125], self[e315], self[e235]]))
                + (Simd32x4::from(self[e412]) * Simd32x4::from([self[e315], self[e235], self[e5], self[e125]]))
                + (Simd32x4::from(self[e431]) * Simd32x4::from([self[e125], self[e5], self[e235], self[e315]]))
                + Simd32x4::from([
                    (-(self[e3] * self[e2]) + (self[e2] * self[e3]) - (self[e125] * self[e431]) + (self[e235] * self[e4]) - (self[e435] * self[e425])
                        + (self[e425] * self[e435])
                        + (self[e415] * self[e12345])
                        + (self[e12345] * self[e415])
                        - (self[e412] * self[e315])
                        + (self[e423] * self[e5])),
                    ((self[e3] * self[e1]) - (self[e1] * self[e3]) + (self[e315] * self[e4]) - (self[e235] * self[e412]) + (self[e435] * self[e415]) + (self[e425] * self[e12345])
                        - (self[e415] * self[e435])
                        + (self[e12345] * self[e425])
                        - (self[e423] * self[e125])
                        + (self[e431] * self[e5])),
                    (-(self[e2] * self[e1]) + (self[e1] * self[e2]) + (self[e125] * self[e4]) - (self[e315] * self[e423]) + (self[e435] * self[e12345])
                        - (self[e425] * self[e415])
                        + (self[e415] * self[e425])
                        + (self[e12345] * self[e435])
                        + (self[e412] * self[e5])
                        - (self[e431] * self[e235])),
                    (-(self[e4] * self[e5])
                        - (self[e3] * self[e435])
                        - (self[e2] * self[e425])
                        - (self[e1] * self[e415])
                        - (self[e125] * self[e412])
                        - (self[e315] * self[e431])
                        - (self[e235] * self[e423])
                        - (self[e435] * self[e3])
                        - (self[e425] * self[e2])
                        - (self[e415] * self[e1])),
                ])),
            // e235, e315, e125, e5
            (-(Simd32x4::from(self[e315]) * Simd32x4::from([self[e3], self[e321], self[e415], self[e2]]))
                + (Simd32x4::from(self[e125]) * Simd32x4::from([self[e2], self[e415], self[e12345], self[e3]]))
                + (Simd32x4::from(self[e5]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e12345]]))
                - (Simd32x4::from(self[e5]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e321]]))
                + (Simd32x4::from(self[e5]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]))
                - (Simd32x4::from(self[e125]) * Simd32x4::from([self[e425], self[e1], self[e321], self[e3]]))
                + (Simd32x4::from(self[e315]) * Simd32x4::from([self[e435], self[e12345], self[e1], self[e2]]))
                - (Simd32x4::from(self[e235]) * Simd32x4::from([self[e321], self[e435], self[e2], self[e1]]))
                + (Simd32x4::from(self[e235]) * Simd32x4::from([self[e12345], self[e3], self[e425], self[e1]]))
                + (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]))
                + Simd32x4::from([
                    ((self[e125] * self[e2]) - (self[e315] * self[e3]) + (self[e321] * self[e235]) - (self[e435] * self[e315])
                        + (self[e425] * self[e125])
                        + (self[e415] * self[e5])),
                    (-(self[e125] * self[e1]) + (self[e235] * self[e3]) + (self[e321] * self[e315]) + (self[e435] * self[e235]) + (self[e425] * self[e5])
                        - (self[e415] * self[e125])),
                    ((self[e315] * self[e1]) - (self[e235] * self[e2]) + (self[e321] * self[e125]) + (self[e435] * self[e5]) - (self[e425] * self[e235])
                        + (self[e415] * self[e315])),
                    (-(self[e125] * self[e435])
                        - (self[e315] * self[e425])
                        - (self[e235] * self[e415])
                        - (self[e435] * self[e125])
                        - (self[e425] * self[e315])
                        - (self[e415] * self[e235])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self[e2]) * Simd32x4::from([self[e435], self[e12345], self[e415], self[e431]]))
                + (Simd32x4::from(self[e1]) * Simd32x4::from([self[e12345], self[e435], self[e425], self[e423]]))
                + (Simd32x4::from(self[e3]) * Simd32x4::from([self[e425], self[e415], self[e12345], self[e412]]))
                + (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]))
                + Simd32x4::from([
                    (-(self[e3] * self[e425]) + (self[e5] * self[e423]) - (self[e125] * self[e431]) + (self[e315] * self[e412]) + (self[e321] * self[e415])
                        - (self[e435] * self[e2])
                        + (self[e415] * self[e321])
                        + (self[e412] * self[e315])
                        - (self[e423] * self[e5])
                        - (self[e431] * self[e125])),
                    (-(self[e1] * self[e435]) + (self[e5] * self[e431]) + (self[e125] * self[e423]) - (self[e235] * self[e412])
                        + (self[e321] * self[e425])
                        + (self[e425] * self[e321])
                        - (self[e415] * self[e3])
                        - (self[e412] * self[e235])
                        + (self[e423] * self[e125])
                        - (self[e431] * self[e5])),
                    (-(self[e2] * self[e415]) + (self[e5] * self[e412]) - (self[e315] * self[e423])
                        + (self[e235] * self[e431])
                        + (self[e321] * self[e435])
                        + (self[e435] * self[e321])
                        - (self[e425] * self[e1])
                        - (self[e412] * self[e5])
                        - (self[e423] * self[e315])
                        + (self[e431] * self[e235])),
                    (-(self[e435] * self[e412]) - (self[e425] * self[e431]) - (self[e415] * self[e423]) + (self[e12345] * self[e4])
                        - (self[e412] * self[e3])
                        - (self[e412] * self[e435])
                        - (self[e431] * self[e2])
                        - (self[e431] * self[e425])
                        - (self[e423] * self[e415])
                        - (self[e423] * self[e1])),
                ])),
        );
    }
}
impl AntiSquare for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      136      144        0
    //    simd4       24       24        0
    // Totals...
    // yes simd      160      168        0
    //  no simd      232      240        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x4::from(self[e42]) * Simd32x4::from([self[e4125], self[e45], self[e23], self[e25]]))
                - (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]))
                - (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e3215]]))
                + (Simd32x4::from(self[e43]) * Simd32x4::from([self[e31], self[e4235], self[e45], self[e35]]))
                + (Simd32x4::from(self[e41]) * Simd32x4::from([self[e45], self[e12], self[e4315], self[e15]]))
                + Simd32x4::from([
                    (-(self[e4315] * self[e43]) + (self[e1234] * self[e4235]) - (self[e45] * self[e41]) + (self[e12] * self[e42])
                        - (self[e31] * self[e43])
                        - (self[e23] * self[e1234])
                        - (self[scalar] * self[e41])
                        - (self[e43] * self[e4315])
                        + (self[e42] * self[e4125])
                        - (self[e42] * self[e12])
                        - (self[e41] * self[scalar])),
                    (-(self[e4125] * self[e41]) + (self[e1234] * self[e4315]) - (self[e45] * self[e42]) - (self[e12] * self[e41]) - (self[e31] * self[e1234])
                        + (self[e23] * self[e43])
                        - (self[scalar] * self[e42])
                        + (self[e43] * self[e4235])
                        - (self[e43] * self[e23])
                        - (self[e42] * self[scalar])
                        - (self[e41] * self[e4125])),
                    (-(self[e4235] * self[e42]) + (self[e1234] * self[e4125]) - (self[e45] * self[e43]) - (self[e12] * self[e1234]) + (self[e31] * self[e41])
                        - (self[e23] * self[e42])
                        - (self[scalar] * self[e43])
                        - (self[e43] * self[scalar])
                        - (self[e42] * self[e4235])
                        - (self[e41] * self[e31])
                        + (self[e41] * self[e4315])),
                    (f32::powi(self[e4125], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4235], 2) - f32::powi(self[e45], 2)
                        + f32::powi(self[e12], 2)
                        + f32::powi(self[e31], 2)
                        + f32::powi(self[e23], 2)
                        - f32::powi(self[scalar], 2)
                        + (self[e43] * self[e35])
                        + (self[e41] * self[e15])
                        + (self[e42] * self[e25])),
                ])),
            // e415, e425, e435, e321
            (-(Simd32x4::from(self[e3215]) * Simd32x4::from([self[e41], self[e42], self[e43], self[e1234]]))
                - (Simd32x4::from(self[e4125]) * Simd32x4::from([self[e4315], self[e4235], self[e45], self[e12]]))
                - (Simd32x4::from(self[e15]) * Simd32x4::from([self[e1234], self[e43], self[e42], self[e41]]))
                - (Simd32x4::from(self[e25]) * Simd32x4::from([self[e43], self[e1234], self[e41], self[e42]]))
                - (Simd32x4::from(self[e12]) * Simd32x4::from([self[e31], self[e23], self[scalar], self[e4125]]))
                - (Simd32x4::from(self[e35]) * Simd32x4::from([self[e42], self[e41], self[e1234], self[e43]]))
                + Simd32x4::from([
                    ((self[e4125] * self[e4315]) - (self[e4235] * self[e45]) + (self[e35] * self[e42]) - (self[e15] * self[e1234]) - (self[e45] * self[e4235])
                        + (self[e12] * self[e31])
                        - (self[e23] * self[scalar])
                        - (self[scalar] * self[e23])
                        + (self[e43] * self[e25])
                        - (self[e41] * self[e3215])),
                    (-(self[e4315] * self[e45]) + (self[e4235] * self[e4125]) - (self[e25] * self[e1234]) + (self[e15] * self[e43])
                        - (self[e45] * self[e4315])
                        - (self[e31] * self[scalar])
                        + (self[e23] * self[e12])
                        - (self[scalar] * self[e31])
                        + (self[e41] * self[e35])
                        - (self[e42] * self[e3215])),
                    ((self[e4315] * self[e4235]) - (self[e4235] * self[e4315]) - (self[e35] * self[e1234]) + (self[e25] * self[e41]) - (self[e45] * self[e4125])
                        + (self[e31] * self[e23])
                        - (self[e23] * self[e31])
                        - (self[scalar] * self[e12])
                        - (self[e43] * self[e3215])
                        + (self[e42] * self[e15])),
                    (-(self[e4315] * self[e31]) - (self[e4235] * self[e23])
                        + (self[e1234] * self[e3215])
                        + (self[e35] * self[e43])
                        + (self[e25] * self[e42])
                        + (self[e15] * self[e41])
                        + (self[e45] * self[scalar])
                        - (self[e31] * self[e4315])
                        - (self[e23] * self[e4235])
                        + (self[scalar] * self[e45])),
                ])),
            // e235, e315, e125, e5
            (-(Simd32x4::from(self[e3215]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))
                - (Simd32x4::from(self[e25]) * Simd32x4::from([self[e4125], self[e45], self[e23], self[e4315]]))
                + (Simd32x4::from(self[e35]) * Simd32x4::from([self[e4315], self[e23], self[e45], self[e4125]]))
                - (Simd32x4::from(self[e15]) * Simd32x4::from([self[e45], self[e12], self[e4315], self[e4235]]))
                - (Simd32x4::from(self[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))
                + (Simd32x4::from(self[e15]) * Simd32x4::from([self[e45], self[e4125], self[e31], self[e4235]]))
                + (Simd32x4::from(self[e25]) * Simd32x4::from([self[e12], self[e45], self[e4235], self[e4315]]))
                - (Simd32x4::from(self[e35]) * Simd32x4::from([self[e31], self[e4235], self[e45], self[e4125]]))
                + Simd32x4::from([
                    ((self[e35] * self[e4315]) + (self[e35] * self[e31])
                        - (self[e25] * self[e4125])
                        - (self[e25] * self[e12])
                        - (self[scalar] * self[e15])
                        - (self[e23] * self[e3215])),
                    (-(self[e35] * self[e4235]) - (self[e35] * self[e23]) + (self[e15] * self[e4125]) + (self[e15] * self[e12])
                        - (self[e31] * self[e3215])
                        - (self[scalar] * self[e25])),
                    ((self[e25] * self[e4235]) + (self[e25] * self[e23])
                        - (self[e15] * self[e4315])
                        - (self[e15] * self[e31])
                        - (self[e12] * self[e3215])
                        - (self[scalar] * self[e35])),
                    ((self[e35] * self[e12]) + (self[e25] * self[e31]) + (self[e15] * self[e23]) + (self[e12] * self[e35]) + (self[e31] * self[e25]) + (self[e23] * self[e15])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self[e4315]) * Simd32x4::from([self[e12], self[scalar], self[e23], self[e42]]))
                + (Simd32x4::from(self[e4235]) * Simd32x4::from([self[scalar], self[e12], self[e31], self[e41]]))
                + (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))
                - (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e15], self[e25], self[e35], self[scalar]]))
                + (Simd32x4::from(self[e4125]) * Simd32x4::from([self[e31], self[e23], self[scalar], self[e43]]))
                + Simd32x4::from([
                    (-(self[e3215] * self[e41]) - (self[e4125] * self[e31]) + (self[e35] * self[e42]) - (self[e25] * self[e43]) + (self[e45] * self[e23])
                        - (self[e12] * self[e4315])
                        + (self[e23] * self[e45])
                        + (self[scalar] * self[e4235])
                        - (self[e43] * self[e25])
                        + (self[e41] * self[e3215])
                        + (self[e42] * self[e35])),
                    (-(self[e3215] * self[e42]) - (self[e4235] * self[e12]) - (self[e35] * self[e41])
                        + (self[e15] * self[e43])
                        + (self[e45] * self[e31])
                        + (self[e31] * self[e45])
                        - (self[e23] * self[e4125])
                        + (self[scalar] * self[e4315])
                        + (self[e43] * self[e15])
                        - (self[e41] * self[e35])
                        + (self[e42] * self[e3215])),
                    (-(self[e3215] * self[e43]) - (self[e4315] * self[e23]) + (self[e25] * self[e41]) - (self[e15] * self[e42])
                        + (self[e45] * self[e12])
                        + (self[e12] * self[e45])
                        - (self[e31] * self[e4235])
                        + (self[scalar] * self[e4125])
                        + (self[e43] * self[e3215])
                        + (self[e41] * self[e25])
                        - (self[e42] * self[e15])),
                    (-(self[e45] * self[e1234]) + (self[e12] * self[e43]) + (self[e31] * self[e42]) + (self[e23] * self[e41])
                        - (self[scalar] * self[e1234])
                        - (self[e43] * self[e4125])
                        + (self[e43] * self[e12])
                        - (self[e42] * self[e4315])
                        + (self[e42] * self[e31])
                        + (self[e41] * self[e23])
                        - (self[e41] * self[e4235])),
                ])),
        );
    }
}
