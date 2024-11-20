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
        return Scalar::from_groups(/* scalar */ other.group0()[3] * self.group0()[3] * -1.0);
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
        return Scalar::from_groups(
            // scalar
            (other.group1()[0] * self.group1()[0]) + (other.group1()[1] * self.group1()[1]) + (other.group1()[2] * self.group1()[2]) - (other.group0()[3] * self.group0()[3]),
        );
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
        return Scalar::from_groups(
            // scalar
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
        );
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
        return Scalar::from_groups(
            // scalar
            (other.group0()[3] * self.group0()[3]) - (other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
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
        return Scalar::from_groups(
            // scalar
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]),
        );
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
        return Scalar::from_groups(
            // scalar
            (other.group1()[3] * self.group1()[3])
                - (other.group0()[0] * self.group2()[0])
                - (other.group0()[1] * self.group2()[1])
                - (other.group0()[2] * self.group2()[2])
                - (other.group2()[0] * self.group0()[0])
                - (other.group2()[1] * self.group0()[1])
                - (other.group2()[2] * self.group0()[2])
                - (other.group1()[0] * self.group1()[0])
                - (other.group1()[1] * self.group1()[1])
                - (other.group1()[2] * self.group1()[2]),
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
        return Scalar::from_groups(
            // scalar
            (other.group1()[3] * self.group1()[3]) + (other.group2()[3] * self.group3()[3]) + (other.group3()[3] * self.group2()[3])
                - (other.group0()[0] * self.group2()[0])
                - (other.group0()[1] * self.group2()[1])
                - (other.group0()[2] * self.group2()[2])
                - (self.group0()[0] * other.group2()[0])
                - (self.group0()[1] * other.group2()[1])
                - (self.group0()[2] * other.group2()[2])
                - (other.group1()[0] * self.group1()[0])
                - (other.group1()[1] * self.group1()[1])
                - (other.group1()[2] * self.group1()[2])
                - (other.group3()[0] * self.group3()[0])
                - (other.group3()[1] * self.group3()[1])
                - (other.group3()[2] * self.group3()[2]),
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
        return Scalar::from_groups(
            // scalar
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]),
        );
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
        return Scalar::from_groups(
            // scalar
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]) - (other.group0()[3] * self.group0()[3]),
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
            (other.group0()[0] * self.group0()[0])
                + (other.group7()[0] * self.group8()[0])
                + (other.group7()[1] * self.group8()[1])
                + (other.group7()[2] * self.group8()[2])
                + (other.group8()[0] * self.group7()[0])
                + (other.group8()[1] * self.group7()[1])
                + (other.group8()[2] * self.group7()[2])
                + (other.group1()[0] * self.group1()[0])
                + (other.group1()[1] * self.group1()[1])
                + (other.group1()[2] * self.group1()[2])
                + (other.group3()[3] * self.group3()[3])
                + (other.group6()[0] * self.group6()[0])
                + (other.group6()[1] * self.group6()[1])
                + (other.group6()[2] * self.group6()[2])
                + (other.group9()[3] * self[e45])
                + (self.group9()[3] * other[e45])
                - (other.group0()[1] * self.group0()[1])
                - (other.group4()[0] * self.group3()[0])
                - (other.group4()[1] * self.group3()[1])
                - (other.group4()[2] * self.group3()[2])
                - (other.group5()[0] * self.group5()[0])
                - (other.group5()[1] * self.group5()[1])
                - (other.group5()[2] * self.group5()[2])
                - (self.group4()[0] * other.group3()[0])
                - (self.group4()[1] * other.group3()[1])
                - (self.group4()[2] * other.group3()[2])
                - (other.group1()[3] * self[e1])
                - (other.group6()[3] * self.group6()[3])
                - (other.group9()[0] * self.group9()[0])
                - (other.group9()[1] * self.group9()[1])
                - (other.group9()[2] * self.group9()[2])
                - (self.group1()[3] * other[e1]),
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
            (other.group0()[3] * self[e4315]) + (self.group0()[3] * other[e4315])
                - (other.group0()[0] * self.group0()[0])
                - (other.group0()[1] * self.group0()[1])
                - (other.group0()[2] * self.group0()[2]),
        );
    }
}
