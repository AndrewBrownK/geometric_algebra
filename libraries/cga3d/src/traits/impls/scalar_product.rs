// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 13
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         3       4       0
//  Average:         5       6       0
//  Maximum:        31      32       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         3       4       0
//  Average:         5       6       0
//  Maximum:        31      32       0
impl std::ops::Div<scalar_product> for AntiFlatPoint {
    type Output = scalar_product_partial<AntiFlatPoint>;
    fn div(self, _rhs: scalar_product) -> Self::Output {
        scalar_product_partial(self)
    }
}
impl ScalarProduct<AntiFlatPoint> for AntiFlatPoint {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn scalar_product(self, other: AntiFlatPoint) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * self[e321] * -1.0);
    }
}
impl std::ops::Div<scalar_product> for AntiFlector {
    type Output = scalar_product_partial<AntiFlector>;
    fn div(self, _rhs: scalar_product) -> Self::Output {
        scalar_product_partial(self)
    }
}
impl ScalarProduct<AntiFlector> for AntiFlector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn scalar_product(self, other: AntiFlector) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]) - (other[e321] * self[e321]));
    }
}
impl std::ops::Div<scalar_product> for AntiLine {
    type Output = scalar_product_partial<AntiLine>;
    fn div(self, _rhs: scalar_product) -> Self::Output {
        scalar_product_partial(self)
    }
}
impl ScalarProduct<AntiLine> for AntiLine {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn scalar_product(self, other: AntiLine) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ -(other[e23] * self[e23]) - (other[e31] * self[e31]) - (other[e12] * self[e12]));
    }
}
impl std::ops::Div<scalar_product> for AntiMotor {
    type Output = scalar_product_partial<AntiMotor>;
    fn div(self, _rhs: scalar_product) -> Self::Output {
        scalar_product_partial(self)
    }
}
impl ScalarProduct<AntiMotor> for AntiMotor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn scalar_product(self, other: AntiMotor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[scalar] * self[scalar]) - (other[e23] * self[e23]) - (other[e31] * self[e31]) - (other[e12] * self[e12]),
        );
    }
}
impl std::ops::Div<scalar_product> for AntiPlane {
    type Output = scalar_product_partial<AntiPlane>;
    fn div(self, _rhs: scalar_product) -> Self::Output {
        scalar_product_partial(self)
    }
}
impl ScalarProduct<AntiPlane> for AntiPlane {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn scalar_product(self, other: AntiPlane) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]));
    }
}
impl std::ops::Div<scalar_product> for AntiScalar {
    type Output = scalar_product_partial<AntiScalar>;
    fn div(self, _rhs: scalar_product) -> Self::Output {
        scalar_product_partial(self)
    }
}
impl ScalarProduct<AntiScalar> for AntiScalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn scalar_product(self, other: AntiScalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e12345] * self[e12345] * -1.0);
    }
}
impl std::ops::Div<scalar_product> for Dipole {
    type Output = scalar_product_partial<Dipole>;
    fn div(self, _rhs: scalar_product) -> Self::Output {
        scalar_product_partial(self)
    }
}
impl ScalarProduct<Dipole> for Dipole {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn scalar_product(self, other: Dipole) -> Scalar {
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
impl std::ops::Div<scalar_product> for DipoleInversion {
    type Output = scalar_product_partial<DipoleInversion>;
    fn div(self, _rhs: scalar_product) -> Self::Output {
        scalar_product_partial(self)
    }
}
impl ScalarProduct<DipoleInversion> for DipoleInversion {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       14       15        0
    fn scalar_product(self, other: DipoleInversion) -> Scalar {
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
impl std::ops::Div<scalar_product> for Line {
    type Output = scalar_product_partial<Line>;
    fn div(self, _rhs: scalar_product) -> Self::Output {
        scalar_product_partial(self)
    }
}
impl ScalarProduct<Line> for Line {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn scalar_product(self, other: Line) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (other[e415] * self[e415]) + (other[e425] * self[e425]) + (other[e435] * self[e435]));
    }
}
impl std::ops::Div<scalar_product> for Motor {
    type Output = scalar_product_partial<Motor>;
    fn div(self, _rhs: scalar_product) -> Self::Output {
        scalar_product_partial(self)
    }
}
impl ScalarProduct<Motor> for Motor {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn scalar_product(self, other: Motor) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e415] * self[e415]) + (other[e425] * self[e425]) + (other[e435] * self[e435]) - (other[e12345] * self[e12345]),
        );
    }
}
impl std::ops::Div<scalar_product> for MultiVector {
    type Output = scalar_product_partial<MultiVector>;
    fn div(self, _rhs: scalar_product) -> Self::Output {
        scalar_product_partial(self)
    }
}
impl ScalarProduct<MultiVector> for MultiVector {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       31       32        0
    fn scalar_product(self, other: MultiVector) -> Scalar {
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
impl std::ops::Div<scalar_product> for Scalar {
    type Output = scalar_product_partial<Scalar>;
    fn div(self, _rhs: scalar_product) -> Self::Output {
        scalar_product_partial(self)
    }
}
impl ScalarProduct<Scalar> for Scalar {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn scalar_product(self, other: Scalar) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]);
    }
}
impl std::ops::Div<scalar_product> for Sphere {
    type Output = scalar_product_partial<Sphere>;
    fn div(self, _rhs: scalar_product) -> Self::Output {
        scalar_product_partial(self)
    }
}
impl ScalarProduct<Sphere> for Sphere {
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn scalar_product(self, other: Sphere) -> Scalar {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            (other[e3215] * self[e1234]) + (other[e1234] * self[e3215]) - (other[e4235] * self[e4235]) - (other[e4315] * self[e4315]) - (other[e4125] * self[e4125]),
        );
    }
}
