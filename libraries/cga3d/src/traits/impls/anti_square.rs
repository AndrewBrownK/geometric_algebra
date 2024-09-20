// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 35
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       6       0
//  Average:        13      32       0
//  Maximum:       194     416       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         3       6       0
//  Average:        16      38       0
//  Maximum:       248     528       0
impl AntiSquare for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       48        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       17       50        0
    //  no simd       17       56        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self[e41] * self[scalar] * -2.0),
                (self[e42] * self[scalar] * -2.0),
                (self[e43] * self[scalar] * -2.0),
                (f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[e45], 2) - f32::powi(self[scalar], 2)
                    + 2.0 * (self[e41] * self[e15])
                    + 2.0 * (self[e42] * self[e25])
                    + 2.0 * (self[e43] * self[e35])),
            ]),
            // e415, e425, e435, e321
            (Simd32x4::from(self[scalar]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e45]]) * Simd32x4::from([-2.0, -2.0, -2.0, 2.0])),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self[e15] * self[scalar] * -2.0),
                (self[e25] * self[scalar] * -2.0),
                (self[e35] * self[scalar] * -2.0),
                (2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (2.0 * (self[e42] * self[e35]) - 2.0 * (self[e43] * self[e25]) + 2.0 * (self[e23] * self[e45])),
                (-2.0 * (self[e41] * self[e35]) + 2.0 * (self[e43] * self[e15]) + 2.0 * (self[e31] * self[e45])),
                (2.0 * (self[e41] * self[e25]) - 2.0 * (self[e42] * self[e15]) + 2.0 * (self[e12] * self[e45])),
                (2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12])),
            ]),
        );
    }
}
impl AntiSquare for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       90        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       37       92        0
    //  no simd       40       98        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                (2.0 * (self[e431] * self[e3]) - 2.0 * (self[e412] * self[e2])),
                (-2.0 * (self[e423] * self[e3]) + 2.0 * (self[e412] * self[e1])),
                (2.0 * (self[e423] * self[e2]) - 2.0 * (self[e431] * self[e1])),
                (-f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) + f32::powi(self[e321], 2)
                    - f32::powi(self[e1], 2)
                    - f32::powi(self[e2], 2)
                    - f32::powi(self[e3], 2)
                    - 2.0 * (self[e423] * self[e235])
                    - 2.0 * (self[e431] * self[e315])
                    - 2.0 * (self[e412] * self[e125])),
            ]) + Simd32x4::from(2.0) * (Simd32x4::from(self[e4]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e5]]))),
            // e415, e425, e435, e321
            Simd32x4::from([
                (2.0 * (self[e423] * self[e5]) + 2.0 * (self[e321] * self[e1]) + 2.0 * (self[e235] * self[e4])),
                (2.0 * (self[e431] * self[e5]) + 2.0 * (self[e321] * self[e2]) + 2.0 * (self[e315] * self[e4])),
                (2.0 * (self[e412] * self[e5]) + 2.0 * (self[e321] * self[e3]) + 2.0 * (self[e125] * self[e4])),
                (-2.0 * (self[e415] * self[e1]) - 2.0 * (self[e425] * self[e2]) - 2.0 * (self[e435] * self[e3])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (2.0 * (self[e415] * self[e5]) - 2.0 * (self[e315] * self[e3]) + 2.0 * (self[e125] * self[e2])),
                (2.0 * (self[e425] * self[e5]) + 2.0 * (self[e235] * self[e3]) - 2.0 * (self[e125] * self[e1])),
                (2.0 * (self[e435] * self[e5]) - 2.0 * (self[e235] * self[e2]) + 2.0 * (self[e315] * self[e1])),
                (-2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-2.0 * (self[e431] * self[e125]) + 2.0 * (self[e412] * self[e315]) + 2.0 * (self[e415] * self[e321])),
                (2.0 * (self[e423] * self[e125]) - 2.0 * (self[e412] * self[e235]) + 2.0 * (self[e425] * self[e321])),
                (-2.0 * (self[e423] * self[e315]) + 2.0 * (self[e431] * self[e235]) + 2.0 * (self[e435] * self[e321])),
                (-2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435])),
            ]),
        );
    }
}
impl AntiSquare for AntiDualNum321 {
    type Output = DualNum321;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum321::from_groups(
            // e321, e12345
            Simd32x2::from([(self[e45] * self[scalar] * 2.0), (-f32::powi(self[e45], 2) - f32::powi(self[scalar], 2))]),
        );
    }
}
impl AntiSquare for AntiDualNum4 {
    type Output = DualNum4;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum4::from_groups(
            // e4, e12345
            (Simd32x2::from([(self[e1234] * self[scalar]), f32::powi(self[scalar], 2)]) * Simd32x2::from([-2.0, -1.0])),
        );
    }
}
impl AntiSquare for AntiDualNum5 {
    type Output = DualNum5;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum5::from_groups(
            // e5, e12345
            (Simd32x2::from([(self[e3215] * self[scalar]), f32::powi(self[scalar], 2)]) * Simd32x2::from([-2.0, -1.0])),
        );
    }
}
impl AntiSquare for AntiFlatPoint {
    type Output = AntiScalar;
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ f32::powi(self[e321], 2));
    }
}
impl AntiSquare for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       18        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (self[e321] * self[e1] * 2.0),
                (self[e321] * self[e2] * 2.0),
                (self[e321] * self[e3] * 2.0),
                (f32::powi(self[e321], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2)),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-2.0 * (self[e315] * self[e3]) + 2.0 * (self[e125] * self[e2])),
                (2.0 * (self[e235] * self[e3]) - 2.0 * (self[e125] * self[e1])),
                (-2.0 * (self[e235] * self[e2]) + 2.0 * (self[e315] * self[e1])),
                0.0,
            ]),
        );
    }
}
impl AntiSquare for AntiLine {
    type Output = DualNum5;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum5::from_groups(/* e5, e12345 */ Simd32x2::from([
            (2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35])),
            (f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2)),
        ]));
    }
}
impl AntiSquare for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       18        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        6       20        0
    //  no simd        9       26        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (self[e23] * self[scalar] * -2.0),
                (self[e31] * self[scalar] * -2.0),
                (self[e12] * self[scalar] * -2.0),
                (f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[scalar], 2)),
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((self[scalar] * self[e15]) * -2.0),
                ((self[scalar] * self[e25]) * -2.0),
                ((self[scalar] * self[e35]) * -2.0),
                (2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35])),
            ]) - Simd32x4::from(2.0) * (Simd32x4::from(self[e3215]) * Simd32x4::from([self[e23], self[e31], self[e12], self[scalar]]))),
        );
    }
}
impl AntiSquare for AntiPlane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (-f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2)));
    }
}
impl AntiSquare for AntiQuadNum {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        8        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return QuadNum::from_groups(/* e4, e5, e321, e12345 */ Simd32x4::from([
            (self[e1234] * self[scalar] * -2.0),
            (self[e3215] * self[scalar] * -2.0),
            (self[e45] * self[scalar] * 2.0),
            (-f32::powi(self[e45], 2) - f32::powi(self[scalar], 2) - 2.0 * (self[e1234] * self[e3215])),
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
    type Output = TripleNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        6        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return TripleNum::from_groups(/* e4, e5, e12345 */ Simd32x3::from([
            (self[e1234] * self[scalar] * -2.0),
            (self[e3215] * self[scalar] * -2.0),
            (-f32::powi(self[scalar], 2) - 2.0 * (self[e1234] * self[e3215])),
        ]));
    }
}
impl AntiSquare for Circle {
    type Output = VersorRoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       24        0
    //    simd2        3        6        0
    // Totals...
    // yes simd       14       30        0
    //  no simd       17       36        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorRoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (-2.0 * (self[e431] * self[e125]) + 2.0 * (self[e412] * self[e315]) + 2.0 * (self[e415] * self[e321])),
                (2.0 * (self[e423] * self[e125]) - 2.0 * (self[e412] * self[e235]) + 2.0 * (self[e425] * self[e321])),
                (-2.0 * (self[e423] * self[e315]) + 2.0 * (self[e431] * self[e235]) + 2.0 * (self[e435] * self[e321])),
                (-2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435])),
            ]),
            // e5, e12345
            (Simd32x2::from([0.0, (-f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) + f32::powi(self[e321], 2))])
                - Simd32x2::from(2.0) * (Simd32x2::from(self[e235]) * Simd32x2::from([self[e415], self[e423]]))
                - Simd32x2::from(2.0) * (Simd32x2::from(self[e315]) * Simd32x2::from([self[e425], self[e431]]))
                - Simd32x2::from(2.0) * (Simd32x2::from(self[e125]) * Simd32x2::from([self[e435], self[e412]]))),
        );
    }
}
impl AntiSquare for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       48        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       17       50        0
    //  no simd       17       56        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self[e423] * self[e12345] * 2.0),
                (self[e431] * self[e12345] * 2.0),
                (self[e412] * self[e12345] * 2.0),
                (-f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) + f32::powi(self[e321], 2) + f32::powi(self[e12345], 2)
                    - 2.0 * (self[e423] * self[e235])
                    - 2.0 * (self[e431] * self[e315])
                    - 2.0 * (self[e412] * self[e125])),
            ]),
            // e415, e425, e435, e321
            (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]) * Simd32x4::from(2.0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self[e235] * self[e12345] * 2.0),
                (self[e315] * self[e12345] * 2.0),
                (self[e125] * self[e12345] * 2.0),
                (-2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-2.0 * (self[e431] * self[e125]) + 2.0 * (self[e412] * self[e315]) + 2.0 * (self[e415] * self[e321])),
                (2.0 * (self[e423] * self[e125]) - 2.0 * (self[e412] * self[e235]) + 2.0 * (self[e425] * self[e321])),
                (-2.0 * (self[e423] * self[e315]) + 2.0 * (self[e431] * self[e235]) + 2.0 * (self[e435] * self[e321])),
                (-2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435])),
            ]),
        );
    }
}
impl AntiSquare for Dipole {
    type Output = VersorRoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       24        0
    //    simd2        3        6        0
    // Totals...
    // yes simd       14       30        0
    //  no simd       17       36        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorRoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (2.0 * (self[e42] * self[e35]) - 2.0 * (self[e43] * self[e25]) + 2.0 * (self[e23] * self[e45])),
                (-2.0 * (self[e41] * self[e35]) + 2.0 * (self[e43] * self[e15]) + 2.0 * (self[e31] * self[e45])),
                (2.0 * (self[e41] * self[e25]) - 2.0 * (self[e42] * self[e15]) + 2.0 * (self[e12] * self[e45])),
                (2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12])),
            ]),
            // e5, e12345
            (Simd32x2::from([0.0, (f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[e45], 2))])
                + Simd32x2::from(2.0) * (Simd32x2::from(self[e15]) * Simd32x2::from([self[e23], self[e41]]))
                + Simd32x2::from(2.0) * (Simd32x2::from(self[e25]) * Simd32x2::from([self[e31], self[e42]]))
                + Simd32x2::from(2.0) * (Simd32x2::from(self[e35]) * Simd32x2::from([self[e12], self[e43]]))),
        );
    }
}
impl AntiSquare for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       90        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       37       92        0
    //  no simd       40       98        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                (2.0 * (self[e42] * self[e4125]) - 2.0 * (self[e43] * self[e4315])),
                (-2.0 * (self[e41] * self[e4125]) + 2.0 * (self[e43] * self[e4235])),
                (2.0 * (self[e41] * self[e4315]) - 2.0 * (self[e42] * self[e4235])),
                (f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[e45], 2)
                    + f32::powi(self[e4235], 2)
                    + f32::powi(self[e4315], 2)
                    + f32::powi(self[e4125], 2)
                    + 2.0 * (self[e41] * self[e15])
                    + 2.0 * (self[e42] * self[e25])
                    + 2.0 * (self[e43] * self[e35])),
            ]) - Simd32x4::from(2.0) * (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e3215]]))),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-2.0 * (self[e41] * self[e3215]) - 2.0 * (self[e45] * self[e4235]) - 2.0 * (self[e15] * self[e1234])),
                (-2.0 * (self[e42] * self[e3215]) - 2.0 * (self[e45] * self[e4315]) - 2.0 * (self[e25] * self[e1234])),
                (-2.0 * (self[e43] * self[e3215]) - 2.0 * (self[e45] * self[e4125]) - 2.0 * (self[e35] * self[e1234])),
                (-2.0 * (self[e23] * self[e4235]) - 2.0 * (self[e31] * self[e4315]) - 2.0 * (self[e12] * self[e4125])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-2.0 * (self[e23] * self[e3215]) - 2.0 * (self[e25] * self[e4125]) + 2.0 * (self[e35] * self[e4315])),
                (-2.0 * (self[e31] * self[e3215]) + 2.0 * (self[e15] * self[e4125]) - 2.0 * (self[e35] * self[e4235])),
                (-2.0 * (self[e12] * self[e3215]) - 2.0 * (self[e15] * self[e4315]) + 2.0 * (self[e25] * self[e4235])),
                (2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (2.0 * (self[e42] * self[e35]) - 2.0 * (self[e43] * self[e25]) + 2.0 * (self[e23] * self[e45])),
                (-2.0 * (self[e41] * self[e35]) + 2.0 * (self[e43] * self[e15]) + 2.0 * (self[e31] * self[e45])),
                (2.0 * (self[e41] * self[e25]) - 2.0 * (self[e42] * self[e15]) + 2.0 * (self[e12] * self[e45])),
                (2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12])),
            ]),
        );
    }
}
impl AntiSquare for DualNum321 {
    type Output = DualNum321;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum321::from_groups(
            // e321, e12345
            Simd32x2::from([(self[e321] * self[e12345] * 2.0), (f32::powi(self[e321], 2) + f32::powi(self[e12345], 2))]),
        );
    }
}
impl AntiSquare for DualNum4 {
    type Output = DualNum4;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum4::from_groups(
            // e4, e12345
            (Simd32x2::from([(self[e4] * self[e12345]), f32::powi(self[e12345], 2)]) * Simd32x2::from([2.0, 1.0])),
        );
    }
}
impl AntiSquare for DualNum5 {
    type Output = DualNum5;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum5::from_groups(
            // e5, e12345
            (Simd32x2::from([(self[e5] * self[e12345]), f32::powi(self[e12345], 2)]) * Simd32x2::from([2.0, 1.0])),
        );
    }
}
impl AntiSquare for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (f32::powi(self[e45], 2) * -1.0));
    }
}
impl AntiSquare for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       18        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (self[e45] * self[e4235] * -2.0),
                (self[e45] * self[e4315] * -2.0),
                (self[e45] * self[e4125] * -2.0),
                (-f32::powi(self[e45], 2) + f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-2.0 * (self[e25] * self[e4125]) + 2.0 * (self[e35] * self[e4315])),
                (2.0 * (self[e15] * self[e4125]) - 2.0 * (self[e35] * self[e4235])),
                (-2.0 * (self[e15] * self[e4315]) + 2.0 * (self[e25] * self[e4235])),
                0.0,
            ]),
        );
    }
}
impl AntiSquare for Line {
    type Output = DualNum5;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return DualNum5::from_groups(/* e5, e12345 */ Simd32x2::from([
            (-2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125])),
            (-f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2)),
        ]));
    }
}
impl AntiSquare for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       18        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        6       20        0
    //  no simd        9       26        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (self[e415] * self[e12345] * 2.0),
                (self[e425] * self[e12345] * 2.0),
                (self[e435] * self[e12345] * 2.0),
                (-f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) + f32::powi(self[e12345], 2)),
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([
                ((self[e12345] * self[e235]) * 2.0),
                ((self[e12345] * self[e315]) * 2.0),
                ((self[e12345] * self[e125]) * 2.0),
                (-2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125])),
            ]) + Simd32x4::from(2.0) * (Simd32x4::from(self[e5]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e12345]]))),
        );
    }
}
impl AntiSquare for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      167      360        0
    //    simd2        4        8        0
    //    simd3       19       40        0
    //    simd4        4        8        0
    // Totals...
    // yes simd      194      416        0
    //  no simd      248      528        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            (Simd32x2::from([
                (2.0 * (self[scalar] * self[e12345])
                    + 2.0 * (self[e1] * self[e4235])
                    + 2.0 * (self[e2] * self[e4315])
                    + 2.0 * (self[e3] * self[e4125])
                    + 2.0 * (self[e5] * self[e1234])
                    - 2.0 * (self[e45] * self[e321])
                    - 2.0 * (self[e41] * self[e235])
                    - 2.0 * (self[e42] * self[e315])
                    - 2.0 * (self[e43] * self[e125])
                    - 2.0 * (self[e23] * self[e415])
                    - 2.0 * (self[e31] * self[e425])
                    - 2.0 * (self[e12] * self[e435])),
                (-f32::powi(self[scalar], 2) + f32::powi(self[e12345], 2) - f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2) - f32::powi(self[e45], 2)
                    + f32::powi(self[e23], 2)
                    + f32::powi(self[e31], 2)
                    + f32::powi(self[e12], 2)
                    - f32::powi(self[e415], 2)
                    - f32::powi(self[e425], 2)
                    - f32::powi(self[e435], 2)
                    + f32::powi(self[e321], 2)
                    + f32::powi(self[e4235], 2)
                    + f32::powi(self[e4315], 2)
                    + f32::powi(self[e4125], 2)
                    + 2.0 * (self[e15] * self[e41])
                    + 2.0 * (self[e25] * self[e42])
                    + 2.0 * (self[e35] * self[e43])
                    - 2.0 * (self[e3215] * self[e1234])),
            ]) + Simd32x2::from(2.0) * (Simd32x2::from(self[e4]) * Simd32x2::from([self[e3215], self[e5]]))
                - Simd32x2::from(2.0) * (Simd32x2::from(self[e423]) * Simd32x2::from([self[e15], self[e235]]))
                - Simd32x2::from(2.0) * (Simd32x2::from(self[e431]) * Simd32x2::from([self[e25], self[e315]]))
                - Simd32x2::from(2.0) * (Simd32x2::from(self[e412]) * Simd32x2::from([self[e35], self[e125]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (2.0 * (self[scalar] * self[e4235]) - 2.0 * (self[e25] * self[e43])
                    + 2.0 * (self[e35] * self[e42])
                    + 2.0 * (self[e45] * self[e23])
                    + 2.0 * (self[e415] * self[e321])
                    - 2.0 * (self[e431] * self[e125])
                    + 2.0 * (self[e412] * self[e315])),
                (2.0 * (self[scalar] * self[e4315]) + 2.0 * (self[e15] * self[e43]) - 2.0 * (self[e35] * self[e41])
                    + 2.0 * (self[e45] * self[e31])
                    + 2.0 * (self[e425] * self[e321])
                    + 2.0 * (self[e423] * self[e125])
                    - 2.0 * (self[e412] * self[e235])),
                (2.0 * (self[scalar] * self[e4125]) - 2.0 * (self[e15] * self[e42])
                    + 2.0 * (self[e25] * self[e41])
                    + 2.0 * (self[e45] * self[e12])
                    + 2.0 * (self[e435] * self[e321])
                    - 2.0 * (self[e423] * self[e315])
                    + 2.0 * (self[e431] * self[e235])),
                (-2.0 * (self[scalar] * self[e1234]) + 2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12])
                    - 2.0 * (self[e415] * self[e423])
                    - 2.0 * (self[e425] * self[e431])
                    - 2.0 * (self[e435] * self[e412])),
            ]) + Simd32x4::from(2.0) * (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]))),
            // e5
            (-2.0 * (self[scalar] * self[e3215]) + 2.0 * (self[e12345] * self[e5]) + 2.0 * (self[e15] * self[e23]) + 2.0 * (self[e25] * self[e31]) + 2.0 * (self[e35] * self[e12])
                - 2.0 * (self[e415] * self[e235])
                - 2.0 * (self[e425] * self[e315])
                - 2.0 * (self[e435] * self[e125])),
            // e15, e25, e35, e45
            (Simd32x4::from([
                (2.0 * (self[scalar] * self[e235]) + 2.0 * (self[e2] * self[e35]) - 2.0 * (self[e3] * self[e25])
                    + 2.0 * (self[e5] * self[e23])
                    + 2.0 * (self[e415] * self[e3215])
                    + 2.0 * (self[e315] * self[e4125])
                    - 2.0 * (self[e125] * self[e4315])),
                (2.0 * (self[scalar] * self[e315]) - 2.0 * (self[e1] * self[e35]) + 2.0 * (self[e3] * self[e15]) + 2.0 * (self[e5] * self[e31]) + 2.0 * (self[e425] * self[e3215])
                    - 2.0 * (self[e235] * self[e4125])
                    + 2.0 * (self[e125] * self[e4235])),
                (2.0 * (self[scalar] * self[e125]) + 2.0 * (self[e1] * self[e25]) - 2.0 * (self[e2] * self[e15])
                    + 2.0 * (self[e5] * self[e12])
                    + 2.0 * (self[e435] * self[e3215])
                    + 2.0 * (self[e235] * self[e4315])
                    - 2.0 * (self[e315] * self[e4235])),
                (-2.0 * (self[scalar] * self[e321]) + 2.0 * (self[e1] * self[e23]) + 2.0 * (self[e2] * self[e31]) + 2.0 * (self[e3] * self[e12])
                    - 2.0 * (self[e415] * self[e4235])
                    - 2.0 * (self[e425] * self[e4315])
                    - 2.0 * (self[e435] * self[e4125])),
            ]) + Simd32x4::from(2.0) * (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e45]]))),
            // e41, e42, e43
            (Simd32x3::from([
                (-2.0 * (self[e2] * self[e43]) + 2.0 * (self[e3] * self[e42]) - 2.0 * (self[e431] * self[e4125]) + 2.0 * (self[e412] * self[e4315])),
                (2.0 * (self[e1] * self[e43]) - 2.0 * (self[e3] * self[e41]) + 2.0 * (self[e423] * self[e4125]) - 2.0 * (self[e412] * self[e4235])),
                (-2.0 * (self[e1] * self[e42]) + 2.0 * (self[e2] * self[e41]) - 2.0 * (self[e423] * self[e4315]) + 2.0 * (self[e431] * self[e4235])),
            ]) + Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * Simd32x3::from([self[e41], self[e42], self[e43]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * Simd32x3::from([self[e23], self[e31], self[e12]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e1234]) * Simd32x3::from([self[e415], self[e425], self[e435]]))),
            // e23, e31, e12
            (Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * Simd32x3::from([self[e415], self[e425], self[e435]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * Simd32x3::from([self[e23], self[e31], self[e12]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * Simd32x3::from([self[e15], self[e25], self[e35]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e5]) * Simd32x3::from([self[e41], self[e42], self[e43]]))
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e45]) * Simd32x3::from([self[e1], self[e2], self[e3]]))
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e321]) * Simd32x3::from([self[e4235], self[e4315], self[e4125]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e3215]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e1234]) * Simd32x3::from([self[e235], self[e315], self[e125]]))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (-2.0 * (self[scalar] * self[e23]) + 2.0 * (self[e1] * self[e321]) + 2.0 * (self[e4] * self[e235]) + 2.0 * (self[e5] * self[e423])
                    - 2.0 * (self[e15] * self[e1234])
                    - 2.0 * (self[e45] * self[e4235])
                    - 2.0 * (self[e41] * self[e3215])),
                (-2.0 * (self[scalar] * self[e31]) + 2.0 * (self[e2] * self[e321]) + 2.0 * (self[e4] * self[e315]) + 2.0 * (self[e5] * self[e431])
                    - 2.0 * (self[e25] * self[e1234])
                    - 2.0 * (self[e45] * self[e4315])
                    - 2.0 * (self[e42] * self[e3215])),
                (-2.0 * (self[scalar] * self[e12]) + 2.0 * (self[e3] * self[e321]) + 2.0 * (self[e4] * self[e125]) + 2.0 * (self[e5] * self[e412])
                    - 2.0 * (self[e35] * self[e1234])
                    - 2.0 * (self[e45] * self[e4125])
                    - 2.0 * (self[e43] * self[e3215])),
                (2.0 * (self[scalar] * self[e45])
                    - 2.0 * (self[e1] * self[e415])
                    - 2.0 * (self[e2] * self[e425])
                    - 2.0 * (self[e3] * self[e435])
                    - 2.0 * (self[e23] * self[e4235])
                    - 2.0 * (self[e31] * self[e4315])
                    - 2.0 * (self[e12] * self[e4125])),
            ]) + Simd32x4::from(2.0) * (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]))),
            // e423, e431, e412
            (Simd32x3::from([
                (-2.0 * (self[e2] * self[e412]) + 2.0 * (self[e3] * self[e431]) + 2.0 * (self[e42] * self[e4125]) - 2.0 * (self[e43] * self[e4315])),
                (2.0 * (self[e1] * self[e412]) - 2.0 * (self[e3] * self[e423]) - 2.0 * (self[e41] * self[e4125]) + 2.0 * (self[e43] * self[e4235])),
                (-2.0 * (self[e1] * self[e431]) + 2.0 * (self[e2] * self[e423]) + 2.0 * (self[e41] * self[e4315]) - 2.0 * (self[e42] * self[e4235])),
            ]) - Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e4]) * Simd32x3::from([self[e415], self[e425], self[e435]]))
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]]))),
            // e235, e315, e125
            (Simd32x3::from([
                (2.0 * (self[e2] * self[e125]) - 2.0 * (self[e3] * self[e315]) - 2.0 * (self[e25] * self[e4125]) + 2.0 * (self[e35] * self[e4315])),
                (-2.0 * (self[e1] * self[e125]) + 2.0 * (self[e3] * self[e235]) + 2.0 * (self[e15] * self[e4125]) - 2.0 * (self[e35] * self[e4235])),
                (2.0 * (self[e1] * self[e315]) - 2.0 * (self[e2] * self[e235]) - 2.0 * (self[e15] * self[e4315]) + 2.0 * (self[e25] * self[e4235])),
            ]) - Simd32x3::from(2.0) * (Simd32x3::from(self[scalar]) * Simd32x3::from([self[e15], self[e25], self[e35]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e12345]) * Simd32x3::from([self[e235], self[e315], self[e125]]))
                + Simd32x3::from(2.0) * (Simd32x3::from(self[e5]) * Simd32x3::from([self[e415], self[e425], self[e435]]))
                - Simd32x3::from(2.0) * (Simd32x3::from(self[e3215]) * Simd32x3::from([self[e23], self[e31], self[e12]]))),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from([
                (-2.0 * (self[scalar] * self[e1]) - 2.0 * (self[e25] * self[e412])
                    + 2.0 * (self[e35] * self[e431])
                    + 2.0 * (self[e45] * self[e415])
                    + 2.0 * (self[e42] * self[e125])
                    - 2.0 * (self[e43] * self[e315])
                    - 2.0 * (self[e23] * self[e321])),
                (-2.0 * (self[scalar] * self[e2]) + 2.0 * (self[e15] * self[e412]) - 2.0 * (self[e35] * self[e423]) + 2.0 * (self[e45] * self[e425])
                    - 2.0 * (self[e41] * self[e125])
                    + 2.0 * (self[e43] * self[e235])
                    - 2.0 * (self[e31] * self[e321])),
                (-2.0 * (self[scalar] * self[e3]) - 2.0 * (self[e15] * self[e431])
                    + 2.0 * (self[e25] * self[e423])
                    + 2.0 * (self[e45] * self[e435])
                    + 2.0 * (self[e41] * self[e315])
                    - 2.0 * (self[e42] * self[e235])
                    - 2.0 * (self[e12] * self[e321])),
                (2.0 * (self[scalar] * self[e5])
                    - 2.0 * (self[e15] * self[e415])
                    - 2.0 * (self[e25] * self[e425])
                    - 2.0 * (self[e35] * self[e435])
                    - 2.0 * (self[e23] * self[e235])
                    - 2.0 * (self[e31] * self[e315])
                    - 2.0 * (self[e12] * self[e125])),
            ]) + Simd32x4::from(2.0) * (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e3215]]))),
            // e1234
            (2.0 * (self[scalar] * self[e4]) + 2.0 * (self[e12345] * self[e1234])
                - 2.0 * (self[e41] * self[e415])
                - 2.0 * (self[e42] * self[e425])
                - 2.0 * (self[e43] * self[e435])
                - 2.0 * (self[e23] * self[e423])
                - 2.0 * (self[e31] * self[e431])
                - 2.0 * (self[e12] * self[e412])),
        );
    }
}
impl AntiSquare for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2)));
    }
}
impl AntiSquare for QuadNum {
    type Output = QuadNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        8        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return QuadNum::from_groups(/* e4, e5, e321, e12345 */ Simd32x4::from([
            (self[e4] * self[e12345] * 2.0),
            (self[e5] * self[e12345] * 2.0),
            (self[e321] * self[e12345] * 2.0),
            (f32::powi(self[e321], 2) + f32::powi(self[e12345], 2) + 2.0 * (self[e4] * self[e5])),
        ]));
    }
}
impl AntiSquare for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        2        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (-f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2) + 2.0 * (self[e4] * self[e5])),
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
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        2        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - 2.0 * (self[e3215] * self[e1234])),
        );
    }
}
impl AntiSquare for TripleNum {
    type Output = TripleNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        6        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return TripleNum::from_groups(/* e4, e5, e12345 */ Simd32x3::from([
            (self[e4] * self[e12345] * 2.0),
            (self[e5] * self[e12345] * 2.0),
            (f32::powi(self[e12345], 2) + 2.0 * (self[e4] * self[e5])),
        ]));
    }
}
impl AntiSquare for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       96        0
    //    simd4        4        8        0
    // Totals...
    // yes simd       44      104        0
    //  no simd       56      128        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                (2.0 * (self[e423] * self[e12345]) + 2.0 * (self[e431] * self[e3]) - 2.0 * (self[e412] * self[e2])),
                (-2.0 * (self[e423] * self[e3]) + 2.0 * (self[e431] * self[e12345]) + 2.0 * (self[e412] * self[e1])),
                (2.0 * (self[e423] * self[e2]) - 2.0 * (self[e431] * self[e1]) + 2.0 * (self[e412] * self[e12345])),
                (f32::powi(self[e12345], 2) - f32::powi(self[e415], 2) - f32::powi(self[e425], 2) - f32::powi(self[e435], 2) + f32::powi(self[e321], 2)
                    - f32::powi(self[e1], 2)
                    - f32::powi(self[e2], 2)
                    - f32::powi(self[e3], 2)
                    - 2.0 * (self[e423] * self[e235])
                    - 2.0 * (self[e431] * self[e315])
                    - 2.0 * (self[e412] * self[e125])),
            ]) + Simd32x4::from(2.0) * (Simd32x4::from(self[e4]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e5]]))),
            // e415, e425, e435, e321
            (Simd32x4::from([
                (2.0 * (self[e423] * self[e5]) + 2.0 * (self[e321] * self[e1]) + 2.0 * (self[e235] * self[e4])),
                (2.0 * (self[e431] * self[e5]) + 2.0 * (self[e321] * self[e2]) + 2.0 * (self[e315] * self[e4])),
                (2.0 * (self[e412] * self[e5]) + 2.0 * (self[e321] * self[e3]) + 2.0 * (self[e125] * self[e4])),
                (-2.0 * (self[e415] * self[e1]) - 2.0 * (self[e425] * self[e2]) - 2.0 * (self[e435] * self[e3])),
            ]) + Simd32x4::from(2.0) * (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e415], self[e425], self[e435], self[e321]]))),
            // e235, e315, e125, e5
            (Simd32x4::from([
                (2.0 * (self[e415] * self[e5]) - 2.0 * (self[e315] * self[e3]) + 2.0 * (self[e125] * self[e2])),
                (2.0 * (self[e425] * self[e5]) + 2.0 * (self[e235] * self[e3]) - 2.0 * (self[e125] * self[e1])),
                (2.0 * (self[e435] * self[e5]) - 2.0 * (self[e235] * self[e2]) + 2.0 * (self[e315] * self[e1])),
                (-2.0 * (self[e415] * self[e235]) - 2.0 * (self[e425] * self[e315]) - 2.0 * (self[e435] * self[e125])),
            ]) + Simd32x4::from(2.0) * (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e235], self[e315], self[e125], self[e5]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (-2.0 * (self[e431] * self[e125]) + 2.0 * (self[e412] * self[e315]) + 2.0 * (self[e415] * self[e321])),
                (2.0 * (self[e423] * self[e125]) - 2.0 * (self[e412] * self[e235]) + 2.0 * (self[e425] * self[e321])),
                (-2.0 * (self[e423] * self[e315]) + 2.0 * (self[e431] * self[e235]) + 2.0 * (self[e435] * self[e321])),
                (-2.0 * (self[e423] * self[e415]) - 2.0 * (self[e431] * self[e425]) - 2.0 * (self[e412] * self[e435])),
            ]) + Simd32x4::from(2.0) * (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]))),
        );
    }
}
impl AntiSquare for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48      112        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       50      116        0
    //  no simd       56      128        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([
                (-2.0 * (self[e41] * self[scalar]) + 2.0 * (self[e42] * self[e4125]) - 2.0 * (self[e43] * self[e4315])),
                (-2.0 * (self[e41] * self[e4125]) - 2.0 * (self[e42] * self[scalar]) + 2.0 * (self[e43] * self[e4235])),
                (2.0 * (self[e41] * self[e4315]) - 2.0 * (self[e42] * self[e4235]) - 2.0 * (self[e43] * self[scalar])),
                (-f32::powi(self[scalar], 2) + f32::powi(self[e23], 2) + f32::powi(self[e31], 2) + f32::powi(self[e12], 2) - f32::powi(self[e45], 2)
                    + f32::powi(self[e4235], 2)
                    + f32::powi(self[e4315], 2)
                    + f32::powi(self[e4125], 2)
                    + 2.0 * (self[e41] * self[e15])
                    + 2.0 * (self[e42] * self[e25])
                    + 2.0 * (self[e43] * self[e35])),
            ]) - Simd32x4::from(2.0) * (Simd32x4::from(self[e1234]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e3215]]))),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-2.0 * (self[e41] * self[e3215]) - 2.0 * (self[scalar] * self[e23]) - 2.0 * (self[e45] * self[e4235]) - 2.0 * (self[e15] * self[e1234])),
                (-2.0 * (self[e42] * self[e3215]) - 2.0 * (self[scalar] * self[e31]) - 2.0 * (self[e45] * self[e4315]) - 2.0 * (self[e25] * self[e1234])),
                (-2.0 * (self[e43] * self[e3215]) - 2.0 * (self[scalar] * self[e12]) - 2.0 * (self[e45] * self[e4125]) - 2.0 * (self[e35] * self[e1234])),
                (2.0 * (self[scalar] * self[e45]) - 2.0 * (self[e23] * self[e4235]) - 2.0 * (self[e31] * self[e4315]) - 2.0 * (self[e12] * self[e4125])),
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from([
                (-2.0 * (self[e23] * self[e3215]) - 2.0 * (self[e25] * self[e4125]) + 2.0 * (self[e35] * self[e4315])),
                (-2.0 * (self[e31] * self[e3215]) + 2.0 * (self[e15] * self[e4125]) - 2.0 * (self[e35] * self[e4235])),
                (-2.0 * (self[e12] * self[e3215]) - 2.0 * (self[e15] * self[e4315]) + 2.0 * (self[e25] * self[e4235])),
                (2.0 * (self[e23] * self[e15]) + 2.0 * (self[e31] * self[e25]) + 2.0 * (self[e12] * self[e35])),
            ]) - Simd32x4::from(2.0) * (Simd32x4::from(self[scalar]) * Simd32x4::from([self[e15], self[e25], self[e35], self[e3215]]))),
            // e1, e2, e3, e4
            Simd32x4::from([
                (2.0 * (self[e42] * self[e35]) - 2.0 * (self[e43] * self[e25]) + 2.0 * (self[scalar] * self[e4235]) + 2.0 * (self[e23] * self[e45])),
                (-2.0 * (self[e41] * self[e35]) + 2.0 * (self[e43] * self[e15]) + 2.0 * (self[scalar] * self[e4315]) + 2.0 * (self[e31] * self[e45])),
                (2.0 * (self[e41] * self[e25]) - 2.0 * (self[e42] * self[e15]) + 2.0 * (self[scalar] * self[e4125]) + 2.0 * (self[e12] * self[e45])),
                (2.0 * (self[e41] * self[e23]) + 2.0 * (self[e42] * self[e31]) + 2.0 * (self[e43] * self[e12]) - 2.0 * (self[scalar] * self[e1234])),
            ]),
        );
    }
}
impl AntiSquare for VersorRoundPoint {
    type Output = VersorRoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4       12        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorRoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e12345]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e4]]) * Simd32x4::from(2.0)),
            // e5, e12345
            Simd32x2::from([
                (self[e5] * self[e12345] * 2.0),
                (-f32::powi(self[e1], 2) - f32::powi(self[e2], 2) - f32::powi(self[e3], 2) + f32::powi(self[e12345], 2) + 2.0 * (self[e4] * self[e5])),
            ]),
        );
    }
}
impl AntiSquare for VersorSphere {
    type Output = VersorRoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4        6        0
    //  no simd        4       12        0
    fn anti_square(self) -> Self::Output {
        use crate::elements::*;
        return VersorRoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]) * Simd32x4::from([self[e4235], self[e4315], self[e4125], self[e1234]]) * Simd32x4::from([2.0, 2.0, 2.0, -2.0])),
            // e5, e12345
            Simd32x2::from([
                (self[e3215] * self[scalar] * -2.0),
                (f32::powi(self[e4235], 2) + f32::powi(self[e4315], 2) + f32::powi(self[e4125], 2) - f32::powi(self[scalar], 2) - 2.0 * (self[e3215] * self[e1234])),
            ]),
        );
    }
}
