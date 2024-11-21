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
//  Minimum:         0       1       0
//   Median:         3       4       0
//  Average:         6       7       0
//  Maximum:        31      32       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         3       4       0
//  Average:         6       7       0
//  Maximum:        31      32       0
impl std::ops::Div<anti_scalar_product> for AntiCircleRotor {
    type Output = anti_scalar_product_partial<AntiCircleRotor>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<AntiCircleRotor> for AntiCircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn anti_scalar_product(self, other: AntiCircleRotor) -> AntiScalar {
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
impl std::ops::Div<anti_scalar_product> for AntiDipoleInversion {
    type Output = anti_scalar_product_partial<AntiDipoleInversion>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<AntiDipoleInversion> for AntiDipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn anti_scalar_product(self, other: AntiDipoleInversion) -> AntiScalar {
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
impl std::ops::Div<anti_scalar_product> for AntiDualNum {
    type Output = anti_scalar_product_partial<AntiDualNum>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<AntiDualNum> for AntiDualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_scalar_product(self, other: AntiDualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[scalar] * self[scalar] * -1.0);
    }
}
impl std::ops::Div<anti_scalar_product> for AntiFlatPoint {
    type Output = anti_scalar_product_partial<AntiFlatPoint>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<AntiFlatPoint> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_scalar_product(self, other: AntiFlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e321] * self[e321]);
    }
}
impl std::ops::Div<anti_scalar_product> for AntiFlector {
    type Output = anti_scalar_product_partial<AntiFlector>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<AntiFlector> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_scalar_product(self, other: AntiFlector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e321] * self[e321]) - (other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]));
    }
}
impl std::ops::Div<anti_scalar_product> for AntiLine {
    type Output = anti_scalar_product_partial<AntiLine>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<AntiLine> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_scalar_product(self, other: AntiLine) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]));
    }
}
impl std::ops::Div<anti_scalar_product> for AntiMotor {
    type Output = anti_scalar_product_partial<AntiMotor>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<AntiMotor> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_scalar_product(self, other: AntiMotor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e23] * self[e23]) + (other[e31] * self[e31]) + (other[e12] * self[e12]) - (other[scalar] * self[scalar]),
        );
    }
}
impl std::ops::Div<anti_scalar_product> for AntiPlane {
    type Output = anti_scalar_product_partial<AntiPlane>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<AntiPlane> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_scalar_product(self, other: AntiPlane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ -(other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]));
    }
}
impl std::ops::Div<anti_scalar_product> for AntiScalar {
    type Output = anti_scalar_product_partial<AntiScalar>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<AntiScalar> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_scalar_product(self, other: AntiScalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345]);
    }
}
impl std::ops::Div<anti_scalar_product> for Circle {
    type Output = anti_scalar_product_partial<Circle>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<Circle> for Circle {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_scalar_product(self, other: Circle) -> AntiScalar {
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
impl std::ops::Div<anti_scalar_product> for CircleRotor {
    type Output = anti_scalar_product_partial<CircleRotor>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<CircleRotor> for CircleRotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       11        0
    fn anti_scalar_product(self, other: CircleRotor) -> AntiScalar {
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
impl std::ops::Div<anti_scalar_product> for Dipole {
    type Output = anti_scalar_product_partial<Dipole>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<Dipole> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_scalar_product(self, other: Dipole) -> AntiScalar {
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
impl std::ops::Div<anti_scalar_product> for DipoleInversion {
    type Output = anti_scalar_product_partial<DipoleInversion>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<DipoleInversion> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn anti_scalar_product(self, other: DipoleInversion) -> AntiScalar {
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
impl std::ops::Div<anti_scalar_product> for DualNum {
    type Output = anti_scalar_product_partial<DualNum>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<DualNum> for DualNum {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_scalar_product(self, other: DualNum) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e12345] * self[e12345]);
    }
}
impl std::ops::Div<anti_scalar_product> for FlatPoint {
    type Output = anti_scalar_product_partial<FlatPoint>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<FlatPoint> for FlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_scalar_product(self, other: FlatPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e45] * self[e45] * -1.0);
    }
}
impl std::ops::Div<anti_scalar_product> for Flector {
    type Output = anti_scalar_product_partial<Flector>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<Flector> for Flector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_scalar_product(self, other: Flector) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]) - (other[e45] * self[e45]),
        );
    }
}
impl std::ops::Div<anti_scalar_product> for Line {
    type Output = anti_scalar_product_partial<Line>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<Line> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_scalar_product(self, other: Line) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ -(other[e415] * self[e415]) - (other[e425] * self[e425]) - (other[e435] * self[e435]));
    }
}
impl std::ops::Div<anti_scalar_product> for Motor {
    type Output = anti_scalar_product_partial<Motor>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<Motor> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_scalar_product(self, other: Motor) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e12345] * self[e12345]) - (other[e415] * self[e415]) - (other[e425] * self[e425]) - (other[e435] * self[e435]),
        );
    }
}
impl std::ops::Div<anti_scalar_product> for MultiVector {
    type Output = anti_scalar_product_partial<MultiVector>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<MultiVector> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       31       32        0
    fn anti_scalar_product(self, other: MultiVector) -> AntiScalar {
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
impl std::ops::Div<anti_scalar_product> for Plane {
    type Output = anti_scalar_product_partial<Plane>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<Plane> for Plane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn anti_scalar_product(self, other: Plane) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]));
    }
}
impl std::ops::Div<anti_scalar_product> for RoundPoint {
    type Output = anti_scalar_product_partial<RoundPoint>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<RoundPoint> for RoundPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_scalar_product(self, other: RoundPoint) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4] * self[e5]) + (other[e5] * self[e4]) - (other[e1] * self[e1]) - (other[e2] * self[e2]) - (other[e3] * self[e3]),
        );
    }
}
impl std::ops::Div<anti_scalar_product> for Scalar {
    type Output = anti_scalar_product_partial<Scalar>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<Scalar> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_scalar_product(self, other: Scalar) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[scalar] * self[scalar] * -1.0);
    }
}
impl std::ops::Div<anti_scalar_product> for Sphere {
    type Output = anti_scalar_product_partial<Sphere>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<Sphere> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_scalar_product(self, other: Sphere) -> AntiScalar {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other[e4235] * self[e4235]) + (other[e4315] * self[e4315]) + (other[e4125] * self[e4125]) - (other[e3215] * self[e1234]) - (other[e1234] * self[e3215]),
        );
    }
}
impl std::ops::Div<anti_scalar_product> for VersorEven {
    type Output = anti_scalar_product_partial<VersorEven>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<VersorEven> for VersorEven {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       16        0
    fn anti_scalar_product(self, other: VersorEven) -> AntiScalar {
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
impl std::ops::Div<anti_scalar_product> for VersorOdd {
    type Output = anti_scalar_product_partial<VersorOdd>;
    fn div(self, _rhs: anti_scalar_product) -> Self::Output {
        anti_scalar_product_partial(self)
    }
}
impl AntiScalarProduct<VersorOdd> for VersorOdd {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       16        0
    fn anti_scalar_product(self, other: VersorOdd) -> AntiScalar {
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
