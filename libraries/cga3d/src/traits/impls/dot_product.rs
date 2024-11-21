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
impl std::ops::Div<dot_product> for AntiCircleRotor {
    type Output = dot_product_partial<AntiCircleRotor>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiCircleRotor> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn dot_product(self, other: AntiCircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) + (other[scalar] * self[scalar])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<AntiDualNum> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: AntiDualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar] * other[scalar]);
    }
}
impl DotProduct<AntiLine> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: AntiLine) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(self[e41] * other[e15]) - (self[e42] * other[e25]) - (self[e43] * other[e35]) - (self[e23] * other[e23]) - (self[e31] * other[e31]) - (self[e12] * other[e12]),
        );
    }
}
impl DotProduct<AntiMotor> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn dot_product(self, other: AntiMotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[scalar] * other[scalar])
                - (self[e41] * other[e15])
                - (self[e42] * other[e25])
                - (self[e43] * other[e35])
                - (self[e23] * other[e23])
                - (self[e31] * other[e31])
                - (self[e12] * other[e12]),
        );
    }
}
impl DotProduct<Dipole> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: Dipole) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45])
                - (self[e41] * other[e15])
                - (self[e42] * other[e25])
                - (self[e43] * other[e35])
                - (self[e23] * other[e23])
                - (self[e31] * other[e31])
                - (self[e12] * other[e12])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43]),
        );
    }
}
impl DotProduct<DipoleInversion> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: DipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45])
                - (self[e41] * other[e15])
                - (self[e42] * other[e25])
                - (self[e43] * other[e35])
                - (self[e23] * other[e23])
                - (self[e31] * other[e31])
                - (self[e12] * other[e12])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43]),
        );
    }
}
impl DotProduct<FlatPoint> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: FlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45]) - (self[e41] * other[e15]) - (self[e42] * other[e25]) - (self[e43] * other[e35]),
        );
    }
}
impl DotProduct<Flector> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45]) - (self[e41] * other[e15]) - (self[e42] * other[e25]) - (self[e43] * other[e35]),
        );
    }
}
impl DotProduct<MultiVector> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45]) + (self[scalar] * other[scalar])
                - (self[e41] * other[e15])
                - (self[e42] * other[e25])
                - (self[e43] * other[e35])
                - (self[e23] * other[e23])
                - (self[e31] * other[e31])
                - (self[e12] * other[e12])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43]),
        );
    }
}
impl DotProduct<Scalar> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: Scalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar] * other[scalar]);
    }
}
impl DotProduct<VersorOdd> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn dot_product(self, other: VersorOdd) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45]) + (self[scalar] * other[scalar])
                - (self[e41] * other[e15])
                - (self[e42] * other[e25])
                - (self[e43] * other[e35])
                - (self[e23] * other[e23])
                - (self[e31] * other[e31])
                - (self[e12] * other[e12])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43]),
        );
    }
}
impl std::ops::Div<dot_product> for AntiDipoleInversion {
    type Output = dot_product_partial<AntiDipoleInversion>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiDipoleInversion> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn dot_product(self, other: AntiDipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                + (other[e1] * self[e1])
                + (other[e2] * self[e2])
                + (other[e3] * self[e3])
                - (other[e321] * self[e321])
                - (other[e4] * self[e5])
                - (other[e5] * self[e4]),
        );
    }
}
impl DotProduct<AntiFlatPoint> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiFlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235]) + (self[e431] * other[e315]) + (self[e412] * other[e125]) - (self[e321] * other[e321]),
        );
    }
}
impl DotProduct<AntiFlector> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: AntiFlector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235]) + (self[e431] * other[e315]) + (self[e412] * other[e125]) + (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3])
                - (self[e321] * other[e321])
                - (self[e4] * other[e5]),
        );
    }
}
impl DotProduct<AntiPlane> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiPlane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]) - (self[e4] * other[e5]));
    }
}
impl DotProduct<Circle> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: Circle) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                + (self[e235] * other[e423])
                + (self[e315] * other[e431])
                + (self[e125] * other[e412])
                - (self[e321] * other[e321]),
        );
    }
}
impl DotProduct<CircleRotor> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: CircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                + (self[e235] * other[e423])
                + (self[e315] * other[e431])
                + (self[e125] * other[e412])
                - (self[e321] * other[e321]),
        );
    }
}
impl DotProduct<DualNum> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: DualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e5] * other[e4] * -1.0);
    }
}
impl DotProduct<Line> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: Line) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435]),
        );
    }
}
impl DotProduct<Motor> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                - (self[e4] * other[e5]),
        );
    }
}
impl DotProduct<MultiVector> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                + (self[e235] * other[e423])
                + (self[e315] * other[e431])
                + (self[e125] * other[e412])
                + (self[e1] * other[e1])
                + (self[e2] * other[e2])
                + (self[e3] * other[e3])
                - (self[e321] * other[e321])
                - (self[e4] * other[e5])
                - (self[e5] * other[e4]),
        );
    }
}
impl DotProduct<RoundPoint> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn dot_product(self, other: RoundPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]) - (self[e4] * other[e5]) - (self[e5] * other[e4]),
        );
    }
}
impl DotProduct<VersorEven> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn dot_product(self, other: VersorEven) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                + (self[e235] * other[e423])
                + (self[e315] * other[e431])
                + (self[e125] * other[e412])
                + (self[e1] * other[e1])
                + (self[e2] * other[e2])
                + (self[e3] * other[e3])
                - (self[e321] * other[e321])
                - (self[e4] * other[e5])
                - (self[e5] * other[e4]),
        );
    }
}
impl std::ops::Div<dot_product> for AntiDualNum {
    type Output = dot_product_partial<AntiDualNum>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiCircleRotor> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: AntiCircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]);
    }
}
impl DotProduct<AntiDualNum> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: AntiDualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]);
    }
}
impl DotProduct<AntiMotor> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn dot_product(self, other: AntiMotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[e1234] * other[e3215]) + (self[scalar] * other[scalar]));
    }
}
impl DotProduct<DipoleInversion> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: DipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e1234] * other[e3215]);
    }
}
impl DotProduct<Flector> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e1234] * other[e3215]);
    }
}
impl DotProduct<MultiVector> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[e1234] * other[e3215]) + (self[scalar] * other[scalar]));
    }
}
impl DotProduct<Plane> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: Plane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e1234] * other[e3215]);
    }
}
impl DotProduct<Scalar> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: Scalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar] * other[scalar]);
    }
}
impl DotProduct<Sphere> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: Sphere) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e1234] * other[e3215]);
    }
}
impl DotProduct<VersorOdd> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn dot_product(self, other: VersorOdd) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[e1234] * other[e3215]) + (self[scalar] * other[scalar]));
    }
}
impl std::ops::Div<dot_product> for AntiFlatPoint {
    type Output = dot_product_partial<AntiFlatPoint>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiDipoleInversion> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiDipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235]) + (other[e431] * self[e315]) + (other[e412] * self[e125]) - (other[e321] * self[e321]),
        );
    }
}
impl DotProduct<AntiFlatPoint> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: AntiFlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * self[e321] * -1.0);
    }
}
impl DotProduct<AntiFlector> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: AntiFlector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321] * other[e321] * -1.0);
    }
}
impl DotProduct<Circle> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Circle) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e235] * other[e423]) + (self[e315] * other[e431]) + (self[e125] * other[e412]) - (self[e321] * other[e321]),
        );
    }
}
impl DotProduct<CircleRotor> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: CircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e235] * other[e423]) + (self[e315] * other[e431]) + (self[e125] * other[e412]) - (self[e321] * other[e321]),
        );
    }
}
impl DotProduct<MultiVector> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e235] * other[e423]) + (self[e315] * other[e431]) + (self[e125] * other[e412]) - (self[e321] * other[e321]),
        );
    }
}
impl DotProduct<VersorEven> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: VersorEven) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e235] * other[e423]) + (self[e315] * other[e431]) + (self[e125] * other[e412]) - (self[e321] * other[e321]),
        );
    }
}
impl std::ops::Div<dot_product> for AntiFlector {
    type Output = dot_product_partial<AntiFlector>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiDipoleInversion> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: AntiDipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235]) + (other[e431] * self[e315]) + (other[e412] * self[e125]) + (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3])
                - (other[e321] * self[e321])
                - (other[e4] * self[e5]),
        );
    }
}
impl DotProduct<AntiFlatPoint> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: AntiFlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * self[e321] * -1.0);
    }
}
impl DotProduct<AntiFlector> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiFlector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) - (other[e321] * self[e321]));
    }
}
impl DotProduct<AntiPlane> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn dot_product(self, other: AntiPlane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]));
    }
}
impl DotProduct<Circle> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Circle) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e235] * other[e423]) + (self[e315] * other[e431]) + (self[e125] * other[e412]) - (self[e321] * other[e321]),
        );
    }
}
impl DotProduct<CircleRotor> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: CircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e235] * other[e423]) + (self[e315] * other[e431]) + (self[e125] * other[e412]) - (self[e321] * other[e321]),
        );
    }
}
impl DotProduct<DualNum> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: DualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e5] * other[e4] * -1.0);
    }
}
impl DotProduct<MultiVector> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e235] * other[e423]) + (self[e315] * other[e431]) + (self[e125] * other[e412]) + (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3])
                - (self[e321] * other[e321])
                - (self[e5] * other[e4]),
        );
    }
}
impl DotProduct<RoundPoint> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: RoundPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]) - (self[e5] * other[e4]));
    }
}
impl DotProduct<VersorEven> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: VersorEven) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e235] * other[e423]) + (self[e315] * other[e431]) + (self[e125] * other[e412]) + (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3])
                - (self[e321] * other[e321])
                - (self[e5] * other[e4]),
        );
    }
}
impl std::ops::Div<dot_product> for AntiLine {
    type Output = dot_product_partial<AntiLine>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiCircleRotor> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: AntiCircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(other[e41] * self[e15]) - (other[e42] * self[e25]) - (other[e43] * self[e35]) - (other[e23] * self[e23]) - (other[e31] * self[e31]) - (other[e12] * self[e12]),
        );
    }
}
impl DotProduct<AntiLine> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn dot_product(self, other: AntiLine) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -(other[e23] * self[e23]) - (other[e31] * self[e31]) - (other[e12] * self[e12]));
    }
}
impl DotProduct<AntiMotor> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn dot_product(self, other: AntiMotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -(self[e23] * other[e23]) - (self[e31] * other[e31]) - (self[e12] * other[e12]));
    }
}
impl DotProduct<Dipole> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: Dipole) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(self[e23] * other[e23]) - (self[e31] * other[e31]) - (self[e12] * other[e12]) - (self[e15] * other[e41]) - (self[e25] * other[e42]) - (self[e35] * other[e43]),
        );
    }
}
impl DotProduct<DipoleInversion> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: DipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(self[e23] * other[e23]) - (self[e31] * other[e31]) - (self[e12] * other[e12]) - (self[e15] * other[e41]) - (self[e25] * other[e42]) - (self[e35] * other[e43]),
        );
    }
}
impl DotProduct<MultiVector> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(self[e23] * other[e23]) - (self[e31] * other[e31]) - (self[e12] * other[e12]) - (self[e15] * other[e41]) - (self[e25] * other[e42]) - (self[e35] * other[e43]),
        );
    }
}
impl DotProduct<VersorOdd> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: VersorOdd) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(self[e23] * other[e23]) - (self[e31] * other[e31]) - (self[e12] * other[e12]) - (self[e15] * other[e41]) - (self[e25] * other[e42]) - (self[e35] * other[e43]),
        );
    }
}
impl std::ops::Div<dot_product> for AntiMotor {
    type Output = dot_product_partial<AntiMotor>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiCircleRotor> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn dot_product(self, other: AntiCircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[scalar] * self[scalar])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12]),
        );
    }
}
impl DotProduct<AntiDualNum> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn dot_product(self, other: AntiDualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e1234] * self[e3215]) + (other[scalar] * self[scalar]));
    }
}
impl DotProduct<AntiLine> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn dot_product(self, other: AntiLine) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -(other[e23] * self[e23]) - (other[e31] * self[e31]) - (other[e12] * self[e12]));
    }
}
impl DotProduct<AntiMotor> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiMotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[scalar] * self[scalar]) - (other[e23] * self[e23]) - (other[e31] * self[e31]) - (other[e12] * self[e12]),
        );
    }
}
impl DotProduct<Dipole> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: Dipole) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(self[e23] * other[e23]) - (self[e31] * other[e31]) - (self[e12] * other[e12]) - (self[e15] * other[e41]) - (self[e25] * other[e42]) - (self[e35] * other[e43]),
        );
    }
}
impl DotProduct<DipoleInversion> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn dot_product(self, other: DipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e3215] * other[e1234])
                - (self[e23] * other[e23])
                - (self[e31] * other[e31])
                - (self[e12] * other[e12])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43]),
        );
    }
}
impl DotProduct<MultiVector> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[scalar] * other[scalar]) + (self[e3215] * other[e1234])
                - (self[e23] * other[e23])
                - (self[e31] * other[e31])
                - (self[e12] * other[e12])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43]),
        );
    }
}
impl DotProduct<Scalar> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: Scalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar] * other[scalar]);
    }
}
impl DotProduct<Sphere> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: Sphere) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e3215] * other[e1234]);
    }
}
impl DotProduct<VersorOdd> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: VersorOdd) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[scalar] * other[scalar]) + (self[e3215] * other[e1234])
                - (self[e23] * other[e23])
                - (self[e31] * other[e31])
                - (self[e12] * other[e12])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43]),
        );
    }
}
impl std::ops::Div<dot_product> for AntiPlane {
    type Output = dot_product_partial<AntiPlane>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiDipoleInversion> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiDipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) - (other[e4] * self[e5]));
    }
}
impl DotProduct<AntiFlector> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn dot_product(self, other: AntiFlector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]));
    }
}
impl DotProduct<AntiPlane> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn dot_product(self, other: AntiPlane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]));
    }
}
impl DotProduct<DualNum> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: DualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e5] * other[e4] * -1.0);
    }
}
impl DotProduct<MultiVector> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]) - (self[e5] * other[e4]));
    }
}
impl DotProduct<RoundPoint> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: RoundPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]) - (self[e5] * other[e4]));
    }
}
impl DotProduct<VersorEven> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: VersorEven) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]) - (self[e5] * other[e4]));
    }
}
impl std::ops::Div<dot_product> for AntiScalar {
    type Output = dot_product_partial<AntiScalar>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiScalar> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: AntiScalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0);
    }
}
impl DotProduct<CircleRotor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: CircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e12345] * other[e12345] * -1.0);
    }
}
impl DotProduct<DualNum> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: DualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e12345] * other[e12345] * -1.0);
    }
}
impl DotProduct<Motor> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e12345] * other[e12345] * -1.0);
    }
}
impl DotProduct<MultiVector> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e12345] * other[e12345] * -1.0);
    }
}
impl DotProduct<VersorEven> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: VersorEven) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e12345] * other[e12345] * -1.0);
    }
}
impl std::ops::Div<dot_product> for Circle {
    type Output = dot_product_partial<Circle>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiDipoleInversion> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: AntiDipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                - (other[e321] * self[e321]),
        );
    }
}
impl DotProduct<AntiFlatPoint> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiFlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e235] * self[e423]) + (other[e315] * self[e431]) + (other[e125] * self[e412]) - (other[e321] * self[e321]),
        );
    }
}
impl DotProduct<AntiFlector> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiFlector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e235] * self[e423]) + (other[e315] * self[e431]) + (other[e125] * self[e412]) - (other[e321] * self[e321]),
        );
    }
}
impl DotProduct<Circle> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: Circle) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                - (other[e321] * self[e321]),
        );
    }
}
impl DotProduct<CircleRotor> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: CircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                + (self[e235] * other[e423])
                + (self[e315] * other[e431])
                + (self[e125] * other[e412])
                - (self[e321] * other[e321]),
        );
    }
}
impl DotProduct<Line> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: Line) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435]),
        );
    }
}
impl DotProduct<Motor> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435]),
        );
    }
}
impl DotProduct<MultiVector> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                + (self[e235] * other[e423])
                + (self[e315] * other[e431])
                + (self[e125] * other[e412])
                - (self[e321] * other[e321]),
        );
    }
}
impl DotProduct<VersorEven> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: VersorEven) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                + (self[e235] * other[e423])
                + (self[e315] * other[e431])
                + (self[e125] * other[e412])
                - (self[e321] * other[e321]),
        );
    }
}
impl std::ops::Div<dot_product> for CircleRotor {
    type Output = dot_product_partial<CircleRotor>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiDipoleInversion> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: AntiDipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                - (other[e321] * self[e321]),
        );
    }
}
impl DotProduct<AntiFlatPoint> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiFlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e235] * self[e423]) + (other[e315] * self[e431]) + (other[e125] * self[e412]) - (other[e321] * self[e321]),
        );
    }
}
impl DotProduct<AntiFlector> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiFlector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e235] * self[e423]) + (other[e315] * self[e431]) + (other[e125] * self[e412]) - (other[e321] * self[e321]),
        );
    }
}
impl DotProduct<AntiScalar> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: AntiScalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0);
    }
}
impl DotProduct<Circle> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: Circle) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                - (other[e321] * self[e321]),
        );
    }
}
impl DotProduct<CircleRotor> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn dot_product(self, other: CircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                - (other[e321] * self[e321])
                - (other[e12345] * self[e12345]),
        );
    }
}
impl DotProduct<DualNum> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: DualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e12345] * other[e12345] * -1.0);
    }
}
impl DotProduct<Line> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: Line) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435]),
        );
    }
}
impl DotProduct<Motor> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                - (self[e12345] * other[e12345]),
        );
    }
}
impl DotProduct<MultiVector> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                + (self[e235] * other[e423])
                + (self[e315] * other[e431])
                + (self[e125] * other[e412])
                - (self[e321] * other[e321])
                - (self[e12345] * other[e12345]),
        );
    }
}
impl DotProduct<VersorEven> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn dot_product(self, other: VersorEven) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                + (self[e235] * other[e423])
                + (self[e315] * other[e431])
                + (self[e125] * other[e412])
                - (self[e321] * other[e321])
                - (self[e12345] * other[e12345]),
        );
    }
}
impl std::ops::Div<dot_product> for Dipole {
    type Output = dot_product_partial<Dipole>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiCircleRotor> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: AntiCircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<AntiLine> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: AntiLine) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(other[e23] * self[e23]) - (other[e31] * self[e31]) - (other[e12] * self[e12]) - (other[e15] * self[e41]) - (other[e25] * self[e42]) - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<AntiMotor> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: AntiMotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(other[e23] * self[e23]) - (other[e31] * self[e31]) - (other[e12] * self[e12]) - (other[e15] * self[e41]) - (other[e25] * self[e42]) - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<Dipole> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: Dipole) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<DipoleInversion> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: DipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45])
                - (self[e41] * other[e15])
                - (self[e42] * other[e25])
                - (self[e43] * other[e35])
                - (self[e23] * other[e23])
                - (self[e31] * other[e31])
                - (self[e12] * other[e12])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43]),
        );
    }
}
impl DotProduct<FlatPoint> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: FlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45]) - (self[e41] * other[e15]) - (self[e42] * other[e25]) - (self[e43] * other[e35]),
        );
    }
}
impl DotProduct<Flector> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45]) - (self[e41] * other[e15]) - (self[e42] * other[e25]) - (self[e43] * other[e35]),
        );
    }
}
impl DotProduct<MultiVector> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45])
                - (self[e41] * other[e15])
                - (self[e42] * other[e25])
                - (self[e43] * other[e35])
                - (self[e23] * other[e23])
                - (self[e31] * other[e31])
                - (self[e12] * other[e12])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43]),
        );
    }
}
impl DotProduct<VersorOdd> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: VersorOdd) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45])
                - (self[e41] * other[e15])
                - (self[e42] * other[e25])
                - (self[e43] * other[e35])
                - (self[e23] * other[e23])
                - (self[e31] * other[e31])
                - (self[e12] * other[e12])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43]),
        );
    }
}
impl std::ops::Div<dot_product> for DipoleInversion {
    type Output = dot_product_partial<DipoleInversion>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiCircleRotor> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: AntiCircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<AntiDualNum> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: AntiDualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e1234] * self[e3215]);
    }
}
impl DotProduct<AntiLine> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: AntiLine) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(other[e23] * self[e23]) - (other[e31] * self[e31]) - (other[e12] * self[e12]) - (other[e15] * self[e41]) - (other[e25] * self[e42]) - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<AntiMotor> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn dot_product(self, other: AntiMotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e3215] * self[e1234])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<Dipole> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: Dipole) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<DipoleInversion> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn dot_product(self, other: DipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) + (other[e1234] * self[e3215]) + (other[e3215] * self[e1234])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43])
                - (other[e4235] * self[e4235])
                - (other[e4315] * self[e4315])
                - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<FlatPoint> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: FlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45]) - (self[e41] * other[e15]) - (self[e42] * other[e25]) - (self[e43] * other[e35]),
        );
    }
}
impl DotProduct<Flector> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45]) + (self[e1234] * other[e3215])
                - (self[e41] * other[e15])
                - (self[e42] * other[e25])
                - (self[e43] * other[e35])
                - (self[e4235] * other[e4235])
                - (self[e4315] * other[e4315])
                - (self[e4125] * other[e4125]),
        );
    }
}
impl DotProduct<MultiVector> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45]) + (self[e1234] * other[e3215]) + (self[e3215] * other[e1234])
                - (self[e41] * other[e15])
                - (self[e42] * other[e25])
                - (self[e43] * other[e35])
                - (self[e23] * other[e23])
                - (self[e31] * other[e31])
                - (self[e12] * other[e12])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43])
                - (self[e4235] * other[e4235])
                - (self[e4315] * other[e4315])
                - (self[e4125] * other[e4125]),
        );
    }
}
impl DotProduct<Plane> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Plane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e1234] * other[e3215]) - (self[e4235] * other[e4235]) - (self[e4315] * other[e4315]) - (self[e4125] * other[e4125]),
        );
    }
}
impl DotProduct<Sphere> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn dot_product(self, other: Sphere) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e1234] * other[e3215]) + (self[e3215] * other[e1234]) - (self[e4235] * other[e4235]) - (self[e4315] * other[e4315]) - (self[e4125] * other[e4125]),
        );
    }
}
impl DotProduct<VersorOdd> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn dot_product(self, other: VersorOdd) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45]) + (self[e1234] * other[e3215]) + (self[e3215] * other[e1234])
                - (self[e41] * other[e15])
                - (self[e42] * other[e25])
                - (self[e43] * other[e35])
                - (self[e23] * other[e23])
                - (self[e31] * other[e31])
                - (self[e12] * other[e12])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43])
                - (self[e4235] * other[e4235])
                - (self[e4315] * other[e4315])
                - (self[e4125] * other[e4125]),
        );
    }
}
impl std::ops::Div<dot_product> for DualNum {
    type Output = dot_product_partial<DualNum>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiDipoleInversion> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: AntiDipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e5] * self[e4] * -1.0);
    }
}
impl DotProduct<AntiFlector> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: AntiFlector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e5] * self[e4] * -1.0);
    }
}
impl DotProduct<AntiPlane> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: AntiPlane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e5] * self[e4] * -1.0);
    }
}
impl DotProduct<AntiScalar> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: AntiScalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0);
    }
}
impl DotProduct<CircleRotor> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: CircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0);
    }
}
impl DotProduct<DualNum> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: DualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0);
    }
}
impl DotProduct<Motor> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -(self[e4] * other[e5]) - (self[e12345] * other[e12345]));
    }
}
impl DotProduct<MultiVector> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -(self[e4] * other[e5]) - (self[e12345] * other[e12345]));
    }
}
impl DotProduct<RoundPoint> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: RoundPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e4] * other[e5] * -1.0);
    }
}
impl DotProduct<VersorEven> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn dot_product(self, other: VersorEven) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -(self[e4] * other[e5]) - (self[e12345] * other[e12345]));
    }
}
impl std::ops::Div<dot_product> for FlatPoint {
    type Output = dot_product_partial<FlatPoint>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiCircleRotor> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiCircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) - (other[e41] * self[e15]) - (other[e42] * self[e25]) - (other[e43] * self[e35]),
        );
    }
}
impl DotProduct<Dipole> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Dipole) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) - (other[e41] * self[e15]) - (other[e42] * self[e25]) - (other[e43] * self[e35]),
        );
    }
}
impl DotProduct<DipoleInversion> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: DipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) - (other[e41] * self[e15]) - (other[e42] * self[e25]) - (other[e43] * self[e35]),
        );
    }
}
impl DotProduct<FlatPoint> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: FlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e45] * self[e45]);
    }
}
impl DotProduct<Flector> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e45] * other[e45]);
    }
}
impl DotProduct<MultiVector> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45]) - (self[e15] * other[e41]) - (self[e25] * other[e42]) - (self[e35] * other[e43]),
        );
    }
}
impl DotProduct<VersorOdd> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: VersorOdd) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45]) - (self[e15] * other[e41]) - (self[e25] * other[e42]) - (self[e35] * other[e43]),
        );
    }
}
impl std::ops::Div<dot_product> for Flector {
    type Output = dot_product_partial<Flector>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiCircleRotor> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiCircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) - (other[e41] * self[e15]) - (other[e42] * self[e25]) - (other[e43] * self[e35]),
        );
    }
}
impl DotProduct<AntiDualNum> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: AntiDualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e1234] * self[e3215]);
    }
}
impl DotProduct<Dipole> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Dipole) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) - (other[e41] * self[e15]) - (other[e42] * self[e25]) - (other[e43] * self[e35]),
        );
    }
}
impl DotProduct<DipoleInversion> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: DipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) + (other[e1234] * self[e3215])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e4235] * self[e4235])
                - (other[e4315] * self[e4315])
                - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<FlatPoint> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: FlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e45] * self[e45]);
    }
}
impl DotProduct<Flector> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<MultiVector> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45]) + (self[e3215] * other[e1234])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43])
                - (self[e4235] * other[e4235])
                - (self[e4315] * other[e4315])
                - (self[e4125] * other[e4125]),
        );
    }
}
impl DotProduct<Plane> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn dot_product(self, other: Plane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -(self[e4235] * other[e4235]) - (self[e4315] * other[e4315]) - (self[e4125] * other[e4125]));
    }
}
impl DotProduct<Sphere> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Sphere) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e3215] * other[e1234]) - (self[e4235] * other[e4235]) - (self[e4315] * other[e4315]) - (self[e4125] * other[e4125]),
        );
    }
}
impl DotProduct<VersorOdd> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: VersorOdd) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e45] * other[e45]) + (self[e3215] * other[e1234])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43])
                - (self[e4235] * other[e4235])
                - (self[e4315] * other[e4315])
                - (self[e4125] * other[e4125]),
        );
    }
}
impl std::ops::Div<dot_product> for Line {
    type Output = dot_product_partial<Line>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiDipoleInversion> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: AntiDipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435]),
        );
    }
}
impl DotProduct<Circle> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: Circle) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435]),
        );
    }
}
impl DotProduct<CircleRotor> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: CircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435]),
        );
    }
}
impl DotProduct<Line> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn dot_product(self, other: Line) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e415] * self[e415]) + (other[e425] * self[e425]) + (other[e435] * self[e435]));
    }
}
impl DotProduct<Motor> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[e415] * other[e415]) + (self[e425] * other[e425]) + (self[e435] * other[e435]));
    }
}
impl DotProduct<MultiVector> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                + (self[e235] * other[e423])
                + (self[e315] * other[e431])
                + (self[e125] * other[e412]),
        );
    }
}
impl DotProduct<VersorEven> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: VersorEven) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                + (self[e235] * other[e423])
                + (self[e315] * other[e431])
                + (self[e125] * other[e412]),
        );
    }
}
impl std::ops::Div<dot_product> for Motor {
    type Output = dot_product_partial<Motor>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiDipoleInversion> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn dot_product(self, other: AntiDipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                - (other[e4] * self[e5]),
        );
    }
}
impl DotProduct<AntiScalar> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: AntiScalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0);
    }
}
impl DotProduct<Circle> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: Circle) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435]),
        );
    }
}
impl DotProduct<CircleRotor> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        7        0
    fn dot_product(self, other: CircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                - (other[e12345] * self[e12345]),
        );
    }
}
impl DotProduct<DualNum> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn dot_product(self, other: DualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -(other[e4] * self[e5]) - (other[e12345] * self[e12345]));
    }
}
impl DotProduct<Line> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn dot_product(self, other: Line) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e415] * self[e415]) + (other[e425] * self[e425]) + (other[e435] * self[e435]));
    }
}
impl DotProduct<Motor> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e415] * self[e415]) + (other[e425] * self[e425]) + (other[e435] * self[e435]) - (other[e12345] * self[e12345]),
        );
    }
}
impl DotProduct<MultiVector> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                + (self[e235] * other[e423])
                + (self[e315] * other[e431])
                + (self[e125] * other[e412])
                - (self[e12345] * other[e12345])
                - (self[e5] * other[e4]),
        );
    }
}
impl DotProduct<RoundPoint> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: RoundPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e5] * other[e4] * -1.0);
    }
}
impl DotProduct<VersorEven> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: VersorEven) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                + (self[e235] * other[e423])
                + (self[e315] * other[e431])
                + (self[e125] * other[e412])
                - (self[e12345] * other[e12345])
                - (self[e5] * other[e4]),
        );
    }
}
impl std::ops::Div<dot_product> for MultiVector {
    type Output = dot_product_partial<MultiVector>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiCircleRotor> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn dot_product(self, other: AntiCircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) + (other[scalar] * self[scalar])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<AntiDipoleInversion> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn dot_product(self, other: AntiDipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                + (other[e1] * self[e1])
                + (other[e2] * self[e2])
                + (other[e3] * self[e3])
                - (other[e321] * self[e321])
                - (other[e4] * self[e5])
                - (other[e5] * self[e4]),
        );
    }
}
impl DotProduct<AntiDualNum> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn dot_product(self, other: AntiDualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e1234] * self[e3215]) + (other[scalar] * self[scalar]));
    }
}
impl DotProduct<AntiFlatPoint> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiFlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e235] * self[e423]) + (other[e315] * self[e431]) + (other[e125] * self[e412]) - (other[e321] * self[e321]),
        );
    }
}
impl DotProduct<AntiFlector> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: AntiFlector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e235] * self[e423]) + (other[e315] * self[e431]) + (other[e125] * self[e412]) + (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3])
                - (other[e321] * self[e321])
                - (other[e5] * self[e4]),
        );
    }
}
impl DotProduct<AntiLine> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: AntiLine) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(other[e23] * self[e23]) - (other[e31] * self[e31]) - (other[e12] * self[e12]) - (other[e15] * self[e41]) - (other[e25] * self[e42]) - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<AntiMotor> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: AntiMotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[scalar] * self[scalar]) + (other[e3215] * self[e1234])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<AntiPlane> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiPlane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) - (other[e5] * self[e4]));
    }
}
impl DotProduct<AntiScalar> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: AntiScalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0);
    }
}
impl DotProduct<Circle> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: Circle) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                - (other[e321] * self[e321]),
        );
    }
}
impl DotProduct<CircleRotor> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn dot_product(self, other: CircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                - (other[e321] * self[e321])
                - (other[e12345] * self[e12345]),
        );
    }
}
impl DotProduct<Dipole> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: Dipole) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<DipoleInversion> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn dot_product(self, other: DipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) + (other[e1234] * self[e3215]) + (other[e3215] * self[e1234])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43])
                - (other[e4235] * self[e4235])
                - (other[e4315] * self[e4315])
                - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<DualNum> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn dot_product(self, other: DualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -(other[e4] * self[e5]) - (other[e12345] * self[e12345]));
    }
}
impl DotProduct<FlatPoint> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: FlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) - (other[e15] * self[e41]) - (other[e25] * self[e42]) - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<Flector> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) + (other[e3215] * self[e1234])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43])
                - (other[e4235] * self[e4235])
                - (other[e4315] * self[e4315])
                - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<Line> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: Line) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412]),
        );
    }
}
impl DotProduct<Motor> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                - (other[e12345] * self[e12345])
                - (other[e5] * self[e4]),
        );
    }
}
impl DotProduct<MultiVector> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       31       32        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[scalar] * self[scalar])
                + (other[e1] * self[e1])
                + (other[e2] * self[e2])
                + (other[e3] * self[e3])
                + (other[e45] * self[e45])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                + (other[e3215] * self[e1234])
                + (other[e1234] * self[e3215])
                - (other[e12345] * self[e12345])
                - (other[e4] * self[e5])
                - (other[e5] * self[e4])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e321] * self[e321])
                - (other[e4235] * self[e4235])
                - (other[e4315] * self[e4315])
                - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<Plane> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Plane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e1234] * other[e3215]) - (self[e4235] * other[e4235]) - (self[e4315] * other[e4315]) - (self[e4125] * other[e4125]),
        );
    }
}
impl DotProduct<RoundPoint> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn dot_product(self, other: RoundPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]) - (self[e4] * other[e5]) - (self[e5] * other[e4]),
        );
    }
}
impl DotProduct<Scalar> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: Scalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar] * other[scalar]);
    }
}
impl DotProduct<Sphere> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn dot_product(self, other: Sphere) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e3215] * other[e1234]) + (self[e1234] * other[e3215]) - (self[e4235] * other[e4235]) - (self[e4315] * other[e4315]) - (self[e4125] * other[e4125]),
        );
    }
}
impl DotProduct<VersorEven> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       16        0
    fn dot_product(self, other: VersorEven) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e1] * other[e1])
                + (self[e2] * other[e2])
                + (self[e3] * other[e3])
                + (self[e415] * other[e415])
                + (self[e425] * other[e425])
                + (self[e435] * other[e435])
                + (self[e423] * other[e235])
                + (self[e431] * other[e315])
                + (self[e412] * other[e125])
                + (self[e235] * other[e423])
                + (self[e315] * other[e431])
                + (self[e125] * other[e412])
                - (self[e12345] * other[e12345])
                - (self[e4] * other[e5])
                - (self[e5] * other[e4])
                - (self[e321] * other[e321]),
        );
    }
}
impl DotProduct<VersorOdd> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       16        0
    fn dot_product(self, other: VersorOdd) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[scalar] * other[scalar]) + (self[e45] * other[e45]) + (self[e3215] * other[e1234]) + (self[e1234] * other[e3215])
                - (self[e15] * other[e41])
                - (self[e25] * other[e42])
                - (self[e35] * other[e43])
                - (self[e41] * other[e15])
                - (self[e42] * other[e25])
                - (self[e43] * other[e35])
                - (self[e23] * other[e23])
                - (self[e31] * other[e31])
                - (self[e12] * other[e12])
                - (self[e4235] * other[e4235])
                - (self[e4315] * other[e4315])
                - (self[e4125] * other[e4125]),
        );
    }
}
impl std::ops::Div<dot_product> for Plane {
    type Output = dot_product_partial<Plane>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiDualNum> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: AntiDualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e1234] * self[e3215]);
    }
}
impl DotProduct<DipoleInversion> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: DipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e1234] * self[e3215]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<Flector> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -(other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]));
    }
}
impl DotProduct<MultiVector> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e1234] * self[e3215]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<Plane> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn dot_product(self, other: Plane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -(other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]));
    }
}
impl DotProduct<Sphere> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Sphere) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e3215] * other[e1234]) - (self[e4235] * other[e4235]) - (self[e4315] * other[e4315]) - (self[e4125] * other[e4125]),
        );
    }
}
impl DotProduct<VersorOdd> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: VersorOdd) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e3215] * other[e1234]) - (self[e4235] * other[e4235]) - (self[e4315] * other[e4315]) - (self[e4125] * other[e4125]),
        );
    }
}
impl std::ops::Div<dot_product> for RoundPoint {
    type Output = dot_product_partial<RoundPoint>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiDipoleInversion> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn dot_product(self, other: AntiDipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) - (other[e4] * self[e5]) - (other[e5] * self[e4]),
        );
    }
}
impl DotProduct<AntiFlector> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiFlector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) - (other[e5] * self[e4]));
    }
}
impl DotProduct<AntiPlane> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiPlane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) - (other[e5] * self[e4]));
    }
}
impl DotProduct<DualNum> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: DualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e4] * self[e5] * -1.0);
    }
}
impl DotProduct<Motor> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e5] * self[e4] * -1.0);
    }
}
impl DotProduct<MultiVector> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) - (other[e4] * self[e5]) - (other[e5] * self[e4]),
        );
    }
}
impl DotProduct<RoundPoint> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn dot_product(self, other: RoundPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) - (other[e4] * self[e5]) - (other[e5] * self[e4]),
        );
    }
}
impl DotProduct<VersorEven> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn dot_product(self, other: VersorEven) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]) - (self[e4] * other[e5]) - (self[e5] * other[e4]),
        );
    }
}
impl std::ops::Div<dot_product> for Scalar {
    type Output = dot_product_partial<Scalar>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiCircleRotor> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: AntiCircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]);
    }
}
impl DotProduct<AntiDualNum> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: AntiDualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]);
    }
}
impl DotProduct<AntiMotor> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: AntiMotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]);
    }
}
impl DotProduct<MultiVector> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]);
    }
}
impl DotProduct<Scalar> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: Scalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]);
    }
}
impl DotProduct<VersorOdd> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: VersorOdd) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[scalar] * other[scalar]);
    }
}
impl std::ops::Div<dot_product> for Sphere {
    type Output = dot_product_partial<Sphere>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiDualNum> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: AntiDualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e1234] * self[e3215]);
    }
}
impl DotProduct<AntiMotor> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: AntiMotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e3215] * self[e1234]);
    }
}
impl DotProduct<DipoleInversion> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn dot_product(self, other: DipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e1234] * self[e3215]) + (other[e3215] * self[e1234]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<Flector> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e3215] * self[e1234]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<MultiVector> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e3215] * self[e1234]) + (other[e1234] * self[e3215]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<Plane> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Plane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e3215] * self[e1234]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<Sphere> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn dot_product(self, other: Sphere) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e3215] * self[e1234]) + (other[e1234] * self[e3215]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<VersorOdd> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn dot_product(self, other: VersorOdd) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (self[e3215] * other[e1234]) + (self[e1234] * other[e3215]) - (self[e4235] * other[e4235]) - (self[e4315] * other[e4315]) - (self[e4125] * other[e4125]),
        );
    }
}
impl std::ops::Div<dot_product> for VersorEven {
    type Output = dot_product_partial<VersorEven>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiDipoleInversion> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn dot_product(self, other: AntiDipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                + (other[e1] * self[e1])
                + (other[e2] * self[e2])
                + (other[e3] * self[e3])
                - (other[e321] * self[e321])
                - (other[e4] * self[e5])
                - (other[e5] * self[e4]),
        );
    }
}
impl DotProduct<AntiFlatPoint> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiFlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e235] * self[e423]) + (other[e315] * self[e431]) + (other[e125] * self[e412]) - (other[e321] * self[e321]),
        );
    }
}
impl DotProduct<AntiFlector> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: AntiFlector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e235] * self[e423]) + (other[e315] * self[e431]) + (other[e125] * self[e412]) + (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3])
                - (other[e321] * self[e321])
                - (other[e5] * self[e4]),
        );
    }
}
impl DotProduct<AntiPlane> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: AntiPlane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) - (other[e5] * self[e4]));
    }
}
impl DotProduct<AntiScalar> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn dot_product(self, other: AntiScalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0);
    }
}
impl DotProduct<Circle> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: Circle) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                - (other[e321] * self[e321]),
        );
    }
}
impl DotProduct<CircleRotor> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn dot_product(self, other: CircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                - (other[e321] * self[e321])
                - (other[e12345] * self[e12345]),
        );
    }
}
impl DotProduct<DualNum> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn dot_product(self, other: DualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -(other[e4] * self[e5]) - (other[e12345] * self[e12345]));
    }
}
impl DotProduct<Line> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: Line) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412]),
        );
    }
}
impl DotProduct<Motor> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                - (other[e12345] * self[e12345])
                - (other[e5] * self[e4]),
        );
    }
}
impl DotProduct<MultiVector> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       16        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e1] * self[e1])
                + (other[e2] * self[e2])
                + (other[e3] * self[e3])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                - (other[e12345] * self[e12345])
                - (other[e4] * self[e5])
                - (other[e5] * self[e4])
                - (other[e321] * self[e321]),
        );
    }
}
impl DotProduct<RoundPoint> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn dot_product(self, other: RoundPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) - (other[e4] * self[e5]) - (other[e5] * self[e4]),
        );
    }
}
impl DotProduct<VersorEven> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       16        0
    fn dot_product(self, other: VersorEven) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e423] * self[e235])
                + (other[e431] * self[e315])
                + (other[e412] * self[e125])
                + (other[e415] * self[e415])
                + (other[e425] * self[e425])
                + (other[e435] * self[e435])
                + (other[e235] * self[e423])
                + (other[e315] * self[e431])
                + (other[e125] * self[e412])
                + (other[e1] * self[e1])
                + (other[e2] * self[e2])
                + (other[e3] * self[e3])
                - (other[e12345] * self[e12345])
                - (other[e321] * self[e321])
                - (other[e5] * self[e4])
                - (other[e4] * self[e5]),
        );
    }
}
impl std::ops::Div<dot_product> for VersorOdd {
    type Output = dot_product_partial<VersorOdd>;
    fn div(self, _rhs: dot_product) -> Self::Output {
        dot_product_partial(self)
    }
}
impl DotProduct<AntiCircleRotor> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn dot_product(self, other: AntiCircleRotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) + (other[scalar] * self[scalar])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<AntiDualNum> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        2        0
    fn dot_product(self, other: AntiDualNum) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e1234] * self[e3215]) + (other[scalar] * self[scalar]));
    }
}
impl DotProduct<AntiLine> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn dot_product(self, other: AntiLine) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            -(other[e23] * self[e23]) - (other[e31] * self[e31]) - (other[e12] * self[e12]) - (other[e15] * self[e41]) - (other[e25] * self[e42]) - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<AntiMotor> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: AntiMotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[scalar] * self[scalar]) + (other[e3215] * self[e1234])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<Dipole> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn dot_product(self, other: Dipole) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<DipoleInversion> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn dot_product(self, other: DipoleInversion) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) + (other[e1234] * self[e3215]) + (other[e3215] * self[e1234])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43])
                - (other[e4235] * self[e4235])
                - (other[e4315] * self[e4315])
                - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<FlatPoint> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: FlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) - (other[e15] * self[e41]) - (other[e25] * self[e42]) - (other[e35] * self[e43]),
        );
    }
}
impl DotProduct<Flector> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7        8        0
    fn dot_product(self, other: Flector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e45] * self[e45]) + (other[e3215] * self[e1234])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43])
                - (other[e4235] * self[e4235])
                - (other[e4315] * self[e4315])
                - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<MultiVector> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       16        0
    fn dot_product(self, other: MultiVector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[scalar] * self[scalar]) + (other[e45] * self[e45]) + (other[e3215] * self[e1234]) + (other[e1234] * self[e3215])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e4235] * self[e4235])
                - (other[e4315] * self[e4315])
                - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<Plane> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn dot_product(self, other: Plane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e3215] * self[e1234]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<Scalar> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn dot_product(self, other: Scalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]);
    }
}
impl DotProduct<Sphere> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn dot_product(self, other: Sphere) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e3215] * self[e1234]) + (other[e1234] * self[e3215]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        );
    }
}
impl DotProduct<VersorOdd> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       16        0
    fn dot_product(self, other: VersorOdd) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[scalar] * self[scalar]) + (other[e45] * self[e45]) + (other[e1234] * self[e3215]) + (other[e3215] * self[e1234])
                - (other[e41] * self[e15])
                - (other[e42] * self[e25])
                - (other[e43] * self[e35])
                - (other[e23] * self[e23])
                - (other[e31] * self[e31])
                - (other[e12] * self[e12])
                - (other[e15] * self[e41])
                - (other[e25] * self[e42])
                - (other[e35] * self[e43])
                - (other[e4235] * self[e4235])
                - (other[e4315] * self[e4315])
                - (other[e4125] * self[e4125]),
        );
    }
}
