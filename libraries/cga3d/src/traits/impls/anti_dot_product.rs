// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 245
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         3       4       0
//  Average:         4       5       0
//  Maximum:        31      32       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         3       4       0
//  Average:         4       5       0
//  Maximum:        31      32       0
impl std::ops::Div<anti_dot_product> for AntiCircleRotor {
    type Output = anti_dot_product_partial<AntiCircleRotor>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiCircleRotor> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn anti_dot_product(self, other: AntiCircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                - (other[e45] * self[e45])
                - (other[scalar] * self[scalar]),
        );
    }
}
impl AntiDotProduct<AntiDualNum> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: AntiDualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[scalar] * other[scalar] * -1.0);
    }
}
impl AntiDotProduct<AntiLine> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: AntiLine) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) + (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]),
        );
    }
}
impl AntiDotProduct<AntiMotor> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn anti_dot_product(self, other: AntiMotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) + (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12])
                - (self[scalar] * other[scalar]),
        );
    }
}
impl AntiDotProduct<Dipole> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: Dipole) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (self[e23] * other[e23])
                + (self[e31] * other[e31])
                + (self[e12] * other[e12])
                + (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                - (self[e45] * other[e45]),
        );
    }
}
impl AntiDotProduct<DipoleInversion> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: DipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (self[e23] * other[e23])
                + (self[e31] * other[e31])
                + (self[e12] * other[e12])
                + (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                - (self[e45] * other[e45]),
        );
    }
}
impl AntiDotProduct<FlatPoint> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: FlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) - (self[e45] * other[e45]),
        );
    }
}
impl AntiDotProduct<Flector> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) - (self[e45] * other[e45]),
        );
    }
}
impl AntiDotProduct<MultiVector> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (self[e23] * other[e23])
                + (self[e31] * other[e31])
                + (self[e12] * other[e12])
                + (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                - (self[e45] * other[e45])
                - (self[scalar] * other[scalar]),
        );
    }
}
impl AntiDotProduct<Scalar> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: Scalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[scalar] * other[scalar] * -1.0);
    }
}
impl AntiDotProduct<VersorOdd> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn anti_dot_product(self, other: VersorOdd) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (self[e23] * other[e23])
                + (self[e31] * other[e31])
                + (self[e12] * other[e12])
                + (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                - (self[e45] * other[e45])
                - (self[scalar] * other[scalar]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for AntiDipoleInversion {
    type Output = anti_dot_product_partial<AntiDipoleInversion>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiDipoleInversion> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn anti_dot_product(self, other: AntiDipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) + (other[e4] * self[e5]) + (other[e5] * self[e4])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412])
                - (other[e1] * self[e1])
                - (other[e2] * self[e2])
                - (other[e3] * self[e3]),
        );
    }
}
impl AntiDotProduct<AntiFlatPoint> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiFlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) - (self[e423] * other[e235]) - (self[e431] * other[e315]) - (self[e412] * other[e125]),
        );
    }
}
impl AntiDotProduct<AntiFlector> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: AntiFlector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) + (self[e4] * other[e5])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e1] * other[e1])
                - (self[e2] * other[e2])
                - (self[e3] * other[e3]),
        );
    }
}
impl AntiDotProduct<AntiPlane> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiPlane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e4] * other[e5]) - (self[e1] * other[e1]) - (self[e2] * other[e2]) - (self[e3] * other[e3]));
    }
}
impl AntiDotProduct<Circle> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: Circle) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        );
    }
}
impl AntiDotProduct<CircleRotor> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: CircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        );
    }
}
impl AntiDotProduct<DualNum> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e5] * other[e4]);
    }
}
impl AntiDotProduct<Line> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: Line) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            -(self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435]),
        );
    }
}
impl AntiDotProduct<Motor> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e4] * other[e5])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435]),
        );
    }
}
impl AntiDotProduct<MultiVector> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) + (self[e4] * other[e5]) + (self[e5] * other[e4])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412])
                - (self[e1] * other[e1])
                - (self[e2] * other[e2])
                - (self[e3] * other[e3]),
        );
    }
}
impl AntiDotProduct<RoundPoint> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_dot_product(self, other: RoundPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e4] * other[e5]) + (self[e5] * other[e4]) - (self[e1] * other[e1]) - (self[e2] * other[e2]) - (self[e3] * other[e3]),
        );
    }
}
impl AntiDotProduct<VersorEven> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn anti_dot_product(self, other: VersorEven) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) + (self[e4] * other[e5]) + (self[e5] * other[e4])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412])
                - (self[e1] * other[e1])
                - (self[e2] * other[e2])
                - (self[e3] * other[e3]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for AntiDualNum {
    type Output = anti_dot_product_partial<AntiDualNum>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiCircleRotor> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: AntiCircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[scalar] * self[scalar] * -1.0);
    }
}
impl AntiDotProduct<AntiDualNum> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: AntiDualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[scalar] * self[scalar] * -1.0);
    }
}
impl AntiDotProduct<AntiMotor> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_dot_product(self, other: AntiMotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ -(self[e1234] * other[e3215]) - (self[scalar] * other[scalar]));
    }
}
impl AntiDotProduct<DipoleInversion> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: DipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e1234] * other[e3215] * -1.0);
    }
}
impl AntiDotProduct<Flector> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e1234] * other[e3215] * -1.0);
    }
}
impl AntiDotProduct<MultiVector> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ -(self[e1234] * other[e3215]) - (self[scalar] * other[scalar]));
    }
}
impl AntiDotProduct<Plane> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: Plane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e1234] * other[e3215] * -1.0);
    }
}
impl AntiDotProduct<Scalar> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: Scalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[scalar] * other[scalar] * -1.0);
    }
}
impl AntiDotProduct<Sphere> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: Sphere) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e1234] * other[e3215] * -1.0);
    }
}
impl AntiDotProduct<VersorOdd> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_dot_product(self, other: VersorOdd) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ -(self[e1234] * other[e3215]) - (self[scalar] * other[scalar]));
    }
}
impl std::ops::Div<anti_dot_product> for AntiFlatPoint {
    type Output = anti_dot_product_partial<AntiFlatPoint>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiDipoleInversion> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiDipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) - (other[e423] * self[e235]) - (other[e431] * self[e315]) - (other[e412] * self[e125]),
        );
    }
}
impl AntiDotProduct<AntiFlatPoint> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: AntiFlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e321] * self[e321]);
    }
}
impl AntiDotProduct<AntiFlector> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: AntiFlector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e321] * other[e321]);
    }
}
impl AntiDotProduct<Circle> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Circle) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]),
        );
    }
}
impl AntiDotProduct<CircleRotor> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: CircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]),
        );
    }
}
impl AntiDotProduct<MultiVector> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]),
        );
    }
}
impl AntiDotProduct<VersorEven> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: VersorEven) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for AntiFlector {
    type Output = anti_dot_product_partial<AntiFlector>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiDipoleInversion> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: AntiDipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) + (other[e4] * self[e5])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e1] * self[e1])
                - (other[e2] * self[e2])
                - (other[e3] * self[e3]),
        );
    }
}
impl AntiDotProduct<AntiFlatPoint> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: AntiFlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e321] * self[e321]);
    }
}
impl AntiDotProduct<AntiFlector> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiFlector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e321] * self[e321]) - (other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]));
    }
}
impl AntiDotProduct<AntiPlane> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_dot_product(self, other: AntiPlane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ -(self[e1] * other[e1]) - (self[e2] * other[e2]) - (self[e3] * other[e3]));
    }
}
impl AntiDotProduct<Circle> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Circle) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]),
        );
    }
}
impl AntiDotProduct<CircleRotor> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: CircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) - (self[e235] * other[e423]) - (self[e315] * other[e431]) - (self[e125] * other[e412]),
        );
    }
}
impl AntiDotProduct<DualNum> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e5] * other[e4]);
    }
}
impl AntiDotProduct<MultiVector> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) + (self[e5] * other[e4])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412])
                - (self[e1] * other[e1])
                - (self[e2] * other[e2])
                - (self[e3] * other[e3]),
        );
    }
}
impl AntiDotProduct<RoundPoint> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: RoundPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e5] * other[e4]) - (self[e1] * other[e1]) - (self[e2] * other[e2]) - (self[e3] * other[e3]));
    }
}
impl AntiDotProduct<VersorEven> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: VersorEven) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) + (self[e5] * other[e4])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412])
                - (self[e1] * other[e1])
                - (self[e2] * other[e2])
                - (self[e3] * other[e3]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for AntiLine {
    type Output = anti_dot_product_partial<AntiLine>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiCircleRotor> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: AntiCircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) + (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]),
        );
    }
}
impl AntiDotProduct<AntiLine> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_dot_product(self, other: AntiLine) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]));
    }
}
impl AntiDotProduct<AntiMotor> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_dot_product(self, other: AntiMotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]));
    }
}
impl AntiDotProduct<Dipole> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: Dipole) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]),
        );
    }
}
impl AntiDotProduct<DipoleInversion> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: DipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]),
        );
    }
}
impl AntiDotProduct<MultiVector> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]),
        );
    }
}
impl AntiDotProduct<VersorOdd> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: VersorOdd) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for AntiMotor {
    type Output = anti_dot_product_partial<AntiMotor>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiCircleRotor> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn anti_dot_product(self, other: AntiCircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) + (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12])
                - (other[scalar] * self[scalar]),
        );
    }
}
impl AntiDotProduct<AntiDualNum> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_dot_product(self, other: AntiDualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ -(other[e1234] * self[e3215]) - (other[scalar] * self[scalar]));
    }
}
impl AntiDotProduct<AntiLine> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_dot_product(self, other: AntiLine) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]));
    }
}
impl AntiDotProduct<AntiMotor> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiMotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) - (other[scalar] * self[scalar]),
        );
    }
}
impl AntiDotProduct<Dipole> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: Dipole) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]),
        );
    }
}
impl AntiDotProduct<DipoleInversion> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn anti_dot_product(self, other: DipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43])
                - (self[e3215] * other[e1234]),
        );
    }
}
impl AntiDotProduct<MultiVector> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43])
                - (self[scalar] * other[scalar])
                - (self[e3215] * other[e1234]),
        );
    }
}
impl AntiDotProduct<Scalar> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: Scalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[scalar] * other[scalar] * -1.0);
    }
}
impl AntiDotProduct<Sphere> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: Sphere) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e3215] * other[e1234] * -1.0);
    }
}
impl AntiDotProduct<VersorOdd> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: VersorOdd) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e23] * other[e23]) + (self[e31] * other[e31]) + (self[e12] * other[e12]) + (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43])
                - (self[scalar] * other[scalar])
                - (self[e3215] * other[e1234]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for AntiPlane {
    type Output = anti_dot_product_partial<AntiPlane>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiDipoleInversion> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiDipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e4] * self[e5]) - (other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]));
    }
}
impl AntiDotProduct<AntiFlector> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_dot_product(self, other: AntiFlector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ -(other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]));
    }
}
impl AntiDotProduct<AntiPlane> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_dot_product(self, other: AntiPlane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ -(other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]));
    }
}
impl AntiDotProduct<DualNum> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e5] * other[e4]);
    }
}
impl AntiDotProduct<MultiVector> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e5] * other[e4]) - (self[e1] * other[e1]) - (self[e2] * other[e2]) - (self[e3] * other[e3]));
    }
}
impl AntiDotProduct<RoundPoint> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: RoundPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e5] * other[e4]) - (self[e1] * other[e1]) - (self[e2] * other[e2]) - (self[e3] * other[e3]));
    }
}
impl AntiDotProduct<VersorEven> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: VersorEven) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e5] * other[e4]) - (self[e1] * other[e1]) - (self[e2] * other[e2]) - (self[e3] * other[e3]));
    }
}
impl std::ops::Div<anti_dot_product> for AntiScalar {
    type Output = anti_dot_product_partial<AntiScalar>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiScalar> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345]);
    }
}
impl AntiDotProduct<CircleRotor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: CircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * other[e12345]);
    }
}
impl AntiDotProduct<DualNum> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * other[e12345]);
    }
}
impl AntiDotProduct<Motor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * other[e12345]);
    }
}
impl AntiDotProduct<MultiVector> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * other[e12345]);
    }
}
impl AntiDotProduct<VersorEven> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: VersorEven) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * other[e12345]);
    }
}
impl std::ops::Div<anti_dot_product> for Circle {
    type Output = anti_dot_product_partial<Circle>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiDipoleInversion> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: AntiDipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<AntiFlatPoint> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiFlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) - (other[e235] * self[e423]) - (other[e315] * self[e431]) - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<AntiFlector> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiFlector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) - (other[e235] * self[e423]) - (other[e315] * self[e431]) - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<Circle> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: Circle) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<CircleRotor> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: CircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        );
    }
}
impl AntiDotProduct<Line> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: Line) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            -(self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435]),
        );
    }
}
impl AntiDotProduct<Motor> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            -(self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435]),
        );
    }
}
impl AntiDotProduct<MultiVector> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        );
    }
}
impl AntiDotProduct<VersorEven> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: VersorEven) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for CircleRotor {
    type Output = anti_dot_product_partial<CircleRotor>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiDipoleInversion> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: AntiDipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<AntiFlatPoint> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiFlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) - (other[e235] * self[e423]) - (other[e315] * self[e431]) - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<AntiFlector> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiFlector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) - (other[e235] * self[e423]) - (other[e315] * self[e431]) - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<AntiScalar> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345]);
    }
}
impl AntiDotProduct<Circle> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: Circle) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<CircleRotor> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn anti_dot_product(self, other: CircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) + (other[e12345] * self[e12345])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<DualNum> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * other[e12345]);
    }
}
impl AntiDotProduct<Line> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: Line) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            -(self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435]),
        );
    }
}
impl AntiDotProduct<Motor> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e12345] * other[e12345])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435]),
        );
    }
}
impl AntiDotProduct<MultiVector> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) + (self[e12345] * other[e12345])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        );
    }
}
impl AntiDotProduct<VersorEven> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn anti_dot_product(self, other: VersorEven) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e321] * other[e321]) + (self[e12345] * other[e12345])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for Dipole {
    type Output = anti_dot_product_partial<Dipole>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiCircleRotor> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: AntiCircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                - (other[e45] * self[e45]),
        );
    }
}
impl AntiDotProduct<AntiLine> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: AntiLine) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]),
        );
    }
}
impl AntiDotProduct<AntiMotor> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: AntiMotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]),
        );
    }
}
impl AntiDotProduct<Dipole> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: Dipole) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                - (other[e45] * self[e45]),
        );
    }
}
impl AntiDotProduct<DipoleInversion> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: DipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (self[e23] * other[e23])
                + (self[e31] * other[e31])
                + (self[e12] * other[e12])
                + (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                - (self[e45] * other[e45]),
        );
    }
}
impl AntiDotProduct<FlatPoint> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: FlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) - (self[e45] * other[e45]),
        );
    }
}
impl AntiDotProduct<Flector> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) - (self[e45] * other[e45]),
        );
    }
}
impl AntiDotProduct<MultiVector> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (self[e23] * other[e23])
                + (self[e31] * other[e31])
                + (self[e12] * other[e12])
                + (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                - (self[e45] * other[e45]),
        );
    }
}
impl AntiDotProduct<VersorOdd> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: VersorOdd) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (self[e23] * other[e23])
                + (self[e31] * other[e31])
                + (self[e12] * other[e12])
                + (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                - (self[e45] * other[e45]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for DipoleInversion {
    type Output = anti_dot_product_partial<DipoleInversion>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiCircleRotor> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: AntiCircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                - (other[e45] * self[e45]),
        );
    }
}
impl AntiDotProduct<AntiDualNum> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: AntiDualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e1234] * self[e3215] * -1.0);
    }
}
impl AntiDotProduct<AntiLine> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: AntiLine) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]),
        );
    }
}
impl AntiDotProduct<AntiMotor> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn anti_dot_product(self, other: AntiMotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43])
                - (other[e3215] * self[e1234]),
        );
    }
}
impl AntiDotProduct<Dipole> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: Dipole) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                - (other[e45] * self[e45]),
        );
    }
}
impl AntiDotProduct<DipoleInversion> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn anti_dot_product(self, other: DipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                + (other[e4235] * self[e4235])
                + (other[e4315] * self[e4315])
                + (other[e4125] * self[e4125])
                - (other[e45] * self[e45])
                - (other[e1234] * self[e3215])
                - (other[e3215] * self[e1234]),
        );
    }
}
impl AntiDotProduct<FlatPoint> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: FlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15]) + (self[e42] * other[e25]) + (self[e43] * other[e35]) - (self[e45] * other[e45]),
        );
    }
}
impl AntiDotProduct<Flector> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (self[e4235] * other[e4235])
                + (self[e4315] * other[e4315])
                + (self[e4125] * other[e4125])
                - (self[e45] * other[e45])
                - (self[e1234] * other[e3215]),
        );
    }
}
impl AntiDotProduct<MultiVector> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (self[e23] * other[e23])
                + (self[e31] * other[e31])
                + (self[e12] * other[e12])
                + (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                + (self[e4235] * other[e4235])
                + (self[e4315] * other[e4315])
                + (self[e4125] * other[e4125])
                - (self[e45] * other[e45])
                - (self[e1234] * other[e3215])
                - (self[e3215] * other[e1234]),
        );
    }
}
impl AntiDotProduct<Plane> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Plane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e4235] * other[e4235]) + (self[e4315] * other[e4315]) + (self[e4125] * other[e4125]) - (self[e1234] * other[e3215]),
        );
    }
}
impl AntiDotProduct<Sphere> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_dot_product(self, other: Sphere) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e4235] * other[e4235]) + (self[e4315] * other[e4315]) + (self[e4125] * other[e4125]) - (self[e1234] * other[e3215]) - (self[e3215] * other[e1234]),
        );
    }
}
impl AntiDotProduct<VersorOdd> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn anti_dot_product(self, other: VersorOdd) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (self[e23] * other[e23])
                + (self[e31] * other[e31])
                + (self[e12] * other[e12])
                + (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                + (self[e4235] * other[e4235])
                + (self[e4315] * other[e4315])
                + (self[e4125] * other[e4125])
                - (self[e45] * other[e45])
                - (self[e1234] * other[e3215])
                - (self[e3215] * other[e1234]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for DualNum {
    type Output = anti_dot_product_partial<DualNum>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiDipoleInversion> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: AntiDipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e5] * self[e4]);
    }
}
impl AntiDotProduct<AntiFlector> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: AntiFlector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e5] * self[e4]);
    }
}
impl AntiDotProduct<AntiPlane> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: AntiPlane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e5] * self[e4]);
    }
}
impl AntiDotProduct<AntiScalar> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345]);
    }
}
impl AntiDotProduct<CircleRotor> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: CircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345]);
    }
}
impl AntiDotProduct<DualNum> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345]);
    }
}
impl AntiDotProduct<Motor> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e4] * other[e5]) + (self[e12345] * other[e12345]));
    }
}
impl AntiDotProduct<MultiVector> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e4] * other[e5]) + (self[e12345] * other[e12345]));
    }
}
impl AntiDotProduct<RoundPoint> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: RoundPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e4] * other[e5]);
    }
}
impl AntiDotProduct<VersorEven> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_dot_product(self, other: VersorEven) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e4] * other[e5]) + (self[e12345] * other[e12345]));
    }
}
impl std::ops::Div<anti_dot_product> for FlatPoint {
    type Output = anti_dot_product_partial<FlatPoint>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiCircleRotor> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiCircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]),
        );
    }
}
impl AntiDotProduct<Dipole> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Dipole) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]),
        );
    }
}
impl AntiDotProduct<DipoleInversion> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: DipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]),
        );
    }
}
impl AntiDotProduct<FlatPoint> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: FlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e45] * self[e45] * -1.0);
    }
}
impl AntiDotProduct<Flector> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e45] * other[e45] * -1.0);
    }
}
impl AntiDotProduct<MultiVector> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]) - (self[e45] * other[e45]),
        );
    }
}
impl AntiDotProduct<VersorOdd> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: VersorOdd) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e15] * other[e41]) + (self[e25] * other[e42]) + (self[e35] * other[e43]) - (self[e45] * other[e45]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for Flector {
    type Output = anti_dot_product_partial<Flector>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiCircleRotor> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiCircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]),
        );
    }
}
impl AntiDotProduct<AntiDualNum> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: AntiDualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e1234] * self[e3215] * -1.0);
    }
}
impl AntiDotProduct<Dipole> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Dipole) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15]) + (other[e42] * self[e25]) + (other[e43] * self[e35]) - (other[e45] * self[e45]),
        );
    }
}
impl AntiDotProduct<DipoleInversion> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: DipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e4235] * self[e4235])
                + (other[e4315] * self[e4315])
                + (other[e4125] * self[e4125])
                - (other[e45] * self[e45])
                - (other[e1234] * self[e3215]),
        );
    }
}
impl AntiDotProduct<FlatPoint> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: FlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e45] * self[e45] * -1.0);
    }
}
impl AntiDotProduct<Flector> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]) - (other[e45] * self[e45]),
        );
    }
}
impl AntiDotProduct<MultiVector> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                + (self[e4235] * other[e4235])
                + (self[e4315] * other[e4315])
                + (self[e4125] * other[e4125])
                - (self[e45] * other[e45])
                - (self[e3215] * other[e1234]),
        );
    }
}
impl AntiDotProduct<Plane> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_dot_product(self, other: Plane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e4235] * other[e4235]) + (self[e4315] * other[e4315]) + (self[e4125] * other[e4125]));
    }
}
impl AntiDotProduct<Sphere> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Sphere) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e4235] * other[e4235]) + (self[e4315] * other[e4315]) + (self[e4125] * other[e4125]) - (self[e3215] * other[e1234]),
        );
    }
}
impl AntiDotProduct<VersorOdd> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: VersorOdd) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                + (self[e4235] * other[e4235])
                + (self[e4315] * other[e4315])
                + (self[e4125] * other[e4125])
                - (self[e45] * other[e45])
                - (self[e3215] * other[e1234]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for Line {
    type Output = anti_dot_product_partial<Line>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiDipoleInversion> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: AntiDipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            -(other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435]),
        );
    }
}
impl AntiDotProduct<Circle> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: Circle) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            -(other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435]),
        );
    }
}
impl AntiDotProduct<CircleRotor> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: CircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            -(other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435]),
        );
    }
}
impl AntiDotProduct<Line> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_dot_product(self, other: Line) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ -(other[e415] * self[e415]) - (other[e425] * self[e425]) - (other[e435] * self[e435]));
    }
}
impl AntiDotProduct<Motor> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ -(self[e415] * other[e415]) - (self[e425] * other[e425]) - (self[e435] * other[e435]));
    }
}
impl AntiDotProduct<MultiVector> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            -(self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        );
    }
}
impl AntiDotProduct<VersorEven> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: VersorEven) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            -(self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for Motor {
    type Output = anti_dot_product_partial<Motor>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiDipoleInversion> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn anti_dot_product(self, other: AntiDipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4] * self[e5])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435]),
        );
    }
}
impl AntiDotProduct<AntiScalar> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345]);
    }
}
impl AntiDotProduct<Circle> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: Circle) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            -(other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435]),
        );
    }
}
impl AntiDotProduct<CircleRotor> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn anti_dot_product(self, other: CircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e12345] * self[e12345])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435]),
        );
    }
}
impl AntiDotProduct<DualNum> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e4] * self[e5]) + (other[e12345] * self[e12345]));
    }
}
impl AntiDotProduct<Line> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_dot_product(self, other: Line) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ -(other[e415] * self[e415]) - (other[e425] * self[e425]) - (other[e435] * self[e435]));
    }
}
impl AntiDotProduct<Motor> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e12345] * self[e12345]) - (other[e415] * self[e415]) - (other[e425] * self[e425]) - (other[e435] * self[e435]),
        );
    }
}
impl AntiDotProduct<MultiVector> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e12345] * other[e12345]) + (self[e5] * other[e4])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        );
    }
}
impl AntiDotProduct<RoundPoint> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: RoundPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e5] * other[e4]);
    }
}
impl AntiDotProduct<VersorEven> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: VersorEven) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e12345] * other[e12345]) + (self[e5] * other[e4])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for MultiVector {
    type Output = anti_dot_product_partial<MultiVector>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiCircleRotor> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn anti_dot_product(self, other: AntiCircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                - (other[e45] * self[e45])
                - (other[scalar] * self[scalar]),
        );
    }
}
impl AntiDotProduct<AntiDipoleInversion> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn anti_dot_product(self, other: AntiDipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) + (other[e4] * self[e5]) + (other[e5] * self[e4])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412])
                - (other[e1] * self[e1])
                - (other[e2] * self[e2])
                - (other[e3] * self[e3]),
        );
    }
}
impl AntiDotProduct<AntiDualNum> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_dot_product(self, other: AntiDualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ -(other[e1234] * self[e3215]) - (other[scalar] * self[scalar]));
    }
}
impl AntiDotProduct<AntiFlatPoint> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiFlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) - (other[e235] * self[e423]) - (other[e315] * self[e431]) - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<AntiFlector> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: AntiFlector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) + (other[e5] * self[e4])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412])
                - (other[e1] * self[e1])
                - (other[e2] * self[e2])
                - (other[e3] * self[e3]),
        );
    }
}
impl AntiDotProduct<AntiLine> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: AntiLine) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]),
        );
    }
}
impl AntiDotProduct<AntiMotor> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: AntiMotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43])
                - (other[scalar] * self[scalar])
                - (other[e3215] * self[e1234]),
        );
    }
}
impl AntiDotProduct<AntiPlane> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiPlane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e5] * self[e4]) - (other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]));
    }
}
impl AntiDotProduct<AntiScalar> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345]);
    }
}
impl AntiDotProduct<Circle> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: Circle) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<CircleRotor> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn anti_dot_product(self, other: CircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) + (other[e12345] * self[e12345])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<Dipole> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: Dipole) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                - (other[e45] * self[e45]),
        );
    }
}
impl AntiDotProduct<DipoleInversion> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn anti_dot_product(self, other: DipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                + (other[e4235] * self[e4235])
                + (other[e4315] * self[e4315])
                + (other[e4125] * self[e4125])
                - (other[e45] * self[e45])
                - (other[e1234] * self[e3215])
                - (other[e3215] * self[e1234]),
        );
    }
}
impl AntiDotProduct<DualNum> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e4] * self[e5]) + (other[e12345] * self[e12345]));
    }
}
impl AntiDotProduct<FlatPoint> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: FlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]) - (other[e45] * self[e45]),
        );
    }
}
impl AntiDotProduct<Flector> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                + (other[e4235] * self[e4235])
                + (other[e4315] * self[e4315])
                + (other[e4125] * self[e4125])
                - (other[e45] * self[e45])
                - (other[e3215] * self[e1234]),
        );
    }
}
impl AntiDotProduct<Line> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: Line) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            -(other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<Motor> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e12345] * self[e12345]) + (other[e5] * self[e4])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<MultiVector> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       31       32        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e12345] * self[e12345])
                + (other[e4] * self[e5])
                + (other[e5] * self[e4])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                + (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e321] * self[e321])
                + (other[e4235] * self[e4235])
                + (other[e4315] * self[e4315])
                + (other[e4125] * self[e4125])
                - (other[scalar] * self[scalar])
                - (other[e1] * self[e1])
                - (other[e2] * self[e2])
                - (other[e3] * self[e3])
                - (other[e45] * self[e45])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412])
                - (other[e3215] * self[e1234])
                - (other[e1234] * self[e3215]),
        );
    }
}
impl AntiDotProduct<Plane> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Plane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e4235] * other[e4235]) + (self[e4315] * other[e4315]) + (self[e4125] * other[e4125]) - (self[e1234] * other[e3215]),
        );
    }
}
impl AntiDotProduct<RoundPoint> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_dot_product(self, other: RoundPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e4] * other[e5]) + (self[e5] * other[e4]) - (self[e1] * other[e1]) - (self[e2] * other[e2]) - (self[e3] * other[e3]),
        );
    }
}
impl AntiDotProduct<Scalar> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: Scalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[scalar] * other[scalar] * -1.0);
    }
}
impl AntiDotProduct<Sphere> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_dot_product(self, other: Sphere) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e4235] * other[e4235]) + (self[e4315] * other[e4315]) + (self[e4125] * other[e4125]) - (self[e3215] * other[e1234]) - (self[e1234] * other[e3215]),
        );
    }
}
impl AntiDotProduct<VersorEven> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       16        0
    fn anti_dot_product(self, other: VersorEven) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e12345] * other[e12345]) + (self[e4] * other[e5]) + (self[e5] * other[e4]) + (self[e321] * other[e321])
                - (self[e1] * other[e1])
                - (self[e2] * other[e2])
                - (self[e3] * other[e3])
                - (self[e415] * other[e415])
                - (self[e425] * other[e425])
                - (self[e435] * other[e435])
                - (self[e423] * other[e235])
                - (self[e431] * other[e315])
                - (self[e412] * other[e125])
                - (self[e235] * other[e423])
                - (self[e315] * other[e431])
                - (self[e125] * other[e412]),
        );
    }
}
impl AntiDotProduct<VersorOdd> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       16        0
    fn anti_dot_product(self, other: VersorOdd) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e15] * other[e41])
                + (self[e25] * other[e42])
                + (self[e35] * other[e43])
                + (self[e41] * other[e15])
                + (self[e42] * other[e25])
                + (self[e43] * other[e35])
                + (self[e23] * other[e23])
                + (self[e31] * other[e31])
                + (self[e12] * other[e12])
                + (self[e4235] * other[e4235])
                + (self[e4315] * other[e4315])
                + (self[e4125] * other[e4125])
                - (self[scalar] * other[scalar])
                - (self[e45] * other[e45])
                - (self[e3215] * other[e1234])
                - (self[e1234] * other[e3215]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for Plane {
    type Output = anti_dot_product_partial<Plane>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiDualNum> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: AntiDualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e1234] * self[e3215] * -1.0);
    }
}
impl AntiDotProduct<DipoleInversion> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: DipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]) - (other[e1234] * self[e3215]),
        );
    }
}
impl AntiDotProduct<Flector> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]));
    }
}
impl AntiDotProduct<MultiVector> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]) - (other[e1234] * self[e3215]),
        );
    }
}
impl AntiDotProduct<Plane> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_dot_product(self, other: Plane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]));
    }
}
impl AntiDotProduct<Sphere> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Sphere) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e4235] * other[e4235]) + (self[e4315] * other[e4315]) + (self[e4125] * other[e4125]) - (self[e3215] * other[e1234]),
        );
    }
}
impl AntiDotProduct<VersorOdd> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: VersorOdd) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e4235] * other[e4235]) + (self[e4315] * other[e4315]) + (self[e4125] * other[e4125]) - (self[e3215] * other[e1234]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for RoundPoint {
    type Output = anti_dot_product_partial<RoundPoint>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiDipoleInversion> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_dot_product(self, other: AntiDipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4] * self[e5]) + (other[e5] * self[e4]) - (other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]),
        );
    }
}
impl AntiDotProduct<AntiFlector> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiFlector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e5] * self[e4]) - (other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]));
    }
}
impl AntiDotProduct<AntiPlane> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiPlane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e5] * self[e4]) - (other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]));
    }
}
impl AntiDotProduct<DualNum> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e4] * self[e5]);
    }
}
impl AntiDotProduct<Motor> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e5] * self[e4]);
    }
}
impl AntiDotProduct<MultiVector> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4] * self[e5]) + (other[e5] * self[e4]) - (other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]),
        );
    }
}
impl AntiDotProduct<RoundPoint> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_dot_product(self, other: RoundPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4] * self[e5]) + (other[e5] * self[e4]) - (other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]),
        );
    }
}
impl AntiDotProduct<VersorEven> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_dot_product(self, other: VersorEven) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e4] * other[e5]) + (self[e5] * other[e4]) - (self[e1] * other[e1]) - (self[e2] * other[e2]) - (self[e3] * other[e3]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for Scalar {
    type Output = anti_dot_product_partial<Scalar>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiCircleRotor> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: AntiCircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[scalar] * self[scalar] * -1.0);
    }
}
impl AntiDotProduct<AntiDualNum> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: AntiDualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[scalar] * self[scalar] * -1.0);
    }
}
impl AntiDotProduct<AntiMotor> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: AntiMotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[scalar] * self[scalar] * -1.0);
    }
}
impl AntiDotProduct<MultiVector> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[scalar] * self[scalar] * -1.0);
    }
}
impl AntiDotProduct<Scalar> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: Scalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[scalar] * self[scalar] * -1.0);
    }
}
impl AntiDotProduct<VersorOdd> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: VersorOdd) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[scalar] * other[scalar] * -1.0);
    }
}
impl std::ops::Div<anti_dot_product> for Sphere {
    type Output = anti_dot_product_partial<Sphere>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiDualNum> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: AntiDualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e1234] * self[e3215] * -1.0);
    }
}
impl AntiDotProduct<AntiMotor> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: AntiMotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e3215] * self[e1234] * -1.0);
    }
}
impl AntiDotProduct<DipoleInversion> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_dot_product(self, other: DipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]) - (other[e1234] * self[e3215]) - (other[e3215] * self[e1234]),
        );
    }
}
impl AntiDotProduct<Flector> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]) - (other[e3215] * self[e1234]),
        );
    }
}
impl AntiDotProduct<MultiVector> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]) - (other[e3215] * self[e1234]) - (other[e1234] * self[e3215]),
        );
    }
}
impl AntiDotProduct<Plane> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Plane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]) - (other[e3215] * self[e1234]),
        );
    }
}
impl AntiDotProduct<Sphere> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_dot_product(self, other: Sphere) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]) - (other[e3215] * self[e1234]) - (other[e1234] * self[e3215]),
        );
    }
}
impl AntiDotProduct<VersorOdd> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_dot_product(self, other: VersorOdd) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self[e4235] * other[e4235]) + (self[e4315] * other[e4315]) + (self[e4125] * other[e4125]) - (self[e3215] * other[e1234]) - (self[e1234] * other[e3215]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for VersorEven {
    type Output = anti_dot_product_partial<VersorEven>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiDipoleInversion> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn anti_dot_product(self, other: AntiDipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) + (other[e4] * self[e5]) + (other[e5] * self[e4])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412])
                - (other[e1] * self[e1])
                - (other[e2] * self[e2])
                - (other[e3] * self[e3]),
        );
    }
}
impl AntiDotProduct<AntiFlatPoint> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiFlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) - (other[e235] * self[e423]) - (other[e315] * self[e431]) - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<AntiFlector> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: AntiFlector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) + (other[e5] * self[e4])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412])
                - (other[e1] * self[e1])
                - (other[e2] * self[e2])
                - (other[e3] * self[e3]),
        );
    }
}
impl AntiDotProduct<AntiPlane> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: AntiPlane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e5] * self[e4]) - (other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]));
    }
}
impl AntiDotProduct<AntiScalar> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_dot_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345]);
    }
}
impl AntiDotProduct<Circle> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: Circle) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<CircleRotor> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn anti_dot_product(self, other: CircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e321] * self[e321]) + (other[e12345] * self[e12345])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<DualNum> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_dot_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e4] * self[e5]) + (other[e12345] * self[e12345]));
    }
}
impl AntiDotProduct<Line> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: Line) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            -(other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<Motor> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e12345] * self[e12345]) + (other[e5] * self[e4])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<MultiVector> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       16        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e12345] * self[e12345]) + (other[e4] * self[e5]) + (other[e5] * self[e4]) + (other[e321] * self[e321])
                - (other[e1] * self[e1])
                - (other[e2] * self[e2])
                - (other[e3] * self[e3])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412]),
        );
    }
}
impl AntiDotProduct<RoundPoint> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_dot_product(self, other: RoundPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4] * self[e5]) + (other[e5] * self[e4]) - (other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]),
        );
    }
}
impl AntiDotProduct<VersorEven> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       16        0
    fn anti_dot_product(self, other: VersorEven) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e12345] * self[e12345]) + (other[e321] * self[e321]) + (other[e5] * self[e4]) + (other[e4] * self[e5])
                - (other[e423] * self[e235])
                - (other[e431] * self[e315])
                - (other[e412] * self[e125])
                - (other[e415] * self[e415])
                - (other[e425] * self[e425])
                - (other[e435] * self[e435])
                - (other[e235] * self[e423])
                - (other[e315] * self[e431])
                - (other[e125] * self[e412])
                - (other[e1] * self[e1])
                - (other[e2] * self[e2])
                - (other[e3] * self[e3]),
        );
    }
}
impl std::ops::Div<anti_dot_product> for VersorOdd {
    type Output = anti_dot_product_partial<VersorOdd>;
    fn div(self, _rhs: anti_dot_product) -> Self::Output {
        anti_dot_product_partial(self)
    }
}
impl AntiDotProduct<AntiCircleRotor> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn anti_dot_product(self, other: AntiCircleRotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                - (other[e45] * self[e45])
                - (other[scalar] * self[scalar]),
        );
    }
}
impl AntiDotProduct<AntiDualNum> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn anti_dot_product(self, other: AntiDualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ -(other[e1234] * self[e3215]) - (other[scalar] * self[scalar]));
    }
}
impl AntiDotProduct<AntiLine> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_dot_product(self, other: AntiLine) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]),
        );
    }
}
impl AntiDotProduct<AntiMotor> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: AntiMotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) + (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43])
                - (other[scalar] * self[scalar])
                - (other[e3215] * self[e1234]),
        );
    }
}
impl AntiDotProduct<Dipole> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_dot_product(self, other: Dipole) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                - (other[e45] * self[e45]),
        );
    }
}
impl AntiDotProduct<DipoleInversion> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn anti_dot_product(self, other: DipoleInversion) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                + (other[e4235] * self[e4235])
                + (other[e4315] * self[e4315])
                + (other[e4125] * self[e4125])
                - (other[e45] * self[e45])
                - (other[e1234] * self[e3215])
                - (other[e3215] * self[e1234]),
        );
    }
}
impl AntiDotProduct<FlatPoint> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: FlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e15] * self[e41]) + (other[e25] * self[e42]) + (other[e35] * self[e43]) - (other[e45] * self[e45]),
        );
    }
}
impl AntiDotProduct<Flector> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn anti_dot_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                + (other[e4235] * self[e4235])
                + (other[e4315] * self[e4315])
                + (other[e4125] * self[e4125])
                - (other[e45] * self[e45])
                - (other[e3215] * self[e1234]),
        );
    }
}
impl AntiDotProduct<MultiVector> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       16        0
    fn anti_dot_product(self, other: MultiVector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                + (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e4235] * self[e4235])
                + (other[e4315] * self[e4315])
                + (other[e4125] * self[e4125])
                - (other[scalar] * self[scalar])
                - (other[e45] * self[e45])
                - (other[e3215] * self[e1234])
                - (other[e1234] * self[e3215]),
        );
    }
}
impl AntiDotProduct<Plane> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_dot_product(self, other: Plane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]) - (other[e3215] * self[e1234]),
        );
    }
}
impl AntiDotProduct<Scalar> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_dot_product(self, other: Scalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[scalar] * self[scalar] * -1.0);
    }
}
impl AntiDotProduct<Sphere> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_dot_product(self, other: Sphere) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]) - (other[e3215] * self[e1234]) - (other[e1234] * self[e3215]),
        );
    }
}
impl AntiDotProduct<VersorOdd> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       16        0
    fn anti_dot_product(self, other: VersorOdd) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e41] * self[e15])
                + (other[e42] * self[e25])
                + (other[e43] * self[e35])
                + (other[e23] * self[e23])
                + (other[e31] * self[e31])
                + (other[e12] * self[e12])
                + (other[e15] * self[e41])
                + (other[e25] * self[e42])
                + (other[e35] * self[e43])
                + (other[e4235] * self[e4235])
                + (other[e4315] * self[e4315])
                + (other[e4125] * self[e4125])
                - (other[scalar] * self[scalar])
                - (other[e45] * self[e45])
                - (other[e1234] * self[e3215])
                - (other[e3215] * self[e1234]),
        );
    }
}
