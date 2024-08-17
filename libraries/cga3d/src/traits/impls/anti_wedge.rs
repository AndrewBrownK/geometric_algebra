// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 253
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         5      11       0
//  Average:        10      18       0
//  Maximum:       129     147       0
//
//  No SIMD:   add/sub    mul    div
//  Minimum:         0       1       0
//   Median:         8      17       0
//  Average:        15      26       0
//  Maximum:       211     243       0
impl InfixAntiWedge for AntiScalar {}
impl AntiWedge<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e12345] * other[e12345]));
    }
}
impl AntiWedge<Circle> for AntiScalar {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e12345]) * other.group0()),
            // e415, e425, e435, e321
            (Simd32x4::from(self[e12345]) * other.group1()),
            // e235, e315, e125
            (Simd32x3::from(self[e12345]) * other.group2()),
        );
    }
}
impl AntiWedge<CircleRotor> for AntiScalar {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e12345]) * other.group0()),
            // e415, e425, e435, e321
            (Simd32x4::from(self[e12345]) * other.group1()),
            // e235, e315, e125, e12345
            (Simd32x4::from(self[e12345]) * other.group2()),
        );
    }
}
impl AntiWedge<Dipole> for AntiScalar {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e12345]) * other.group0()),
            // e23, e31, e12, e45
            (Simd32x4::from(self[e12345]) * other.group1()),
            // e15, e25, e35
            (Simd32x3::from(self[e12345]) * other.group2()),
        );
    }
}
impl AntiWedge<DipoleInversion> for AntiScalar {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e12345]) * other.group0()),
            // e23, e31, e12, e45
            (Simd32x4::from(self[e12345]) * other.group1()),
            // e15, e25, e35, e1234
            (Simd32x4::from(self[e12345]) * other.group2()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[e12345]) * other.group3()),
        );
    }
}
impl AntiWedge<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* e5, e12345 */ (Simd32x2::from(self[e12345]) * other.group0()));
    }
}
impl AntiWedge<FlatPoint> for AntiScalar {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ (Simd32x4::from(self[e12345]) * other.group0()));
    }
}
impl AntiWedge<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from(self[e12345]) * other.group0()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[e12345]) * other.group1()),
        );
    }
}
impl AntiWedge<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(self[e12345]) * other.group0()),
            // e235, e315, e125
            (Simd32x3::from(self[e12345]) * other.group1()),
        );
    }
}
impl AntiWedge<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(self[e12345]) * other.group0()),
            // e235, e315, e125, e5
            (Simd32x4::from(self[e12345]) * other.group1()),
        );
    }
}
impl AntiWedge<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            (Simd32x2::from(self[e12345]) * other.group0()),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e12345]) * other.group1()),
            // e5
            (self[e12345] * other[e1]),
            // e15, e25, e35, e45
            (Simd32x4::from(self[e12345]) * other.group3()),
            // e41, e42, e43
            (Simd32x3::from(self[e12345]) * other.group4()),
            // e415, e425, e435, e321
            (Simd32x4::from(self[e12345]) * other.group5()),
            // e423, e431, e412
            (Simd32x3::from(self[e12345]) * other.group6()),
            // e235, e315, e125
            (Simd32x3::from(self[e12345]) * other.group7()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[e12345]) * other.group8()),
            // e1234
            (self[e12345] * other[e35]),
            // e12, e31, e23
            (Simd32x3::from(self[e12345]) * other.group10()),
        );
    }
}
impl AntiWedge<Plane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (Simd32x4::from(self[e12345]) * other.group0()));
    }
}
impl AntiWedge<RoundPoint> for AntiScalar {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ (Simd32x4::from(self[e12345]) * other.group0()), /* e5 */ (self[e12345] * other[e2]));
    }
}
impl AntiWedge<Scalar> for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[e12345] * other[scalar]));
    }
}
impl AntiWedge<Sphere> for AntiScalar {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[e12345]) * other.group0()),
            // e1234
            (self[e12345] * other[e4315]),
        );
    }
}
impl AntiWedge<VersorEven> for AntiScalar {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from(self[e12345]) * other.group0()),
            // e415, e425, e435, e321
            (Simd32x4::from(self[e12345]) * other.group1()),
            // e235, e315, e125, e5
            (Simd32x4::from(self[e12345]) * other.group2()),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e12345]) * other.group3()),
        );
    }
}
impl AntiWedge<VersorOdd> for AntiScalar {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(self[e12345]) * other.group0()),
            // e23, e31, e12, e45
            (Simd32x4::from(self[e12345]) * other.group1()),
            // e15, e25, e35, e1234
            (Simd32x4::from(self[e12345]) * other.group2()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[e12345]) * other.group3()),
        );
    }
}
impl InfixAntiWedge for Circle {}
impl AntiWedge<AntiScalar> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(other[e12345])),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other[e12345])),
            // e235, e315, e125
            (self.group2() * Simd32x3::from(other[e12345])),
        );
    }
}
impl AntiWedge<Circle> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       25       30        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(self.group2()[2] * other.group0()[1])
                    + (self.group2()[1] * other.group0()[2])
                    + (self.group1()[3] * other.group1()[0])
                    + (self.group1()[0] * other.group1()[3])
                    - (self.group0()[1] * other.group2()[2])
                    + (self.group0()[2] * other.group2()[1])),
                ((self.group2()[2] * other.group0()[0]) - (self.group2()[0] * other.group0()[2])
                    + (self.group1()[3] * other.group1()[1])
                    + (self.group1()[1] * other.group1()[3])
                    + (self.group0()[0] * other.group2()[2])
                    - (self.group0()[2] * other.group2()[0])),
                (-(self.group2()[1] * other.group0()[0])
                    + (self.group2()[0] * other.group0()[1])
                    + (self.group1()[3] * other.group1()[2])
                    + (self.group1()[2] * other.group1()[3])
                    - (self.group0()[0] * other.group2()[1])
                    + (self.group0()[1] * other.group2()[0])),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e5
            (-(self.group2()[2] * other.group1()[2])
                - (self.group2()[1] * other.group1()[1])
                - (self.group2()[0] * other.group1()[0])
                - (self.group1()[2] * other.group2()[2])
                - (self.group1()[0] * other.group2()[0])
                - (self.group1()[1] * other.group2()[1])),
        );
    }
}
impl AntiWedge<CircleRotor> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       36        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       25       40        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] * other.group2()[3]),
                (self.group0()[1] * other.group2()[3]),
                (self.group0()[2] * other.group2()[3]),
                0.0,
            ]),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other.group2()[3])),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] * other.group2()[3]),
                (self.group2()[1] * other.group2()[3]),
                (self.group2()[2] * other.group2()[3]),
                (-(self.group2()[2] * other.group1()[2])
                    - (self.group2()[1] * other.group1()[1])
                    - (self.group2()[0] * other.group1()[0])
                    - (self.group1()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group2()[0])
                    - (self.group1()[1] * other.group2()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(self.group2()[2] * other.group0()[1])
                    + (self.group2()[1] * other.group0()[2])
                    + (self.group1()[3] * other.group1()[0])
                    + (self.group1()[0] * other.group1()[3])
                    - (self.group0()[1] * other.group2()[2])
                    + (self.group0()[2] * other.group2()[1])),
                ((self.group2()[2] * other.group0()[0]) - (self.group2()[0] * other.group0()[2])
                    + (self.group1()[3] * other.group1()[1])
                    + (self.group1()[1] * other.group1()[3])
                    + (self.group0()[0] * other.group2()[2])
                    - (self.group0()[2] * other.group2()[0])),
                (-(self.group2()[1] * other.group0()[0])
                    + (self.group2()[0] * other.group0()[1])
                    + (self.group1()[3] * other.group1()[2])
                    + (self.group1()[2] * other.group1()[3])
                    - (self.group0()[0] * other.group2()[1])
                    + (self.group0()[1] * other.group2()[0])),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
        );
    }
}
impl AntiWedge<Dipole> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        return Scalar::from_groups(
            // scalar
            (-(self.group2()[2] * other.group0()[2])
                - (self.group2()[1] * other.group0()[1])
                - (self.group2()[0] * other.group0()[0])
                - (self.group1()[3] * other.group1()[3])
                - (self.group1()[2] * other.group1()[2])
                - (self.group1()[1] * other.group1()[1])
                - (self.group1()[0] * other.group1()[0])
                - (self.group0()[2] * other.group2()[2])
                - (self.group0()[0] * other.group2()[0])
                - (self.group0()[1] * other.group2()[1])),
        );
    }
}
impl AntiWedge<DipoleInversion> for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       26       37        0
    //  no simd       29       40        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                ((self.group1()[0] * other.group2()[3]) - (self.group0()[1] * other.group3()[2]) + (self.group0()[2] * other.group3()[1])),
                ((self.group1()[1] * other.group2()[3]) + (self.group0()[0] * other.group3()[2]) - (self.group0()[2] * other.group3()[0])),
                ((self.group1()[2] * other.group2()[3]) - (self.group0()[0] * other.group3()[1]) + (self.group0()[1] * other.group3()[0])),
                (-(self.group2()[2] * other.group0()[2])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])),
            ]),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group3(), 0, 1, 2, 2))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group2()[3]) + (self.group0()[0] * other.group3()[3])),
                    ((self.group2()[1] * other.group2()[3]) + (self.group0()[1] * other.group3()[3])),
                    ((self.group2()[2] * other.group2()[3]) + (self.group0()[2] * other.group3()[3])),
                    (-(self.group1()[0] * other.group3()[0]) - (self.group1()[1] * other.group3()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group2()[2] * other.group3()[1]) + (self.group1()[0] * other.group3()[3]) + (self.group2()[1] * other.group3()[2])),
                ((self.group2()[2] * other.group3()[0]) + (self.group1()[1] * other.group3()[3]) - (self.group2()[0] * other.group3()[2])),
                (-(self.group2()[1] * other.group3()[0]) + (self.group1()[2] * other.group3()[3]) + (self.group2()[0] * other.group3()[1])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<DualNum> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(other.group0()[1])),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other.group0()[1])),
            // e235, e315, e125
            (self.group2() * Simd32x3::from(other.group0()[1])),
        );
    }
}
impl AntiWedge<FlatPoint> for Circle {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        return Scalar::from_groups(
            // scalar
            (-(self.group1()[3] * other.group0()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl AntiWedge<Flector> for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       25        0
    //  no simd       17       28        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-(self.group0()[1] * other.group1()[2]) + (self.group0()[2] * other.group1()[1])),
                ((self.group0()[0] * other.group1()[2]) - (self.group0()[2] * other.group1()[0])),
                (-(self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
                (-(self.group1()[3] * other.group0()[3])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group0()[0] * other.group0()[0])
                    - (self.group0()[1] * other.group0()[1])),
            ]),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group1(), 0, 1, 2, 2))
                + Simd32x4::from([
                    (self.group0()[0] * other.group1()[3]),
                    (self.group0()[1] * other.group1()[3]),
                    (self.group0()[2] * other.group1()[3]),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group2()[2] * other.group1()[1]) + (self.group1()[0] * other.group1()[3]) + (self.group2()[1] * other.group1()[2])),
                ((self.group2()[2] * other.group1()[0]) + (self.group1()[1] * other.group1()[3]) - (self.group2()[0] * other.group1()[2])),
                (-(self.group2()[1] * other.group1()[0]) + (self.group1()[2] * other.group1()[3]) + (self.group2()[0] * other.group1()[1])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<Line> for Circle {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       18        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                ((self.group1()[3] * other.group0()[0]) - (self.group0()[1] * other.group1()[2]) + (self.group0()[2] * other.group1()[1])),
                ((self.group1()[3] * other.group0()[1]) + (self.group0()[0] * other.group1()[2]) - (self.group0()[2] * other.group1()[0])),
                ((self.group1()[3] * other.group0()[2]) - (self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
                (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
            ]),
            // e5
            (-(self.group2()[2] * other.group0()[2])
                - (self.group2()[1] * other.group0()[1])
                - (self.group2()[0] * other.group0()[0])
                - (self.group1()[2] * other.group1()[2])
                - (self.group1()[0] * other.group1()[0])
                - (self.group1()[1] * other.group1()[1])),
        );
    }
}
impl AntiWedge<Motor> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       13       25        0
    //  no simd       13       28        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]),
                (self.group0()[1] * other.group0()[3]),
                (self.group0()[2] * other.group0()[3]),
                0.0,
            ]),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other.group0()[3])),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] * other.group0()[3]),
                (self.group2()[1] * other.group0()[3]),
                (self.group2()[2] * other.group0()[3]),
                (-(self.group2()[2] * other.group0()[2])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                ((self.group1()[3] * other.group0()[0]) - (self.group0()[1] * other.group1()[2]) + (self.group0()[2] * other.group1()[1])),
                ((self.group1()[3] * other.group0()[1]) + (self.group0()[0] * other.group1()[2]) - (self.group0()[2] * other.group1()[0])),
                ((self.group1()[3] * other.group0()[2]) - (self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
                (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
            ]),
        );
    }
}
impl AntiWedge<MultiVector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       48        0
    //    simd3        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       43       58        0
    //  no simd       54       80        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group2()[2] * other.group4()[2])
                    - (self.group2()[1] * other.group4()[1])
                    - (self.group2()[0] * other.group4()[0])
                    - (self.group1()[3] * other.group3()[3])
                    - (self.group1()[2] * other.group10()[0])
                    - (self.group1()[1] * other.group10()[1])
                    - (self.group1()[0] * other.group10()[2])
                    - (self.group0()[2] * other.group3()[2])
                    - (self.group0()[0] * other.group3()[0])
                    - (self.group0()[1] * other.group3()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(self.group2()[2] * other.group6()[1])
                    + (self.group2()[1] * other.group6()[2])
                    + (self.group1()[3] * other.group5()[0])
                    + (self.group1()[0] * other.group5()[3])
                    - (self.group0()[1] * other.group7()[2])
                    + (self.group0()[2] * other.group7()[1])),
                ((self.group2()[2] * other.group6()[0]) - (self.group2()[0] * other.group6()[2])
                    + (self.group1()[3] * other.group5()[1])
                    + (self.group1()[1] * other.group5()[3])
                    + (self.group0()[0] * other.group7()[2])
                    - (self.group0()[2] * other.group7()[0])),
                (-(self.group2()[1] * other.group6()[0])
                    + (self.group2()[0] * other.group6()[1])
                    + (self.group1()[3] * other.group5()[2])
                    + (self.group1()[2] * other.group5()[3])
                    - (self.group0()[0] * other.group7()[1])
                    + (self.group0()[1] * other.group7()[0])),
                (-(self.group1()[2] * other.group6()[2])
                    - (self.group1()[1] * other.group6()[1])
                    - (self.group1()[0] * other.group6()[0])
                    - (self.group0()[2] * other.group5()[2])
                    - (self.group0()[0] * other.group5()[0])
                    - (self.group0()[1] * other.group5()[1])),
            ]),
            // e5
            (-(self.group2()[2] * other.group5()[2])
                - (self.group2()[1] * other.group5()[1])
                - (self.group2()[0] * other.group5()[0])
                - (self.group1()[2] * other.group7()[2])
                - (self.group1()[0] * other.group7()[0])
                - (self.group1()[1] * other.group7()[1])),
            // e15, e25, e35, e45
            (-(swizzle!(other.group8(), 1, 2, 0, 2) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group8()[3]) + (self.group2()[1] * other.group8()[2])),
                    ((self.group2()[2] * other.group8()[0]) + (self.group1()[1] * other.group8()[3])),
                    ((self.group1()[2] * other.group8()[3]) + (self.group2()[0] * other.group8()[1])),
                    (-(self.group1()[0] * other.group8()[0]) - (self.group1()[1] * other.group8()[1])),
                ])),
            // e41, e42, e43
            ((Simd32x3::from(other[e35]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (swizzle!(self.group0(), 1, 2, 0) * Simd32x3::from([other.group8()[2], other.group8()[0], other.group8()[1]]))
                + (swizzle!(self.group0(), 2, 0, 1) * Simd32x3::from([other.group8()[1], other.group8()[2], other.group8()[0]]))),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other.group0()[1])),
            // e423, e431, e412
            (self.group0() * Simd32x3::from(other.group0()[1])),
            // e235, e315, e125
            (self.group2() * Simd32x3::from(other.group0()[1])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            ((swizzle!(self.group2(), 2, 1, 0) * Simd32x3::from(other[e35])) + (swizzle!(self.group0(), 2, 1, 0) * Simd32x3::from(other.group8()[3]))
                - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group8()[2], other.group8()[1], other.group8()[0]]))),
        );
    }
}
impl AntiWedge<Plane> for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       14       24        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            (-(swizzle!(self.group0(), 1, 2, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]))
                + (swizzle!(self.group0(), 2, 0, 1) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]))),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2))
                + Simd32x4::from([
                    (self.group0()[0] * other.group0()[3]),
                    (self.group0()[1] * other.group0()[3]),
                    (self.group0()[2] * other.group0()[3]),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
            // e15, e25, e35
            (-(swizzle!(self.group2(), 2, 0, 1) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]))
                + (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (swizzle!(self.group2(), 1, 2, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]))),
        );
    }
}
impl AntiWedge<Sphere> for Circle {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        4        6        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       20       30        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            ((Simd32x3::from(other[e4315]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (swizzle!(self.group0(), 1, 2, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]))
                + (swizzle!(self.group0(), 2, 0, 1) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]))),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2))
                + Simd32x4::from([
                    ((self.group2()[0] * other[e4315]) + (self.group0()[0] * other.group0()[3])),
                    ((self.group2()[1] * other[e4315]) + (self.group0()[1] * other.group0()[3])),
                    ((self.group2()[2] * other[e4315]) + (self.group0()[2] * other.group0()[3])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
            // e15, e25, e35
            (-(swizzle!(self.group2(), 2, 0, 1) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]))
                + (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (swizzle!(self.group2(), 1, 2, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]))),
        );
    }
}
impl AntiWedge<VersorEven> for Circle {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       32        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       22       34        0
    //  no simd       25       40        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]),
                (self.group0()[1] * other.group0()[3]),
                (self.group0()[2] * other.group0()[3]),
                0.0,
            ]),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other.group0()[3])),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] * other.group0()[3]),
                (self.group2()[1] * other.group0()[3]),
                (self.group2()[2] * other.group0()[3]),
                (-(self.group2()[2] * other.group1()[2])
                    - (self.group2()[1] * other.group1()[1])
                    - (self.group2()[0] * other.group1()[0])
                    - (self.group1()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group2()[0])
                    - (self.group1()[1] * other.group2()[1])),
            ]),
            // e1, e2, e3, e4
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[1] * other.group0()[2]) + (self.group1()[3] * other.group1()[0]) + (self.group1()[0] * other.group1()[3])
                        - (self.group0()[1] * other.group2()[2])
                        + (self.group0()[2] * other.group2()[1])),
                    ((self.group2()[2] * other.group0()[0])
                        + (self.group1()[3] * other.group1()[1])
                        + (self.group1()[1] * other.group1()[3])
                        + (self.group0()[0] * other.group2()[2])
                        - (self.group0()[2] * other.group2()[0])),
                    ((self.group2()[0] * other.group0()[1]) + (self.group1()[3] * other.group1()[2]) + (self.group1()[2] * other.group1()[3])
                        - (self.group0()[0] * other.group2()[1])
                        + (self.group0()[1] * other.group2()[0])),
                    (-(self.group1()[1] * other.group0()[1])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group0()[2] * other.group1()[2])
                        - (self.group0()[0] * other.group1()[0])
                        - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl AntiWedge<VersorOdd> for Circle {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       26       37        0
    //  no simd       29       40        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                ((self.group1()[0] * other.group2()[3]) - (self.group0()[1] * other.group3()[2]) + (self.group0()[2] * other.group3()[1])),
                ((self.group1()[1] * other.group2()[3]) + (self.group0()[0] * other.group3()[2]) - (self.group0()[2] * other.group3()[0])),
                ((self.group1()[2] * other.group2()[3]) - (self.group0()[0] * other.group3()[1]) + (self.group0()[1] * other.group3()[0])),
                (-(self.group2()[2] * other.group0()[2])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])),
            ]),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group3(), 0, 1, 2, 2))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group2()[3]) + (self.group0()[0] * other.group3()[3])),
                    ((self.group2()[1] * other.group2()[3]) + (self.group0()[1] * other.group3()[3])),
                    ((self.group2()[2] * other.group2()[3]) + (self.group0()[2] * other.group3()[3])),
                    (-(self.group1()[0] * other.group3()[0]) - (self.group1()[1] * other.group3()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group2()[2] * other.group3()[1]) + (self.group1()[0] * other.group3()[3]) + (self.group2()[1] * other.group3()[2])),
                ((self.group2()[2] * other.group3()[0]) + (self.group1()[1] * other.group3()[3]) - (self.group2()[0] * other.group3()[2])),
                (-(self.group2()[1] * other.group3()[0]) + (self.group1()[2] * other.group3()[3]) + (self.group2()[0] * other.group3()[1])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl InfixAntiWedge for CircleRotor {}
impl AntiWedge<AntiScalar> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(other[e12345])),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other[e12345])),
            // e235, e315, e125, e12345
            (self.group2() * Simd32x4::from(other[e12345])),
        );
    }
}
impl AntiWedge<Circle> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       36        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       25       37        0
    //  no simd       25       40        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group2()[3] * other.group0()[0]),
                (self.group2()[3] * other.group0()[1]),
                (self.group2()[3] * other.group0()[2]),
                0.0,
            ]),
            // e415, e425, e435, e321
            (Simd32x4::from(self.group2()[3]) * other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[3] * other.group2()[0]),
                (self.group2()[3] * other.group2()[1]),
                (self.group2()[3] * other.group2()[2]),
                (-(self.group2()[2] * other.group1()[2])
                    - (self.group2()[1] * other.group1()[1])
                    - (self.group2()[0] * other.group1()[0])
                    - (self.group1()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group2()[0])
                    - (self.group1()[1] * other.group2()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(self.group2()[2] * other.group0()[1])
                    + (self.group2()[1] * other.group0()[2])
                    + (self.group1()[3] * other.group1()[0])
                    + (self.group1()[0] * other.group1()[3])
                    - (self.group0()[1] * other.group2()[2])
                    + (self.group0()[2] * other.group2()[1])),
                ((self.group2()[2] * other.group0()[0]) - (self.group2()[0] * other.group0()[2])
                    + (self.group1()[3] * other.group1()[1])
                    + (self.group1()[1] * other.group1()[3])
                    + (self.group0()[0] * other.group2()[2])
                    - (self.group0()[2] * other.group2()[0])),
                (-(self.group2()[1] * other.group0()[0])
                    + (self.group2()[0] * other.group0()[1])
                    + (self.group1()[3] * other.group1()[2])
                    + (self.group1()[2] * other.group1()[3])
                    - (self.group0()[0] * other.group2()[1])
                    + (self.group0()[1] * other.group2()[0])),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
        );
    }
}
impl AntiWedge<CircleRotor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       43        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       32       45        0
    //  no simd       35       51        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                ((self.group0()[0] * other.group2()[3]) + (self.group2()[3] * other.group0()[0])),
                ((self.group0()[1] * other.group2()[3]) + (self.group2()[3] * other.group0()[1])),
                ((self.group0()[2] * other.group2()[3]) + (self.group2()[3] * other.group0()[2])),
                (self.group2()[3] * other.group2()[3]),
            ]),
            // e415, e425, e435, e321
            ((self.group1() * Simd32x4::from(other.group2()[3])) + (Simd32x4::from(self.group2()[3]) * other.group1())),
            // e235, e315, e125, e5
            Simd32x4::from([
                ((self.group2()[0] * other.group2()[3]) + (self.group2()[3] * other.group2()[0])),
                ((self.group2()[1] * other.group2()[3]) + (self.group2()[3] * other.group2()[1])),
                ((self.group2()[2] * other.group2()[3]) + (self.group2()[3] * other.group2()[2])),
                (-(self.group2()[2] * other.group1()[2])
                    - (self.group2()[1] * other.group1()[1])
                    - (self.group2()[0] * other.group1()[0])
                    - (self.group1()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group2()[0])
                    - (self.group1()[1] * other.group2()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(self.group2()[2] * other.group0()[1])
                    + (self.group2()[1] * other.group0()[2])
                    + (self.group1()[3] * other.group1()[0])
                    + (self.group1()[0] * other.group1()[3])
                    - (self.group0()[1] * other.group2()[2])
                    + (self.group0()[2] * other.group2()[1])),
                ((self.group2()[2] * other.group0()[0]) - (self.group2()[0] * other.group0()[2])
                    + (self.group1()[3] * other.group1()[1])
                    + (self.group1()[1] * other.group1()[3])
                    + (self.group0()[0] * other.group2()[2])
                    - (self.group0()[2] * other.group2()[0])),
                (-(self.group2()[1] * other.group0()[0])
                    + (self.group2()[0] * other.group0()[1])
                    + (self.group1()[3] * other.group1()[2])
                    + (self.group1()[2] * other.group1()[3])
                    - (self.group0()[0] * other.group2()[1])
                    + (self.group0()[1] * other.group2()[0])),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
        );
    }
}
impl AntiWedge<Dipole> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       16        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       17        0
    //  no simd        9       20        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group2()[3] * other.group0()[0]),
                (self.group2()[3] * other.group0()[1]),
                (self.group2()[3] * other.group0()[2]),
                (-(self.group2()[2] * other.group0()[2])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])),
            ]),
            // e23, e31, e12, e45
            (Simd32x4::from(self.group2()[3]) * other.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[3] * other.group2()[0]),
                (self.group2()[3] * other.group2()[1]),
                (self.group2()[3] * other.group2()[2]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<DipoleInversion> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       43        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       33       46        0
    //  no simd       39       55        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                ((self.group2()[3] * other.group0()[0]) + (self.group1()[0] * other.group2()[3]) - (self.group0()[1] * other.group3()[2]) + (self.group0()[2] * other.group3()[1])),
                ((self.group2()[3] * other.group0()[1]) + (self.group1()[1] * other.group2()[3]) + (self.group0()[0] * other.group3()[2]) - (self.group0()[2] * other.group3()[0])),
                ((self.group2()[3] * other.group0()[2]) + (self.group1()[2] * other.group2()[3]) - (self.group0()[0] * other.group3()[1]) + (self.group0()[1] * other.group3()[0])),
                (-(self.group2()[2] * other.group0()[2])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])),
            ]),
            // e23, e31, e12, e45
            ((Simd32x4::from(self.group2()[3]) * other.group1()) - (swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group3(), 0, 1, 2, 2))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group2()[3]) + (self.group0()[0] * other.group3()[3])),
                    ((self.group2()[1] * other.group2()[3]) + (self.group0()[1] * other.group3()[3])),
                    ((self.group2()[2] * other.group2()[3]) + (self.group0()[2] * other.group3()[3])),
                    (-(self.group1()[0] * other.group3()[0]) - (self.group1()[1] * other.group3()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group2()[3] * other.group2()[0]) - (self.group2()[2] * other.group3()[1]) + (self.group1()[0] * other.group3()[3]) + (self.group2()[1] * other.group3()[2])),
                ((self.group2()[3] * other.group2()[1]) + (self.group2()[2] * other.group3()[0]) + (self.group1()[1] * other.group3()[3]) - (self.group2()[0] * other.group3()[2])),
                ((self.group2()[3] * other.group2()[2]) - (self.group2()[1] * other.group3()[0]) + (self.group1()[2] * other.group3()[3]) + (self.group2()[0] * other.group3()[1])),
                (self.group2()[3] * other.group2()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group2()[3]) * other.group3()),
        );
    }
}
impl AntiWedge<DualNum> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from(other.group0()[1]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]])),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other.group0()[1])),
            // e235, e315, e125, e5
            (self.group2() * Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]])),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<FlatPoint> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(self.group1()[3] * other.group0()[3])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group0()[0] * other.group0()[0])
                    - (self.group0()[1] * other.group0()[1])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, (self.group2()[3] * other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[3] * other.group0()[0]),
                (self.group2()[3] * other.group0()[1]),
                (self.group2()[3] * other.group0()[2]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<Flector> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       21       36        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-(self.group0()[1] * other.group1()[2]) + (self.group0()[2] * other.group1()[1])),
                ((self.group0()[0] * other.group1()[2]) - (self.group0()[2] * other.group1()[0])),
                (-(self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
                (-(self.group1()[3] * other.group0()[3])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group0()[0] * other.group0()[0])
                    - (self.group0()[1] * other.group0()[1])),
            ]),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group1(), 0, 1, 2, 2))
                + Simd32x4::from([
                    (self.group0()[0] * other.group1()[3]),
                    (self.group0()[1] * other.group1()[3]),
                    (self.group0()[2] * other.group1()[3]),
                    ((self.group2()[3] * other.group0()[3]) - (self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group2()[3] * other.group0()[0]) - (self.group2()[2] * other.group1()[1]) + (self.group1()[0] * other.group1()[3]) + (self.group2()[1] * other.group1()[2])),
                ((self.group2()[3] * other.group0()[1]) + (self.group2()[2] * other.group1()[0]) + (self.group1()[1] * other.group1()[3]) - (self.group2()[0] * other.group1()[2])),
                ((self.group2()[3] * other.group0()[2]) - (self.group2()[1] * other.group1()[0]) + (self.group1()[2] * other.group1()[3]) + (self.group2()[0] * other.group1()[1])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group2()[3]) * other.group1()),
        );
    }
}
impl AntiWedge<Line> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       24        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group2()[3] * other.group0()[0]),
                (self.group2()[3] * other.group0()[1]),
                (self.group2()[3] * other.group0()[2]),
                0.0,
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[3] * other.group1()[0]),
                (self.group2()[3] * other.group1()[1]),
                (self.group2()[3] * other.group1()[2]),
                (-(self.group2()[2] * other.group0()[2])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                ((self.group1()[3] * other.group0()[0]) - (self.group0()[1] * other.group1()[2]) + (self.group0()[2] * other.group1()[1])),
                ((self.group1()[3] * other.group0()[1]) + (self.group0()[0] * other.group1()[2]) - (self.group0()[2] * other.group1()[0])),
                ((self.group1()[3] * other.group0()[2]) - (self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
                (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
            ]),
        );
    }
}
impl AntiWedge<Motor> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       17       30        0
    //  no simd       20       36        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]])),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((self.group1()[0] * other.group0()[3]) + (self.group2()[3] * other.group0()[0])),
                ((self.group1()[1] * other.group0()[3]) + (self.group2()[3] * other.group0()[1])),
                ((self.group1()[2] * other.group0()[3]) + (self.group2()[3] * other.group0()[2])),
                (self.group1()[3] * other.group0()[3]),
            ]),
            // e235, e315, e125, e5
            ((self.group2() * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[3]]))
                + Simd32x4::from([
                    (self.group2()[3] * other.group1()[0]),
                    (self.group2()[3] * other.group1()[1]),
                    (self.group2()[3] * other.group1()[2]),
                    (-(self.group2()[2] * other.group0()[2])
                        - (self.group2()[1] * other.group0()[1])
                        - (self.group2()[0] * other.group0()[0])
                        - (self.group1()[2] * other.group1()[2])
                        - (self.group1()[0] * other.group1()[0])
                        - (self.group1()[1] * other.group1()[1])),
                ])),
            // e1, e2, e3, e4
            Simd32x4::from([
                ((self.group1()[3] * other.group0()[0]) - (self.group0()[1] * other.group1()[2]) + (self.group0()[2] * other.group1()[1])),
                ((self.group1()[3] * other.group0()[1]) + (self.group0()[0] * other.group1()[2]) - (self.group0()[2] * other.group1()[0])),
                ((self.group1()[3] * other.group0()[2]) - (self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
                (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
            ]),
        );
    }
}
impl AntiWedge<MultiVector> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd3        8       12        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       52       70        0
    //  no simd       80      112        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                ((self.group2()[3] * other.group0()[0])
                    - (self.group2()[2] * other.group4()[2])
                    - (self.group2()[1] * other.group4()[1])
                    - (self.group2()[0] * other.group4()[0])
                    - (self.group1()[3] * other.group3()[3])
                    - (self.group1()[2] * other.group10()[0])
                    - (self.group1()[1] * other.group10()[1])
                    - (self.group1()[0] * other.group10()[2])
                    - (self.group0()[2] * other.group3()[2])
                    - (self.group0()[0] * other.group3()[0])
                    - (self.group0()[1] * other.group3()[1])),
                (self.group2()[3] * other.group0()[1]),
            ]),
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group2()[3]) * other.group1())
                + Simd32x4::from([
                    (-(self.group2()[2] * other.group6()[1])
                        + (self.group2()[1] * other.group6()[2])
                        + (self.group1()[3] * other.group5()[0])
                        + (self.group1()[0] * other.group5()[3])
                        - (self.group0()[1] * other.group7()[2])
                        + (self.group0()[2] * other.group7()[1])),
                    ((self.group2()[2] * other.group6()[0]) - (self.group2()[0] * other.group6()[2])
                        + (self.group1()[3] * other.group5()[1])
                        + (self.group1()[1] * other.group5()[3])
                        + (self.group0()[0] * other.group7()[2])
                        - (self.group0()[2] * other.group7()[0])),
                    (-(self.group2()[1] * other.group6()[0])
                        + (self.group2()[0] * other.group6()[1])
                        + (self.group1()[3] * other.group5()[2])
                        + (self.group1()[2] * other.group5()[3])
                        - (self.group0()[0] * other.group7()[1])
                        + (self.group0()[1] * other.group7()[0])),
                    (-(self.group1()[2] * other.group6()[2])
                        - (self.group1()[1] * other.group6()[1])
                        - (self.group1()[0] * other.group6()[0])
                        - (self.group0()[2] * other.group5()[2])
                        - (self.group0()[0] * other.group5()[0])
                        - (self.group0()[1] * other.group5()[1])),
                ])),
            // e5
            ((self.group2()[3] * other[e1])
                - (self.group2()[2] * other.group5()[2])
                - (self.group2()[1] * other.group5()[1])
                - (self.group2()[0] * other.group5()[0])
                - (self.group1()[2] * other.group7()[2])
                - (self.group1()[0] * other.group7()[0])
                - (self.group1()[1] * other.group7()[1])),
            // e15, e25, e35, e45
            ((Simd32x4::from(self.group2()[3]) * other.group3())
                - (swizzle!(other.group8(), 1, 2, 0, 2) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group8()[3]) + (self.group2()[1] * other.group8()[2])),
                    ((self.group2()[2] * other.group8()[0]) + (self.group1()[1] * other.group8()[3])),
                    ((self.group1()[2] * other.group8()[3]) + (self.group2()[0] * other.group8()[1])),
                    (-(self.group1()[0] * other.group8()[0]) - (self.group1()[1] * other.group8()[1])),
                ])),
            // e41, e42, e43
            ((Simd32x3::from(self.group2()[3]) * other.group4()) + (Simd32x3::from(other[e35]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (swizzle!(self.group0(), 1, 2, 0) * Simd32x3::from([other.group8()[2], other.group8()[0], other.group8()[1]]))
                + (swizzle!(self.group0(), 2, 0, 1) * Simd32x3::from([other.group8()[1], other.group8()[2], other.group8()[0]]))),
            // e415, e425, e435, e321
            ((self.group1() * Simd32x4::from(other.group0()[1])) + (Simd32x4::from(self.group2()[3]) * other.group5())),
            // e423, e431, e412
            ((self.group0() * Simd32x3::from(other.group0()[1])) + (Simd32x3::from(self.group2()[3]) * other.group6())),
            // e235, e315, e125
            ((Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]])) + (Simd32x3::from(self.group2()[3]) * other.group7())),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group2()[3]) * other.group8()),
            // e1234
            (self.group2()[3] * other[e35]),
            // e12, e31, e23
            ((Simd32x3::from(self.group2()[3]) * other.group10())
                + (Simd32x3::from(other[e35]) * Simd32x3::from([self.group2()[2], self.group2()[1], self.group2()[0]]))
                + (swizzle!(self.group0(), 2, 1, 0) * Simd32x3::from(other.group8()[3]))
                - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group8()[2], other.group8()[1], other.group8()[0]]))),
        );
    }
}
impl AntiWedge<Plane> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       14       28        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (-(swizzle!(self.group0(), 1, 2, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]))
                + (swizzle!(self.group0(), 2, 0, 1) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]))),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2))
                + Simd32x4::from([
                    (self.group0()[0] * other.group0()[3]),
                    (self.group0()[1] * other.group0()[3]),
                    (self.group0()[2] * other.group0()[3]),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group2()[2] * other.group0()[1]) + (self.group1()[0] * other.group0()[3]) + (self.group2()[1] * other.group0()[2])),
                ((self.group2()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3]) - (self.group2()[0] * other.group0()[2])),
                (-(self.group2()[1] * other.group0()[0]) + (self.group1()[2] * other.group0()[3]) + (self.group2()[0] * other.group0()[1])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group2()[3]) * other.group0()),
        );
    }
}
impl AntiWedge<RoundPoint> for CircleRotor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self.group2()[3]) * other.group0()),
            // e5
            (self.group2()[3] * other[e2]),
        );
    }
}
impl AntiWedge<Scalar> for CircleRotor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self.group2()[3] * other[scalar]));
    }
}
impl AntiWedge<Sphere> for CircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        2        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       20       35        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            ((Simd32x3::from(other[e4315]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (swizzle!(self.group0(), 1, 2, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]))
                + (swizzle!(self.group0(), 2, 0, 1) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]))),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2))
                + Simd32x4::from([
                    ((self.group2()[0] * other[e4315]) + (self.group0()[0] * other.group0()[3])),
                    ((self.group2()[1] * other[e4315]) + (self.group0()[1] * other.group0()[3])),
                    ((self.group2()[2] * other[e4315]) + (self.group0()[2] * other.group0()[3])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group2()[2] * other.group0()[1]) + (self.group1()[0] * other.group0()[3]) + (self.group2()[1] * other.group0()[2])),
                ((self.group2()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3]) - (self.group2()[0] * other.group0()[2])),
                (-(self.group2()[1] * other.group0()[0]) + (self.group1()[2] * other.group0()[3]) + (self.group2()[0] * other.group0()[1])),
                (self.group2()[3] * other[e4315]),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group2()[3]) * other.group0()),
        );
    }
}
impl AntiWedge<VersorEven> for CircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       28       41        0
    //  no simd       40       56        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                ((self.group0()[0] * other.group0()[3]) + (self.group2()[3] * other.group0()[0])),
                ((self.group0()[1] * other.group0()[3]) + (self.group2()[3] * other.group0()[1])),
                ((self.group0()[2] * other.group0()[3]) + (self.group2()[3] * other.group0()[2])),
                (self.group2()[3] * other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            ((self.group1() * Simd32x4::from(other.group0()[3])) + (Simd32x4::from(self.group2()[3]) * other.group1())),
            // e235, e315, e125, e5
            ((self.group2() * Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group2()[3]]))
                + Simd32x4::from([
                    (self.group2()[3] * other.group2()[0]),
                    (self.group2()[3] * other.group2()[1]),
                    (self.group2()[3] * other.group2()[2]),
                    (-(self.group2()[2] * other.group1()[2])
                        - (self.group2()[1] * other.group1()[1])
                        - (self.group2()[0] * other.group1()[0])
                        - (self.group1()[2] * other.group2()[2])
                        - (self.group1()[0] * other.group2()[0])
                        - (self.group1()[1] * other.group2()[1])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group2()[3]) * other.group3())
                - (swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[1] * other.group0()[2]) + (self.group1()[3] * other.group1()[0]) + (self.group1()[0] * other.group1()[3])
                        - (self.group0()[1] * other.group2()[2])
                        + (self.group0()[2] * other.group2()[1])),
                    ((self.group2()[2] * other.group0()[0])
                        + (self.group1()[3] * other.group1()[1])
                        + (self.group1()[1] * other.group1()[3])
                        + (self.group0()[0] * other.group2()[2])
                        - (self.group0()[2] * other.group2()[0])),
                    ((self.group2()[0] * other.group0()[1]) + (self.group1()[3] * other.group1()[2]) + (self.group1()[2] * other.group1()[3])
                        - (self.group0()[0] * other.group2()[1])
                        + (self.group0()[1] * other.group2()[0])),
                    (-(self.group1()[1] * other.group0()[1])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group0()[2] * other.group1()[2])
                        - (self.group0()[0] * other.group1()[0])
                        - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl AntiWedge<VersorOdd> for CircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       31       44        0
    //  no simd       40       56        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x4::from(self.group2()[3]) * other.group0())
                + Simd32x4::from([
                    ((self.group1()[0] * other.group2()[3]) - (self.group0()[1] * other.group3()[2]) + (self.group0()[2] * other.group3()[1])),
                    ((self.group1()[1] * other.group2()[3]) + (self.group0()[0] * other.group3()[2]) - (self.group0()[2] * other.group3()[0])),
                    ((self.group1()[2] * other.group2()[3]) - (self.group0()[0] * other.group3()[1]) + (self.group0()[1] * other.group3()[0])),
                    (-(self.group2()[2] * other.group0()[2])
                        - (self.group2()[1] * other.group0()[1])
                        - (self.group2()[0] * other.group0()[0])
                        - (self.group1()[3] * other.group1()[3])
                        - (self.group1()[2] * other.group1()[2])
                        - (self.group1()[1] * other.group1()[1])
                        - (self.group1()[0] * other.group1()[0])
                        - (self.group0()[2] * other.group2()[2])
                        - (self.group0()[0] * other.group2()[0])
                        - (self.group0()[1] * other.group2()[1])),
                ])),
            // e23, e31, e12, e45
            ((Simd32x4::from(self.group2()[3]) * other.group1()) - (swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group3(), 0, 1, 2, 2))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group2()[3]) + (self.group0()[0] * other.group3()[3])),
                    ((self.group2()[1] * other.group2()[3]) + (self.group0()[1] * other.group3()[3])),
                    ((self.group2()[2] * other.group2()[3]) + (self.group0()[2] * other.group3()[3])),
                    (-(self.group1()[0] * other.group3()[0]) - (self.group1()[1] * other.group3()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group2()[3] * other.group2()[0]) - (self.group2()[2] * other.group3()[1]) + (self.group1()[0] * other.group3()[3]) + (self.group2()[1] * other.group3()[2])),
                ((self.group2()[3] * other.group2()[1]) + (self.group2()[2] * other.group3()[0]) + (self.group1()[1] * other.group3()[3]) - (self.group2()[0] * other.group3()[2])),
                ((self.group2()[3] * other.group2()[2]) - (self.group2()[1] * other.group3()[0]) + (self.group1()[2] * other.group3()[3]) + (self.group2()[0] * other.group3()[1])),
                (self.group2()[3] * other.group2()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group2()[3]) * other.group3()),
        );
    }
}
impl InfixAntiWedge for Dipole {}
impl AntiWedge<AntiScalar> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other[e12345])),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(other[e12345])),
            // e15, e25, e35
            (self.group2() * Simd32x3::from(other[e12345])),
        );
    }
}
impl AntiWedge<Circle> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        return Scalar::from_groups(
            // scalar
            (-(self.group2()[2] * other.group0()[2])
                - (self.group2()[1] * other.group0()[1])
                - (self.group2()[0] * other.group0()[0])
                - (self.group1()[3] * other.group1()[3])
                - (self.group1()[2] * other.group1()[2])
                - (self.group1()[1] * other.group1()[1])
                - (self.group1()[0] * other.group1()[0])
                - (self.group0()[2] * other.group2()[2])
                - (self.group0()[0] * other.group2()[0])
                - (self.group0()[1] * other.group2()[1])),
        );
    }
}
impl AntiWedge<CircleRotor> for Dipole {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       16        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       17        0
    //  no simd        9       20        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] * other.group2()[3]),
                (self.group0()[1] * other.group2()[3]),
                (self.group0()[2] * other.group2()[3]),
                (-(self.group2()[2] * other.group0()[2])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])),
            ]),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(other.group2()[3])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] * other.group2()[3]),
                (self.group2()[1] * other.group2()[3]),
                (self.group2()[2] * other.group2()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<DipoleInversion> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (-(Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                - (swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group3()[3]) + (self.group1()[1] * other.group3()[2])),
                    ((self.group1()[2] * other.group3()[0]) + (self.group0()[1] * other.group3()[3])),
                    ((self.group0()[2] * other.group3()[3]) + (self.group1()[0] * other.group3()[1])),
                    (-(self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
                ])),
            // e5
            ((self.group2()[2] * other.group3()[2]) + (self.group2()[1] * other.group3()[1]) + (self.group1()[3] * other.group3()[3]) + (self.group2()[0] * other.group3()[0])),
        );
    }
}
impl AntiWedge<DualNum> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other.group0()[1])),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(other.group0()[1])),
            // e15, e25, e35
            (self.group2() * Simd32x3::from(other.group0()[1])),
        );
    }
}
impl AntiWedge<Flector> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (-(swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group1()[3]) + (self.group1()[1] * other.group1()[2])),
                    ((self.group1()[2] * other.group1()[0]) + (self.group0()[1] * other.group1()[3])),
                    ((self.group0()[2] * other.group1()[3]) + (self.group1()[0] * other.group1()[1])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
            // e5
            ((self.group2()[2] * other.group1()[2]) + (self.group2()[1] * other.group1()[1]) + (self.group1()[3] * other.group1()[3]) + (self.group2()[0] * other.group1()[0])),
        );
    }
}
impl AntiWedge<Line> for Dipole {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        return Scalar::from_groups(
            // scalar
            (-(self.group1()[2] * other.group0()[2])
                - (self.group1()[1] * other.group0()[1])
                - (self.group1()[0] * other.group0()[0])
                - (self.group0()[2] * other.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])),
        );
    }
}
impl AntiWedge<Motor> for Dipole {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        5       16        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]),
                (self.group0()[1] * other.group0()[3]),
                (self.group0()[2] * other.group0()[3]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(other.group0()[3])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] * other.group0()[3]),
                (self.group2()[1] * other.group0()[3]),
                (self.group2()[2] * other.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<MultiVector> for Dipole {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       22        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       24       40        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group2()[2] * other.group6()[2])
                    - (self.group2()[1] * other.group6()[1])
                    - (self.group2()[0] * other.group6()[0])
                    - (self.group1()[3] * other.group5()[3])
                    - (self.group1()[2] * other.group5()[2])
                    - (self.group1()[1] * other.group5()[1])
                    - (self.group1()[0] * other.group5()[0])
                    - (self.group0()[2] * other.group7()[2])
                    - (self.group0()[0] * other.group7()[0])
                    - (self.group0()[1] * other.group7()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            (-(Simd32x4::from(other[e35]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                - (swizzle!(other.group8(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group8()[3]) + (self.group1()[1] * other.group8()[2])),
                    ((self.group1()[2] * other.group8()[0]) + (self.group0()[1] * other.group8()[3])),
                    ((self.group0()[2] * other.group8()[3]) + (self.group1()[0] * other.group8()[1])),
                    (-(self.group0()[0] * other.group8()[0]) - (self.group0()[1] * other.group8()[1])),
                ])),
            // e5
            ((self.group2()[2] * other.group8()[2]) + (self.group2()[1] * other.group8()[1]) + (self.group1()[3] * other.group8()[3]) + (self.group2()[0] * other.group8()[0])),
            // e15, e25, e35, e45
            (Simd32x4::from(other.group0()[1]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]])),
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other.group0()[1])),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            (Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group1()[2], self.group1()[1], self.group1()[0]])),
        );
    }
}
impl AntiWedge<Plane> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2])),
                    ((self.group1()[2] * other.group0()[0]) + (self.group0()[1] * other.group0()[3])),
                    ((self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[1])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
            // e5
            ((self.group2()[2] * other.group0()[2]) + (self.group2()[1] * other.group0()[1]) + (self.group1()[3] * other.group0()[3]) + (self.group2()[0] * other.group0()[0])),
        );
    }
}
impl AntiWedge<Sphere> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (-(Simd32x4::from(other[e4315]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                - (swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2])),
                    ((self.group1()[2] * other.group0()[0]) + (self.group0()[1] * other.group0()[3])),
                    ((self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[1])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
            // e5
            ((self.group2()[2] * other.group0()[2]) + (self.group2()[1] * other.group0()[1]) + (self.group1()[3] * other.group0()[3]) + (self.group2()[0] * other.group0()[0])),
        );
    }
}
impl AntiWedge<VersorEven> for Dipole {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       16        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       17        0
    //  no simd        9       20        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]),
                (self.group0()[1] * other.group0()[3]),
                (self.group0()[2] * other.group0()[3]),
                (-(self.group2()[2] * other.group0()[2])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])),
            ]),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(other.group0()[3])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] * other.group0()[3]),
                (self.group2()[1] * other.group0()[3]),
                (self.group2()[2] * other.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<VersorOdd> for Dipole {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (-(Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                - (swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group3()[3]) + (self.group1()[1] * other.group3()[2])),
                    ((self.group1()[2] * other.group3()[0]) + (self.group0()[1] * other.group3()[3])),
                    ((self.group0()[2] * other.group3()[3]) + (self.group1()[0] * other.group3()[1])),
                    (-(self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
                ])),
            // e5
            ((self.group2()[2] * other.group3()[2]) + (self.group2()[1] * other.group3()[1]) + (self.group1()[3] * other.group3()[3]) + (self.group2()[0] * other.group3()[0])),
        );
    }
}
impl InfixAntiWedge for DipoleInversion {}
impl AntiWedge<AntiScalar> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other[e12345])),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(other[e12345])),
            // e15, e25, e35, e1234
            (self.group2() * Simd32x4::from(other[e12345])),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(other[e12345])),
        );
    }
}
impl AntiWedge<Circle> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       26       37        0
    //  no simd       29       40        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-(self.group3()[2] * other.group0()[1]) + (self.group2()[3] * other.group1()[0]) + (self.group3()[1] * other.group0()[2])),
                ((self.group3()[2] * other.group0()[0]) + (self.group2()[3] * other.group1()[1]) - (self.group3()[0] * other.group0()[2])),
                (-(self.group3()[1] * other.group0()[0]) + (self.group2()[3] * other.group1()[2]) + (self.group3()[0] * other.group0()[1])),
                (-(self.group2()[2] * other.group0()[2])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])),
            ]),
            // e23, e31, e12, e45
            (-(swizzle!(self.group3(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group0()[0]) + (self.group2()[3] * other.group2()[0])),
                    ((self.group3()[3] * other.group0()[1]) + (self.group2()[3] * other.group2()[1])),
                    ((self.group3()[3] * other.group0()[2]) + (self.group2()[3] * other.group2()[2])),
                    (-(self.group3()[0] * other.group1()[0]) - (self.group3()[1] * other.group1()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group3()[3] * other.group1()[0]) - (self.group3()[1] * other.group2()[2]) + (self.group3()[2] * other.group2()[1])),
                ((self.group3()[3] * other.group1()[1]) + (self.group3()[0] * other.group2()[2]) - (self.group3()[2] * other.group2()[0])),
                ((self.group3()[3] * other.group1()[2]) - (self.group3()[0] * other.group2()[1]) + (self.group3()[1] * other.group2()[0])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<CircleRotor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       43        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       33       46        0
    //  no simd       39       55        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-(self.group3()[2] * other.group0()[1])
                    + (self.group3()[1] * other.group0()[2])
                    + (self.group0()[0] * other.group2()[3])
                    + (self.group2()[3] * other.group1()[0])),
                ((self.group3()[2] * other.group0()[0]) - (self.group3()[0] * other.group0()[2]) + (self.group0()[1] * other.group2()[3]) + (self.group2()[3] * other.group1()[1])),
                (-(self.group3()[1] * other.group0()[0])
                    + (self.group3()[0] * other.group0()[1])
                    + (self.group0()[2] * other.group2()[3])
                    + (self.group2()[3] * other.group1()[2])),
                (-(self.group2()[2] * other.group0()[2])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])),
            ]),
            // e23, e31, e12, e45
            (-(swizzle!(self.group3(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + (self.group1() * Simd32x4::from(other.group2()[3]))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group0()[0]) + (self.group2()[3] * other.group2()[0])),
                    ((self.group3()[3] * other.group0()[1]) + (self.group2()[3] * other.group2()[1])),
                    ((self.group3()[3] * other.group0()[2]) + (self.group2()[3] * other.group2()[2])),
                    (-(self.group3()[1] * other.group1()[1]) - (self.group3()[0] * other.group1()[0])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group3()[3] * other.group1()[0]) + (self.group3()[2] * other.group2()[1]) + (self.group2()[0] * other.group2()[3]) - (self.group3()[1] * other.group2()[2])),
                ((self.group3()[3] * other.group1()[1]) - (self.group3()[2] * other.group2()[0]) + (self.group2()[1] * other.group2()[3]) + (self.group3()[0] * other.group2()[2])),
                ((self.group3()[3] * other.group1()[2]) + (self.group3()[1] * other.group2()[0]) + (self.group2()[2] * other.group2()[3]) - (self.group3()[0] * other.group2()[1])),
                (self.group2()[3] * other.group2()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(other.group2()[3])),
        );
    }
}
impl AntiWedge<Dipole> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                + (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group3()[3] * other.group0()[0]) - (self.group3()[2] * other.group1()[1])),
                    (-(self.group3()[3] * other.group0()[1]) - (self.group3()[0] * other.group1()[2])),
                    (-(self.group3()[3] * other.group0()[2]) - (self.group3()[1] * other.group1()[0])),
                    ((self.group3()[1] * other.group0()[1]) + (self.group3()[0] * other.group0()[0])),
                ])),
            // e5
            (-(self.group3()[3] * other.group1()[3]) - (self.group3()[2] * other.group2()[2]) - (self.group3()[0] * other.group2()[0]) - (self.group3()[1] * other.group2()[1])),
        );
    }
}
impl AntiWedge<DipoleInversion> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       27       36        0
    //  no simd       48       60        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                ((self.group2()[3] * other.group3()[0]) - (self.group3()[0] * other.group2()[3])),
                ((self.group2()[3] * other.group3()[1]) - (self.group3()[1] * other.group2()[3])),
                ((self.group2()[3] * other.group3()[2]) - (self.group3()[2] * other.group2()[3])),
                0.0,
            ]),
            // e415, e425, e435, e321
            (-(swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[3]]))
                + (swizzle!(other.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[3]]))),
            // e235, e315, e125, e5
            ((swizzle!(other.group3(), 3, 3, 3, 2) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[2]]))
                - (Simd32x4::from(self.group3()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group1()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group3()[2] * other.group2()[2]) - (self.group3()[1] * other.group2()[1]) - (self.group3()[0] * other.group2()[0])
                        + (self.group2()[1] * other.group3()[1])
                        + (self.group1()[3] * other.group3()[3])
                        + (self.group2()[0] * other.group3()[0])),
                ])),
            // e1, e2, e3, e4
            ((swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                - (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                - (swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group3()[3] * other.group0()[0]) - (self.group3()[2] * other.group1()[1])
                        + (self.group0()[0] * other.group3()[3])
                        + (self.group1()[1] * other.group3()[2])),
                    (-(self.group3()[3] * other.group0()[1]) - (self.group3()[0] * other.group1()[2])
                        + (self.group1()[2] * other.group3()[0])
                        + (self.group0()[1] * other.group3()[3])),
                    (-(self.group3()[3] * other.group0()[2]) - (self.group3()[1] * other.group1()[0])
                        + (self.group0()[2] * other.group3()[3])
                        + (self.group1()[0] * other.group3()[1])),
                    ((self.group3()[1] * other.group0()[1]) + (self.group3()[0] * other.group0()[0])
                        - (self.group0()[0] * other.group3()[0])
                        - (self.group0()[1] * other.group3()[1])),
                ])),
        );
    }
}
impl AntiWedge<DualNum> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       16        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] * other.group0()[1]),
                (self.group0()[1] * other.group0()[1]),
                (self.group0()[2] * other.group0()[1]),
                (self.group2()[3] * other.group0()[0]),
            ]),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(other.group0()[1])),
            // e15, e25, e35, e1234
            (self.group2() * Simd32x4::from(other.group0()[1])),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(other.group0()[1])),
        );
    }
}
impl AntiWedge<FlatPoint> for DipoleInversion {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self.group2()[3]) * other.group0()),
            // e5
            (-(self.group3()[3] * other.group0()[3]) - (self.group3()[2] * other.group0()[2]) - (self.group3()[0] * other.group0()[0]) - (self.group3()[1] * other.group0()[1])),
        );
    }
}
impl AntiWedge<Flector> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       28        0
    //  no simd       28       40        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group2()[3] * other.group1()[0]),
                (self.group2()[3] * other.group1()[1]),
                (self.group2()[3] * other.group1()[2]),
                0.0,
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group3()[1] * other.group1()[2]) + (self.group3()[2] * other.group1()[1])),
                ((self.group3()[0] * other.group1()[2]) - (self.group3()[2] * other.group1()[0])),
                (-(self.group3()[0] * other.group1()[1]) + (self.group3()[1] * other.group1()[0])),
                (self.group2()[3] * other.group1()[3]),
            ]),
            // e235, e315, e125, e5
            ((swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[2]]))
                - (Simd32x4::from(self.group3()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group3()[2] * other.group0()[2]) - (self.group3()[1] * other.group0()[1]) - (self.group3()[0] * other.group0()[0])
                        + (self.group2()[1] * other.group1()[1])
                        + (self.group1()[3] * other.group1()[3])
                        + (self.group2()[0] * other.group1()[0])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group2()[3]) * other.group0())
                - (swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group1()[3]) + (self.group1()[1] * other.group1()[2])),
                    ((self.group1()[2] * other.group1()[0]) + (self.group0()[1] * other.group1()[3])),
                    ((self.group0()[2] * other.group1()[3]) + (self.group1()[0] * other.group1()[1])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl AntiWedge<Line> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       24        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group2()[3] * other.group0()[0]),
                (self.group2()[3] * other.group0()[1]),
                (self.group2()[3] * other.group0()[2]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group2()[3] * other.group1()[0]),
                (self.group2()[3] * other.group1()[1]),
                (self.group2()[3] * other.group1()[2]),
                (-(self.group3()[2] * other.group0()[2]) - (self.group3()[0] * other.group0()[0]) - (self.group3()[1] * other.group0()[1])),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group3()[3] * other.group0()[0]) - (self.group3()[1] * other.group1()[2]) + (self.group3()[2] * other.group1()[1])),
                ((self.group3()[3] * other.group0()[1]) + (self.group3()[0] * other.group1()[2]) - (self.group3()[2] * other.group1()[0])),
                ((self.group3()[3] * other.group0()[2]) - (self.group3()[0] * other.group1()[1]) + (self.group3()[1] * other.group1()[0])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<Motor> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       31        0
    //  no simd       24       40        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                + Simd32x4::from([
                    (self.group0()[0] * other.group0()[3]),
                    (self.group0()[1] * other.group0()[3]),
                    (self.group0()[2] * other.group0()[3]),
                    (-(self.group1()[2] * other.group0()[2])
                        - (self.group1()[1] * other.group0()[1])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group0()[2] * other.group1()[2])
                        - (self.group0()[0] * other.group1()[0])
                        - (self.group0()[1] * other.group1()[1])),
                ])),
            // e23, e31, e12, e45
            ((self.group1() * Simd32x4::from(other.group0()[3]))
                + Simd32x4::from([
                    (self.group2()[3] * other.group1()[0]),
                    (self.group2()[3] * other.group1()[1]),
                    (self.group2()[3] * other.group1()[2]),
                    (-(self.group3()[2] * other.group0()[2]) - (self.group3()[1] * other.group0()[1]) - (self.group3()[0] * other.group0()[0])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group3()[3] * other.group0()[0]) + (self.group3()[2] * other.group1()[1]) + (self.group2()[0] * other.group0()[3]) - (self.group3()[1] * other.group1()[2])),
                ((self.group3()[3] * other.group0()[1]) - (self.group3()[2] * other.group1()[0]) + (self.group2()[1] * other.group0()[3]) + (self.group3()[0] * other.group1()[2])),
                ((self.group3()[3] * other.group0()[2]) + (self.group3()[1] * other.group1()[0]) + (self.group2()[2] * other.group0()[3]) - (self.group3()[0] * other.group1()[1])),
                (self.group2()[3] * other.group0()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(other.group0()[3])),
        );
    }
}
impl AntiWedge<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       48        0
    //    simd3        8       12        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       52       69        0
    //  no simd       89      120        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                ((self.group3()[3] * other.group1()[3])
                    + (self.group3()[2] * other.group1()[2])
                    + (self.group3()[1] * other.group1()[1])
                    + (self.group3()[0] * other.group1()[0])
                    + (self.group2()[3] * other[e1])
                    - (self.group2()[2] * other.group6()[2])
                    - (self.group2()[1] * other.group6()[1])
                    - (self.group2()[0] * other.group6()[0])
                    - (self.group1()[3] * other.group5()[3])
                    - (self.group1()[2] * other.group5()[2])
                    - (self.group1()[1] * other.group5()[1])
                    - (self.group1()[0] * other.group5()[0])
                    - (self.group0()[2] * other.group7()[2])
                    - (self.group0()[0] * other.group7()[0])
                    - (self.group0()[1] * other.group7()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            ((swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group10()[0], other.group10()[2], other.group10()[1], other.group4()[2]]))
                + (Simd32x4::from(self.group2()[3]) * other.group3())
                - (Simd32x4::from(other[e35]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                - (swizzle!(other.group8(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group3()[3] * other.group4()[0]) - (self.group3()[2] * other.group10()[1])
                        + (self.group0()[0] * other.group8()[3])
                        + (self.group1()[1] * other.group8()[2])),
                    (-(self.group3()[3] * other.group4()[1]) - (self.group3()[0] * other.group10()[0])
                        + (self.group1()[2] * other.group8()[0])
                        + (self.group0()[1] * other.group8()[3])),
                    (-(self.group3()[3] * other.group4()[2]) - (self.group3()[1] * other.group10()[2])
                        + (self.group0()[2] * other.group8()[3])
                        + (self.group1()[0] * other.group8()[1])),
                    ((self.group3()[1] * other.group4()[1]) + (self.group3()[0] * other.group4()[0])
                        - (self.group0()[0] * other.group8()[0])
                        - (self.group0()[1] * other.group8()[1])),
                ])),
            // e5
            (-(self.group3()[3] * other.group3()[3]) - (self.group3()[2] * other.group3()[2]) - (self.group3()[1] * other.group3()[1]) - (self.group3()[0] * other.group3()[0])
                + (self.group2()[2] * other.group8()[2])
                + (self.group2()[1] * other.group8()[1])
                + (self.group1()[3] * other.group8()[3])
                + (self.group2()[0] * other.group8()[0])),
            // e15, e25, e35, e45
            ((Simd32x4::from(other.group0()[1]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                - (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group7()[2], other.group7()[0], other.group7()[1], other.group5()[2]]))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group5()[0]) + (self.group3()[2] * other.group7()[1])),
                    ((self.group3()[3] * other.group5()[1]) + (self.group3()[0] * other.group7()[2])),
                    ((self.group3()[3] * other.group5()[2]) + (self.group3()[1] * other.group7()[0])),
                    (-(self.group3()[1] * other.group5()[1]) - (self.group3()[0] * other.group5()[0])),
                ])),
            // e41, e42, e43
            (-(swizzle!(other.group6(), 1, 2, 0) * Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]))
                + (swizzle!(other.group6(), 2, 0, 1) * Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[0]]))
                + (self.group0() * Simd32x3::from(other.group0()[1]))
                + (Simd32x3::from(self.group2()[3]) * Simd32x3::from([other.group5()[0], other.group5()[1], other.group5()[2]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([other.group8()[2], other.group8()[0], other.group8()[1], other[e35]]))
                + (swizzle!(other.group8(), 1, 2, 0, 3) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[3]]))),
            // e423, e431, e412
            ((Simd32x3::from(self.group2()[3]) * Simd32x3::from([other.group8()[0], other.group8()[1], other.group8()[2]]))
                - (Simd32x3::from(other[e35]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]))),
            // e235, e315, e125
            ((Simd32x3::from(other.group8()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]))
                - (Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group8()[0], other.group8()[1], other.group8()[2]]))),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(other.group0()[1])),
            // e1234
            (self.group2()[3] * other.group0()[1]),
            // e12, e31, e23
            ((Simd32x3::from(self.group3()[3]) * swizzle!(other.group6(), 2, 1, 0))
                - (Simd32x3::from(other.group5()[3]) * Simd32x3::from([self.group3()[2], self.group3()[1], self.group3()[0]]))
                + (Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group1()[2], self.group1()[1], self.group1()[0]]))
                + (Simd32x3::from(self.group2()[3]) * swizzle!(other.group7(), 2, 1, 0))),
        );
    }
}
impl AntiWedge<Plane> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       27        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       17       35        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group2()[3] * other.group0()[0]),
                (self.group2()[3] * other.group0()[1]),
                (self.group2()[3] * other.group0()[2]),
                0.0,
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group3()[1] * other.group0()[2]) + (self.group3()[2] * other.group0()[1])),
                ((self.group3()[0] * other.group0()[2]) - (self.group3()[2] * other.group0()[0])),
                (-(self.group3()[0] * other.group0()[1]) + (self.group3()[1] * other.group0()[0])),
                (self.group2()[3] * other.group0()[3]),
            ]),
            // e235, e315, e125, e5
            ((swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[2]]))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group0()[0]) * -1.0),
                    ((self.group3()[3] * other.group0()[1]) * -1.0),
                    ((self.group3()[3] * other.group0()[2]) * -1.0),
                    ((self.group2()[1] * other.group0()[1]) + (self.group1()[3] * other.group0()[3]) + (self.group2()[0] * other.group0()[0])),
                ])),
            // e1, e2, e3, e4
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2])),
                    ((self.group1()[2] * other.group0()[0]) + (self.group0()[1] * other.group0()[3])),
                    ((self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[1])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<RoundPoint> for DipoleInversion {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            ((self.group3()[3] * other.group0()[3])
                + (self.group3()[2] * other.group0()[2])
                + (self.group3()[1] * other.group0()[1])
                + (self.group2()[3] * other[e2])
                + (self.group3()[0] * other.group0()[0])),
        );
    }
}
impl AntiWedge<Sphere> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       23        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       13       28        0
    //  no simd       25       43        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                ((self.group2()[3] * other.group0()[0]) - (self.group3()[0] * other[e4315])),
                ((self.group2()[3] * other.group0()[1]) - (self.group3()[1] * other[e4315])),
                ((self.group2()[3] * other.group0()[2]) - (self.group3()[2] * other[e4315])),
                0.0,
            ]),
            // e415, e425, e435, e321
            (-(swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other[e4315]]))
                + (swizzle!(other.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[3]]))),
            // e235, e315, e125, e5
            ((swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[2]]))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group0()[0]) * -1.0),
                    ((self.group3()[3] * other.group0()[1]) * -1.0),
                    ((self.group3()[3] * other.group0()[2]) * -1.0),
                    ((self.group2()[1] * other.group0()[1]) + (self.group1()[3] * other.group0()[3]) + (self.group2()[0] * other.group0()[0])),
                ])),
            // e1, e2, e3, e4
            (-(Simd32x4::from(other[e4315]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                - (swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2])),
                    ((self.group1()[2] * other.group0()[0]) + (self.group0()[1] * other.group0()[3])),
                    ((self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[1])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<VersorEven> for DipoleInversion {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       29       42        0
    //  no simd       44       60        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[2]]))
                + (swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group3()[3]]))
                + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]))
                + Simd32x4::from([
                    (self.group0()[0] * other.group0()[3]),
                    (self.group0()[1] * other.group0()[3]),
                    (self.group0()[2] * other.group0()[3]),
                    ((self.group3()[2] * other.group3()[2]) + (self.group3()[1] * other.group3()[1]) + (self.group3()[0] * other.group3()[0])
                        - (self.group2()[1] * other.group0()[1])
                        - (self.group2()[0] * other.group0()[0])
                        - (self.group1()[3] * other.group1()[3])
                        - (self.group1()[2] * other.group1()[2])
                        - (self.group1()[1] * other.group1()[1])
                        - (self.group1()[0] * other.group1()[0])
                        - (self.group0()[2] * other.group2()[2])
                        - (self.group0()[0] * other.group2()[0])
                        - (self.group0()[1] * other.group2()[1])),
                ])),
            // e23, e31, e12, e45
            ((other.group0() * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group1()[3]]))
                - (swizzle!(self.group3(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group0()[3]) + (self.group2()[3] * other.group2()[0])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group2()[3] * other.group2()[1])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group2()[3] * other.group2()[2])),
                    (-(self.group3()[1] * other.group1()[1]) - (self.group3()[0] * other.group1()[0])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group3()[3] * other.group1()[0]) + (self.group3()[2] * other.group2()[1]) + (self.group2()[0] * other.group0()[3]) - (self.group3()[1] * other.group2()[2])),
                ((self.group3()[3] * other.group1()[1]) - (self.group3()[2] * other.group2()[0]) + (self.group2()[1] * other.group0()[3]) + (self.group3()[0] * other.group2()[2])),
                ((self.group3()[3] * other.group1()[2]) + (self.group3()[1] * other.group2()[0]) + (self.group2()[2] * other.group0()[3]) - (self.group3()[0] * other.group2()[1])),
                (self.group2()[3] * other.group0()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(other.group0()[3])),
        );
    }
}
impl AntiWedge<VersorOdd> for DipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       27       36        0
    //  no simd       48       60        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                ((self.group2()[3] * other.group3()[0]) - (self.group3()[0] * other.group2()[3])),
                ((self.group2()[3] * other.group3()[1]) - (self.group3()[1] * other.group2()[3])),
                ((self.group2()[3] * other.group3()[2]) - (self.group3()[2] * other.group2()[3])),
                0.0,
            ]),
            // e415, e425, e435, e321
            (-(swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[3]]))
                + (swizzle!(other.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[3]]))),
            // e235, e315, e125, e5
            ((swizzle!(other.group3(), 3, 3, 3, 2) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[2]]))
                - (Simd32x4::from(self.group3()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group1()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group3()[2] * other.group2()[2]) - (self.group3()[1] * other.group2()[1]) - (self.group3()[0] * other.group2()[0])
                        + (self.group2()[1] * other.group3()[1])
                        + (self.group1()[3] * other.group3()[3])
                        + (self.group2()[0] * other.group3()[0])),
                ])),
            // e1, e2, e3, e4
            ((swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                - (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                - (swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group3()[3] * other.group0()[0]) - (self.group3()[2] * other.group1()[1])
                        + (self.group0()[0] * other.group3()[3])
                        + (self.group1()[1] * other.group3()[2])),
                    (-(self.group3()[3] * other.group0()[1]) - (self.group3()[0] * other.group1()[2])
                        + (self.group1()[2] * other.group3()[0])
                        + (self.group0()[1] * other.group3()[3])),
                    (-(self.group3()[3] * other.group0()[2]) - (self.group3()[1] * other.group1()[0])
                        + (self.group0()[2] * other.group3()[3])
                        + (self.group1()[0] * other.group3()[1])),
                    ((self.group3()[1] * other.group0()[1]) + (self.group3()[0] * other.group0()[0])
                        - (self.group0()[0] * other.group3()[0])
                        - (self.group0()[1] * other.group3()[1])),
                ])),
        );
    }
}
impl InfixAntiWedge for DualNum {}
impl AntiWedge<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* e5, e12345 */ (self.group0() * Simd32x2::from(other[e12345])));
    }
}
impl AntiWedge<Circle> for DualNum {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self.group0()[1]) * other.group0()),
            // e415, e425, e435, e321
            (Simd32x4::from(self.group0()[1]) * other.group1()),
            // e235, e315, e125
            (Simd32x3::from(self.group0()[1]) * other.group2()),
        );
    }
}
impl AntiWedge<CircleRotor> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]])),
            // e415, e425, e435, e321
            (Simd32x4::from(self.group0()[1]) * other.group1()),
            // e235, e315, e125, e5
            (other.group2() * Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]])),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<Dipole> for DualNum {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self.group0()[1]) * other.group0()),
            // e23, e31, e12, e45
            (Simd32x4::from(self.group0()[1]) * other.group1()),
            // e15, e25, e35
            (Simd32x3::from(self.group0()[1]) * other.group2()),
        );
    }
}
impl AntiWedge<DipoleInversion> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       16        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[1] * other.group0()[0]),
                (self.group0()[1] * other.group0()[1]),
                (self.group0()[1] * other.group0()[2]),
                (self.group0()[0] * other.group2()[3]),
            ]),
            // e23, e31, e12, e45
            (Simd32x4::from(self.group0()[1]) * other.group1()),
            // e15, e25, e35, e1234
            (Simd32x4::from(self.group0()[1]) * other.group2()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[1]) * other.group3()),
        );
    }
}
impl AntiWedge<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            ((self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0])),
            (self.group0()[1] * other.group0()[1]),
        ]));
    }
}
impl AntiWedge<FlatPoint> for DualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ (Simd32x4::from(self.group0()[1]) * other.group0()));
    }
}
impl AntiWedge<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from(self.group0()[1]) * other.group0()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[1]) * other.group1()),
        );
    }
}
impl AntiWedge<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(self.group0()[1]) * other.group0()),
            // e235, e315, e125
            (Simd32x3::from(self.group0()[1]) * other.group1()),
        );
    }
}
impl AntiWedge<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(self.group0()[1]) * other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[1] * other.group1()[0]),
                (self.group0()[1] * other.group1()[1]),
                (self.group0()[1] * other.group1()[2]),
                ((self.group0()[0] * other.group0()[3]) + (self.group0()[1] * other.group1()[3])),
            ]),
        );
    }
}
impl AntiWedge<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       14        0
    //  no simd        2       34        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([((self.group0()[0] * other[e35]) + (self.group0()[1] * other.group0()[0])), (self.group0()[1] * other.group0()[1])]),
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[1]) * other.group1()),
            // e5
            ((self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other[e1])),
            // e15, e25, e35, e45
            (Simd32x4::from(self.group0()[1]) * other.group3()),
            // e41, e42, e43
            (Simd32x3::from(self.group0()[1]) * other.group4()),
            // e415, e425, e435, e321
            (Simd32x4::from(self.group0()[1]) * other.group5()),
            // e423, e431, e412
            (Simd32x3::from(self.group0()[1]) * other.group6()),
            // e235, e315, e125
            (Simd32x3::from(self.group0()[1]) * other.group7()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[1]) * other.group8()),
            // e1234
            (self.group0()[1] * other[e35]),
            // e12, e31, e23
            (Simd32x3::from(self.group0()[1]) * other.group10()),
        );
    }
}
impl AntiWedge<Plane> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (Simd32x4::from(self.group0()[1]) * other.group0()));
    }
}
impl AntiWedge<RoundPoint> for DualNum {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[1]) * other.group0()),
            // e5
            (self.group0()[1] * other[e2]),
        );
    }
}
impl AntiWedge<Scalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self.group0()[1] * other[scalar]));
    }
}
impl AntiWedge<Sphere> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[0] * other[e4315])]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[1] * other[e4315])]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[1]) * other.group0()),
        );
    }
}
impl AntiWedge<VersorEven> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from(self.group0()[1]) * other.group0()),
            // e415, e425, e435, e321
            (Simd32x4::from(self.group0()[1]) * other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[1] * other.group2()[0]),
                (self.group0()[1] * other.group2()[1]),
                (self.group0()[1] * other.group2()[2]),
                ((self.group0()[0] * other.group0()[3]) + (self.group0()[1] * other.group2()[3])),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[1]) * other.group3()),
        );
    }
}
impl AntiWedge<VersorOdd> for DualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[1] * other.group0()[0]),
                (self.group0()[1] * other.group0()[1]),
                (self.group0()[1] * other.group0()[2]),
                ((self.group0()[0] * other.group2()[3]) + (self.group0()[1] * other.group0()[3])),
            ]),
            // e23, e31, e12, e45
            (Simd32x4::from(self.group0()[1]) * other.group1()),
            // e15, e25, e35, e1234
            (Simd32x4::from(self.group0()[1]) * other.group2()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[1]) * other.group3()),
        );
    }
}
impl InfixAntiWedge for FlatPoint {}
impl AntiWedge<AntiScalar> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * Simd32x4::from(other[e12345])));
    }
}
impl AntiWedge<Circle> for FlatPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        return Scalar::from_groups(
            // scalar
            (-(self.group0()[3] * other.group1()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl AntiWedge<CircleRotor> for FlatPoint {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(self.group0()[3] * other.group1()[3])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group0()[0] * other.group0()[0])
                    - (self.group0()[1] * other.group0()[1])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * other.group2()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group0()[0] * other.group2()[3]),
                (self.group0()[1] * other.group2()[3]),
                (self.group0()[2] * other.group2()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<DipoleInversion> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group2()[3]) * Simd32x4::from(-1.0)),
            // e5
            ((self.group0()[3] * other.group3()[3]) + (self.group0()[2] * other.group3()[2]) + (self.group0()[0] * other.group3()[0]) + (self.group0()[1] * other.group3()[1])),
        );
    }
}
impl AntiWedge<DualNum> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * Simd32x4::from(other.group0()[1])));
    }
}
impl AntiWedge<Flector> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            ((self.group0()[3] * other.group1()[3]) + (self.group0()[2] * other.group1()[2]) + (self.group0()[0] * other.group1()[0]) + (self.group0()[1] * other.group1()[1])),
            0.0,
        ]));
    }
}
impl AntiWedge<Motor> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * Simd32x4::from(other.group0()[3])));
    }
}
impl AntiWedge<MultiVector> for FlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        6       20        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group0()[3] * other.group5()[3])
                    - (self.group0()[2] * other.group6()[2])
                    - (self.group0()[0] * other.group6()[0])
                    - (self.group0()[1] * other.group6()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other[e35]) * Simd32x4::from(-1.0)),
            // e5
            ((self.group0()[3] * other.group8()[3]) + (self.group0()[2] * other.group8()[2]) + (self.group0()[0] * other.group8()[0]) + (self.group0()[1] * other.group8()[1])),
            // e15, e25, e35, e45
            (self.group0() * Simd32x4::from(other.group0()[1])),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl AntiWedge<Plane> for FlatPoint {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            ((self.group0()[3] * other.group0()[3]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
            0.0,
        ]));
    }
}
impl AntiWedge<Sphere> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other[e4315]) * Simd32x4::from(-1.0)),
            // e5
            ((self.group0()[3] * other.group0()[3]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl AntiWedge<VersorEven> for FlatPoint {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(self.group0()[3] * other.group1()[3])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group0()[0] * other.group0()[0])
                    - (self.group0()[1] * other.group0()[1])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]),
                (self.group0()[1] * other.group0()[3]),
                (self.group0()[2] * other.group0()[3]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<VersorOdd> for FlatPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd        3       12        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group2()[3]) * Simd32x4::from(-1.0)),
            // e5
            ((self.group0()[3] * other.group3()[3]) + (self.group0()[2] * other.group3()[2]) + (self.group0()[0] * other.group3()[0]) + (self.group0()[1] * other.group3()[1])),
        );
    }
}
impl InfixAntiWedge for Flector {}
impl AntiWedge<AntiScalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            (self.group0() * Simd32x4::from(other[e12345])),
            // e4235, e4315, e4125, e3215
            (self.group1() * Simd32x4::from(other[e12345])),
        );
    }
}
impl AntiWedge<Circle> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       14       25        0
    //  no simd       17       28        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                ((self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1])),
                (-(self.group1()[0] * other.group0()[2]) + (self.group1()[2] * other.group0()[0])),
                ((self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0])),
                (-(self.group0()[3] * other.group1()[3])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group0()[0] * other.group0()[0])
                    - (self.group0()[1] * other.group0()[1])),
            ]),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    (self.group1()[3] * other.group0()[0]),
                    (self.group1()[3] * other.group0()[1]),
                    (self.group1()[3] * other.group0()[2]),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group1()[3] * other.group1()[0]) - (self.group1()[1] * other.group2()[2]) + (self.group1()[2] * other.group2()[1])),
                ((self.group1()[3] * other.group1()[1]) + (self.group1()[0] * other.group2()[2]) - (self.group1()[2] * other.group2()[0])),
                ((self.group1()[3] * other.group1()[2]) - (self.group1()[0] * other.group2()[1]) + (self.group1()[1] * other.group2()[0])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<CircleRotor> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       21       36        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                ((self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1])),
                (-(self.group1()[0] * other.group0()[2]) + (self.group1()[2] * other.group0()[0])),
                ((self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0])),
                (-(self.group0()[3] * other.group1()[3])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group0()[0] * other.group0()[0])
                    - (self.group0()[1] * other.group0()[1])),
            ]),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    (self.group1()[3] * other.group0()[0]),
                    (self.group1()[3] * other.group0()[1]),
                    (self.group1()[3] * other.group0()[2]),
                    (-(self.group1()[1] * other.group1()[1]) + (self.group0()[3] * other.group2()[3]) - (self.group1()[0] * other.group1()[0])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group1()[3] * other.group1()[0]) + (self.group1()[2] * other.group2()[1]) + (self.group0()[0] * other.group2()[3]) - (self.group1()[1] * other.group2()[2])),
                ((self.group1()[3] * other.group1()[1]) - (self.group1()[2] * other.group2()[0]) + (self.group0()[1] * other.group2()[3]) + (self.group1()[0] * other.group2()[2])),
                ((self.group1()[3] * other.group1()[2]) + (self.group1()[1] * other.group2()[0]) + (self.group0()[2] * other.group2()[3]) - (self.group1()[0] * other.group2()[1])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            (self.group1() * Simd32x4::from(other.group2()[3])),
        );
    }
}
impl AntiWedge<Dipole> for Flector {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[3] * other.group0()[0]) - (self.group1()[2] * other.group1()[1])),
                    (-(self.group1()[3] * other.group0()[1]) - (self.group1()[0] * other.group1()[2])),
                    (-(self.group1()[3] * other.group0()[2]) - (self.group1()[1] * other.group1()[0])),
                    ((self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1])),
                ])),
            // e5
            (-(self.group1()[3] * other.group1()[3]) - (self.group1()[2] * other.group2()[2]) - (self.group1()[0] * other.group2()[0]) - (self.group1()[1] * other.group2()[1])),
        );
    }
}
impl AntiWedge<DipoleInversion> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       28        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       32        0
    //  no simd       28       44        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group1()[0] * other.group2()[3] * -1.0),
                (self.group1()[1] * other.group2()[3] * -1.0),
                (self.group1()[2] * other.group2()[3] * -1.0),
                0.0,
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group1()[1] * other.group3()[2]) + (self.group1()[2] * other.group3()[1])),
                ((self.group1()[0] * other.group3()[2]) - (self.group1()[2] * other.group3()[0])),
                (-(self.group1()[0] * other.group3()[1]) + (self.group1()[1] * other.group3()[0])),
                (self.group1()[3] * other.group2()[3] * -1.0),
            ]),
            // e235, e315, e125, e5
            ((Simd32x4::from(other.group3()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]))
                - (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group1()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group1()[2] * other.group2()[2]) - (self.group1()[1] * other.group2()[1]) - (self.group1()[0] * other.group2()[0])
                        + (self.group0()[2] * other.group3()[2])
                        + (self.group0()[0] * other.group3()[0])
                        + (self.group0()[1] * other.group3()[1])),
                ])),
            // e1, e2, e3, e4
            (-(self.group0() * Simd32x4::from(other.group2()[3]))
                + (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[3] * other.group0()[0]) - (self.group1()[2] * other.group1()[1])),
                    (-(self.group1()[3] * other.group0()[1]) - (self.group1()[0] * other.group1()[2])),
                    (-(self.group1()[3] * other.group0()[2]) - (self.group1()[1] * other.group1()[0])),
                    ((self.group1()[1] * other.group0()[1]) + (self.group1()[0] * other.group0()[0])),
                ])),
        );
    }
}
impl AntiWedge<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (self.group0() * Simd32x4::from(other.group0()[1])),
            // e4235, e4315, e4125, e3215
            (self.group1() * Simd32x4::from(other.group0()[1])),
        );
    }
}
impl AntiWedge<FlatPoint> for Flector {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (-(self.group1()[3] * other.group0()[3]) - (self.group1()[2] * other.group0()[2]) - (self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
            0.0,
        ]));
    }
}
impl AntiWedge<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       16       20        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (-(self.group1()[1] * other.group1()[2]) + (self.group1()[2] * other.group1()[1])),
                ((self.group1()[0] * other.group1()[2]) - (self.group1()[2] * other.group1()[0])),
                (-(self.group1()[0] * other.group1()[1]) + (self.group1()[1] * other.group1()[0])),
                0.0,
            ]),
            // e235, e315, e125, e5
            ((Simd32x4::from(other.group1()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]))
                - (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group1()[2] * other.group0()[2]) - (self.group1()[1] * other.group0()[1]) - (self.group1()[0] * other.group0()[0])
                        + (self.group0()[2] * other.group1()[2])
                        + (self.group0()[0] * other.group1()[0])
                        + (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl AntiWedge<Line> for Flector {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            (-(swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group0()[0]) + (self.group1()[2] * other.group1()[1])),
                    ((self.group1()[3] * other.group0()[1]) + (self.group1()[0] * other.group1()[2])),
                    ((self.group1()[3] * other.group0()[2]) + (self.group1()[1] * other.group1()[0])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            ((other.group0() * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group0()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    ((self.group1()[2] * other.group1()[1]) + (self.group0()[0] * other.group0()[3])),
                    ((self.group0()[1] * other.group0()[3]) + (self.group1()[0] * other.group1()[2])),
                    ((self.group1()[1] * other.group1()[0]) + (self.group0()[2] * other.group0()[3])),
                    (-(self.group1()[1] * other.group0()[1]) - (self.group1()[0] * other.group0()[0])),
                ])),
            // e4235, e4315, e4125, e3215
            (self.group1() * Simd32x4::from(other.group0()[3])),
        );
    }
}
impl AntiWedge<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       40        0
    //    simd3        3        8        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       32       53        0
    //  no simd       50       84        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                ((self.group1()[3] * other.group1()[3]) + (self.group1()[2] * other.group1()[2]) + (self.group1()[1] * other.group1()[1]) + (self.group1()[0] * other.group1()[0])
                    - (self.group0()[3] * other.group5()[3])
                    - (self.group0()[2] * other.group6()[2])
                    - (self.group0()[0] * other.group6()[0])
                    - (self.group0()[1] * other.group6()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            (-(self.group0() * Simd32x4::from(other[e35]))
                + (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group10()[0], other.group10()[2], other.group10()[1], other.group4()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[3] * other.group4()[0]) - (self.group1()[2] * other.group10()[1])),
                    (-(self.group1()[3] * other.group4()[1]) - (self.group1()[0] * other.group10()[0])),
                    (-(self.group1()[3] * other.group4()[2]) - (self.group1()[1] * other.group10()[2])),
                    ((self.group1()[1] * other.group4()[1]) + (self.group1()[0] * other.group4()[0])),
                ])),
            // e5
            (-(self.group1()[3] * other.group3()[3]) - (self.group1()[2] * other.group3()[2]) - (self.group1()[1] * other.group3()[1]) - (self.group1()[0] * other.group3()[0])
                + (self.group0()[3] * other.group8()[3])
                + (self.group0()[2] * other.group8()[2])
                + (self.group0()[0] * other.group8()[0])
                + (self.group0()[1] * other.group8()[1])),
            // e15, e25, e35, e45
            ((self.group0() * Simd32x4::from(other.group0()[1]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group7()[2], other.group7()[0], other.group7()[1], other.group5()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group5()[0]) + (self.group1()[2] * other.group7()[1])),
                    ((self.group1()[3] * other.group5()[1]) + (self.group1()[0] * other.group7()[2])),
                    ((self.group1()[3] * other.group5()[2]) + (self.group1()[1] * other.group7()[0])),
                    (-(self.group1()[1] * other.group5()[1]) - (self.group1()[0] * other.group5()[0])),
                ])),
            // e41, e42, e43
            ((swizzle!(other.group6(), 2, 0, 1) * Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]))
                - (swizzle!(other.group6(), 1, 2, 0) * Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]))),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group1()[1] * other.group8()[2]) + (self.group1()[2] * other.group8()[1])),
                ((self.group1()[0] * other.group8()[2]) - (self.group1()[2] * other.group8()[0])),
                (-(self.group1()[0] * other.group8()[1]) + (self.group1()[1] * other.group8()[0])),
                (self.group1()[3] * other[e35] * -1.0),
            ]),
            // e423, e431, e412
            (Simd32x3::from(other[e35]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            ((Simd32x3::from(other.group8()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group8()[0], other.group8()[1], other.group8()[2]]))),
            // e4235, e4315, e4125, e3215
            (self.group1() * Simd32x4::from(other.group0()[1])),
            // e1234
            0.0,
            // e12, e31, e23
            (-(Simd32x3::from(other.group5()[3]) * Simd32x3::from([self.group1()[2], self.group1()[1], self.group1()[0]]))
                + (Simd32x3::from(self.group1()[3]) * swizzle!(other.group6(), 2, 1, 0))),
        );
    }
}
impl AntiWedge<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       16        0
    //  no simd        9       19        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (-(self.group1()[1] * other.group0()[2]) + (self.group1()[2] * other.group0()[1])),
                ((self.group1()[0] * other.group0()[2]) - (self.group1()[2] * other.group0()[0])),
                (-(self.group1()[0] * other.group0()[1]) + (self.group1()[1] * other.group0()[0])),
                0.0,
            ]),
            // e235, e315, e125, e5
            ((Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group0()[0]) * -1.0),
                    ((self.group1()[3] * other.group0()[1]) * -1.0),
                    ((self.group1()[3] * other.group0()[2]) * -1.0),
                    ((self.group0()[2] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<RoundPoint> for Flector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        return Scalar::from_groups(
            // scalar
            ((self.group1()[3] * other.group0()[3]) + (self.group1()[2] * other.group0()[2]) + (self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1])),
        );
    }
}
impl AntiWedge<Sphere> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       23        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        6       26        0
    //  no simd        9       35        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group1()[0] * other[e4315] * -1.0),
                (self.group1()[1] * other[e4315] * -1.0),
                (self.group1()[2] * other[e4315] * -1.0),
                0.0,
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group1()[1] * other.group0()[2]) + (self.group1()[2] * other.group0()[1])),
                ((self.group1()[0] * other.group0()[2]) - (self.group1()[2] * other.group0()[0])),
                (-(self.group1()[0] * other.group0()[1]) + (self.group1()[1] * other.group0()[0])),
                (self.group1()[3] * other[e4315] * -1.0),
            ]),
            // e235, e315, e125, e5
            ((Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group0()[0]) * -1.0),
                    ((self.group1()[3] * other.group0()[1]) * -1.0),
                    ((self.group1()[3] * other.group0()[2]) * -1.0),
                    ((self.group0()[2] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other[e4315]) * Simd32x4::from(-1.0)),
        );
    }
}
impl AntiWedge<VersorEven> for Flector {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       20        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       19       25        0
    //  no simd       31       40        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((swizzle!(self.group1(), 1, 2, 0, 3) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group3()[3]]))
                - (swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    ((self.group1()[2] * other.group3()[2]) + (self.group1()[1] * other.group3()[1]) + (self.group1()[0] * other.group3()[0])
                        - (self.group0()[3] * other.group1()[3])
                        - (self.group0()[0] * other.group0()[0])
                        - (self.group0()[1] * other.group0()[1])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + (other.group0() * Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group0()[3]]))
                + Simd32x4::from([0.0, 0.0, 0.0, (-(self.group1()[1] * other.group1()[1]) - (self.group1()[0] * other.group1()[0]))])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group1()[3] * other.group1()[0]) + (self.group1()[2] * other.group2()[1]) + (self.group0()[0] * other.group0()[3]) - (self.group1()[1] * other.group2()[2])),
                ((self.group1()[3] * other.group1()[1]) - (self.group1()[2] * other.group2()[0]) + (self.group0()[1] * other.group0()[3]) + (self.group1()[0] * other.group2()[2])),
                ((self.group1()[3] * other.group1()[2]) + (self.group1()[1] * other.group2()[0]) + (self.group0()[2] * other.group0()[3]) - (self.group1()[0] * other.group2()[1])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            (self.group1() * Simd32x4::from(other.group0()[3])),
        );
    }
}
impl AntiWedge<VersorOdd> for Flector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       28        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       32        0
    //  no simd       28       44        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group1()[0] * other.group2()[3] * -1.0),
                (self.group1()[1] * other.group2()[3] * -1.0),
                (self.group1()[2] * other.group2()[3] * -1.0),
                0.0,
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group1()[1] * other.group3()[2]) + (self.group1()[2] * other.group3()[1])),
                ((self.group1()[0] * other.group3()[2]) - (self.group1()[2] * other.group3()[0])),
                (-(self.group1()[0] * other.group3()[1]) + (self.group1()[1] * other.group3()[0])),
                (self.group1()[3] * other.group2()[3] * -1.0),
            ]),
            // e235, e315, e125, e5
            ((Simd32x4::from(other.group3()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]))
                - (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group1()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group1()[2] * other.group2()[2]) - (self.group1()[1] * other.group2()[1]) - (self.group1()[0] * other.group2()[0])
                        + (self.group0()[2] * other.group3()[2])
                        + (self.group0()[0] * other.group3()[0])
                        + (self.group0()[1] * other.group3()[1])),
                ])),
            // e1, e2, e3, e4
            (-(self.group0() * Simd32x4::from(other.group2()[3]))
                + (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[3] * other.group0()[0]) - (self.group1()[2] * other.group1()[1])),
                    (-(self.group1()[3] * other.group0()[1]) - (self.group1()[0] * other.group1()[2])),
                    (-(self.group1()[3] * other.group0()[2]) - (self.group1()[1] * other.group1()[0])),
                    ((self.group1()[1] * other.group0()[1]) + (self.group1()[0] * other.group0()[0])),
                ])),
        );
    }
}
impl InfixAntiWedge for Line {}
impl AntiWedge<AntiScalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e415, e425, e435
            (self.group0() * Simd32x3::from(other[e12345])),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(other[e12345])),
        );
    }
}
impl AntiWedge<Circle> for Line {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       18        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(self.group1()[2] * other.group0()[1]) + (self.group0()[0] * other.group1()[3]) + (self.group1()[1] * other.group0()[2])),
                ((self.group1()[2] * other.group0()[0]) + (self.group0()[1] * other.group1()[3]) - (self.group1()[0] * other.group0()[2])),
                (-(self.group1()[1] * other.group0()[0]) + (self.group0()[2] * other.group1()[3]) + (self.group1()[0] * other.group0()[1])),
                (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
            ]),
            // e5
            (-(self.group1()[2] * other.group1()[2])
                - (self.group1()[1] * other.group1()[1])
                - (self.group1()[0] * other.group1()[0])
                - (self.group0()[2] * other.group2()[2])
                - (self.group0()[0] * other.group2()[0])
                - (self.group0()[1] * other.group2()[1])),
        );
    }
}
impl AntiWedge<CircleRotor> for Line {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       24        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group2()[3]),
                (self.group0()[1] * other.group2()[3]),
                (self.group0()[2] * other.group2()[3]),
                0.0,
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] * other.group2()[3]),
                (self.group1()[1] * other.group2()[3]),
                (self.group1()[2] * other.group2()[3]),
                (-(self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(self.group1()[2] * other.group0()[1]) + (self.group0()[0] * other.group1()[3]) + (self.group1()[1] * other.group0()[2])),
                ((self.group1()[2] * other.group0()[0]) + (self.group0()[1] * other.group1()[3]) - (self.group1()[0] * other.group0()[2])),
                (-(self.group1()[1] * other.group0()[0]) + (self.group0()[2] * other.group1()[3]) + (self.group1()[0] * other.group0()[1])),
                (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
            ]),
        );
    }
}
impl AntiWedge<Dipole> for Line {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        return Scalar::from_groups(
            // scalar
            (-(self.group1()[2] * other.group0()[2])
                - (self.group1()[1] * other.group0()[1])
                - (self.group1()[0] * other.group0()[0])
                - (self.group0()[2] * other.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])),
        );
    }
}
impl AntiWedge<DipoleInversion> for Line {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       24        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] * other.group2()[3]),
                (self.group0()[1] * other.group2()[3]),
                (self.group0()[2] * other.group2()[3]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] * other.group2()[3]),
                (self.group1()[1] * other.group2()[3]),
                (self.group1()[2] * other.group2()[3]),
                (-(self.group0()[2] * other.group3()[2]) - (self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group1()[2] * other.group3()[1]) + (self.group0()[0] * other.group3()[3]) + (self.group1()[1] * other.group3()[2])),
                ((self.group1()[2] * other.group3()[0]) + (self.group0()[1] * other.group3()[3]) - (self.group1()[0] * other.group3()[2])),
                (-(self.group1()[1] * other.group3()[0]) + (self.group0()[2] * other.group3()[3]) + (self.group1()[0] * other.group3()[1])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            (self.group0() * Simd32x3::from(other.group0()[1])),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(other.group0()[1])),
        );
    }
}
impl AntiWedge<Flector> for Line {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            (-(swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group1()[3]) + (self.group1()[1] * other.group1()[2])),
                    ((self.group1()[2] * other.group1()[0]) + (self.group0()[1] * other.group1()[3])),
                    ((self.group0()[2] * other.group1()[3]) + (self.group1()[0] * other.group1()[1])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl AntiWedge<Line> for Line {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (-(self.group1()[2] * other.group0()[2])
                - (self.group1()[1] * other.group0()[1])
                - (self.group1()[0] * other.group0()[0])
                - (self.group0()[2] * other.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])),
            0.0,
        ]));
    }
}
impl AntiWedge<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]),
                (self.group0()[1] * other.group0()[3]),
                (self.group0()[2] * other.group0()[3]),
                0.0,
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] * other.group0()[3]),
                (self.group1()[1] * other.group0()[3]),
                (self.group1()[2] * other.group0()[3]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
        );
    }
}
impl AntiWedge<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       35        0
    //    simd3        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       26       48        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group1()[2] * other.group4()[2])
                    - (self.group1()[1] * other.group4()[1])
                    - (self.group1()[0] * other.group4()[0])
                    - (self.group0()[2] * other.group10()[0])
                    - (self.group0()[0] * other.group10()[2])
                    - (self.group0()[1] * other.group10()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(self.group1()[2] * other.group6()[1]) + (self.group0()[0] * other.group5()[3]) + (self.group1()[1] * other.group6()[2])),
                ((self.group1()[2] * other.group6()[0]) + (self.group0()[1] * other.group5()[3]) - (self.group1()[0] * other.group6()[2])),
                (-(self.group1()[1] * other.group6()[0]) + (self.group0()[2] * other.group5()[3]) + (self.group1()[0] * other.group6()[1])),
                (-(self.group0()[2] * other.group6()[2]) - (self.group0()[0] * other.group6()[0]) - (self.group0()[1] * other.group6()[1])),
            ]),
            // e5
            (-(self.group1()[2] * other.group5()[2])
                - (self.group1()[1] * other.group5()[1])
                - (self.group1()[0] * other.group5()[0])
                - (self.group0()[2] * other.group7()[2])
                - (self.group0()[0] * other.group7()[0])
                - (self.group0()[1] * other.group7()[1])),
            // e15, e25, e35, e45
            (-(swizzle!(other.group8(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group8()[3]) + (self.group1()[1] * other.group8()[2])),
                    ((self.group1()[2] * other.group8()[0]) + (self.group0()[1] * other.group8()[3])),
                    ((self.group0()[2] * other.group8()[3]) + (self.group1()[0] * other.group8()[1])),
                    (-(self.group0()[0] * other.group8()[0]) - (self.group0()[1] * other.group8()[1])),
                ])),
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other[e35])),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group0()[1]),
                (self.group0()[1] * other.group0()[1]),
                (self.group0()[2] * other.group0()[1]),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(other.group0()[1])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            (swizzle!(self.group1(), 2, 1, 0) * Simd32x3::from(other[e35])),
        );
    }
}
impl AntiWedge<Plane> for Line {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2])),
                    ((self.group1()[2] * other.group0()[0]) + (self.group0()[1] * other.group0()[3])),
                    ((self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[1])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<Sphere> for Line {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        8       18        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other[e4315])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] * other[e4315]),
                (self.group1()[1] * other[e4315]),
                (self.group1()[2] * other[e4315]),
                (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
            ]),
            // e15, e25, e35
            (-(swizzle!(self.group1(), 2, 0, 1) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]))
                + (self.group0() * Simd32x3::from(other.group0()[3]))
                + (swizzle!(self.group1(), 1, 2, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]))),
        );
    }
}
impl AntiWedge<VersorEven> for Line {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       20        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       13       24        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]),
                (self.group0()[1] * other.group0()[3]),
                (self.group0()[2] * other.group0()[3]),
                0.0,
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] * other.group0()[3]),
                (self.group1()[1] * other.group0()[3]),
                (self.group1()[2] * other.group0()[3]),
                (-(self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])),
            ]),
            // e1, e2, e3, e4
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group1()[3]) + (self.group1()[1] * other.group0()[2])),
                    ((self.group1()[2] * other.group0()[0]) + (self.group0()[1] * other.group1()[3])),
                    ((self.group0()[2] * other.group1()[3]) + (self.group1()[0] * other.group0()[1])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<VersorOdd> for Line {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       24        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] * other.group2()[3]),
                (self.group0()[1] * other.group2()[3]),
                (self.group0()[2] * other.group2()[3]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] * other.group2()[3]),
                (self.group1()[1] * other.group2()[3]),
                (self.group1()[2] * other.group2()[3]),
                (-(self.group0()[2] * other.group3()[2]) - (self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group1()[2] * other.group3()[1]) + (self.group0()[0] * other.group3()[3]) + (self.group1()[1] * other.group3()[2])),
                ((self.group1()[2] * other.group3()[0]) + (self.group0()[1] * other.group3()[3]) - (self.group1()[0] * other.group3()[2])),
                (-(self.group1()[1] * other.group3()[0]) + (self.group0()[2] * other.group3()[3]) + (self.group1()[0] * other.group3()[1])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl InfixAntiWedge for Motor {}
impl AntiWedge<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0() * Simd32x4::from(other[e12345])),
            // e235, e315, e125, e5
            (self.group1() * Simd32x4::from(other[e12345])),
        );
    }
}
impl AntiWedge<Circle> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       13       25        0
    //  no simd       13       28        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[3] * other.group0()[0]),
                (self.group0()[3] * other.group0()[1]),
                (self.group0()[3] * other.group0()[2]),
                0.0,
            ]),
            // e415, e425, e435, e321
            (Simd32x4::from(self.group0()[3]) * other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[3] * other.group2()[0]),
                (self.group0()[3] * other.group2()[1]),
                (self.group0()[3] * other.group2()[2]),
                (-(self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(self.group1()[2] * other.group0()[1]) + (self.group0()[0] * other.group1()[3]) + (self.group1()[1] * other.group0()[2])),
                ((self.group1()[2] * other.group0()[0]) + (self.group0()[1] * other.group1()[3]) - (self.group1()[0] * other.group0()[2])),
                (-(self.group1()[1] * other.group0()[0]) + (self.group0()[2] * other.group1()[3]) + (self.group1()[0] * other.group0()[1])),
                (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
            ]),
        );
    }
}
impl AntiWedge<CircleRotor> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       17       30        0
    //  no simd       20       36        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]])),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((self.group0()[0] * other.group2()[3]) + (self.group0()[3] * other.group1()[0])),
                ((self.group0()[1] * other.group2()[3]) + (self.group0()[3] * other.group1()[1])),
                ((self.group0()[2] * other.group2()[3]) + (self.group0()[3] * other.group1()[2])),
                (self.group0()[3] * other.group1()[3]),
            ]),
            // e235, e315, e125, e5
            ((other.group2() * Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[3]]))
                + Simd32x4::from([
                    (self.group1()[0] * other.group2()[3]),
                    (self.group1()[1] * other.group2()[3]),
                    (self.group1()[2] * other.group2()[3]),
                    (-(self.group1()[2] * other.group1()[2])
                        - (self.group1()[1] * other.group1()[1])
                        - (self.group1()[0] * other.group1()[0])
                        - (self.group0()[2] * other.group2()[2])
                        - (self.group0()[0] * other.group2()[0])
                        - (self.group0()[1] * other.group2()[1])),
                ])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(self.group1()[2] * other.group0()[1]) + (self.group0()[0] * other.group1()[3]) + (self.group1()[1] * other.group0()[2])),
                ((self.group1()[2] * other.group0()[0]) + (self.group0()[1] * other.group1()[3]) - (self.group1()[0] * other.group0()[2])),
                (-(self.group1()[1] * other.group0()[0]) + (self.group0()[2] * other.group1()[3]) + (self.group1()[0] * other.group0()[1])),
                (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
            ]),
        );
    }
}
impl AntiWedge<Dipole> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       13        0
    //  no simd        5       16        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[3] * other.group0()[0]),
                (self.group0()[3] * other.group0()[1]),
                (self.group0()[3] * other.group0()[2]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e23, e31, e12, e45
            (Simd32x4::from(self.group0()[3]) * other.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group0()[3] * other.group2()[0]),
                (self.group0()[3] * other.group2()[1]),
                (self.group0()[3] * other.group2()[2]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<DipoleInversion> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       31        0
    //  no simd       24       40        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + Simd32x4::from([
                    (self.group0()[3] * other.group0()[0]),
                    (self.group0()[3] * other.group0()[1]),
                    (self.group0()[3] * other.group0()[2]),
                    (-(self.group1()[2] * other.group0()[2])
                        - (self.group1()[1] * other.group0()[1])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group0()[2] * other.group1()[2])
                        - (self.group0()[0] * other.group1()[0])
                        - (self.group0()[1] * other.group1()[1])),
                ])),
            // e23, e31, e12, e45
            ((Simd32x4::from(self.group0()[3]) * other.group1())
                + Simd32x4::from([
                    (self.group1()[0] * other.group2()[3]),
                    (self.group1()[1] * other.group2()[3]),
                    (self.group1()[2] * other.group2()[3]),
                    (-(self.group0()[2] * other.group3()[2]) - (self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group1()[2] * other.group3()[1])
                    + (self.group1()[1] * other.group3()[2])
                    + (self.group0()[0] * other.group3()[3])
                    + (self.group0()[3] * other.group2()[0])),
                ((self.group1()[2] * other.group3()[0]) - (self.group1()[0] * other.group3()[2]) + (self.group0()[1] * other.group3()[3]) + (self.group0()[3] * other.group2()[1])),
                (-(self.group1()[1] * other.group3()[0])
                    + (self.group1()[0] * other.group3()[1])
                    + (self.group0()[2] * other.group3()[3])
                    + (self.group0()[3] * other.group2()[2])),
                (self.group0()[3] * other.group2()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[3]) * other.group3()),
        );
    }
}
impl AntiWedge<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0() * Simd32x4::from(other.group0()[1])),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] * other.group0()[1]),
                (self.group1()[1] * other.group0()[1]),
                (self.group1()[2] * other.group0()[1]),
                ((self.group0()[3] * other.group0()[0]) + (self.group1()[3] * other.group0()[1])),
            ]),
        );
    }
}
impl AntiWedge<FlatPoint> for Motor {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ (Simd32x4::from(self.group0()[3]) * other.group0()));
    }
}
impl AntiWedge<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (-(swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + (self.group0() * Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[3], other.group0()[3]]))
                + Simd32x4::from([
                    ((self.group1()[1] * other.group1()[2]) + (self.group0()[3] * other.group0()[0])),
                    ((self.group1()[2] * other.group1()[0]) + (self.group0()[3] * other.group0()[1])),
                    ((self.group1()[0] * other.group1()[1]) + (self.group0()[3] * other.group0()[2])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[3]) * other.group1()),
        );
    }
}
impl AntiWedge<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (self.group0()[3] * other.group0()[0]),
                (self.group0()[3] * other.group0()[1]),
                (self.group0()[3] * other.group0()[2]),
                0.0,
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[3] * other.group1()[0]),
                (self.group0()[3] * other.group1()[1]),
                (self.group0()[3] * other.group1()[2]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
        );
    }
}
impl AntiWedge<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       16       21        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                ((self.group0()[0] * other.group0()[3]) + (self.group0()[3] * other.group0()[0])),
                ((self.group0()[1] * other.group0()[3]) + (self.group0()[3] * other.group0()[1])),
                ((self.group0()[2] * other.group0()[3]) + (self.group0()[3] * other.group0()[2])),
                (self.group0()[3] * other.group0()[3]),
            ]),
            // e235, e315, e125, e5
            ((Simd32x4::from(self.group0()[3]) * other.group1())
                + (self.group1() * Simd32x4::from(other.group0()[3]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group1()[2] * other.group0()[2])
                        - (self.group1()[1] * other.group0()[1])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group0()[2] * other.group1()[2])
                        - (self.group0()[0] * other.group1()[0])
                        - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl AntiWedge<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       45        0
    //    simd3        3        7        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       35       56        0
    //  no simd       50       82        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                ((self.group1()[3] * other[e35]) - (self.group1()[2] * other.group4()[2]) - (self.group1()[1] * other.group4()[1]) - (self.group1()[0] * other.group4()[0])
                    + (self.group0()[3] * other.group0()[0])
                    - (self.group0()[2] * other.group10()[0])
                    - (self.group0()[0] * other.group10()[2])
                    - (self.group0()[1] * other.group10()[1])),
                (self.group0()[3] * other.group0()[1]),
            ]),
            // e1, e2, e3, e4
            ((self.group0() * Simd32x4::from([other.group5()[3], other.group5()[3], other.group5()[3], other.group1()[3]]))
                + Simd32x4::from([
                    (-(self.group1()[2] * other.group6()[1]) + (self.group1()[1] * other.group6()[2]) + (self.group0()[3] * other.group1()[0])),
                    ((self.group1()[2] * other.group6()[0]) - (self.group1()[0] * other.group6()[2]) + (self.group0()[3] * other.group1()[1])),
                    (-(self.group1()[1] * other.group6()[0]) + (self.group1()[0] * other.group6()[1]) + (self.group0()[3] * other.group1()[2])),
                    (-(self.group0()[2] * other.group6()[2]) - (self.group0()[0] * other.group6()[0]) - (self.group0()[1] * other.group6()[1])),
                ])),
            // e5
            ((self.group1()[3] * other.group0()[1]) - (self.group1()[2] * other.group5()[2]) - (self.group1()[1] * other.group5()[1]) - (self.group1()[0] * other.group5()[0])
                + (self.group0()[3] * other[e1])
                - (self.group0()[2] * other.group7()[2])
                - (self.group0()[0] * other.group7()[0])
                - (self.group0()[1] * other.group7()[1])),
            // e15, e25, e35, e45
            (-(swizzle!(other.group8(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + (self.group0() * Simd32x4::from([other.group8()[3], other.group8()[3], other.group8()[3], other.group3()[3]]))
                + Simd32x4::from([
                    ((self.group1()[1] * other.group8()[2]) + (self.group0()[3] * other.group3()[0])),
                    ((self.group1()[2] * other.group8()[0]) + (self.group0()[3] * other.group3()[1])),
                    ((self.group1()[0] * other.group8()[1]) + (self.group0()[3] * other.group3()[2])),
                    (-(self.group0()[0] * other.group8()[0]) - (self.group0()[1] * other.group8()[1])),
                ])),
            // e41, e42, e43
            ((Simd32x3::from(other[e35]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])) + (Simd32x3::from(self.group0()[3]) * other.group4())),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((self.group0()[0] * other.group0()[1]) + (self.group0()[3] * other.group5()[0])),
                ((self.group0()[1] * other.group0()[1]) + (self.group0()[3] * other.group5()[1])),
                ((self.group0()[2] * other.group0()[1]) + (self.group0()[3] * other.group5()[2])),
                (self.group0()[3] * other.group5()[3]),
            ]),
            // e423, e431, e412
            (Simd32x3::from(self.group0()[3]) * other.group6()),
            // e235, e315, e125
            ((Simd32x3::from(self.group0()[3]) * other.group7()) + (Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[3]) * other.group8()),
            // e1234
            (self.group0()[3] * other[e35]),
            // e12, e31, e23
            ((Simd32x3::from(self.group0()[3]) * other.group10()) + (Simd32x3::from(other[e35]) * Simd32x3::from([self.group1()[2], self.group1()[1], self.group1()[0]]))),
        );
    }
}
impl AntiWedge<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2])),
                    ((self.group1()[2] * other.group0()[0]) + (self.group0()[1] * other.group0()[3])),
                    ((self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[1])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[3]) * other.group0()),
        );
    }
}
impl AntiWedge<RoundPoint> for Motor {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[3]) * other.group0()),
            // e5
            (self.group0()[3] * other[e2]),
        );
    }
}
impl AntiWedge<Scalar> for Motor {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self.group0()[3] * other[scalar]));
    }
}
impl AntiWedge<Sphere> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       18        0
    //  no simd        8       24        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(other[e4315]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] * other[e4315]),
                (self.group1()[1] * other[e4315]),
                (self.group1()[2] * other[e4315]),
                (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group1()[2] * other.group0()[1]) + (self.group0()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2])),
                ((self.group1()[2] * other.group0()[0]) + (self.group0()[1] * other.group0()[3]) - (self.group1()[0] * other.group0()[2])),
                (-(self.group1()[1] * other.group0()[0]) + (self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[1])),
                (self.group0()[3] * other[e4315]),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[3]) * other.group0()),
        );
    }
}
impl AntiWedge<VersorEven> for Motor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       21        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       28       41        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from(self.group0()[3]) * other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((self.group0()[0] * other.group0()[3]) + (self.group0()[3] * other.group1()[0])),
                ((self.group0()[1] * other.group0()[3]) + (self.group0()[3] * other.group1()[1])),
                ((self.group0()[2] * other.group0()[3]) + (self.group0()[3] * other.group1()[2])),
                (self.group0()[3] * other.group1()[3]),
            ]),
            // e235, e315, e125, e5
            ((Simd32x4::from(self.group0()[3]) * other.group2())
                + (self.group1() * Simd32x4::from(other.group0()[3]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group1()[2] * other.group1()[2])
                        - (self.group1()[1] * other.group1()[1])
                        - (self.group1()[0] * other.group1()[0])
                        - (self.group0()[2] * other.group2()[2])
                        - (self.group0()[0] * other.group2()[0])
                        - (self.group0()[1] * other.group2()[1])),
                ])),
            // e1, e2, e3, e4
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + (self.group0() * Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[3], other.group3()[3]]))
                + Simd32x4::from([
                    ((self.group1()[1] * other.group0()[2]) + (self.group0()[3] * other.group3()[0])),
                    ((self.group1()[2] * other.group0()[0]) + (self.group0()[3] * other.group3()[1])),
                    ((self.group1()[0] * other.group0()[1]) + (self.group0()[3] * other.group3()[2])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<VersorOdd> for Motor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       28       41        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from(self.group0()[3]) * other.group0())
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group1()[2] * other.group0()[2])
                        - (self.group1()[1] * other.group0()[1])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group0()[2] * other.group1()[2])
                        - (self.group0()[0] * other.group1()[0])
                        - (self.group0()[1] * other.group1()[1])),
                ])),
            // e23, e31, e12, e45
            ((Simd32x4::from(self.group0()[3]) * other.group1())
                + Simd32x4::from([
                    (self.group1()[0] * other.group2()[3]),
                    (self.group1()[1] * other.group2()[3]),
                    (self.group1()[2] * other.group2()[3]),
                    (-(self.group0()[2] * other.group3()[2]) - (self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group1()[2] * other.group3()[1])
                    + (self.group1()[1] * other.group3()[2])
                    + (self.group0()[0] * other.group3()[3])
                    + (self.group0()[3] * other.group2()[0])),
                ((self.group1()[2] * other.group3()[0]) - (self.group1()[0] * other.group3()[2]) + (self.group0()[1] * other.group3()[3]) + (self.group0()[3] * other.group2()[1])),
                (-(self.group1()[1] * other.group3()[0])
                    + (self.group1()[0] * other.group3()[1])
                    + (self.group0()[2] * other.group3()[3])
                    + (self.group0()[3] * other.group2()[2])),
                (self.group0()[3] * other.group2()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[3]) * other.group3()),
        );
    }
}
impl InfixAntiWedge for MultiVector {}
impl AntiWedge<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       32        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            (self.group0() * Simd32x2::from(other[e12345])),
            // e1, e2, e3, e4
            (self.group1() * Simd32x4::from(other[e12345])),
            // e5
            (self[e1] * other[e12345]),
            // e15, e25, e35, e45
            (self.group3() * Simd32x4::from(other[e12345])),
            // e41, e42, e43
            (self.group4() * Simd32x3::from(other[e12345])),
            // e415, e425, e435, e321
            (self.group5() * Simd32x4::from(other[e12345])),
            // e423, e431, e412
            (self.group6() * Simd32x3::from(other[e12345])),
            // e235, e315, e125
            (self.group7() * Simd32x3::from(other[e12345])),
            // e4235, e4315, e4125, e3215
            (self.group8() * Simd32x4::from(other[e12345])),
            // e1234
            (self[e35] * other[e12345]),
            // e12, e31, e23
            (self.group10() * Simd32x3::from(other[e12345])),
        );
    }
}
impl AntiWedge<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       48        0
    //    simd3        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       43       58        0
    //  no simd       54       80        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group10()[2] * other.group1()[0])
                    - (self.group10()[1] * other.group1()[1])
                    - (self.group10()[0] * other.group1()[2])
                    - (self.group4()[2] * other.group2()[2])
                    - (self.group4()[1] * other.group2()[1])
                    - (self.group4()[0] * other.group2()[0])
                    - (self.group3()[3] * other.group1()[3])
                    - (self.group3()[2] * other.group0()[2])
                    - (self.group3()[0] * other.group0()[0])
                    - (self.group3()[1] * other.group0()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(self.group7()[2] * other.group0()[1]) + (self.group7()[1] * other.group0()[2]) + (self.group6()[2] * other.group2()[1])
                    - (self.group6()[1] * other.group2()[2])
                    + (self.group5()[0] * other.group1()[3])
                    + (self.group5()[3] * other.group1()[0])),
                ((self.group7()[2] * other.group0()[0]) - (self.group7()[0] * other.group0()[2]) - (self.group6()[2] * other.group2()[0])
                    + (self.group6()[0] * other.group2()[2])
                    + (self.group5()[1] * other.group1()[3])
                    + (self.group5()[3] * other.group1()[1])),
                (-(self.group7()[1] * other.group0()[0]) + (self.group7()[0] * other.group0()[1]) + (self.group6()[1] * other.group2()[0])
                    - (self.group6()[0] * other.group2()[1])
                    + (self.group5()[2] * other.group1()[3])
                    + (self.group5()[3] * other.group1()[2])),
                (-(self.group6()[2] * other.group1()[2])
                    - (self.group6()[1] * other.group1()[1])
                    - (self.group6()[0] * other.group1()[0])
                    - (self.group5()[2] * other.group0()[2])
                    - (self.group5()[0] * other.group0()[0])
                    - (self.group5()[1] * other.group0()[1])),
            ]),
            // e5
            (-(self.group7()[2] * other.group1()[2])
                - (self.group7()[1] * other.group1()[1])
                - (self.group7()[0] * other.group1()[0])
                - (self.group5()[2] * other.group2()[2])
                - (self.group5()[0] * other.group2()[0])
                - (self.group5()[1] * other.group2()[1])),
            // e15, e25, e35, e45
            (-(swizzle!(self.group8(), 1, 2, 0, 2) * Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group8()[3] * other.group1()[0]) + (self.group8()[2] * other.group2()[1])),
                    ((self.group8()[3] * other.group1()[1]) + (self.group8()[0] * other.group2()[2])),
                    ((self.group8()[3] * other.group1()[2]) + (self.group8()[1] * other.group2()[0])),
                    (-(self.group8()[0] * other.group1()[0]) - (self.group8()[1] * other.group1()[1])),
                ])),
            // e41, e42, e43
            ((Simd32x3::from(self[e35]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (swizzle!(other.group0(), 2, 0, 1) * Simd32x3::from([self.group8()[1], self.group8()[2], self.group8()[0]]))
                - (swizzle!(other.group0(), 1, 2, 0) * Simd32x3::from([self.group8()[2], self.group8()[0], self.group8()[1]]))),
            // e415, e425, e435, e321
            (Simd32x4::from(self.group0()[1]) * other.group1()),
            // e423, e431, e412
            (Simd32x3::from(self.group0()[1]) * other.group0()),
            // e235, e315, e125
            (Simd32x3::from(self.group0()[1]) * other.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            ((Simd32x3::from(self[e35]) * swizzle!(other.group2(), 2, 1, 0))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group8()[2], self.group8()[1], self.group8()[0]]))
                + (Simd32x3::from(self.group8()[3]) * swizzle!(other.group0(), 2, 1, 0))),
        );
    }
}
impl AntiWedge<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd3        8       12        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       52       70        0
    //  no simd       80      112        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group10()[2] * other.group1()[0])
                    - (self.group10()[1] * other.group1()[1])
                    - (self.group10()[0] * other.group1()[2])
                    - (self.group4()[2] * other.group2()[2])
                    - (self.group4()[1] * other.group2()[1])
                    - (self.group4()[0] * other.group2()[0])
                    - (self.group3()[3] * other.group1()[3])
                    - (self.group3()[2] * other.group0()[2])
                    - (self.group3()[1] * other.group0()[1])
                    + (self.group0()[0] * other.group2()[3])
                    - (self.group3()[0] * other.group0()[0])),
                (self.group0()[1] * other.group2()[3]),
            ]),
            // e1, e2, e3, e4
            ((swizzle!(other.group2(), 1, 2, 0, 3) * Simd32x4::from([self.group6()[2], self.group6()[0], self.group6()[1], self.group1()[3]]))
                + Simd32x4::from([
                    (-(self.group7()[2] * other.group0()[1]) + (self.group7()[1] * other.group0()[2]) - (self.group6()[1] * other.group2()[2])
                        + (self.group5()[3] * other.group1()[0])
                        + (self.group1()[0] * other.group2()[3])
                        + (self.group5()[0] * other.group1()[3])),
                    ((self.group7()[2] * other.group0()[0]) - (self.group7()[0] * other.group0()[2]) - (self.group6()[2] * other.group2()[0])
                        + (self.group5()[3] * other.group1()[1])
                        + (self.group1()[1] * other.group2()[3])
                        + (self.group5()[1] * other.group1()[3])),
                    (-(self.group7()[1] * other.group0()[0]) + (self.group7()[0] * other.group0()[1]) - (self.group6()[0] * other.group2()[1])
                        + (self.group5()[3] * other.group1()[2])
                        + (self.group1()[2] * other.group2()[3])
                        + (self.group5()[2] * other.group1()[3])),
                    (-(self.group6()[2] * other.group1()[2])
                        - (self.group6()[1] * other.group1()[1])
                        - (self.group6()[0] * other.group1()[0])
                        - (self.group5()[2] * other.group0()[2])
                        - (self.group5()[1] * other.group0()[1])
                        - (self.group5()[0] * other.group0()[0])),
                ])),
            // e5
            (-(self.group7()[2] * other.group1()[2])
                - (self.group7()[1] * other.group1()[1])
                - (self.group7()[0] * other.group1()[0])
                - (self.group5()[2] * other.group2()[2])
                - (self.group5()[1] * other.group2()[1])
                + (self[e1] * other.group2()[3])
                - (self.group5()[0] * other.group2()[0])),
            // e15, e25, e35, e45
            ((swizzle!(other.group2(), 1, 3, 0, 3) * Simd32x4::from([self.group8()[2], self.group3()[1], self.group8()[1], self.group3()[3]]))
                - (swizzle!(self.group8(), 1, 2, 0, 2) * Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group8()[3] * other.group1()[0]) + (self.group3()[0] * other.group2()[3])),
                    ((self.group8()[3] * other.group1()[1]) + (self.group8()[0] * other.group2()[2])),
                    ((self.group8()[3] * other.group1()[2]) + (self.group3()[2] * other.group2()[3])),
                    (-(self.group8()[1] * other.group1()[1]) - (self.group8()[0] * other.group1()[0])),
                ])),
            // e41, e42, e43
            ((Simd32x3::from(self[e35]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (swizzle!(other.group0(), 1, 2, 0) * Simd32x3::from([self.group8()[2], self.group8()[0], self.group8()[1]]))
                + (self.group4() * Simd32x3::from(other.group2()[3]))
                + (swizzle!(other.group0(), 2, 0, 1) * Simd32x3::from([self.group8()[1], self.group8()[2], self.group8()[0]]))),
            // e415, e425, e435, e321
            ((Simd32x4::from(self.group0()[1]) * other.group1()) + (self.group5() * Simd32x4::from(other.group2()[3]))),
            // e423, e431, e412
            ((Simd32x3::from(self.group0()[1]) * other.group0()) + (self.group6() * Simd32x3::from(other.group2()[3]))),
            // e235, e315, e125
            ((Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])) + (self.group7() * Simd32x3::from(other.group2()[3]))),
            // e4235, e4315, e4125, e3215
            (self.group8() * Simd32x4::from(other.group2()[3])),
            // e1234
            (self[e35] * other.group2()[3]),
            // e12, e31, e23
            ((self.group10() * Simd32x3::from(other.group2()[3])) + (Simd32x3::from(self[e35]) * Simd32x3::from([other.group2()[2], other.group2()[1], other.group2()[0]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group8()[2], self.group8()[1], self.group8()[0]]))
                + (Simd32x3::from(self.group8()[3]) * swizzle!(other.group0(), 2, 1, 0))),
        );
    }
}
impl AntiWedge<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       22        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       24       40        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group7()[2] * other.group0()[2])
                    - (self.group7()[1] * other.group0()[1])
                    - (self.group7()[0] * other.group0()[0])
                    - (self.group6()[2] * other.group2()[2])
                    - (self.group6()[1] * other.group2()[1])
                    - (self.group6()[0] * other.group2()[0])
                    - (self.group5()[3] * other.group1()[3])
                    - (self.group5()[2] * other.group1()[2])
                    - (self.group5()[0] * other.group1()[0])
                    - (self.group5()[1] * other.group1()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            ((Simd32x4::from(self[e35]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                + (swizzle!(self.group8(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group8()[3] * other.group0()[0]) - (self.group8()[2] * other.group1()[1])),
                    (-(self.group8()[3] * other.group0()[1]) - (self.group8()[0] * other.group1()[2])),
                    (-(self.group8()[3] * other.group0()[2]) - (self.group8()[1] * other.group1()[0])),
                    ((self.group8()[0] * other.group0()[0]) + (self.group8()[1] * other.group0()[1])),
                ])),
            // e5
            (-(self.group8()[3] * other.group1()[3]) - (self.group8()[2] * other.group2()[2]) - (self.group8()[0] * other.group2()[0]) - (self.group8()[1] * other.group2()[1])),
            // e15, e25, e35, e45
            (Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (Simd32x3::from(self.group0()[1]) * other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            (Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
    }
}
impl AntiWedge<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       48        0
    //    simd3        8       12        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       52       69        0
    //  no simd       89      120        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group7()[2] * other.group0()[2])
                    - (self.group7()[1] * other.group0()[1])
                    - (self.group7()[0] * other.group0()[0])
                    - (self.group6()[2] * other.group2()[2])
                    - (self.group6()[1] * other.group2()[1])
                    - (self.group6()[0] * other.group2()[0])
                    - (self.group5()[3] * other.group1()[3])
                    - (self.group5()[2] * other.group1()[2])
                    - (self.group5()[1] * other.group1()[1])
                    - (self.group5()[0] * other.group1()[0])
                    + (self[e1] * other.group2()[3])
                    + (self.group1()[3] * other.group3()[3])
                    + (self.group1()[2] * other.group3()[2])
                    + (self.group1()[0] * other.group3()[0])
                    + (self.group1()[1] * other.group3()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            (-(swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group10()[0], self.group10()[2], self.group10()[1], self.group4()[2]]))
                + (Simd32x4::from(self[e35]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                + (swizzle!(self.group8(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                - (self.group3() * Simd32x4::from(other.group2()[3]))
                + Simd32x4::from([
                    ((self.group10()[1] * other.group3()[2]) - (self.group8()[3] * other.group0()[0]) - (self.group8()[2] * other.group1()[1])
                        + (self.group4()[0] * other.group3()[3])),
                    ((self.group10()[0] * other.group3()[0]) - (self.group8()[3] * other.group0()[1]) - (self.group8()[0] * other.group1()[2])
                        + (self.group4()[1] * other.group3()[3])),
                    ((self.group10()[2] * other.group3()[1]) - (self.group8()[3] * other.group0()[2]) - (self.group8()[1] * other.group1()[0])
                        + (self.group4()[2] * other.group3()[3])),
                    ((self.group8()[1] * other.group0()[1]) + (self.group8()[0] * other.group0()[0])
                        - (self.group4()[1] * other.group3()[1])
                        - (self.group4()[0] * other.group3()[0])),
                ])),
            // e5
            (-(self.group8()[3] * other.group1()[3]) - (self.group8()[2] * other.group2()[2]) - (self.group8()[1] * other.group2()[1]) - (self.group8()[0] * other.group2()[0])
                + (self.group3()[3] * other.group3()[3])
                + (self.group3()[2] * other.group3()[2])
                + (self.group3()[0] * other.group3()[0])
                + (self.group3()[1] * other.group3()[1])),
            // e15, e25, e35, e45
            (-(swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group7()[2], self.group7()[0], self.group7()[1], self.group5()[2]]))
                + (Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                + Simd32x4::from([
                    ((self.group7()[1] * other.group3()[2]) + (self.group5()[0] * other.group3()[3])),
                    ((self.group7()[2] * other.group3()[0]) + (self.group5()[1] * other.group3()[3])),
                    ((self.group7()[0] * other.group3()[1]) + (self.group5()[2] * other.group3()[3])),
                    (-(self.group5()[1] * other.group3()[1]) - (self.group5()[0] * other.group3()[0])),
                ])),
            // e41, e42, e43
            ((swizzle!(self.group6(), 2, 0, 1) * Simd32x3::from([other.group3()[1], other.group3()[2], other.group3()[0]]))
                - (swizzle!(self.group6(), 1, 2, 0) * Simd32x3::from([other.group3()[2], other.group3()[0], other.group3()[1]]))
                + (Simd32x3::from(self.group0()[1]) * other.group0())
                + (Simd32x3::from(other.group2()[3]) * Simd32x3::from([self.group5()[0], self.group5()[1], self.group5()[2]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group8(), 1, 2, 0, 3) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[3]]))
                + (swizzle!(other.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group8()[2], self.group8()[0], self.group8()[1], self[e35]]))),
            // e423, e431, e412
            (-(Simd32x3::from(other.group2()[3]) * Simd32x3::from([self.group8()[0], self.group8()[1], self.group8()[2]]))
                + (Simd32x3::from(self[e35]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))),
            // e235, e315, e125
            ((Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group8()[0], self.group8()[1], self.group8()[2]]))
                - (Simd32x3::from(self.group8()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[1]) * other.group3()),
            // e1234
            (self.group0()[1] * other.group2()[3]),
            // e12, e31, e23
            ((swizzle!(self.group7(), 2, 1, 0) * Simd32x3::from(other.group2()[3]))
                + (swizzle!(self.group6(), 2, 1, 0) * Simd32x3::from(other.group3()[3]))
                + (Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]]))
                - (Simd32x3::from(self.group5()[3]) * Simd32x3::from([other.group3()[2], other.group3()[1], other.group3()[0]]))),
        );
    }
}
impl AntiWedge<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       14        0
    //  no simd        2       34        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([((self.group0()[0] * other.group0()[1]) + (self[e35] * other.group0()[0])), (self.group0()[1] * other.group0()[1])]),
            // e1, e2, e3, e4
            (self.group1() * Simd32x4::from(other.group0()[1])),
            // e5
            ((self.group0()[1] * other.group0()[0]) + (self[e1] * other.group0()[1])),
            // e15, e25, e35, e45
            (self.group3() * Simd32x4::from(other.group0()[1])),
            // e41, e42, e43
            (self.group4() * Simd32x3::from(other.group0()[1])),
            // e415, e425, e435, e321
            (self.group5() * Simd32x4::from(other.group0()[1])),
            // e423, e431, e412
            (self.group6() * Simd32x3::from(other.group0()[1])),
            // e235, e315, e125
            (self.group7() * Simd32x3::from(other.group0()[1])),
            // e4235, e4315, e4125, e3215
            (self.group8() * Simd32x4::from(other.group0()[1])),
            // e1234
            (self[e35] * other.group0()[1]),
            // e12, e31, e23
            (self.group10() * Simd32x3::from(other.group0()[1])),
        );
    }
}
impl AntiWedge<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       10        0
    //  no simd        6       16        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group6()[2] * other.group0()[2])
                    - (self.group6()[1] * other.group0()[1])
                    - (self.group5()[3] * other.group0()[3])
                    - (self.group6()[0] * other.group0()[0])),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e35]) * other.group0()),
            // e5
            (-(self.group8()[3] * other.group0()[3]) - (self.group8()[2] * other.group0()[2]) - (self.group8()[0] * other.group0()[0]) - (self.group8()[1] * other.group0()[1])),
            // e15, e25, e35, e45
            (Simd32x4::from(self.group0()[1]) * other.group0()),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl AntiWedge<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       39        0
    //    simd3        3        7        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       32       51        0
    //  no simd       50       80        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group6()[2] * other.group0()[2])
                    - (self.group6()[1] * other.group0()[1])
                    - (self.group6()[0] * other.group0()[0])
                    - (self.group5()[3] * other.group0()[3])
                    + (self.group1()[3] * other.group1()[3])
                    + (self.group1()[2] * other.group1()[2])
                    + (self.group1()[0] * other.group1()[0])
                    + (self.group1()[1] * other.group1()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            (-(swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group10()[0], self.group10()[2], self.group10()[1], self.group4()[2]]))
                + (Simd32x4::from(self[e35]) * other.group0())
                + Simd32x4::from([
                    ((self.group10()[1] * other.group1()[2]) + (self.group4()[0] * other.group1()[3])),
                    ((self.group10()[0] * other.group1()[0]) + (self.group4()[1] * other.group1()[3])),
                    ((self.group10()[2] * other.group1()[1]) + (self.group4()[2] * other.group1()[3])),
                    (-(self.group4()[0] * other.group1()[0]) - (self.group4()[1] * other.group1()[1])),
                ])),
            // e5
            (-(self.group8()[3] * other.group0()[3]) - (self.group8()[2] * other.group0()[2]) - (self.group8()[1] * other.group0()[1]) - (self.group8()[0] * other.group0()[0])
                + (self.group3()[3] * other.group1()[3])
                + (self.group3()[2] * other.group1()[2])
                + (self.group3()[0] * other.group1()[0])
                + (self.group3()[1] * other.group1()[1])),
            // e15, e25, e35, e45
            (-(swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group7()[2], self.group7()[0], self.group7()[1], self.group5()[2]]))
                + (Simd32x4::from(self.group0()[1]) * other.group0())
                + Simd32x4::from([
                    ((self.group7()[1] * other.group1()[2]) + (self.group5()[0] * other.group1()[3])),
                    ((self.group7()[2] * other.group1()[0]) + (self.group5()[1] * other.group1()[3])),
                    ((self.group7()[0] * other.group1()[1]) + (self.group5()[2] * other.group1()[3])),
                    (-(self.group5()[1] * other.group1()[1]) - (self.group5()[0] * other.group1()[0])),
                ])),
            // e41, e42, e43
            (-(swizzle!(self.group6(), 1, 2, 0) * Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]))
                + (swizzle!(self.group6(), 2, 0, 1) * Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]))),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group8()[1] * other.group1()[2]) + (self.group8()[2] * other.group1()[1])),
                ((self.group8()[0] * other.group1()[2]) - (self.group8()[2] * other.group1()[0])),
                (-(self.group8()[0] * other.group1()[1]) + (self.group8()[1] * other.group1()[0])),
                (self[e35] * other.group1()[3]),
            ]),
            // e423, e431, e412
            (Simd32x3::from(self[e35]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e235, e315, e125
            ((Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group8()[0], self.group8()[1], self.group8()[2]]))
                - (Simd32x3::from(self.group8()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[1]) * other.group1()),
            // e1234
            0.0,
            // e12, e31, e23
            (-(Simd32x3::from(self.group5()[3]) * Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]]))
                + (swizzle!(self.group6(), 2, 1, 0) * Simd32x3::from(other.group1()[3]))),
        );
    }
}
impl AntiWedge<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       35        0
    //    simd3        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       26       48        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group10()[2] * other.group0()[0])
                    - (self.group10()[1] * other.group0()[1])
                    - (self.group10()[0] * other.group0()[2])
                    - (self.group4()[2] * other.group1()[2])
                    - (self.group4()[0] * other.group1()[0])
                    - (self.group4()[1] * other.group1()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                ((self.group6()[2] * other.group1()[1]) + (self.group5()[3] * other.group0()[0]) - (self.group6()[1] * other.group1()[2])),
                (-(self.group6()[2] * other.group1()[0]) + (self.group5()[3] * other.group0()[1]) + (self.group6()[0] * other.group1()[2])),
                ((self.group6()[1] * other.group1()[0]) + (self.group5()[3] * other.group0()[2]) - (self.group6()[0] * other.group1()[1])),
                (-(self.group6()[2] * other.group0()[2]) - (self.group6()[0] * other.group0()[0]) - (self.group6()[1] * other.group0()[1])),
            ]),
            // e5
            (-(self.group7()[2] * other.group0()[2])
                - (self.group7()[1] * other.group0()[1])
                - (self.group7()[0] * other.group0()[0])
                - (self.group5()[2] * other.group1()[2])
                - (self.group5()[0] * other.group1()[0])
                - (self.group5()[1] * other.group1()[1])),
            // e15, e25, e35, e45
            (-(swizzle!(self.group8(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    ((self.group8()[3] * other.group0()[0]) + (self.group8()[2] * other.group1()[1])),
                    ((self.group8()[3] * other.group0()[1]) + (self.group8()[0] * other.group1()[2])),
                    ((self.group8()[3] * other.group0()[2]) + (self.group8()[1] * other.group1()[0])),
                    (-(self.group8()[0] * other.group0()[0]) - (self.group8()[1] * other.group0()[1])),
                ])),
            // e41, e42, e43
            (Simd32x3::from(self[e35]) * other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[1] * other.group0()[0]),
                (self.group0()[1] * other.group0()[1]),
                (self.group0()[1] * other.group0()[2]),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from(self.group0()[1]) * other.group1()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            (Simd32x3::from(self[e35]) * swizzle!(other.group1(), 2, 1, 0)),
        );
    }
}
impl AntiWedge<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       45        0
    //    simd3        3        7        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       35       56        0
    //  no simd       50       82        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group10()[2] * other.group0()[0]) - (self.group10()[1] * other.group0()[1]) - (self.group10()[0] * other.group0()[2]) + (self[e35] * other.group1()[3])
                    - (self.group4()[2] * other.group1()[2])
                    - (self.group4()[1] * other.group1()[1])
                    + (self.group0()[0] * other.group0()[3])
                    - (self.group4()[0] * other.group1()[0])),
                (self.group0()[1] * other.group0()[3]),
            ]),
            // e1, e2, e3, e4
            ((self.group1() * Simd32x4::from(other.group0()[3]))
                + Simd32x4::from([
                    ((self.group6()[2] * other.group1()[1]) - (self.group6()[1] * other.group1()[2]) + (self.group5()[3] * other.group0()[0])),
                    (-(self.group6()[2] * other.group1()[0]) + (self.group6()[0] * other.group1()[2]) + (self.group5()[3] * other.group0()[1])),
                    ((self.group6()[1] * other.group1()[0]) - (self.group6()[0] * other.group1()[1]) + (self.group5()[3] * other.group0()[2])),
                    (-(self.group6()[2] * other.group0()[2]) - (self.group6()[1] * other.group0()[1]) - (self.group6()[0] * other.group0()[0])),
                ])),
            // e5
            (-(self.group7()[2] * other.group0()[2])
                - (self.group7()[1] * other.group0()[1])
                - (self.group7()[0] * other.group0()[0])
                - (self.group5()[2] * other.group1()[2])
                - (self.group5()[1] * other.group1()[1])
                - (self.group5()[0] * other.group1()[0])
                + (self.group0()[1] * other.group1()[3])
                + (self[e1] * other.group0()[3])),
            // e15, e25, e35, e45
            ((other.group0() * Simd32x4::from([self.group8()[3], self.group8()[3], self.group8()[3], self.group3()[3]]))
                - (swizzle!(self.group8(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    ((self.group8()[2] * other.group1()[1]) + (self.group3()[0] * other.group0()[3])),
                    ((self.group3()[1] * other.group0()[3]) + (self.group8()[0] * other.group1()[2])),
                    ((self.group8()[1] * other.group1()[0]) + (self.group3()[2] * other.group0()[3])),
                    (-(self.group8()[1] * other.group0()[1]) - (self.group8()[0] * other.group0()[0])),
                ])),
            // e41, e42, e43
            ((self.group4() * Simd32x3::from(other.group0()[3])) + (Simd32x3::from(self[e35]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((self.group0()[1] * other.group0()[0]) + (self.group5()[0] * other.group0()[3])),
                ((self.group0()[1] * other.group0()[1]) + (self.group5()[1] * other.group0()[3])),
                ((self.group0()[1] * other.group0()[2]) + (self.group5()[2] * other.group0()[3])),
                (self.group5()[3] * other.group0()[3]),
            ]),
            // e423, e431, e412
            (self.group6() * Simd32x3::from(other.group0()[3])),
            // e235, e315, e125
            ((Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])) + (self.group7() * Simd32x3::from(other.group0()[3]))),
            // e4235, e4315, e4125, e3215
            (self.group8() * Simd32x4::from(other.group0()[3])),
            // e1234
            (self[e35] * other.group0()[3]),
            // e12, e31, e23
            ((Simd32x3::from(self[e35]) * Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])) + (self.group10() * Simd32x3::from(other.group0()[3]))),
        );
    }
}
impl AntiWedge<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       95      107        0
    //    simd3       20       24        0
    //    simd4       14       16        0
    // Totals...
    // yes simd      129      147        0
    //  no simd      211      243        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group10()[2] * other.group5()[0]) - (self.group10()[1] * other.group5()[1]) - (self.group10()[0] * other.group5()[2])
                    + (self[e35] * other[e1])
                    + (self.group8()[3] * other.group1()[3])
                    + (self.group8()[2] * other.group1()[2])
                    + (self.group8()[1] * other.group1()[1])
                    + (self.group8()[0] * other.group1()[0])
                    - (self.group7()[2] * other.group4()[2])
                    - (self.group7()[1] * other.group4()[1])
                    - (self.group7()[0] * other.group4()[0])
                    - (self.group6()[2] * other.group3()[2])
                    - (self.group6()[1] * other.group3()[1])
                    - (self.group6()[0] * other.group3()[0])
                    - (self.group5()[3] * other.group3()[3])
                    - (self.group5()[2] * other.group10()[0])
                    - (self.group5()[1] * other.group10()[1])
                    - (self.group5()[0] * other.group10()[2])
                    - (self.group4()[2] * other.group7()[2])
                    - (self.group4()[1] * other.group7()[1])
                    - (self.group4()[0] * other.group7()[0])
                    - (self.group3()[3] * other.group5()[3])
                    - (self.group3()[2] * other.group6()[2])
                    - (self.group3()[1] * other.group6()[1])
                    - (self.group3()[0] * other.group6()[0])
                    + (self[e1] * other[e35])
                    + (self.group1()[3] * other.group8()[3])
                    + (self.group1()[2] * other.group8()[2])
                    + (self.group1()[1] * other.group8()[1])
                    + (self.group1()[0] * other.group8()[0])
                    + (self.group0()[0] * other.group0()[1])
                    + (self.group0()[1] * other.group0()[0])),
                (self.group0()[1] * other.group0()[1]),
            ]),
            // e1, e2, e3, e4
            (-(swizzle!(other.group8(), 1, 2, 0, 2) * Simd32x4::from([self.group10()[0], self.group10()[2], self.group10()[1], self.group4()[2]]))
                + (Simd32x4::from(self[e35]) * other.group3())
                + (swizzle!(self.group8(), 1, 2, 0, 2) * Simd32x4::from([other.group10()[0], other.group10()[2], other.group10()[1], other.group4()[2]]))
                - (self.group3() * Simd32x4::from(other[e35]))
                + (Simd32x4::from(self.group0()[1]) * other.group1())
                + (self.group1() * Simd32x4::from(other.group0()[1]))
                + Simd32x4::from([
                    ((self.group10()[1] * other.group8()[2])
                        - (self.group8()[3] * other.group4()[0])
                        - (self.group8()[2] * other.group10()[1])
                        - (self.group7()[2] * other.group6()[1])
                        + (self.group7()[1] * other.group6()[2])
                        + (self.group6()[2] * other.group7()[1])
                        - (self.group6()[1] * other.group7()[2])
                        + (self.group5()[3] * other.group5()[0])
                        + (self.group5()[0] * other.group5()[3])
                        + (self.group4()[0] * other.group8()[3])),
                    ((self.group10()[0] * other.group8()[0]) - (self.group8()[3] * other.group4()[1]) - (self.group8()[0] * other.group10()[0])
                        + (self.group7()[2] * other.group6()[0])
                        - (self.group7()[0] * other.group6()[2])
                        - (self.group6()[2] * other.group7()[0])
                        + (self.group6()[0] * other.group7()[2])
                        + (self.group5()[3] * other.group5()[1])
                        + (self.group5()[1] * other.group5()[3])
                        + (self.group4()[1] * other.group8()[3])),
                    ((self.group10()[2] * other.group8()[1])
                        - (self.group8()[3] * other.group4()[2])
                        - (self.group8()[1] * other.group10()[2])
                        - (self.group7()[1] * other.group6()[0])
                        + (self.group7()[0] * other.group6()[1])
                        + (self.group6()[1] * other.group7()[0])
                        - (self.group6()[0] * other.group7()[1])
                        + (self.group5()[3] * other.group5()[2])
                        + (self.group5()[2] * other.group5()[3])
                        + (self.group4()[2] * other.group8()[3])),
                    ((self.group8()[1] * other.group4()[1]) + (self.group8()[0] * other.group4()[0])
                        - (self.group6()[2] * other.group5()[2])
                        - (self.group6()[1] * other.group5()[1])
                        - (self.group6()[0] * other.group5()[0])
                        - (self.group5()[2] * other.group6()[2])
                        - (self.group5()[1] * other.group6()[1])
                        - (self.group5()[0] * other.group6()[0])
                        - (self.group4()[1] * other.group8()[1])
                        - (self.group4()[0] * other.group8()[0])),
                ])),
            // e5
            (-(self.group8()[3] * other.group3()[3])
                - (self.group8()[2] * other.group3()[2])
                - (self.group8()[1] * other.group3()[1])
                - (self.group8()[0] * other.group3()[0])
                - (self.group7()[2] * other.group5()[2])
                - (self.group7()[1] * other.group5()[1])
                - (self.group7()[0] * other.group5()[0])
                - (self.group5()[2] * other.group7()[2])
                - (self.group5()[1] * other.group7()[1])
                - (self.group5()[0] * other.group7()[0])
                + (self.group3()[3] * other.group8()[3])
                + (self.group3()[2] * other.group8()[2])
                + (self.group3()[1] * other.group8()[1])
                + (self.group3()[0] * other.group8()[0])
                + (self.group0()[1] * other[e1])
                + (self[e1] * other.group0()[1])),
            // e15, e25, e35, e45
            (-(swizzle!(self.group8(), 1, 2, 0, 2) * Simd32x4::from([other.group7()[2], other.group7()[0], other.group7()[1], other.group5()[2]]))
                - (swizzle!(other.group8(), 1, 2, 0, 2) * Simd32x4::from([self.group7()[2], self.group7()[0], self.group7()[1], self.group5()[2]]))
                + (Simd32x4::from(self.group0()[1]) * other.group3())
                + (self.group3() * Simd32x4::from(other.group0()[1]))
                + Simd32x4::from([
                    ((self.group8()[3] * other.group5()[0])
                        + (self.group8()[2] * other.group7()[1])
                        + (self.group7()[1] * other.group8()[2])
                        + (self.group5()[0] * other.group8()[3])),
                    ((self.group8()[3] * other.group5()[1])
                        + (self.group8()[0] * other.group7()[2])
                        + (self.group7()[2] * other.group8()[0])
                        + (self.group5()[1] * other.group8()[3])),
                    ((self.group8()[3] * other.group5()[2])
                        + (self.group8()[1] * other.group7()[0])
                        + (self.group7()[0] * other.group8()[1])
                        + (self.group5()[2] * other.group8()[3])),
                    (-(self.group8()[1] * other.group5()[1])
                        - (self.group8()[0] * other.group5()[0])
                        - (self.group5()[1] * other.group8()[1])
                        - (self.group5()[0] * other.group8()[0])),
                ])),
            // e41, e42, e43
            ((Simd32x3::from(self[e35]) * Simd32x3::from([other.group5()[0], other.group5()[1], other.group5()[2]]))
                - (swizzle!(other.group6(), 1, 2, 0) * Simd32x3::from([self.group8()[2], self.group8()[0], self.group8()[1]]))
                + (swizzle!(other.group6(), 2, 0, 1) * Simd32x3::from([self.group8()[1], self.group8()[2], self.group8()[0]]))
                + (swizzle!(self.group6(), 2, 0, 1) * Simd32x3::from([other.group8()[1], other.group8()[2], other.group8()[0]]))
                - (swizzle!(self.group6(), 1, 2, 0) * Simd32x3::from([other.group8()[2], other.group8()[0], other.group8()[1]]))
                + (Simd32x3::from(other[e35]) * Simd32x3::from([self.group5()[0], self.group5()[1], self.group5()[2]]))
                + (Simd32x3::from(self.group0()[1]) * other.group4())
                + (self.group4() * Simd32x3::from(other.group0()[1]))),
            // e415, e425, e435, e321
            ((swizzle!(other.group8(), 1, 2, 0, 3) * Simd32x4::from([self.group8()[2], self.group8()[0], self.group8()[1], self[e35]]))
                - (swizzle!(self.group8(), 1, 2, 0, 3) * Simd32x4::from([other.group8()[2], other.group8()[0], other.group8()[1], other[e35]]))
                + (Simd32x4::from(self.group0()[1]) * other.group5())
                + (self.group5() * Simd32x4::from(other.group0()[1]))),
            // e423, e431, e412
            ((Simd32x3::from(self[e35]) * Simd32x3::from([other.group8()[0], other.group8()[1], other.group8()[2]]))
                - (Simd32x3::from(other[e35]) * Simd32x3::from([self.group8()[0], self.group8()[1], self.group8()[2]]))
                + (Simd32x3::from(self.group0()[1]) * other.group6())
                + (self.group6() * Simd32x3::from(other.group0()[1]))),
            // e235, e315, e125
            (-(Simd32x3::from(self.group8()[3]) * Simd32x3::from([other.group8()[0], other.group8()[1], other.group8()[2]]))
                + (Simd32x3::from(other.group8()[3]) * Simd32x3::from([self.group8()[0], self.group8()[1], self.group8()[2]]))
                + (Simd32x3::from(self.group0()[1]) * other.group7())
                + (self.group7() * Simd32x3::from(other.group0()[1]))),
            // e4235, e4315, e4125, e3215
            ((Simd32x4::from(self.group0()[1]) * other.group8()) + (self.group8() * Simd32x4::from(other.group0()[1]))),
            // e1234
            ((self.group0()[1] * other[e35]) + (self[e35] * other.group0()[1])),
            // e12, e31, e23
            ((self.group10() * Simd32x3::from(other.group0()[1]))
                + (Simd32x3::from(self[e35]) * swizzle!(other.group7(), 2, 1, 0))
                + (Simd32x3::from(self.group8()[3]) * swizzle!(other.group6(), 2, 1, 0))
                - (Simd32x3::from(other.group5()[3]) * Simd32x3::from([self.group8()[2], self.group8()[1], self.group8()[0]]))
                + (swizzle!(self.group7(), 2, 1, 0) * Simd32x3::from(other[e35]))
                + (swizzle!(self.group6(), 2, 1, 0) * Simd32x3::from(other.group8()[3]))
                + (Simd32x3::from(self.group0()[1]) * other.group10())
                - (Simd32x3::from(self.group5()[3]) * Simd32x3::from([other.group8()[2], other.group8()[1], other.group8()[0]]))),
        );
    }
}
impl AntiWedge<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       31        0
    //    simd3        3        7        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       22       41        0
    //  no simd       34       64        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                ((self.group1()[3] * other.group0()[3]) + (self.group1()[2] * other.group0()[2]) + (self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group10()[0], self.group10()[2], self.group10()[1], self.group4()[2]]))
                + Simd32x4::from([
                    ((self.group10()[1] * other.group0()[2]) + (self.group4()[0] * other.group0()[3])),
                    ((self.group4()[1] * other.group0()[3]) + (self.group10()[0] * other.group0()[0])),
                    ((self.group10()[2] * other.group0()[1]) + (self.group4()[2] * other.group0()[3])),
                    (-(self.group4()[0] * other.group0()[0]) - (self.group4()[1] * other.group0()[1])),
                ])),
            // e5
            ((self.group3()[3] * other.group0()[3]) + (self.group3()[2] * other.group0()[2]) + (self.group3()[0] * other.group0()[0]) + (self.group3()[1] * other.group0()[1])),
            // e15, e25, e35, e45
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group7()[2], self.group7()[0], self.group7()[1], self.group5()[2]]))
                + Simd32x4::from([
                    ((self.group5()[0] * other.group0()[3]) + (self.group7()[1] * other.group0()[2])),
                    ((self.group7()[2] * other.group0()[0]) + (self.group5()[1] * other.group0()[3])),
                    ((self.group5()[2] * other.group0()[3]) + (self.group7()[0] * other.group0()[1])),
                    (-(self.group5()[0] * other.group0()[0]) - (self.group5()[1] * other.group0()[1])),
                ])),
            // e41, e42, e43
            (-(swizzle!(self.group6(), 1, 2, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]))
                + (swizzle!(self.group6(), 2, 0, 1) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]))),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group8()[1] * other.group0()[2]) + (self.group8()[2] * other.group0()[1])),
                ((self.group8()[0] * other.group0()[2]) - (self.group8()[2] * other.group0()[0])),
                (-(self.group8()[0] * other.group0()[1]) + (self.group8()[1] * other.group0()[0])),
                (self[e35] * other.group0()[3]),
            ]),
            // e423, e431, e412
            (Simd32x3::from(self[e35]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            ((Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group8()[0], self.group8()[1], self.group8()[2]]))
                - (Simd32x3::from(self.group8()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[1]) * other.group0()),
            // e1234
            0.0,
            // e12, e31, e23
            (-(Simd32x3::from(self.group5()[3]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[0]]))
                + (swizzle!(self.group6(), 2, 1, 0) * Simd32x3::from(other.group0()[3]))),
        );
    }
}
impl AntiWedge<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                ((self[e35] * other[e2])
                    + (self.group8()[3] * other.group0()[3])
                    + (self.group8()[2] * other.group0()[2])
                    + (self.group8()[0] * other.group0()[0])
                    + (self.group8()[1] * other.group0()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[1]) * other.group0()),
            // e5
            (self.group0()[1] * other[e2]),
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl AntiWedge<Scalar> for MultiVector {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self.group0()[1] * other[scalar]));
    }
}
impl AntiWedge<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       26        0
    //    simd3        6       10        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       25       42        0
    //  no simd       49       80        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                ((self[e1] * other[e4315])
                    + (self.group1()[3] * other.group0()[3])
                    + (self.group1()[2] * other.group0()[2])
                    + (self.group1()[0] * other.group0()[0])
                    + (self.group1()[1] * other.group0()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group10()[0], self.group10()[2], self.group10()[1], self.group4()[2]]))
                - (self.group3() * Simd32x4::from(other[e4315]))
                + Simd32x4::from([
                    ((self.group10()[1] * other.group0()[2]) + (self.group4()[0] * other.group0()[3])),
                    ((self.group10()[0] * other.group0()[0]) + (self.group4()[1] * other.group0()[3])),
                    ((self.group10()[2] * other.group0()[1]) + (self.group4()[2] * other.group0()[3])),
                    (-(self.group4()[1] * other.group0()[1]) - (self.group4()[0] * other.group0()[0])),
                ])),
            // e5
            ((self.group3()[3] * other.group0()[3]) + (self.group3()[2] * other.group0()[2]) + (self.group3()[0] * other.group0()[0]) + (self.group3()[1] * other.group0()[1])),
            // e15, e25, e35, e45
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group7()[2], self.group7()[0], self.group7()[1], self.group5()[2]]))
                + Simd32x4::from([
                    ((self.group5()[0] * other.group0()[3]) + (self.group7()[1] * other.group0()[2])),
                    ((self.group7()[2] * other.group0()[0]) + (self.group5()[1] * other.group0()[3])),
                    ((self.group5()[2] * other.group0()[3]) + (self.group7()[0] * other.group0()[1])),
                    (-(self.group5()[0] * other.group0()[0]) - (self.group5()[1] * other.group0()[1])),
                ])),
            // e41, e42, e43
            ((swizzle!(self.group6(), 2, 0, 1) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]))
                + (Simd32x3::from(other[e4315]) * Simd32x3::from([self.group5()[0], self.group5()[1], self.group5()[2]]))
                - (swizzle!(self.group6(), 1, 2, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group8(), 1, 2, 0, 3) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other[e4315]]))
                + (swizzle!(other.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group8()[2], self.group8()[0], self.group8()[1], self[e35]]))),
            // e423, e431, e412
            (-(Simd32x3::from(other[e4315]) * Simd32x3::from([self.group8()[0], self.group8()[1], self.group8()[2]]))
                + (Simd32x3::from(self[e35]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e235, e315, e125
            ((Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group8()[0], self.group8()[1], self.group8()[2]]))
                - (Simd32x3::from(self.group8()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[1]) * other.group0()),
            // e1234
            (self.group0()[1] * other[e4315]),
            // e12, e31, e23
            ((swizzle!(self.group7(), 2, 1, 0) * Simd32x3::from(other[e4315]))
                - (Simd32x3::from(self.group5()[3]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[0]]))
                + (swizzle!(self.group6(), 2, 1, 0) * Simd32x3::from(other.group0()[3]))),
        );
    }
}
impl AntiWedge<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       60        0
    //    simd3        7       10        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       58       78        0
    //  no simd       90      122        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group10()[2] * other.group1()[0]) - (self.group10()[1] * other.group1()[1]) - (self.group10()[0] * other.group1()[2])
                    + (self[e35] * other.group2()[3])
                    + (self.group8()[3] * other.group3()[3])
                    + (self.group8()[2] * other.group3()[2])
                    + (self.group8()[1] * other.group3()[1])
                    + (self.group8()[0] * other.group3()[0])
                    - (self.group4()[2] * other.group2()[2])
                    - (self.group4()[1] * other.group2()[1])
                    - (self.group4()[0] * other.group2()[0])
                    - (self.group3()[3] * other.group1()[3])
                    - (self.group3()[2] * other.group0()[2])
                    - (self.group3()[1] * other.group0()[1])
                    + (self.group0()[0] * other.group0()[3])
                    - (self.group3()[0] * other.group0()[0])),
                (self.group0()[1] * other.group0()[3]),
            ]),
            // e1, e2, e3, e4
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group7()[2], self.group7()[0], self.group7()[1], self.group5()[2]]))
                + (swizzle!(other.group0(), 2, 0, 1, 3) * Simd32x4::from([self.group7()[1], self.group7()[2], self.group7()[0], self.group1()[3]]))
                + (Simd32x4::from(self.group0()[1]) * other.group3())
                + Simd32x4::from([
                    ((self.group6()[2] * other.group2()[1]) - (self.group6()[1] * other.group2()[2])
                        + (self.group5()[3] * other.group1()[0])
                        + (self.group5()[0] * other.group1()[3])
                        + (self.group1()[0] * other.group0()[3])),
                    (-(self.group6()[2] * other.group2()[0])
                        + (self.group6()[0] * other.group2()[2])
                        + (self.group5()[3] * other.group1()[1])
                        + (self.group5()[1] * other.group1()[3])
                        + (self.group1()[1] * other.group0()[3])),
                    ((self.group6()[1] * other.group2()[0]) - (self.group6()[0] * other.group2()[1])
                        + (self.group5()[3] * other.group1()[2])
                        + (self.group5()[2] * other.group1()[3])
                        + (self.group1()[2] * other.group0()[3])),
                    (-(self.group6()[2] * other.group1()[2])
                        - (self.group6()[1] * other.group1()[1])
                        - (self.group6()[0] * other.group1()[0])
                        - (self.group5()[1] * other.group0()[1])
                        - (self.group5()[0] * other.group0()[0])),
                ])),
            // e5
            (-(self.group7()[2] * other.group1()[2])
                - (self.group7()[1] * other.group1()[1])
                - (self.group7()[0] * other.group1()[0])
                - (self.group5()[2] * other.group2()[2])
                - (self.group5()[1] * other.group2()[1])
                - (self.group5()[0] * other.group2()[0])
                + (self.group0()[1] * other.group2()[3])
                + (self[e1] * other.group0()[3])),
            // e15, e25, e35, e45
            ((self.group3() * Simd32x4::from(other.group0()[3]))
                - (swizzle!(self.group8(), 1, 2, 0, 2) * Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group8()[3] * other.group1()[0]) + (self.group8()[2] * other.group2()[1])),
                    ((self.group8()[3] * other.group1()[1]) + (self.group8()[0] * other.group2()[2])),
                    ((self.group8()[3] * other.group1()[2]) + (self.group8()[1] * other.group2()[0])),
                    (-(self.group8()[1] * other.group1()[1]) - (self.group8()[0] * other.group1()[0])),
                ])),
            // e41, e42, e43
            ((Simd32x3::from(self[e35]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (self.group4() * Simd32x3::from(other.group0()[3]))
                + Simd32x3::from([
                    (-(self.group8()[2] * other.group0()[1]) + (self.group8()[1] * other.group0()[2])),
                    ((self.group8()[2] * other.group0()[0]) - (self.group8()[0] * other.group0()[2])),
                    (-(self.group8()[1] * other.group0()[0]) + (self.group8()[0] * other.group0()[1])),
                ])),
            // e415, e425, e435, e321
            ((Simd32x4::from(self.group0()[1]) * other.group1()) + (self.group5() * Simd32x4::from(other.group0()[3]))),
            // e423, e431, e412
            ((Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])) + (self.group6() * Simd32x3::from(other.group0()[3]))),
            // e235, e315, e125
            ((Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])) + (self.group7() * Simd32x3::from(other.group0()[3]))),
            // e4235, e4315, e4125, e3215
            (self.group8() * Simd32x4::from(other.group0()[3])),
            // e1234
            (self[e35] * other.group0()[3]),
            // e12, e31, e23
            ((self.group10() * Simd32x3::from(other.group0()[3])) + (Simd32x3::from(self[e35]) * Simd32x3::from([other.group2()[2], other.group2()[1], other.group2()[0]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group8()[2], self.group8()[1], self.group8()[0]]))
                + (Simd32x3::from(self.group8()[3]) * Simd32x3::from([other.group0()[2], other.group0()[1], other.group0()[0]]))),
        );
    }
}
impl AntiWedge<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       49        0
    //    simd3        8       12        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       53       70        0
    //  no simd       90      121        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (-(self.group7()[2] * other.group0()[2])
                    - (self.group7()[1] * other.group0()[1])
                    - (self.group7()[0] * other.group0()[0])
                    - (self.group6()[2] * other.group2()[2])
                    - (self.group6()[1] * other.group2()[1])
                    - (self.group6()[0] * other.group2()[0])
                    - (self.group5()[3] * other.group1()[3])
                    - (self.group5()[2] * other.group1()[2])
                    - (self.group5()[1] * other.group1()[1])
                    - (self.group5()[0] * other.group1()[0])
                    + (self[e1] * other.group2()[3])
                    + (self.group1()[3] * other.group3()[3])
                    + (self.group1()[2] * other.group3()[2])
                    + (self.group1()[1] * other.group3()[1])
                    + (self.group0()[1] * other.group0()[3])
                    + (self.group1()[0] * other.group3()[0])),
                0.0,
            ]),
            // e1, e2, e3, e4
            (-(swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group10()[0], self.group10()[2], self.group10()[1], self.group4()[2]]))
                + (Simd32x4::from(self[e35]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                + (swizzle!(self.group8(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                - (self.group3() * Simd32x4::from(other.group2()[3]))
                + Simd32x4::from([
                    ((self.group10()[1] * other.group3()[2]) - (self.group8()[3] * other.group0()[0]) - (self.group8()[2] * other.group1()[1])
                        + (self.group4()[0] * other.group3()[3])),
                    ((self.group10()[0] * other.group3()[0]) - (self.group8()[3] * other.group0()[1]) - (self.group8()[0] * other.group1()[2])
                        + (self.group4()[1] * other.group3()[3])),
                    ((self.group10()[2] * other.group3()[1]) - (self.group8()[3] * other.group0()[2]) - (self.group8()[1] * other.group1()[0])
                        + (self.group4()[2] * other.group3()[3])),
                    ((self.group8()[1] * other.group0()[1]) + (self.group8()[0] * other.group0()[0])
                        - (self.group4()[1] * other.group3()[1])
                        - (self.group4()[0] * other.group3()[0])),
                ])),
            // e5
            (-(self.group8()[3] * other.group1()[3]) - (self.group8()[2] * other.group2()[2]) - (self.group8()[1] * other.group2()[1]) - (self.group8()[0] * other.group2()[0])
                + (self.group3()[3] * other.group3()[3])
                + (self.group3()[2] * other.group3()[2])
                + (self.group3()[0] * other.group3()[0])
                + (self.group3()[1] * other.group3()[1])),
            // e15, e25, e35, e45
            (-(swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group7()[2], self.group7()[0], self.group7()[1], self.group5()[2]]))
                + (Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                + Simd32x4::from([
                    ((self.group7()[1] * other.group3()[2]) + (self.group5()[0] * other.group3()[3])),
                    ((self.group7()[2] * other.group3()[0]) + (self.group5()[1] * other.group3()[3])),
                    ((self.group7()[0] * other.group3()[1]) + (self.group5()[2] * other.group3()[3])),
                    (-(self.group5()[1] * other.group3()[1]) - (self.group5()[0] * other.group3()[0])),
                ])),
            // e41, e42, e43
            ((swizzle!(self.group6(), 2, 0, 1) * Simd32x3::from([other.group3()[1], other.group3()[2], other.group3()[0]]))
                - (swizzle!(self.group6(), 1, 2, 0) * Simd32x3::from([other.group3()[2], other.group3()[0], other.group3()[1]]))
                + (Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + (Simd32x3::from(other.group2()[3]) * Simd32x3::from([self.group5()[0], self.group5()[1], self.group5()[2]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group8(), 1, 2, 0, 3) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[3]]))
                + (swizzle!(other.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group8()[2], self.group8()[0], self.group8()[1], self[e35]]))),
            // e423, e431, e412
            (-(Simd32x3::from(other.group2()[3]) * Simd32x3::from([self.group8()[0], self.group8()[1], self.group8()[2]]))
                + (Simd32x3::from(self[e35]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))),
            // e235, e315, e125
            ((Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group8()[0], self.group8()[1], self.group8()[2]]))
                - (Simd32x3::from(self.group8()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[1]) * other.group3()),
            // e1234
            (self.group0()[1] * other.group2()[3]),
            // e12, e31, e23
            ((swizzle!(self.group7(), 2, 1, 0) * Simd32x3::from(other.group2()[3]))
                + (swizzle!(self.group6(), 2, 1, 0) * Simd32x3::from(other.group3()[3]))
                + (Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]]))
                - (Simd32x3::from(self.group5()[3]) * Simd32x3::from([other.group3()[2], other.group3()[1], other.group3()[0]]))),
        );
    }
}
impl InfixAntiWedge for Plane {}
impl AntiWedge<AntiScalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (self.group0() * Simd32x4::from(other[e12345])));
    }
}
impl AntiWedge<Circle> for Plane {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       14       24        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            ((swizzle!(other.group0(), 2, 0, 1) * Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]))
                - (swizzle!(other.group0(), 1, 2, 0) * Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]))),
            // e23, e31, e12, e45
            (-(swizzle!(self.group0(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    (self.group0()[3] * other.group0()[0]),
                    (self.group0()[3] * other.group0()[1]),
                    (self.group0()[3] * other.group0()[2]),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
            // e15, e25, e35
            ((Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (swizzle!(other.group2(), 2, 0, 1) * Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]))
                + (swizzle!(other.group2(), 1, 2, 0) * Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]))),
        );
    }
}
impl AntiWedge<CircleRotor> for Plane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       14       28        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            ((swizzle!(other.group0(), 2, 0, 1) * Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]))
                - (swizzle!(other.group0(), 1, 2, 0) * Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]))),
            // e23, e31, e12, e45
            (-(swizzle!(self.group0(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    (self.group0()[3] * other.group0()[0]),
                    (self.group0()[3] * other.group0()[1]),
                    (self.group0()[3] * other.group0()[2]),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group0()[3] * other.group1()[0]) - (self.group0()[1] * other.group2()[2]) + (self.group0()[2] * other.group2()[1])),
                ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group2()[2]) - (self.group0()[2] * other.group2()[0])),
                ((self.group0()[3] * other.group1()[2]) - (self.group0()[0] * other.group2()[1]) + (self.group0()[1] * other.group2()[0])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x4::from(other.group2()[3])),
        );
    }
}
impl AntiWedge<Dipole> for Plane {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       13        0
    //  no simd       11       16        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group0()[3] * other.group0()[0]) - (self.group0()[2] * other.group1()[1])),
                    (-(self.group0()[3] * other.group0()[1]) - (self.group0()[0] * other.group1()[2])),
                    (-(self.group0()[3] * other.group0()[2]) - (self.group0()[1] * other.group1()[0])),
                    ((self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
            // e5
            (-(self.group0()[3] * other.group1()[3]) - (self.group0()[2] * other.group2()[2]) - (self.group0()[0] * other.group2()[0]) - (self.group0()[1] * other.group2()[1])),
        );
    }
}
impl AntiWedge<DipoleInversion> for Plane {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       28        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       30        0
    //  no simd       17       36        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] * other.group2()[3] * -1.0),
                (self.group0()[1] * other.group2()[3] * -1.0),
                (self.group0()[2] * other.group2()[3] * -1.0),
                0.0,
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group0()[1] * other.group3()[2]) + (self.group0()[2] * other.group3()[1])),
                ((self.group0()[0] * other.group3()[2]) - (self.group0()[2] * other.group3()[0])),
                (-(self.group0()[0] * other.group3()[1]) + (self.group0()[1] * other.group3()[0])),
                (self.group0()[3] * other.group2()[3] * -1.0),
            ]),
            // e235, e315, e125, e5
            (-(Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group1()[3]]))
                + Simd32x4::from([
                    (self.group0()[0] * other.group3()[3]),
                    (self.group0()[1] * other.group3()[3]),
                    (self.group0()[2] * other.group3()[3]),
                    (-(self.group0()[2] * other.group2()[2]) - (self.group0()[0] * other.group2()[0]) - (self.group0()[1] * other.group2()[1])),
                ])),
            // e1, e2, e3, e4
            ((swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group0()[3] * other.group0()[0]) - (self.group0()[2] * other.group1()[1])),
                    (-(self.group0()[3] * other.group0()[1]) - (self.group0()[0] * other.group1()[2])),
                    (-(self.group0()[3] * other.group0()[2]) - (self.group0()[1] * other.group1()[0])),
                    ((self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<DualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (self.group0() * Simd32x4::from(other.group0()[1])));
    }
}
impl AntiWedge<FlatPoint> for Plane {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([
            (-(self.group0()[3] * other.group0()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
            0.0,
        ]));
    }
}
impl AntiWedge<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        9       16        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (-(self.group0()[1] * other.group1()[2]) + (self.group0()[2] * other.group1()[1])),
                ((self.group0()[0] * other.group1()[2]) - (self.group0()[2] * other.group1()[0])),
                (-(self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
                0.0,
            ]),
            // e235, e315, e125, e5
            (-(Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]))
                + Simd32x4::from([
                    (self.group0()[0] * other.group1()[3]),
                    (self.group0()[1] * other.group1()[3]),
                    (self.group0()[2] * other.group1()[3]),
                    (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<Line> for Plane {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group0()[0]) + (self.group0()[2] * other.group1()[1])),
                    ((self.group0()[3] * other.group0()[1]) + (self.group0()[0] * other.group1()[2])),
                    ((self.group0()[3] * other.group0()[2]) + (self.group0()[1] * other.group1()[0])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group0()[0]) + (self.group0()[2] * other.group1()[1])),
                    ((self.group0()[3] * other.group0()[1]) + (self.group0()[0] * other.group1()[2])),
                    ((self.group0()[3] * other.group0()[2]) + (self.group0()[1] * other.group1()[0])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x4::from(other.group0()[3])),
        );
    }
}
impl AntiWedge<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       32        0
    //    simd3        3        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       22       43        0
    //  no simd       34       68        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                ((self.group0()[3] * other.group1()[3]) + (self.group0()[2] * other.group1()[2]) + (self.group0()[0] * other.group1()[0]) + (self.group0()[1] * other.group1()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            ((swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group10()[0], other.group10()[2], other.group10()[1], other.group4()[2]]))
                + Simd32x4::from([
                    (-(self.group0()[3] * other.group4()[0]) - (self.group0()[2] * other.group10()[1])),
                    (-(self.group0()[3] * other.group4()[1]) - (self.group0()[0] * other.group10()[0])),
                    (-(self.group0()[3] * other.group4()[2]) - (self.group0()[1] * other.group10()[2])),
                    ((self.group0()[0] * other.group4()[0]) + (self.group0()[1] * other.group4()[1])),
                ])),
            // e5
            (-(self.group0()[3] * other.group3()[3]) - (self.group0()[2] * other.group3()[2]) - (self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
            // e15, e25, e35, e45
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group7()[2], other.group7()[0], other.group7()[1], other.group5()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group5()[0]) + (self.group0()[2] * other.group7()[1])),
                    ((self.group0()[3] * other.group5()[1]) + (self.group0()[0] * other.group7()[2])),
                    ((self.group0()[3] * other.group5()[2]) + (self.group0()[1] * other.group7()[0])),
                    (-(self.group0()[0] * other.group5()[0]) - (self.group0()[1] * other.group5()[1])),
                ])),
            // e41, e42, e43
            ((swizzle!(other.group6(), 2, 0, 1) * Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]))
                - (swizzle!(other.group6(), 1, 2, 0) * Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]))),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group0()[1] * other.group8()[2]) + (self.group0()[2] * other.group8()[1])),
                ((self.group0()[0] * other.group8()[2]) - (self.group0()[2] * other.group8()[0])),
                (-(self.group0()[0] * other.group8()[1]) + (self.group0()[1] * other.group8()[0])),
                (self.group0()[3] * other[e35] * -1.0),
            ]),
            // e423, e431, e412
            (Simd32x3::from(other[e35]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            ((Simd32x3::from(other.group8()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group8()[0], other.group8()[1], other.group8()[2]]))),
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x4::from(other.group0()[1])),
            // e1234
            0.0,
            // e12, e31, e23
            (-(Simd32x3::from(other.group5()[3]) * Simd32x3::from([self.group0()[2], self.group0()[1], self.group0()[0]]))
                + (Simd32x3::from(self.group0()[3]) * swizzle!(other.group6(), 2, 1, 0))),
        );
    }
}
impl AntiWedge<Plane> for Plane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       12        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from([
                (-(self.group0()[1] * other.group0()[2]) + (self.group0()[2] * other.group0()[1])),
                ((self.group0()[0] * other.group0()[2]) - (self.group0()[2] * other.group0()[0])),
                (-(self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0])),
            ]),
            // e235, e315, e125
            ((Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
        );
    }
}
impl AntiWedge<RoundPoint> for Plane {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        return Scalar::from_groups(
            // scalar
            ((self.group0()[3] * other.group0()[3]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl AntiWedge<Sphere> for Plane {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        1        4        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        6       20        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other[e4315]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group0()[1] * other.group0()[2]) + (self.group0()[2] * other.group0()[1])),
                ((self.group0()[0] * other.group0()[2]) - (self.group0()[2] * other.group0()[0])),
                (-(self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0])),
                (self.group0()[3] * other[e4315] * -1.0),
            ]),
            // e235, e315, e125
            ((Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
        );
    }
}
impl AntiWedge<VersorEven> for Plane {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       23        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       17       35        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group3()[3]]))
                + Simd32x4::from([
                    ((self.group0()[2] * other.group0()[1]) * -1.0),
                    ((self.group0()[0] * other.group0()[2]) * -1.0),
                    ((self.group0()[1] * other.group0()[0]) * -1.0),
                    ((self.group0()[2] * other.group3()[2]) + (self.group0()[0] * other.group3()[0]) + (self.group0()[1] * other.group3()[1])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(self.group0(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    (self.group0()[3] * other.group0()[0]),
                    (self.group0()[3] * other.group0()[1]),
                    (self.group0()[3] * other.group0()[2]),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group0()[3] * other.group1()[0]) - (self.group0()[1] * other.group2()[2]) + (self.group0()[2] * other.group2()[1])),
                ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group2()[2]) - (self.group0()[2] * other.group2()[0])),
                ((self.group0()[3] * other.group1()[2]) - (self.group0()[0] * other.group2()[1]) + (self.group0()[1] * other.group2()[0])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x4::from(other.group0()[3])),
        );
    }
}
impl AntiWedge<VersorOdd> for Plane {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       28        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       30        0
    //  no simd       17       36        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[0] * other.group2()[3] * -1.0),
                (self.group0()[1] * other.group2()[3] * -1.0),
                (self.group0()[2] * other.group2()[3] * -1.0),
                0.0,
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group0()[1] * other.group3()[2]) + (self.group0()[2] * other.group3()[1])),
                ((self.group0()[0] * other.group3()[2]) - (self.group0()[2] * other.group3()[0])),
                (-(self.group0()[0] * other.group3()[1]) + (self.group0()[1] * other.group3()[0])),
                (self.group0()[3] * other.group2()[3] * -1.0),
            ]),
            // e235, e315, e125, e5
            (-(Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group1()[3]]))
                + Simd32x4::from([
                    (self.group0()[0] * other.group3()[3]),
                    (self.group0()[1] * other.group3()[3]),
                    (self.group0()[2] * other.group3()[3]),
                    (-(self.group0()[2] * other.group2()[2]) - (self.group0()[0] * other.group2()[0]) - (self.group0()[1] * other.group2()[1])),
                ])),
            // e1, e2, e3, e4
            ((swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group0()[3] * other.group0()[0]) - (self.group0()[2] * other.group1()[1])),
                    (-(self.group0()[3] * other.group0()[1]) - (self.group0()[0] * other.group1()[2])),
                    (-(self.group0()[3] * other.group0()[2]) - (self.group0()[1] * other.group1()[0])),
                    ((self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl InfixAntiWedge for RoundPoint {}
impl AntiWedge<AntiScalar> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ (self.group0() * Simd32x4::from(other[e12345])), /* e5 */ (self[e2] * other[e12345]));
    }
}
impl AntiWedge<CircleRotor> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group2()[3])),
            // e5
            (self[e2] * other.group2()[3]),
        );
    }
}
impl AntiWedge<DipoleInversion> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            ((self[e2] * other.group2()[3])
                + (self.group0()[3] * other.group3()[3])
                + (self.group0()[2] * other.group3()[2])
                + (self.group0()[0] * other.group3()[0])
                + (self.group0()[1] * other.group3()[1])),
        );
    }
}
impl AntiWedge<DualNum> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group0()[1])),
            // e5
            (self[e2] * other.group0()[1]),
        );
    }
}
impl AntiWedge<Flector> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        return Scalar::from_groups(
            // scalar
            ((self.group0()[3] * other.group1()[3]) + (self.group0()[2] * other.group1()[2]) + (self.group0()[0] * other.group1()[0]) + (self.group0()[1] * other.group1()[1])),
        );
    }
}
impl AntiWedge<Motor> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group0()[3])),
            // e5
            (self[e2] * other.group0()[3]),
        );
    }
}
impl AntiWedge<MultiVector> for RoundPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                ((self[e2] * other[e35])
                    + (self.group0()[3] * other.group8()[3])
                    + (self.group0()[2] * other.group8()[2])
                    + (self.group0()[0] * other.group8()[0])
                    + (self.group0()[1] * other.group8()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group0()[1])),
            // e5
            (self[e2] * other.group0()[1]),
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl AntiWedge<Plane> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        return Scalar::from_groups(
            // scalar
            ((self.group0()[3] * other.group0()[3]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl AntiWedge<Sphere> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            ((self[e2] * other[e4315])
                + (self.group0()[3] * other.group0()[3])
                + (self.group0()[2] * other.group0()[2])
                + (self.group0()[0] * other.group0()[0])
                + (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl AntiWedge<VersorEven> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group0()[3])),
            // e5
            (self[e2] * other.group0()[3]),
        );
    }
}
impl AntiWedge<VersorOdd> for RoundPoint {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            ((self[e2] * other.group2()[3])
                + (self.group0()[3] * other.group3()[3])
                + (self.group0()[2] * other.group3()[2])
                + (self.group0()[0] * other.group3()[0])
                + (self.group0()[1] * other.group3()[1])),
        );
    }
}
impl InfixAntiWedge for Scalar {}
impl AntiWedge<AntiScalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[scalar] * other[e12345]));
    }
}
impl AntiWedge<CircleRotor> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[scalar] * other.group2()[3]));
    }
}
impl AntiWedge<DualNum> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[scalar] * other.group0()[1]));
    }
}
impl AntiWedge<Motor> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[scalar] * other.group0()[3]));
    }
}
impl AntiWedge<MultiVector> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[scalar] * other.group0()[1]));
    }
}
impl AntiWedge<VersorEven> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[scalar] * other.group0()[3]));
    }
}
impl InfixAntiWedge for Sphere {}
impl AntiWedge<AntiScalar> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x4::from(other[e12345])),
            // e1234
            (self[e4315] * other[e12345]),
        );
    }
}
impl AntiWedge<Circle> for Sphere {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        4        6        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       20       30        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            ((Simd32x3::from(self[e4315]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (swizzle!(other.group0(), 2, 0, 1) * Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]))
                - (swizzle!(other.group0(), 1, 2, 0) * Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]))),
            // e23, e31, e12, e45
            (-(swizzle!(self.group0(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self[e4315] * other.group2()[0]) + (self.group0()[3] * other.group0()[0])),
                    ((self[e4315] * other.group2()[1]) + (self.group0()[3] * other.group0()[1])),
                    ((self[e4315] * other.group2()[2]) + (self.group0()[3] * other.group0()[2])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
            // e15, e25, e35
            ((Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (swizzle!(other.group2(), 2, 0, 1) * Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]))
                + (swizzle!(other.group2(), 1, 2, 0) * Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]))),
        );
    }
}
impl AntiWedge<CircleRotor> for Sphere {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        2        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       20       35        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            ((Simd32x3::from(self[e4315]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (swizzle!(other.group0(), 2, 0, 1) * Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]))
                - (swizzle!(other.group0(), 1, 2, 0) * Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]))),
            // e23, e31, e12, e45
            (-(swizzle!(self.group0(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self[e4315] * other.group2()[0]) + (self.group0()[3] * other.group0()[0])),
                    ((self[e4315] * other.group2()[1]) + (self.group0()[3] * other.group0()[1])),
                    ((self[e4315] * other.group2()[2]) + (self.group0()[3] * other.group0()[2])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group0()[3] * other.group1()[0]) - (self.group0()[1] * other.group2()[2]) + (self.group0()[2] * other.group2()[1])),
                ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group2()[2]) - (self.group0()[2] * other.group2()[0])),
                ((self.group0()[3] * other.group1()[2]) - (self.group0()[0] * other.group2()[1]) + (self.group0()[1] * other.group2()[0])),
                (self[e4315] * other.group2()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x4::from(other.group2()[3])),
        );
    }
}
impl AntiWedge<Dipole> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((Simd32x4::from(self[e4315]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                + (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group0()[3] * other.group0()[0]) - (self.group0()[2] * other.group1()[1])),
                    (-(self.group0()[3] * other.group0()[1]) - (self.group0()[0] * other.group1()[2])),
                    (-(self.group0()[3] * other.group0()[2]) - (self.group0()[1] * other.group1()[0])),
                    ((self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
            // e5
            (-(self.group0()[3] * other.group1()[3]) - (self.group0()[2] * other.group2()[2]) - (self.group0()[0] * other.group2()[0]) - (self.group0()[1] * other.group2()[1])),
        );
    }
}
impl AntiWedge<DipoleInversion> for Sphere {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       20        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       13       25        0
    //  no simd       25       40        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-(self.group0()[0] * other.group2()[3]) + (self[e4315] * other.group3()[0])),
                (-(self.group0()[1] * other.group2()[3]) + (self[e4315] * other.group3()[1])),
                (-(self.group0()[2] * other.group2()[3]) + (self[e4315] * other.group3()[2])),
                0.0,
            ]),
            // e415, e425, e435, e321
            (-(swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[3]]))
                + (swizzle!(other.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self[e4315]]))),
            // e235, e315, e125, e5
            (-(Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group1()[3]]))
                + Simd32x4::from([
                    (self.group0()[0] * other.group3()[3]),
                    (self.group0()[1] * other.group3()[3]),
                    (self.group0()[2] * other.group3()[3]),
                    (-(self.group0()[2] * other.group2()[2]) - (self.group0()[0] * other.group2()[0]) - (self.group0()[1] * other.group2()[1])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self[e4315]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                + (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group0()[3] * other.group0()[0]) - (self.group0()[2] * other.group1()[1])),
                    (-(self.group0()[3] * other.group0()[1]) - (self.group0()[0] * other.group1()[2])),
                    (-(self.group0()[3] * other.group0()[2]) - (self.group0()[1] * other.group1()[0])),
                    ((self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<DualNum> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self[e4315] * other.group0()[0])]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (self[e4315] * other.group0()[1])]),
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x4::from(other.group0()[1])),
        );
    }
}
impl AntiWedge<FlatPoint> for Sphere {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e4315]) * other.group0()),
            // e5
            (-(self.group0()[3] * other.group0()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl AntiWedge<Flector> for Sphere {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        6       18        0
    //  no simd        9       24        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([(self[e4315] * other.group1()[0]), (self[e4315] * other.group1()[1]), (self[e4315] * other.group1()[2]), 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group0()[1] * other.group1()[2]) + (self.group0()[2] * other.group1()[1])),
                ((self.group0()[0] * other.group1()[2]) - (self.group0()[2] * other.group1()[0])),
                (-(self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
                (self[e4315] * other.group1()[3]),
            ]),
            // e235, e315, e125, e5
            (-(Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]))
                + Simd32x4::from([
                    (self.group0()[0] * other.group1()[3]),
                    (self.group0()[1] * other.group1()[3]),
                    (self.group0()[2] * other.group1()[3]),
                    (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e4315]) * other.group0()),
        );
    }
}
impl AntiWedge<Line> for Sphere {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        8       18        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4315]) * other.group0()),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e4315] * other.group1()[0]),
                (self[e4315] * other.group1()[1]),
                (self[e4315] * other.group1()[2]),
                (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
            ]),
            // e15, e25, e35
            ((Simd32x3::from(self.group0()[3]) * other.group0()) - (swizzle!(other.group1(), 2, 0, 1) * Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]))
                + (swizzle!(other.group1(), 1, 2, 0) * Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]))),
        );
    }
}
impl AntiWedge<Motor> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       18        0
    //  no simd        8       24        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(self[e4315]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self[e4315] * other.group1()[0]),
                (self[e4315] * other.group1()[1]),
                (self[e4315] * other.group1()[2]),
                (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group0()[3] * other.group0()[0]) - (self.group0()[1] * other.group1()[2]) + (self.group0()[2] * other.group1()[1])),
                ((self.group0()[3] * other.group0()[1]) + (self.group0()[0] * other.group1()[2]) - (self.group0()[2] * other.group1()[0])),
                ((self.group0()[3] * other.group0()[2]) - (self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
                (self[e4315] * other.group0()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x4::from(other.group0()[3])),
        );
    }
}
impl AntiWedge<MultiVector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       26        0
    //    simd3        6       10        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       25       42        0
    //  no simd       49       80        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                ((self[e4315] * other[e1])
                    + (self.group0()[3] * other.group1()[3])
                    + (self.group0()[2] * other.group1()[2])
                    + (self.group0()[0] * other.group1()[0])
                    + (self.group0()[1] * other.group1()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            ((Simd32x4::from(self[e4315]) * other.group3())
                + (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group10()[0], other.group10()[2], other.group10()[1], other.group4()[2]]))
                + Simd32x4::from([
                    (-(self.group0()[3] * other.group4()[0]) - (self.group0()[2] * other.group10()[1])),
                    (-(self.group0()[3] * other.group4()[1]) - (self.group0()[0] * other.group10()[0])),
                    (-(self.group0()[3] * other.group4()[2]) - (self.group0()[1] * other.group10()[2])),
                    ((self.group0()[0] * other.group4()[0]) + (self.group0()[1] * other.group4()[1])),
                ])),
            // e5
            (-(self.group0()[3] * other.group3()[3]) - (self.group0()[2] * other.group3()[2]) - (self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
            // e15, e25, e35, e45
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group7()[2], other.group7()[0], other.group7()[1], other.group5()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group5()[0]) + (self.group0()[2] * other.group7()[1])),
                    ((self.group0()[3] * other.group5()[1]) + (self.group0()[0] * other.group7()[2])),
                    ((self.group0()[3] * other.group5()[2]) + (self.group0()[1] * other.group7()[0])),
                    (-(self.group0()[0] * other.group5()[0]) - (self.group0()[1] * other.group5()[1])),
                ])),
            // e41, e42, e43
            ((Simd32x3::from(self[e4315]) * Simd32x3::from([other.group5()[0], other.group5()[1], other.group5()[2]]))
                + (swizzle!(other.group6(), 2, 0, 1) * Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]))
                - (swizzle!(other.group6(), 1, 2, 0) * Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([other.group8()[2], other.group8()[0], other.group8()[1], other[e35]]))
                + (swizzle!(other.group8(), 1, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self[e4315]]))),
            // e423, e431, e412
            (-(Simd32x3::from(other[e35]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self[e4315]) * Simd32x3::from([other.group8()[0], other.group8()[1], other.group8()[2]]))),
            // e235, e315, e125
            ((Simd32x3::from(other.group8()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group8()[0], other.group8()[1], other.group8()[2]]))),
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x4::from(other.group0()[1])),
            // e1234
            (self[e4315] * other.group0()[1]),
            // e12, e31, e23
            ((Simd32x3::from(self[e4315]) * swizzle!(other.group7(), 2, 1, 0))
                - (Simd32x3::from(other.group5()[3]) * Simd32x3::from([self.group0()[2], self.group0()[1], self.group0()[0]]))
                + (Simd32x3::from(self.group0()[3]) * swizzle!(other.group6(), 2, 1, 0))),
        );
    }
}
impl AntiWedge<Plane> for Sphere {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       16        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[e4315]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group0()[1] * other.group0()[2]) + (self.group0()[2] * other.group0()[1])),
                ((self.group0()[0] * other.group0()[2]) - (self.group0()[2] * other.group0()[0])),
                (-(self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0])),
                (self[e4315] * other.group0()[3]),
            ]),
            // e235, e315, e125
            ((Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
        );
    }
}
impl AntiWedge<RoundPoint> for Sphere {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            ((self[e4315] * other[e2])
                + (self.group0()[3] * other.group0()[3])
                + (self.group0()[2] * other.group0()[2])
                + (self.group0()[0] * other.group0()[0])
                + (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl AntiWedge<Sphere> for Sphere {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd       10       20        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            (-(Simd32x3::from(other[e4315]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self[e4315]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other[e4315]]))
                + (swizzle!(other.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self[e4315]]))),
            // e235, e315, e125
            ((Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
        );
    }
}
impl AntiWedge<VersorEven> for Sphere {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       15       31        0
    //  no simd       24       43        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x4::from(self[e4315]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]))
                + (swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group3()[3]]))
                + Simd32x4::from([
                    ((self.group0()[2] * other.group0()[1]) * -1.0),
                    ((self.group0()[0] * other.group0()[2]) * -1.0),
                    ((self.group0()[1] * other.group0()[0]) * -1.0),
                    ((self.group0()[2] * other.group3()[2]) + (self.group0()[0] * other.group3()[0]) + (self.group0()[1] * other.group3()[1])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(self.group0(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self[e4315] * other.group2()[0]) + (self.group0()[3] * other.group0()[0])),
                    ((self[e4315] * other.group2()[1]) + (self.group0()[3] * other.group0()[1])),
                    ((self[e4315] * other.group2()[2]) + (self.group0()[3] * other.group0()[2])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group0()[3] * other.group1()[0]) - (self.group0()[1] * other.group2()[2]) + (self.group0()[2] * other.group2()[1])),
                ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group2()[2]) - (self.group0()[2] * other.group2()[0])),
                ((self.group0()[3] * other.group1()[2]) - (self.group0()[0] * other.group2()[1]) + (self.group0()[1] * other.group2()[0])),
                (self[e4315] * other.group0()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x4::from(other.group0()[3])),
        );
    }
}
impl AntiWedge<VersorOdd> for Sphere {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       20        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       13       25        0
    //  no simd       25       40        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (-(self.group0()[0] * other.group2()[3]) + (self[e4315] * other.group3()[0])),
                (-(self.group0()[1] * other.group2()[3]) + (self[e4315] * other.group3()[1])),
                (-(self.group0()[2] * other.group2()[3]) + (self[e4315] * other.group3()[2])),
                0.0,
            ]),
            // e415, e425, e435, e321
            (-(swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[3]]))
                + (swizzle!(other.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self[e4315]]))),
            // e235, e315, e125, e5
            (-(Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group1()[3]]))
                + Simd32x4::from([
                    (self.group0()[0] * other.group3()[3]),
                    (self.group0()[1] * other.group3()[3]),
                    (self.group0()[2] * other.group3()[3]),
                    (-(self.group0()[2] * other.group2()[2]) - (self.group0()[0] * other.group2()[0]) - (self.group0()[1] * other.group2()[1])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self[e4315]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                + (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group0()[3] * other.group0()[0]) - (self.group0()[2] * other.group1()[1])),
                    (-(self.group0()[3] * other.group0()[1]) - (self.group0()[0] * other.group1()[2])),
                    (-(self.group0()[3] * other.group0()[2]) - (self.group0()[1] * other.group1()[0])),
                    ((self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl InfixAntiWedge for VersorEven {}
impl AntiWedge<AntiScalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() * Simd32x4::from(other[e12345])),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other[e12345])),
            // e235, e315, e125, e5
            (self.group2() * Simd32x4::from(other[e12345])),
            // e1, e2, e3, e4
            (self.group3() * Simd32x4::from(other[e12345])),
        );
    }
}
impl AntiWedge<Circle> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       32        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       22       34        0
    //  no simd       25       40        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[3] * other.group0()[0]),
                (self.group0()[3] * other.group0()[1]),
                (self.group0()[3] * other.group0()[2]),
                0.0,
            ]),
            // e415, e425, e435, e321
            (Simd32x4::from(self.group0()[3]) * other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[3] * other.group2()[0]),
                (self.group0()[3] * other.group2()[1]),
                (self.group0()[3] * other.group2()[2]),
                (-(self.group2()[2] * other.group1()[2])
                    - (self.group2()[1] * other.group1()[1])
                    - (self.group2()[0] * other.group1()[0])
                    - (self.group1()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group2()[0])
                    - (self.group1()[1] * other.group2()[1])),
            ]),
            // e1, e2, e3, e4
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group1()[2]]))
                + Simd32x4::from([
                    (-(self.group2()[2] * other.group0()[1])
                        + (self.group2()[1] * other.group0()[2])
                        + (self.group1()[3] * other.group1()[0])
                        + (self.group1()[0] * other.group1()[3])
                        + (self.group0()[2] * other.group2()[1])),
                    ((self.group2()[2] * other.group0()[0]) - (self.group2()[0] * other.group0()[2])
                        + (self.group1()[3] * other.group1()[1])
                        + (self.group1()[1] * other.group1()[3])
                        + (self.group0()[0] * other.group2()[2])),
                    (-(self.group2()[1] * other.group0()[0])
                        + (self.group2()[0] * other.group0()[1])
                        + (self.group1()[3] * other.group1()[2])
                        + (self.group1()[2] * other.group1()[3])
                        + (self.group0()[1] * other.group2()[0])),
                    (-(self.group1()[2] * other.group0()[2])
                        - (self.group1()[1] * other.group0()[1])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group0()[0] * other.group1()[0])
                        - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl AntiWedge<CircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       28       41        0
    //  no simd       40       56        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                ((self.group0()[0] * other.group2()[3]) + (self.group0()[3] * other.group0()[0])),
                ((self.group0()[1] * other.group2()[3]) + (self.group0()[3] * other.group0()[1])),
                ((self.group0()[2] * other.group2()[3]) + (self.group0()[3] * other.group0()[2])),
                (self.group0()[3] * other.group2()[3]),
            ]),
            // e415, e425, e435, e321
            ((Simd32x4::from(self.group0()[3]) * other.group1()) + (self.group1() * Simd32x4::from(other.group2()[3]))),
            // e235, e315, e125, e5
            ((other.group2() * Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group2()[3]]))
                + Simd32x4::from([
                    (self.group2()[0] * other.group2()[3]),
                    (self.group2()[1] * other.group2()[3]),
                    (self.group2()[2] * other.group2()[3]),
                    (-(self.group2()[2] * other.group1()[2])
                        - (self.group2()[1] * other.group1()[1])
                        - (self.group2()[0] * other.group1()[0])
                        - (self.group1()[2] * other.group2()[2])
                        - (self.group1()[0] * other.group2()[0])
                        - (self.group1()[1] * other.group2()[1])),
                ])),
            // e1, e2, e3, e4
            ((self.group3() * Simd32x4::from(other.group2()[3]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group1()[2]]))
                + Simd32x4::from([
                    (-(self.group2()[2] * other.group0()[1])
                        + (self.group2()[1] * other.group0()[2])
                        + (self.group1()[3] * other.group1()[0])
                        + (self.group1()[0] * other.group1()[3])
                        + (self.group0()[2] * other.group2()[1])),
                    ((self.group2()[2] * other.group0()[0]) - (self.group2()[0] * other.group0()[2])
                        + (self.group1()[3] * other.group1()[1])
                        + (self.group1()[1] * other.group1()[3])
                        + (self.group0()[0] * other.group2()[2])),
                    (-(self.group2()[1] * other.group0()[0])
                        + (self.group2()[0] * other.group0()[1])
                        + (self.group1()[3] * other.group1()[2])
                        + (self.group1()[2] * other.group1()[3])
                        + (self.group0()[1] * other.group2()[0])),
                    (-(self.group1()[2] * other.group0()[2])
                        - (self.group1()[1] * other.group0()[1])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group0()[0] * other.group1()[0])
                        - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl AntiWedge<Dipole> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       16        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       17        0
    //  no simd        9       20        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[3] * other.group0()[0]),
                (self.group0()[3] * other.group0()[1]),
                (self.group0()[3] * other.group0()[2]),
                (-(self.group2()[2] * other.group0()[2])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])),
            ]),
            // e23, e31, e12, e45
            (Simd32x4::from(self.group0()[3]) * other.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group0()[3] * other.group2()[0]),
                (self.group0()[3] * other.group2()[1]),
                (self.group0()[3] * other.group2()[2]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<DipoleInversion> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       29       42        0
    //  no simd       44       60        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[2]]))
                + (swizzle!(other.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group3()[3]]))
                + Simd32x4::from([
                    (self.group0()[3] * other.group0()[0]),
                    (self.group0()[3] * other.group0()[1]),
                    (self.group0()[3] * other.group0()[2]),
                    ((self.group3()[2] * other.group3()[2]) + (self.group3()[1] * other.group3()[1]) + (self.group3()[0] * other.group3()[0])
                        - (self.group2()[2] * other.group0()[2])
                        - (self.group2()[1] * other.group0()[1])
                        - (self.group2()[0] * other.group0()[0])
                        - (self.group1()[3] * other.group1()[3])
                        - (self.group1()[2] * other.group1()[2])
                        - (self.group1()[1] * other.group1()[1])
                        - (self.group1()[0] * other.group1()[0])
                        - (self.group0()[0] * other.group2()[0])
                        - (self.group0()[1] * other.group2()[1])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group3(), 0, 1, 2, 2))
                + (self.group0() * Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group1()[3]]))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group2()[3]) + (self.group0()[3] * other.group1()[0])),
                    ((self.group2()[1] * other.group2()[3]) + (self.group0()[3] * other.group1()[1])),
                    ((self.group2()[2] * other.group2()[3]) + (self.group0()[3] * other.group1()[2])),
                    (-(self.group1()[1] * other.group3()[1]) - (self.group1()[0] * other.group3()[0])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group2()[2] * other.group3()[1])
                    + (self.group2()[1] * other.group3()[2])
                    + (self.group0()[3] * other.group2()[0])
                    + (self.group1()[0] * other.group3()[3])),
                ((self.group2()[2] * other.group3()[0]) - (self.group2()[0] * other.group3()[2]) + (self.group0()[3] * other.group2()[1]) + (self.group1()[1] * other.group3()[3])),
                (-(self.group2()[1] * other.group3()[0])
                    + (self.group2()[0] * other.group3()[1])
                    + (self.group0()[3] * other.group2()[2])
                    + (self.group1()[2] * other.group3()[3])),
                (self.group0()[3] * other.group2()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[3]) * other.group3()),
        );
    }
}
impl AntiWedge<DualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() * Simd32x4::from(other.group0()[1])),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other.group0()[1])),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] * other.group0()[1]),
                (self.group2()[1] * other.group0()[1]),
                (self.group2()[2] * other.group0()[1]),
                ((self.group0()[3] * other.group0()[0]) + (self.group2()[3] * other.group0()[1])),
            ]),
            // e1, e2, e3, e4
            (self.group3() * Simd32x4::from(other.group0()[1])),
        );
    }
}
impl AntiWedge<FlatPoint> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(self.group1()[3] * other.group0()[3])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group0()[0] * other.group0()[0])
                    - (self.group0()[1] * other.group0()[1])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * other.group0()[3])]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group0()[3] * other.group0()[0]),
                (self.group0()[3] * other.group0()[1]),
                (self.group0()[3] * other.group0()[2]),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<Flector> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       20        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       19       25        0
    //  no simd       31       40        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + (swizzle!(other.group1(), 1, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group3()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    ((self.group3()[2] * other.group1()[2]) + (self.group3()[1] * other.group1()[1]) + (self.group3()[0] * other.group1()[0])
                        - (self.group1()[3] * other.group0()[3])
                        - (self.group0()[0] * other.group0()[0])
                        - (self.group0()[1] * other.group0()[1])),
                ])),
            // e23, e31, e12, e45
            ((self.group0() * Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[3], other.group0()[3]]))
                - (swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group1(), 0, 1, 2, 2))
                + Simd32x4::from([0.0, 0.0, 0.0, (-(self.group1()[1] * other.group1()[1]) - (self.group1()[0] * other.group1()[0]))])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group2()[2] * other.group1()[1])
                    + (self.group2()[1] * other.group1()[2])
                    + (self.group0()[3] * other.group0()[0])
                    + (self.group1()[0] * other.group1()[3])),
                ((self.group2()[2] * other.group1()[0]) - (self.group2()[0] * other.group1()[2]) + (self.group0()[3] * other.group0()[1]) + (self.group1()[1] * other.group1()[3])),
                (-(self.group2()[1] * other.group1()[0])
                    + (self.group2()[0] * other.group1()[1])
                    + (self.group0()[3] * other.group0()[2])
                    + (self.group1()[2] * other.group1()[3])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[3]) * other.group1()),
        );
    }
}
impl AntiWedge<Line> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       20        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       13       24        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[3] * other.group0()[0]),
                (self.group0()[3] * other.group0()[1]),
                (self.group0()[3] * other.group0()[2]),
                0.0,
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[3] * other.group1()[0]),
                (self.group0()[3] * other.group1()[1]),
                (self.group0()[3] * other.group1()[2]),
                (-(self.group2()[2] * other.group0()[2])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])),
            ]),
            // e1, e2, e3, e4
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group0()[0]) + (self.group0()[2] * other.group1()[1])),
                    ((self.group1()[3] * other.group0()[1]) + (self.group0()[0] * other.group1()[2])),
                    ((self.group1()[3] * other.group0()[2]) + (self.group0()[1] * other.group1()[0])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<Motor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       21        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       28       41        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() * Simd32x4::from(other.group0()[3])),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((self.group0()[3] * other.group0()[0]) + (self.group1()[0] * other.group0()[3])),
                ((self.group0()[3] * other.group0()[1]) + (self.group1()[1] * other.group0()[3])),
                ((self.group0()[3] * other.group0()[2]) + (self.group1()[2] * other.group0()[3])),
                (self.group1()[3] * other.group0()[3]),
            ]),
            // e235, e315, e125, e5
            ((Simd32x4::from(self.group0()[3]) * other.group1())
                + (self.group2() * Simd32x4::from(other.group0()[3]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group2()[2] * other.group0()[2])
                        - (self.group2()[1] * other.group0()[1])
                        - (self.group2()[0] * other.group0()[0])
                        - (self.group1()[2] * other.group1()[2])
                        - (self.group1()[1] * other.group1()[1])
                        - (self.group1()[0] * other.group1()[0])),
                ])),
            // e1, e2, e3, e4
            ((self.group3() * Simd32x4::from(other.group0()[3]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group0()[0]) + (self.group0()[2] * other.group1()[1])),
                    ((self.group1()[3] * other.group0()[1]) + (self.group0()[0] * other.group1()[2])),
                    ((self.group1()[3] * other.group0()[2]) + (self.group0()[1] * other.group1()[0])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       45       60        0
    //    simd3        7       10        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       58       78        0
    //  no simd       90      122        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                ((self.group3()[3] * other.group8()[3])
                    + (self.group3()[2] * other.group8()[2])
                    + (self.group3()[1] * other.group8()[1])
                    + (self.group3()[0] * other.group8()[0])
                    + (self.group2()[3] * other[e35])
                    - (self.group2()[2] * other.group4()[2])
                    - (self.group2()[1] * other.group4()[1])
                    - (self.group2()[0] * other.group4()[0])
                    - (self.group1()[3] * other.group3()[3])
                    - (self.group1()[2] * other.group10()[0])
                    - (self.group1()[1] * other.group10()[1])
                    - (self.group1()[0] * other.group10()[2])
                    + (self.group0()[3] * other.group0()[0])
                    - (self.group0()[2] * other.group3()[2])
                    - (self.group0()[0] * other.group3()[0])
                    - (self.group0()[1] * other.group3()[1])),
                (self.group0()[3] * other.group0()[1]),
            ]),
            // e1, e2, e3, e4
            ((self.group3() * Simd32x4::from(other.group0()[1])) + (Simd32x4::from(self.group0()[3]) * other.group1())
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group7()[2], other.group7()[0], other.group7()[1], other.group5()[2]]))
                + Simd32x4::from([
                    (-(self.group2()[2] * other.group6()[1])
                        + (self.group2()[1] * other.group6()[2])
                        + (self.group1()[3] * other.group5()[0])
                        + (self.group1()[0] * other.group5()[3])
                        + (self.group0()[2] * other.group7()[1])),
                    ((self.group2()[2] * other.group6()[0]) - (self.group2()[0] * other.group6()[2])
                        + (self.group1()[3] * other.group5()[1])
                        + (self.group1()[1] * other.group5()[3])
                        + (self.group0()[0] * other.group7()[2])),
                    (-(self.group2()[1] * other.group6()[0])
                        + (self.group2()[0] * other.group6()[1])
                        + (self.group1()[3] * other.group5()[2])
                        + (self.group1()[2] * other.group5()[3])
                        + (self.group0()[1] * other.group7()[0])),
                    (-(self.group1()[2] * other.group6()[2])
                        - (self.group1()[1] * other.group6()[1])
                        - (self.group1()[0] * other.group6()[0])
                        - (self.group0()[0] * other.group5()[0])
                        - (self.group0()[1] * other.group5()[1])),
                ])),
            // e5
            ((self.group2()[3] * other.group0()[1])
                - (self.group2()[2] * other.group5()[2])
                - (self.group2()[1] * other.group5()[1])
                - (self.group2()[0] * other.group5()[0])
                - (self.group1()[2] * other.group7()[2])
                - (self.group1()[1] * other.group7()[1])
                + (self.group0()[3] * other[e1])
                - (self.group1()[0] * other.group7()[0])),
            // e15, e25, e35, e45
            (-(swizzle!(other.group8(), 1, 2, 0, 2) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[2]]))
                + (Simd32x4::from(self.group0()[3]) * other.group3())
                + Simd32x4::from([
                    ((self.group2()[1] * other.group8()[2]) + (self.group1()[0] * other.group8()[3])),
                    ((self.group2()[2] * other.group8()[0]) + (self.group1()[1] * other.group8()[3])),
                    ((self.group2()[0] * other.group8()[1]) + (self.group1()[2] * other.group8()[3])),
                    (-(self.group1()[1] * other.group8()[1]) - (self.group1()[0] * other.group8()[0])),
                ])),
            // e41, e42, e43
            ((Simd32x3::from(other[e35]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[3]) * other.group4())
                + Simd32x3::from([
                    (-(self.group0()[1] * other.group8()[2]) + (self.group0()[2] * other.group8()[1])),
                    ((self.group0()[0] * other.group8()[2]) - (self.group0()[2] * other.group8()[0])),
                    (-(self.group0()[0] * other.group8()[1]) + (self.group0()[1] * other.group8()[0])),
                ])),
            // e415, e425, e435, e321
            ((Simd32x4::from(self.group0()[3]) * other.group5()) + (self.group1() * Simd32x4::from(other.group0()[1]))),
            // e423, e431, e412
            ((Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])) + (Simd32x3::from(self.group0()[3]) * other.group6())),
            // e235, e315, e125
            ((Simd32x3::from(self.group0()[3]) * other.group7()) + (Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]))),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[3]) * other.group8()),
            // e1234
            (self.group0()[3] * other[e35]),
            // e12, e31, e23
            ((Simd32x3::from(other[e35]) * Simd32x3::from([self.group2()[2], self.group2()[1], self.group2()[0]]))
                - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group8()[2], other.group8()[1], other.group8()[0]]))
                + (Simd32x3::from(other.group8()[3]) * Simd32x3::from([self.group0()[2], self.group0()[1], self.group0()[0]]))
                + (Simd32x3::from(self.group0()[3]) * other.group10())),
        );
    }
}
impl AntiWedge<Plane> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       23        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       11       26        0
    //  no simd       17       35        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((swizzle!(other.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group3()[3]]))
                + Simd32x4::from([
                    ((self.group0()[1] * other.group0()[2]) * -1.0),
                    ((self.group0()[2] * other.group0()[0]) * -1.0),
                    ((self.group0()[0] * other.group0()[1]) * -1.0),
                    ((self.group3()[2] * other.group0()[2]) + (self.group3()[0] * other.group0()[0]) + (self.group3()[1] * other.group0()[1])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2))
                + Simd32x4::from([
                    (self.group0()[0] * other.group0()[3]),
                    (self.group0()[1] * other.group0()[3]),
                    (self.group0()[2] * other.group0()[3]),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group2()[2] * other.group0()[1]) + (self.group1()[0] * other.group0()[3]) + (self.group2()[1] * other.group0()[2])),
                ((self.group2()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3]) - (self.group2()[0] * other.group0()[2])),
                (-(self.group2()[1] * other.group0()[0]) + (self.group1()[2] * other.group0()[3]) + (self.group2()[0] * other.group0()[1])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[3]) * other.group0()),
        );
    }
}
impl AntiWedge<RoundPoint> for VersorEven {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[3]) * other.group0()),
            // e5
            (self.group0()[3] * other[e2]),
        );
    }
}
impl AntiWedge<Scalar> for VersorEven {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn anti_wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self.group0()[3] * other[scalar]));
    }
}
impl AntiWedge<Sphere> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       15       31        0
    //  no simd       24       43        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x4::from(other[e4315]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]))
                + (swizzle!(other.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group3()[3]]))
                + Simd32x4::from([
                    ((self.group0()[1] * other.group0()[2]) * -1.0),
                    ((self.group0()[2] * other.group0()[0]) * -1.0),
                    ((self.group0()[0] * other.group0()[1]) * -1.0),
                    ((self.group3()[2] * other.group0()[2]) + (self.group3()[1] * other.group0()[1]) + (self.group3()[0] * other.group0()[0])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2))
                + Simd32x4::from([
                    ((self.group2()[0] * other[e4315]) + (self.group0()[0] * other.group0()[3])),
                    ((self.group2()[1] * other[e4315]) + (self.group0()[1] * other.group0()[3])),
                    ((self.group2()[2] * other[e4315]) + (self.group0()[2] * other.group0()[3])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group2()[2] * other.group0()[1]) + (self.group1()[0] * other.group0()[3]) + (self.group2()[1] * other.group0()[2])),
                ((self.group2()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3]) - (self.group2()[0] * other.group0()[2])),
                (-(self.group2()[1] * other.group0()[0]) + (self.group1()[2] * other.group0()[3]) + (self.group2()[0] * other.group0()[1])),
                (self.group0()[3] * other[e4315]),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[3]) * other.group0()),
        );
    }
}
impl AntiWedge<VersorEven> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       29        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       27       37        0
    //  no simd       48       61        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                ((self.group0()[0] * other.group0()[3]) + (self.group0()[3] * other.group0()[0])),
                ((self.group0()[1] * other.group0()[3]) + (self.group0()[3] * other.group0()[1])),
                ((self.group0()[2] * other.group0()[3]) + (self.group0()[3] * other.group0()[2])),
                (self.group0()[3] * other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            ((Simd32x4::from(self.group0()[3]) * other.group1()) + (self.group1() * Simd32x4::from(other.group0()[3]))),
            // e235, e315, e125, e5
            ((Simd32x4::from(self.group0()[3]) * other.group2())
                + (self.group2() * Simd32x4::from(other.group0()[3]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group2()[2] * other.group1()[2])
                        - (self.group2()[1] * other.group1()[1])
                        - (self.group2()[0] * other.group1()[0])
                        - (self.group1()[2] * other.group2()[2])
                        - (self.group1()[1] * other.group2()[1])
                        - (self.group1()[0] * other.group2()[0])),
                ])),
            // e1, e2, e3, e4
            ((self.group3() * Simd32x4::from(other.group0()[3]))
                - (swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group1()[2]]))
                + (Simd32x4::from(self.group0()[3]) * other.group3())
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[1] * other.group0()[2])
                        + (self.group1()[3] * other.group1()[0])
                        + (self.group1()[0] * other.group1()[3])
                        + (self.group0()[2] * other.group2()[1])),
                    ((self.group2()[2] * other.group0()[0])
                        + (self.group1()[3] * other.group1()[1])
                        + (self.group1()[1] * other.group1()[3])
                        + (self.group0()[0] * other.group2()[2])),
                    ((self.group2()[0] * other.group0()[1])
                        + (self.group1()[3] * other.group1()[2])
                        + (self.group1()[2] * other.group1()[3])
                        + (self.group0()[1] * other.group2()[0])),
                    (-(self.group1()[1] * other.group0()[1])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group0()[0] * other.group1()[0])
                        - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl AntiWedge<VersorOdd> for VersorEven {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       33        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       30       40        0
    //  no simd       48       61        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]))
                + (Simd32x4::from(self.group0()[3]) * other.group0())
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[2]]))
                + (swizzle!(other.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group3()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    ((self.group3()[2] * other.group3()[2]) + (self.group3()[1] * other.group3()[1]) + (self.group3()[0] * other.group3()[0])
                        - (self.group2()[2] * other.group0()[2])
                        - (self.group2()[1] * other.group0()[1])
                        - (self.group2()[0] * other.group0()[0])
                        - (self.group1()[3] * other.group1()[3])
                        - (self.group1()[2] * other.group1()[2])
                        - (self.group1()[1] * other.group1()[1])
                        - (self.group1()[0] * other.group1()[0])
                        - (self.group0()[0] * other.group2()[0])
                        - (self.group0()[1] * other.group2()[1])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group3(), 0, 1, 2, 2))
                + (self.group0() * Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group1()[3]]))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group2()[3]) + (self.group0()[3] * other.group1()[0])),
                    ((self.group2()[1] * other.group2()[3]) + (self.group0()[3] * other.group1()[1])),
                    ((self.group2()[2] * other.group2()[3]) + (self.group0()[3] * other.group1()[2])),
                    (-(self.group1()[1] * other.group3()[1]) - (self.group1()[0] * other.group3()[0])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (-(self.group2()[2] * other.group3()[1])
                    + (self.group2()[1] * other.group3()[2])
                    + (self.group0()[3] * other.group2()[0])
                    + (self.group1()[0] * other.group3()[3])),
                ((self.group2()[2] * other.group3()[0]) - (self.group2()[0] * other.group3()[2]) + (self.group0()[3] * other.group2()[1]) + (self.group1()[1] * other.group3()[3])),
                (-(self.group2()[1] * other.group3()[0])
                    + (self.group2()[0] * other.group3()[1])
                    + (self.group0()[3] * other.group2()[2])
                    + (self.group1()[2] * other.group3()[3])),
                (self.group0()[3] * other.group2()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[3]) * other.group3()),
        );
    }
}
impl InfixAntiWedge for VersorOdd {}
impl AntiWedge<AntiScalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() * Simd32x4::from(other[e12345])),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(other[e12345])),
            // e15, e25, e35, e1234
            (self.group2() * Simd32x4::from(other[e12345])),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(other[e12345])),
        );
    }
}
impl AntiWedge<Circle> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       26       37        0
    //  no simd       29       40        0
    fn anti_wedge(self, other: Circle) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (-(self.group3()[2] * other.group0()[1]) + (self.group2()[3] * other.group1()[0]) + (self.group3()[1] * other.group0()[2])),
                ((self.group3()[2] * other.group0()[0]) + (self.group2()[3] * other.group1()[1]) - (self.group3()[0] * other.group0()[2])),
                (-(self.group3()[1] * other.group0()[0]) + (self.group2()[3] * other.group1()[2]) + (self.group3()[0] * other.group0()[1])),
                (-(self.group2()[2] * other.group0()[2])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])),
            ]),
            // e23, e31, e12, e45
            (-(swizzle!(self.group3(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group0()[0]) + (self.group2()[3] * other.group2()[0])),
                    ((self.group3()[3] * other.group0()[1]) + (self.group2()[3] * other.group2()[1])),
                    ((self.group3()[3] * other.group0()[2]) + (self.group2()[3] * other.group2()[2])),
                    (-(self.group3()[0] * other.group1()[0]) - (self.group3()[1] * other.group1()[1])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group3()[3] * other.group1()[0]) - (self.group3()[1] * other.group2()[2]) + (self.group3()[2] * other.group2()[1])),
                ((self.group3()[3] * other.group1()[1]) + (self.group3()[0] * other.group2()[2]) - (self.group3()[2] * other.group2()[0])),
                ((self.group3()[3] * other.group1()[2]) - (self.group3()[0] * other.group2()[1]) + (self.group3()[1] * other.group2()[0])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<CircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       31       44        0
    //  no simd       40       56        0
    fn anti_wedge(self, other: CircleRotor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((self.group0() * Simd32x4::from(other.group2()[3]))
                + Simd32x4::from([
                    (-(self.group3()[2] * other.group0()[1]) + (self.group3()[1] * other.group0()[2]) + (self.group2()[3] * other.group1()[0])),
                    ((self.group3()[2] * other.group0()[0]) - (self.group3()[0] * other.group0()[2]) + (self.group2()[3] * other.group1()[1])),
                    (-(self.group3()[1] * other.group0()[0]) + (self.group3()[0] * other.group0()[1]) + (self.group2()[3] * other.group1()[2])),
                    (-(self.group2()[2] * other.group0()[2])
                        - (self.group2()[1] * other.group0()[1])
                        - (self.group2()[0] * other.group0()[0])
                        - (self.group1()[3] * other.group1()[3])
                        - (self.group1()[2] * other.group1()[2])
                        - (self.group1()[1] * other.group1()[1])
                        - (self.group1()[0] * other.group1()[0])
                        - (self.group0()[2] * other.group2()[2])
                        - (self.group0()[0] * other.group2()[0])
                        - (self.group0()[1] * other.group2()[1])),
                ])),
            // e23, e31, e12, e45
            (-(swizzle!(self.group3(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + (self.group1() * Simd32x4::from(other.group2()[3]))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group0()[0]) + (self.group2()[3] * other.group2()[0])),
                    ((self.group3()[3] * other.group0()[1]) + (self.group2()[3] * other.group2()[1])),
                    ((self.group3()[3] * other.group0()[2]) + (self.group2()[3] * other.group2()[2])),
                    (-(self.group3()[1] * other.group1()[1]) - (self.group3()[0] * other.group1()[0])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group3()[3] * other.group1()[0]) + (self.group3()[2] * other.group2()[1]) + (self.group2()[0] * other.group2()[3]) - (self.group3()[1] * other.group2()[2])),
                ((self.group3()[3] * other.group1()[1]) - (self.group3()[2] * other.group2()[0]) + (self.group2()[1] * other.group2()[3]) + (self.group3()[0] * other.group2()[2])),
                ((self.group3()[3] * other.group1()[2]) + (self.group3()[1] * other.group2()[0]) + (self.group2()[2] * other.group2()[3]) - (self.group3()[0] * other.group2()[1])),
                (self.group2()[3] * other.group2()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(other.group2()[3])),
        );
    }
}
impl AntiWedge<Dipole> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn anti_wedge(self, other: Dipole) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                + (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group3()[3] * other.group0()[0]) - (self.group3()[2] * other.group1()[1])),
                    (-(self.group3()[3] * other.group0()[1]) - (self.group3()[0] * other.group1()[2])),
                    (-(self.group3()[3] * other.group0()[2]) - (self.group3()[1] * other.group1()[0])),
                    ((self.group3()[1] * other.group0()[1]) + (self.group3()[0] * other.group0()[0])),
                ])),
            // e5
            (-(self.group3()[3] * other.group1()[3]) - (self.group3()[2] * other.group2()[2]) - (self.group3()[0] * other.group2()[0]) - (self.group3()[1] * other.group2()[1])),
        );
    }
}
impl AntiWedge<DipoleInversion> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       27       36        0
    //  no simd       48       60        0
    fn anti_wedge(self, other: DipoleInversion) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                ((self.group2()[3] * other.group3()[0]) - (self.group3()[0] * other.group2()[3])),
                ((self.group2()[3] * other.group3()[1]) - (self.group3()[1] * other.group2()[3])),
                ((self.group2()[3] * other.group3()[2]) - (self.group3()[2] * other.group2()[3])),
                0.0,
            ]),
            // e415, e425, e435, e321
            (-(swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[3]]))
                + (swizzle!(other.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[3]]))),
            // e235, e315, e125, e5
            ((swizzle!(other.group3(), 3, 3, 3, 2) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[2]]))
                - (Simd32x4::from(self.group3()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group1()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group3()[2] * other.group2()[2]) - (self.group3()[1] * other.group2()[1]) - (self.group3()[0] * other.group2()[0])
                        + (self.group2()[1] * other.group3()[1])
                        + (self.group1()[3] * other.group3()[3])
                        + (self.group2()[0] * other.group3()[0])),
                ])),
            // e1, e2, e3, e4
            ((swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                - (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                - (swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group3()[3] * other.group0()[0]) - (self.group3()[2] * other.group1()[1])
                        + (self.group0()[0] * other.group3()[3])
                        + (self.group1()[1] * other.group3()[2])),
                    (-(self.group3()[3] * other.group0()[1]) - (self.group3()[0] * other.group1()[2])
                        + (self.group1()[2] * other.group3()[0])
                        + (self.group0()[1] * other.group3()[3])),
                    (-(self.group3()[3] * other.group0()[2]) - (self.group3()[1] * other.group1()[0])
                        + (self.group0()[2] * other.group3()[3])
                        + (self.group1()[0] * other.group3()[1])),
                    ((self.group3()[1] * other.group0()[1]) + (self.group3()[0] * other.group0()[0])
                        - (self.group0()[0] * other.group3()[0])
                        - (self.group0()[1] * other.group3()[1])),
                ])),
        );
    }
}
impl AntiWedge<DualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn anti_wedge(self, other: DualNum) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] * other.group0()[1]),
                (self.group0()[1] * other.group0()[1]),
                (self.group0()[2] * other.group0()[1]),
                ((self.group0()[3] * other.group0()[1]) + (self.group2()[3] * other.group0()[0])),
            ]),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(other.group0()[1])),
            // e15, e25, e35, e1234
            (self.group2() * Simd32x4::from(other.group0()[1])),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(other.group0()[1])),
        );
    }
}
impl AntiWedge<FlatPoint> for VersorOdd {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn anti_wedge(self, other: FlatPoint) -> Self::Output {
        return RoundPoint::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self.group2()[3]) * other.group0()),
            // e5
            (-(self.group3()[3] * other.group0()[3]) - (self.group3()[2] * other.group0()[2]) - (self.group3()[0] * other.group0()[0]) - (self.group3()[1] * other.group0()[1])),
        );
    }
}
impl AntiWedge<Flector> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       28        0
    //  no simd       28       40        0
    fn anti_wedge(self, other: Flector) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group2()[3] * other.group1()[0]),
                (self.group2()[3] * other.group1()[1]),
                (self.group2()[3] * other.group1()[2]),
                0.0,
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group3()[1] * other.group1()[2]) + (self.group3()[2] * other.group1()[1])),
                ((self.group3()[0] * other.group1()[2]) - (self.group3()[2] * other.group1()[0])),
                (-(self.group3()[0] * other.group1()[1]) + (self.group3()[1] * other.group1()[0])),
                (self.group2()[3] * other.group1()[3]),
            ]),
            // e235, e315, e125, e5
            ((swizzle!(other.group1(), 3, 3, 3, 2) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[2]]))
                - (Simd32x4::from(self.group3()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group3()[2] * other.group0()[2]) - (self.group3()[1] * other.group0()[1]) - (self.group3()[0] * other.group0()[0])
                        + (self.group2()[1] * other.group1()[1])
                        + (self.group1()[3] * other.group1()[3])
                        + (self.group2()[0] * other.group1()[0])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group2()[3]) * other.group0())
                - (swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group1()[3]) + (self.group1()[1] * other.group1()[2])),
                    ((self.group1()[2] * other.group1()[0]) + (self.group0()[1] * other.group1()[3])),
                    ((self.group0()[2] * other.group1()[3]) + (self.group1()[0] * other.group1()[1])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl AntiWedge<Line> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       24        0
    fn anti_wedge(self, other: Line) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group2()[3] * other.group0()[0]),
                (self.group2()[3] * other.group0()[1]),
                (self.group2()[3] * other.group0()[2]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group2()[3] * other.group1()[0]),
                (self.group2()[3] * other.group1()[1]),
                (self.group2()[3] * other.group1()[2]),
                (-(self.group3()[2] * other.group0()[2]) - (self.group3()[0] * other.group0()[0]) - (self.group3()[1] * other.group0()[1])),
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group3()[3] * other.group0()[0]) - (self.group3()[1] * other.group1()[2]) + (self.group3()[2] * other.group1()[1])),
                ((self.group3()[3] * other.group0()[1]) + (self.group3()[0] * other.group1()[2]) - (self.group3()[2] * other.group1()[0])),
                ((self.group3()[3] * other.group0()[2]) - (self.group3()[0] * other.group1()[1]) + (self.group3()[1] * other.group1()[0])),
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl AntiWedge<Motor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       28       41        0
    fn anti_wedge(self, other: Motor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            ((self.group0() * Simd32x4::from(other.group0()[3]))
                + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group1()[2] * other.group0()[2])
                        - (self.group1()[1] * other.group0()[1])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group0()[2] * other.group1()[2])
                        - (self.group0()[0] * other.group1()[0])
                        - (self.group0()[1] * other.group1()[1])),
                ])),
            // e23, e31, e12, e45
            ((self.group1() * Simd32x4::from(other.group0()[3]))
                + Simd32x4::from([
                    (self.group2()[3] * other.group1()[0]),
                    (self.group2()[3] * other.group1()[1]),
                    (self.group2()[3] * other.group1()[2]),
                    (-(self.group3()[2] * other.group0()[2]) - (self.group3()[1] * other.group0()[1]) - (self.group3()[0] * other.group0()[0])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group3()[3] * other.group0()[0]) + (self.group3()[2] * other.group1()[1]) + (self.group2()[0] * other.group0()[3]) - (self.group3()[1] * other.group1()[2])),
                ((self.group3()[3] * other.group0()[1]) - (self.group3()[2] * other.group1()[0]) + (self.group2()[1] * other.group0()[3]) + (self.group3()[0] * other.group1()[2])),
                ((self.group3()[3] * other.group0()[2]) + (self.group3()[1] * other.group1()[0]) + (self.group2()[2] * other.group0()[3]) - (self.group3()[0] * other.group1()[1])),
                (self.group2()[3] * other.group0()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(other.group0()[3])),
        );
    }
}
impl AntiWedge<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       49        0
    //    simd3        8       12        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       53       70        0
    //  no simd       90      121        0
    fn anti_wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                ((self.group3()[3] * other.group1()[3])
                    + (self.group3()[2] * other.group1()[2])
                    + (self.group3()[1] * other.group1()[1])
                    + (self.group3()[0] * other.group1()[0])
                    + (self.group2()[3] * other[e1])
                    - (self.group2()[2] * other.group6()[2])
                    - (self.group2()[1] * other.group6()[1])
                    - (self.group2()[0] * other.group6()[0])
                    - (self.group1()[3] * other.group5()[3])
                    - (self.group1()[2] * other.group5()[2])
                    - (self.group1()[1] * other.group5()[1])
                    - (self.group1()[0] * other.group5()[0])
                    + (self.group0()[3] * other.group0()[1])
                    - (self.group0()[2] * other.group7()[2])
                    - (self.group0()[0] * other.group7()[0])
                    - (self.group0()[1] * other.group7()[1])),
                0.0,
            ]),
            // e1, e2, e3, e4
            ((swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group10()[0], other.group10()[2], other.group10()[1], other.group4()[2]]))
                + (Simd32x4::from(self.group2()[3]) * other.group3())
                - (Simd32x4::from(other[e35]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                - (swizzle!(other.group8(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group3()[3] * other.group4()[0]) - (self.group3()[2] * other.group10()[1])
                        + (self.group0()[0] * other.group8()[3])
                        + (self.group1()[1] * other.group8()[2])),
                    (-(self.group3()[3] * other.group4()[1]) - (self.group3()[0] * other.group10()[0])
                        + (self.group1()[2] * other.group8()[0])
                        + (self.group0()[1] * other.group8()[3])),
                    (-(self.group3()[3] * other.group4()[2]) - (self.group3()[1] * other.group10()[2])
                        + (self.group0()[2] * other.group8()[3])
                        + (self.group1()[0] * other.group8()[1])),
                    ((self.group3()[1] * other.group4()[1]) + (self.group3()[0] * other.group4()[0])
                        - (self.group0()[0] * other.group8()[0])
                        - (self.group0()[1] * other.group8()[1])),
                ])),
            // e5
            (-(self.group3()[3] * other.group3()[3]) - (self.group3()[2] * other.group3()[2]) - (self.group3()[1] * other.group3()[1]) - (self.group3()[0] * other.group3()[0])
                + (self.group2()[2] * other.group8()[2])
                + (self.group2()[1] * other.group8()[1])
                + (self.group1()[3] * other.group8()[3])
                + (self.group2()[0] * other.group8()[0])),
            // e15, e25, e35, e45
            ((Simd32x4::from(other.group0()[1]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                - (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group7()[2], other.group7()[0], other.group7()[1], other.group5()[2]]))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group5()[0]) + (self.group3()[2] * other.group7()[1])),
                    ((self.group3()[3] * other.group5()[1]) + (self.group3()[0] * other.group7()[2])),
                    ((self.group3()[3] * other.group5()[2]) + (self.group3()[1] * other.group7()[0])),
                    (-(self.group3()[1] * other.group5()[1]) - (self.group3()[0] * other.group5()[0])),
                ])),
            // e41, e42, e43
            (-(swizzle!(other.group6(), 1, 2, 0) * Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]))
                + (swizzle!(other.group6(), 2, 0, 1) * Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[0]]))
                + (Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self.group2()[3]) * Simd32x3::from([other.group5()[0], other.group5()[1], other.group5()[2]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([other.group8()[2], other.group8()[0], other.group8()[1], other[e35]]))
                + (swizzle!(other.group8(), 1, 2, 0, 3) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[3]]))),
            // e423, e431, e412
            ((Simd32x3::from(self.group2()[3]) * Simd32x3::from([other.group8()[0], other.group8()[1], other.group8()[2]]))
                - (Simd32x3::from(other[e35]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]))),
            // e235, e315, e125
            ((Simd32x3::from(other.group8()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]))
                - (Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group8()[0], other.group8()[1], other.group8()[2]]))),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(other.group0()[1])),
            // e1234
            (self.group2()[3] * other.group0()[1]),
            // e12, e31, e23
            ((Simd32x3::from(self.group3()[3]) * swizzle!(other.group6(), 2, 1, 0))
                - (Simd32x3::from(other.group5()[3]) * Simd32x3::from([self.group3()[2], self.group3()[1], self.group3()[0]]))
                + (Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group1()[2], self.group1()[1], self.group1()[0]]))
                + (Simd32x3::from(self.group2()[3]) * swizzle!(other.group7(), 2, 1, 0))),
        );
    }
}
impl AntiWedge<Plane> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       27        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       17       35        0
    fn anti_wedge(self, other: Plane) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group2()[3] * other.group0()[0]),
                (self.group2()[3] * other.group0()[1]),
                (self.group2()[3] * other.group0()[2]),
                0.0,
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group3()[1] * other.group0()[2]) + (self.group3()[2] * other.group0()[1])),
                ((self.group3()[0] * other.group0()[2]) - (self.group3()[2] * other.group0()[0])),
                (-(self.group3()[0] * other.group0()[1]) + (self.group3()[1] * other.group0()[0])),
                (self.group2()[3] * other.group0()[3]),
            ]),
            // e235, e315, e125, e5
            ((swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[2]]))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group0()[0]) * -1.0),
                    ((self.group3()[3] * other.group0()[1]) * -1.0),
                    ((self.group3()[3] * other.group0()[2]) * -1.0),
                    ((self.group2()[1] * other.group0()[1]) + (self.group1()[3] * other.group0()[3]) + (self.group2()[0] * other.group0()[0])),
                ])),
            // e1, e2, e3, e4
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2])),
                    ((self.group1()[2] * other.group0()[0]) + (self.group0()[1] * other.group0()[3])),
                    ((self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[1])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<RoundPoint> for VersorOdd {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn anti_wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(
            // scalar
            ((self.group3()[3] * other.group0()[3])
                + (self.group3()[2] * other.group0()[2])
                + (self.group3()[1] * other.group0()[1])
                + (self.group2()[3] * other[e2])
                + (self.group3()[0] * other.group0()[0])),
        );
    }
}
impl AntiWedge<Sphere> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       23        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       13       28        0
    //  no simd       25       43        0
    fn anti_wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                ((self.group2()[3] * other.group0()[0]) - (self.group3()[0] * other[e4315])),
                ((self.group2()[3] * other.group0()[1]) - (self.group3()[1] * other[e4315])),
                ((self.group2()[3] * other.group0()[2]) - (self.group3()[2] * other[e4315])),
                0.0,
            ]),
            // e415, e425, e435, e321
            (-(swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other[e4315]]))
                + (swizzle!(other.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[3]]))),
            // e235, e315, e125, e5
            ((swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[2]]))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group0()[0]) * -1.0),
                    ((self.group3()[3] * other.group0()[1]) * -1.0),
                    ((self.group3()[3] * other.group0()[2]) * -1.0),
                    ((self.group2()[1] * other.group0()[1]) + (self.group1()[3] * other.group0()[3]) + (self.group2()[0] * other.group0()[0])),
                ])),
            // e1, e2, e3, e4
            (-(Simd32x4::from(other[e4315]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                - (swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2])),
                    ((self.group1()[2] * other.group0()[0]) + (self.group0()[1] * other.group0()[3])),
                    ((self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[1])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl AntiWedge<VersorEven> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       33        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       30       40        0
    //  no simd       48       61        0
    fn anti_wedge(self, other: VersorEven) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[2]]))
                + (swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group3()[3]]))
                + (self.group0() * Simd32x4::from(other.group0()[3]))
                + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    ((self.group3()[2] * other.group3()[2]) + (self.group3()[1] * other.group3()[1]) + (self.group3()[0] * other.group3()[0])
                        - (self.group2()[1] * other.group0()[1])
                        - (self.group2()[0] * other.group0()[0])
                        - (self.group1()[3] * other.group1()[3])
                        - (self.group1()[2] * other.group1()[2])
                        - (self.group1()[1] * other.group1()[1])
                        - (self.group1()[0] * other.group1()[0])
                        - (self.group0()[2] * other.group2()[2])
                        - (self.group0()[0] * other.group2()[0])
                        - (self.group0()[1] * other.group2()[1])),
                ])),
            // e23, e31, e12, e45
            ((other.group0() * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group1()[3]]))
                - (swizzle!(self.group3(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group0()[3]) + (self.group2()[3] * other.group2()[0])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group2()[3] * other.group2()[1])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group2()[3] * other.group2()[2])),
                    (-(self.group3()[1] * other.group1()[1]) - (self.group3()[0] * other.group1()[0])),
                ])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                ((self.group3()[3] * other.group1()[0]) + (self.group3()[2] * other.group2()[1]) + (self.group2()[0] * other.group0()[3]) - (self.group3()[1] * other.group2()[2])),
                ((self.group3()[3] * other.group1()[1]) - (self.group3()[2] * other.group2()[0]) + (self.group2()[1] * other.group0()[3]) + (self.group3()[0] * other.group2()[2])),
                ((self.group3()[3] * other.group1()[2]) + (self.group3()[1] * other.group2()[0]) + (self.group2()[2] * other.group0()[3]) - (self.group3()[0] * other.group2()[1])),
                (self.group2()[3] * other.group0()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(other.group0()[3])),
        );
    }
}
impl AntiWedge<VersorOdd> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       27       36        0
    //  no simd       48       60        0
    fn anti_wedge(self, other: VersorOdd) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                ((self.group2()[3] * other.group3()[0]) - (self.group3()[0] * other.group2()[3])),
                ((self.group2()[3] * other.group3()[1]) - (self.group3()[1] * other.group2()[3])),
                ((self.group2()[3] * other.group3()[2]) - (self.group3()[2] * other.group2()[3])),
                0.0,
            ]),
            // e415, e425, e435, e321
            (-(swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[3]]))
                + (swizzle!(other.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[3]]))),
            // e235, e315, e125, e5
            ((swizzle!(other.group3(), 3, 3, 3, 2) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[2]]))
                - (Simd32x4::from(self.group3()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group1()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    (-(self.group3()[2] * other.group2()[2]) - (self.group3()[1] * other.group2()[1]) - (self.group3()[0] * other.group2()[0])
                        + (self.group2()[1] * other.group3()[1])
                        + (self.group1()[3] * other.group3()[3])
                        + (self.group2()[0] * other.group3()[0])),
                ])),
            // e1, e2, e3, e4
            ((swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                - (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                - (swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    (-(self.group3()[3] * other.group0()[0]) - (self.group3()[2] * other.group1()[1])
                        + (self.group0()[0] * other.group3()[3])
                        + (self.group1()[1] * other.group3()[2])),
                    (-(self.group3()[3] * other.group0()[1]) - (self.group3()[0] * other.group1()[2])
                        + (self.group1()[2] * other.group3()[0])
                        + (self.group0()[1] * other.group3()[3])),
                    (-(self.group3()[3] * other.group0()[2]) - (self.group3()[1] * other.group1()[0])
                        + (self.group0()[2] * other.group3()[3])
                        + (self.group1()[0] * other.group3()[1])),
                    ((self.group3()[1] * other.group0()[1]) + (self.group3()[0] * other.group0()[0])
                        - (self.group0()[0] * other.group3()[0])
                        - (self.group0()[1] * other.group3()[1])),
                ])),
        );
    }
}
