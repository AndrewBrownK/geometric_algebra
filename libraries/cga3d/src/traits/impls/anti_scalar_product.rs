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
        return AntiScalar::from_groups(
            // e12345
            (other.group0()[0] * self.group2()[0])
                + (other.group0()[1] * self.group2()[1])
                + (other.group0()[2] * self.group2()[2])
                + (self.group0()[0] * other.group2()[0])
                + (self.group0()[1] * other.group2()[1])
                + (self.group0()[2] * other.group2()[2])
                + (other.group1()[0] * self.group1()[0])
                + (other.group1()[1] * self.group1()[1])
                + (other.group1()[2] * self.group1()[2])
                - (other.group1()[3] * self.group1()[3])
                - (other.group2()[3] * self.group2()[3]),
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
        return AntiScalar::from_groups(
            // e12345
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
        return AntiScalar::from_groups(/* e12345 */ other.group0()[1] * self.group0()[1] * -1.0);
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
        return AntiScalar::from_groups(/* e12345 */ other.group0()[3] * self.group0()[3]);
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
        return AntiScalar::from_groups(
            // e12345
            (other.group0()[3] * self.group0()[3]) - (other.group1()[0] * self.group1()[0]) - (other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2]),
        );
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
        return AntiScalar::from_groups(
            // e12345
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]),
        );
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
        return AntiScalar::from_groups(
            // e12345
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]) - (other.group0()[3] * self.group0()[3]),
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
        return AntiScalar::from_groups(
            // e12345
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
        );
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
        return AntiScalar::from_groups(
            // e12345
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
        return AntiScalar::from_groups(
            // e12345
            (other.group1()[3] * self.group1()[3]) + (other.group2()[3] * self.group2()[3])
                - (other.group0()[0] * self.group2()[0])
                - (other.group0()[1] * self.group2()[1])
                - (other.group0()[2] * self.group2()[2])
                - (self.group0()[0] * other.group2()[0])
                - (self.group0()[1] * other.group2()[1])
                - (self.group0()[2] * other.group2()[2])
                - (other.group1()[0] * self.group1()[0])
                - (other.group1()[1] * self.group1()[1])
                - (other.group1()[2] * self.group1()[2]),
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
        return AntiScalar::from_groups(
            // e12345
            (other.group0()[0] * self.group2()[0])
                + (other.group0()[1] * self.group2()[1])
                + (other.group0()[2] * self.group2()[2])
                + (other.group2()[0] * self.group0()[0])
                + (other.group2()[1] * self.group0()[1])
                + (other.group2()[2] * self.group0()[2])
                + (other.group1()[0] * self.group1()[0])
                + (other.group1()[1] * self.group1()[1])
                + (other.group1()[2] * self.group1()[2])
                - (other.group1()[3] * self.group1()[3]),
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
        return AntiScalar::from_groups(
            // e12345
            (other.group0()[0] * self.group2()[0])
                + (other.group0()[1] * self.group2()[1])
                + (other.group0()[2] * self.group2()[2])
                + (self.group0()[0] * other.group2()[0])
                + (self.group0()[1] * other.group2()[1])
                + (self.group0()[2] * other.group2()[2])
                + (other.group1()[0] * self.group1()[0])
                + (other.group1()[1] * self.group1()[1])
                + (other.group1()[2] * self.group1()[2])
                + (other.group3()[0] * self.group3()[0])
                + (other.group3()[1] * self.group3()[1])
                + (other.group3()[2] * self.group3()[2])
                - (other.group1()[3] * self.group1()[3])
                - (other.group2()[3] * self.group3()[3])
                - (other.group3()[3] * self.group2()[3]),
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
        return AntiScalar::from_groups(/* e12345 */ other.group0()[1] * self.group0()[1]);
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
        return AntiScalar::from_groups(/* e12345 */ other.group0()[3] * self.group0()[3] * -1.0);
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
        return AntiScalar::from_groups(
            // e12345
            (other.group1()[0] * self.group1()[0]) + (other.group1()[1] * self.group1()[1]) + (other.group1()[2] * self.group1()[2]) - (other.group0()[3] * self.group0()[3]),
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
        return AntiScalar::from_groups(
            // e12345
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
        );
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
        return AntiScalar::from_groups(
            // e12345
            (other.group0()[3] * self.group0()[3]) - (other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
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
            (other.group0()[1] * self.group0()[1])
                + (other.group4()[0] * self.group3()[0])
                + (other.group4()[1] * self.group3()[1])
                + (other.group4()[2] * self.group3()[2])
                + (other.group5()[0] * self.group5()[0])
                + (other.group5()[1] * self.group5()[1])
                + (other.group5()[2] * self.group5()[2])
                + (self.group4()[0] * other.group3()[0])
                + (self.group4()[1] * other.group3()[1])
                + (self.group4()[2] * other.group3()[2])
                + (other.group1()[3] * self[e1])
                + (other.group6()[3] * self.group6()[3])
                + (other.group9()[0] * self.group9()[0])
                + (other.group9()[1] * self.group9()[1])
                + (other.group9()[2] * self.group9()[2])
                + (self.group1()[3] * other[e1])
                - (other.group0()[0] * self.group0()[0])
                - (other.group7()[0] * self.group8()[0])
                - (other.group7()[1] * self.group8()[1])
                - (other.group7()[2] * self.group8()[2])
                - (other.group8()[0] * self.group7()[0])
                - (other.group8()[1] * self.group7()[1])
                - (other.group8()[2] * self.group7()[2])
                - (other.group1()[0] * self.group1()[0])
                - (other.group1()[1] * self.group1()[1])
                - (other.group1()[2] * self.group1()[2])
                - (other.group3()[3] * self.group3()[3])
                - (other.group6()[0] * self.group6()[0])
                - (other.group6()[1] * self.group6()[1])
                - (other.group6()[2] * self.group6()[2])
                - (other.group9()[3] * self[e45])
                - (self.group9()[3] * other[e45]),
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
        return AntiScalar::from_groups(
            // e12345
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]),
        );
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
            (other.group0()[3] * self[e2]) + (self.group0()[3] * other[e2])
                - (other.group0()[0] * self.group0()[0])
                - (other.group0()[1] * self.group0()[1])
                - (other.group0()[2] * self.group0()[2]),
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
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2])
                - (other.group0()[3] * self[e4315])
                - (self.group0()[3] * other[e4315]),
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
        return AntiScalar::from_groups(
            // e12345
            (other.group0()[3] * self.group0()[3]) + (other.group1()[3] * self.group1()[3]) + (other.group2()[3] * self.group3()[3]) + (other.group3()[3] * self.group2()[3])
                - (other.group0()[0] * self.group2()[0])
                - (other.group0()[1] * self.group2()[1])
                - (other.group0()[2] * self.group2()[2])
                - (other.group1()[0] * self.group1()[0])
                - (other.group1()[1] * self.group1()[1])
                - (other.group1()[2] * self.group1()[2])
                - (other.group2()[0] * self.group0()[0])
                - (other.group2()[1] * self.group0()[1])
                - (other.group2()[2] * self.group0()[2])
                - (other.group3()[0] * self.group3()[0])
                - (other.group3()[1] * self.group3()[1])
                - (other.group3()[2] * self.group3()[2]),
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
        return AntiScalar::from_groups(
            // e12345
            (other.group0()[0] * self.group2()[0])
                + (other.group0()[1] * self.group2()[1])
                + (other.group0()[2] * self.group2()[2])
                + (other.group1()[0] * self.group1()[0])
                + (other.group1()[1] * self.group1()[1])
                + (other.group1()[2] * self.group1()[2])
                + (other.group2()[0] * self.group0()[0])
                + (other.group2()[1] * self.group0()[1])
                + (other.group2()[2] * self.group0()[2])
                + (other.group3()[0] * self.group3()[0])
                + (other.group3()[1] * self.group3()[1])
                + (other.group3()[2] * self.group3()[2])
                - (other.group0()[3] * self.group0()[3])
                - (other.group1()[3] * self.group1()[3])
                - (other.group2()[3] * self.group3()[3])
                - (other.group3()[3] * self.group2()[3]),
        );
    }
}
