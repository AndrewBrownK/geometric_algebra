// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 193
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         5       9       0
//  Average:         8      14       0
//  Maximum:       131     153       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         6      12       0
//  Average:        13      21       0
//  Maximum:       211     243       0
impl InfixWedge for AntiScalar {}
impl Wedge<MultiVector> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e12345] * other.group0()[0]));
    }
}
impl Wedge<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e12345] * other[scalar]));
    }
}
impl Wedge<VersorOdd> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e12345] * other.group0()[3]));
    }
}
impl InfixWedge for Circle {}
impl Wedge<Dipole> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
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
impl Wedge<DipoleInversion> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
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
impl Wedge<DualNum> for Circle {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]])),
        );
    }
}
impl Wedge<FlatPoint> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (-(self.group1()[3] * other.group0()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Flector> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Flector) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (-(self.group1()[3] * other.group0()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Motor> for Circle {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other.group1()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]])),
        );
    }
}
impl Wedge<MultiVector> for Circle {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       22        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       24       40        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other.group0()[0])),
            // e423, e431, e412
            (self.group0() * Simd32x3::from(other.group0()[0])),
            // e235, e315, e125
            (self.group2() * Simd32x3::from(other.group0()[0])),
            // e4235, e4315, e4125, e3215
            ((swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[2]]))
                + (Simd32x4::from(other[e1]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + Simd32x4::from([
                    (-(self.group2()[0] * other.group1()[3]) - (self.group1()[1] * other.group1()[2])),
                    (-(self.group2()[1] * other.group1()[3]) - (self.group1()[2] * other.group1()[0])),
                    (-(self.group2()[2] * other.group1()[3]) - (self.group1()[0] * other.group1()[1])),
                    ((self.group2()[1] * other.group1()[1]) + (self.group2()[0] * other.group1()[0])),
                ])),
            // e1234
            (-(self.group1()[3] * other.group1()[3]) - (self.group0()[2] * other.group1()[2]) - (self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<RoundPoint> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[2]]))
                + (Simd32x4::from(other[e2]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + Simd32x4::from([
                    (-(self.group2()[0] * other.group0()[3]) - (self.group1()[1] * other.group0()[2])),
                    (-(self.group2()[1] * other.group0()[3]) - (self.group1()[2] * other.group0()[0])),
                    (-(self.group2()[2] * other.group0()[3]) - (self.group1()[0] * other.group0()[1])),
                    ((self.group2()[1] * other.group0()[1]) + (self.group2()[0] * other.group0()[0])),
                ])),
            // e1234
            (-(self.group1()[3] * other.group0()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Scalar> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(other[scalar])),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other[scalar])),
            // e235, e315, e125
            (self.group2() * Simd32x3::from(other[scalar])),
        );
    }
}
impl Wedge<VersorEven> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[2]]))
                + (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + Simd32x4::from([
                    (-(self.group2()[0] * other.group3()[3]) - (self.group1()[1] * other.group3()[2])),
                    (-(self.group2()[1] * other.group3()[3]) - (self.group1()[2] * other.group3()[0])),
                    (-(self.group2()[2] * other.group3()[3]) - (self.group1()[0] * other.group3()[1])),
                    ((self.group2()[1] * other.group3()[1]) + (self.group2()[0] * other.group3()[0])),
                ])),
            // e1234
            (-(self.group1()[3] * other.group3()[3]) - (self.group0()[2] * other.group3()[2]) - (self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
        );
    }
}
impl Wedge<VersorOdd> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       20        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(other.group0()[3])),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other.group0()[3])),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self.group2()[0] * other.group0()[3]),
                (self.group2()[1] * other.group0()[3]),
                (self.group2()[2] * other.group0()[3]),
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
        );
    }
}
impl InfixWedge for CircleRotor {}
impl Wedge<Dipole> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
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
impl Wedge<DipoleInversion> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
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
impl Wedge<DualNum> for CircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]])),
        );
    }
}
impl Wedge<FlatPoint> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (-(self.group1()[3] * other.group0()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Flector> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Flector) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (-(self.group1()[3] * other.group0()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Motor> for CircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other.group1()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]])),
        );
    }
}
impl Wedge<MultiVector> for CircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       25       41        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other.group0()[0])),
            // e423, e431, e412
            (self.group0() * Simd32x3::from(other.group0()[0])),
            // e235, e315, e125
            (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]])),
            // e4235, e4315, e4125, e3215
            ((swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[2]]))
                + (Simd32x4::from(other[e1]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + Simd32x4::from([
                    (-(self.group2()[0] * other.group1()[3]) - (self.group1()[1] * other.group1()[2])),
                    (-(self.group2()[1] * other.group1()[3]) - (self.group1()[2] * other.group1()[0])),
                    (-(self.group2()[2] * other.group1()[3]) - (self.group1()[0] * other.group1()[1])),
                    ((self.group2()[1] * other.group1()[1]) + (self.group2()[0] * other.group1()[0])),
                ])),
            // e1234
            (-(self.group1()[3] * other.group1()[3]) - (self.group0()[2] * other.group1()[2]) - (self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<RoundPoint> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[2]]))
                + (Simd32x4::from(other[e2]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + Simd32x4::from([
                    (-(self.group2()[0] * other.group0()[3]) - (self.group1()[1] * other.group0()[2])),
                    (-(self.group2()[1] * other.group0()[3]) - (self.group1()[2] * other.group0()[0])),
                    (-(self.group2()[2] * other.group0()[3]) - (self.group1()[0] * other.group0()[1])),
                    ((self.group2()[1] * other.group0()[1]) + (self.group2()[0] * other.group0()[0])),
                ])),
            // e1234
            (-(self.group1()[3] * other.group0()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Scalar> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(other[scalar])),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other[scalar])),
            // e235, e315, e125, e12345
            (self.group2() * Simd32x4::from(other[scalar])),
        );
    }
}
impl Wedge<VersorEven> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            ((swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[2]]))
                + (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + Simd32x4::from([
                    (-(self.group2()[0] * other.group3()[3]) - (self.group1()[1] * other.group3()[2])),
                    (-(self.group2()[1] * other.group3()[3]) - (self.group1()[2] * other.group3()[0])),
                    (-(self.group2()[2] * other.group3()[3]) - (self.group1()[0] * other.group3()[1])),
                    ((self.group2()[1] * other.group3()[1]) + (self.group2()[0] * other.group3()[0])),
                ])),
            // e1234
            (-(self.group1()[3] * other.group3()[3]) - (self.group0()[2] * other.group3()[2]) - (self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
        );
    }
}
impl Wedge<VersorOdd> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       10       21        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(other.group0()[3])),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other.group0()[3])),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self.group2()[0] * other.group0()[3]),
                (self.group2()[1] * other.group0()[3]),
                (self.group2()[2] * other.group0()[3]),
                ((self.group2()[3] * other.group0()[3])
                    - (self.group2()[2] * other.group0()[2])
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
        );
    }
}
impl InfixWedge for Dipole {}
impl Wedge<Circle> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: Circle) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
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
impl Wedge<CircleRotor> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
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
impl Wedge<Dipole> for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       25       30        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                ((self.group2()[2] * other.group0()[1]) - (self.group2()[1] * other.group0()[2])
                    + (self.group1()[3] * other.group1()[0])
                    + (self.group1()[0] * other.group1()[3])
                    + (self.group0()[1] * other.group2()[2])
                    - (self.group0()[2] * other.group2()[1])),
                (-(self.group2()[2] * other.group0()[0])
                    + (self.group2()[0] * other.group0()[2])
                    + (self.group1()[3] * other.group1()[1])
                    + (self.group1()[1] * other.group1()[3])
                    - (self.group0()[0] * other.group2()[2])
                    + (self.group0()[2] * other.group2()[0])),
                ((self.group2()[1] * other.group0()[0]) - (self.group2()[0] * other.group0()[1])
                    + (self.group1()[3] * other.group1()[2])
                    + (self.group1()[2] * other.group1()[3])
                    + (self.group0()[0] * other.group2()[1])
                    - (self.group0()[1] * other.group2()[0])),
                (-(self.group2()[2] * other.group1()[2])
                    - (self.group2()[1] * other.group1()[1])
                    - (self.group2()[0] * other.group1()[0])
                    - (self.group1()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group2()[0])
                    - (self.group1()[1] * other.group2()[1])),
            ]),
            // e1234
            (-(self.group1()[2] * other.group0()[2])
                - (self.group1()[1] * other.group0()[1])
                - (self.group1()[0] * other.group0()[0])
                - (self.group0()[2] * other.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])),
        );
    }
}
impl Wedge<DipoleInversion> for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       26        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       22       27        0
    //  no simd       25       30        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (-(swizzle!(other.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * other.group0()[1]) - (self.group2()[1] * other.group0()[2])
                        + (self.group1()[3] * other.group1()[0])
                        + (self.group1()[0] * other.group1()[3])
                        + (self.group0()[1] * other.group2()[2])),
                    (-(self.group2()[2] * other.group0()[0])
                        + (self.group2()[0] * other.group0()[2])
                        + (self.group1()[3] * other.group1()[1])
                        + (self.group1()[1] * other.group1()[3])
                        + (self.group0()[2] * other.group2()[0])),
                    ((self.group2()[1] * other.group0()[0]) - (self.group2()[0] * other.group0()[1])
                        + (self.group1()[3] * other.group1()[2])
                        + (self.group1()[2] * other.group1()[3])
                        + (self.group0()[0] * other.group2()[1])),
                    (-(self.group2()[2] * other.group1()[2])
                        - (self.group2()[1] * other.group1()[1])
                        - (self.group2()[0] * other.group1()[0])
                        - (self.group1()[0] * other.group2()[0])
                        - (self.group1()[1] * other.group2()[1])),
                ])),
            // e1234
            (-(self.group1()[2] * other.group0()[2])
                - (self.group1()[1] * other.group0()[1])
                - (self.group1()[0] * other.group0()[0])
                - (self.group0()[2] * other.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])),
        );
    }
}
impl Wedge<DualNum> for Dipole {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            (self.group0() * Simd32x3::from(other.group0()[0])),
            // e235, e315, e125
            (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])),
        );
    }
}
impl Wedge<FlatPoint> for Dipole {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group0()[3]) + (self.group0()[1] * other.group0()[2])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group0()[2] * other.group0()[0])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group0()[0] * other.group0()[1])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl Wedge<Flector> for Dipole {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group0()[3]) + (self.group0()[1] * other.group0()[2])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group0()[2] * other.group0()[0])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group0()[0] * other.group0()[1])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl Wedge<Line> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: Line) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (-(self.group1()[2] * other.group0()[2])
                - (self.group1()[1] * other.group0()[1])
                - (self.group1()[0] * other.group0()[0])
                - (self.group0()[2] * other.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])),
        );
    }
}
impl Wedge<Motor> for Dipole {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (self.group0()[0] * other.group1()[3]),
                (self.group0()[1] * other.group1()[3]),
                (self.group0()[2] * other.group1()[3]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] * other.group1()[3]),
                (self.group1()[1] * other.group1()[3]),
                (self.group1()[2] * other.group1()[3]),
                0.0,
            ]),
        );
    }
}
impl Wedge<MultiVector> for Dipole {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       44        0
    //    simd3        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       40       55        0
    //  no simd       54       80        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            (Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]])),
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other.group0()[0])),
            // e415, e425, e435, e321
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group1(), 0, 1, 2, 2))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group1()[3]) + (self.group0()[0] * other[e1])),
                    ((self.group2()[1] * other.group1()[3]) + (self.group0()[1] * other[e1])),
                    ((self.group2()[2] * other.group1()[3]) + (self.group0()[2] * other[e1])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
            // e423, e431, e412
            ((Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (swizzle!(self.group0(), 1, 2, 0) * Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]))
                - (swizzle!(self.group0(), 2, 0, 1) * Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]))),
            // e235, e315, e125
            ((swizzle!(self.group2(), 2, 0, 1) * Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]))
                + (Simd32x3::from(other[e1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (swizzle!(self.group2(), 1, 2, 0) * Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]))),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * other.group4()[1]) - (self.group2()[1] * other.group4()[2])
                        + (self.group1()[3] * other.group10()[2])
                        + (self.group1()[0] * other.group3()[3])
                        + (self.group0()[1] * other.group3()[2])),
                    (-(self.group2()[2] * other.group4()[0])
                        + (self.group2()[0] * other.group4()[2])
                        + (self.group1()[3] * other.group10()[1])
                        + (self.group1()[1] * other.group3()[3])
                        + (self.group0()[2] * other.group3()[0])),
                    ((self.group2()[1] * other.group4()[0]) - (self.group2()[0] * other.group4()[1])
                        + (self.group1()[3] * other.group10()[0])
                        + (self.group1()[2] * other.group3()[3])
                        + (self.group0()[0] * other.group3()[1])),
                    (-(self.group2()[2] * other.group10()[0])
                        - (self.group2()[1] * other.group10()[1])
                        - (self.group2()[0] * other.group10()[2])
                        - (self.group1()[0] * other.group3()[0])
                        - (self.group1()[1] * other.group3()[1])),
                ])),
            // e1234
            (-(self.group1()[2] * other.group4()[2])
                - (self.group1()[1] * other.group4()[1])
                - (self.group1()[0] * other.group4()[0])
                - (self.group0()[2] * other.group10()[0])
                - (self.group0()[0] * other.group10()[2])
                - (self.group0()[1] * other.group10()[1])),
            // e12, e31, e23
            (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[2], self.group1()[1], self.group1()[0]])),
        );
    }
}
impl Wedge<RoundPoint> for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        4        6        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       20       30        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            ((Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (swizzle!(self.group0(), 1, 2, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]))
                - (swizzle!(self.group0(), 2, 0, 1) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group0()[3]) + (self.group0()[0] * other[e2])),
                    ((self.group2()[1] * other.group0()[3]) + (self.group0()[1] * other[e2])),
                    ((self.group2()[2] * other.group0()[3]) + (self.group0()[2] * other[e2])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
            // e235, e315, e125
            ((swizzle!(self.group2(), 2, 0, 1) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]))
                + (Simd32x3::from(other[e2]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (swizzle!(self.group2(), 1, 2, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]))),
        );
    }
}
impl Wedge<Scalar> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other[scalar])),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(other[scalar])),
            // e15, e25, e35
            (self.group2() * Simd32x3::from(other[scalar])),
        );
    }
}
impl Wedge<VersorEven> for Dipole {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        0
    //    simd3        2        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       22       31        0
    //  no simd       29       40        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            ((Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (swizzle!(self.group0(), 1, 2, 0) * Simd32x3::from([other.group3()[2], other.group3()[0], other.group3()[1]]))
                - (swizzle!(self.group0(), 2, 0, 1) * Simd32x3::from([other.group3()[1], other.group3()[2], other.group3()[0]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group3(), 0, 1, 2, 2))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group3()[3]) + (self.group0()[0] * other.group2()[3])),
                    ((self.group2()[1] * other.group3()[3]) + (self.group0()[1] * other.group2()[3])),
                    ((self.group2()[2] * other.group3()[3]) + (self.group0()[2] * other.group2()[3])),
                    (-(self.group1()[0] * other.group3()[0]) - (self.group1()[1] * other.group3()[1])),
                ])),
            // e235, e315, e125, e12345
            Simd32x4::from([
                ((self.group2()[2] * other.group3()[1]) + (self.group1()[0] * other.group2()[3]) - (self.group2()[1] * other.group3()[2])),
                (-(self.group2()[2] * other.group3()[0]) + (self.group1()[1] * other.group2()[3]) + (self.group2()[0] * other.group3()[2])),
                ((self.group2()[1] * other.group3()[0]) + (self.group1()[2] * other.group2()[3]) - (self.group2()[0] * other.group3()[1])),
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
        );
    }
}
impl Wedge<VersorOdd> for Dipole {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       29        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       22       32        0
    //  no simd       25       40        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other.group0()[3])),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(other.group0()[3])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] * other.group0()[3]),
                (self.group2()[1] * other.group0()[3]),
                (self.group2()[2] * other.group0()[3]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(other.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * other.group0()[1]) - (self.group2()[1] * other.group0()[2])
                        + (self.group1()[3] * other.group1()[0])
                        + (self.group1()[0] * other.group1()[3])
                        + (self.group0()[1] * other.group2()[2])),
                    (-(self.group2()[2] * other.group0()[0])
                        + (self.group2()[0] * other.group0()[2])
                        + (self.group1()[3] * other.group1()[1])
                        + (self.group1()[1] * other.group1()[3])
                        + (self.group0()[2] * other.group2()[0])),
                    ((self.group2()[1] * other.group0()[0]) - (self.group2()[0] * other.group0()[1])
                        + (self.group1()[3] * other.group1()[2])
                        + (self.group1()[2] * other.group1()[3])
                        + (self.group0()[0] * other.group2()[1])),
                    (-(self.group2()[2] * other.group1()[2])
                        - (self.group2()[1] * other.group1()[1])
                        - (self.group2()[0] * other.group1()[0])
                        - (self.group1()[0] * other.group2()[0])
                        - (self.group1()[1] * other.group2()[1])),
                ])),
        );
    }
}
impl InfixWedge for DipoleInversion {}
impl Wedge<Circle> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: Circle) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
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
impl Wedge<CircleRotor> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
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
impl Wedge<Dipole> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       26        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       22       27        0
    //  no simd       25       30        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * other.group0()[1])
                        + (self.group1()[3] * other.group1()[0])
                        + (self.group1()[0] * other.group1()[3])
                        + (self.group0()[1] * other.group2()[2])
                        - (self.group0()[2] * other.group2()[1])),
                    ((self.group2()[0] * other.group0()[2]) + (self.group1()[3] * other.group1()[1]) + (self.group1()[1] * other.group1()[3])
                        - (self.group0()[0] * other.group2()[2])
                        + (self.group0()[2] * other.group2()[0])),
                    ((self.group2()[1] * other.group0()[0])
                        + (self.group1()[3] * other.group1()[2])
                        + (self.group1()[2] * other.group1()[3])
                        + (self.group0()[0] * other.group2()[1])
                        - (self.group0()[1] * other.group2()[0])),
                    (-(self.group2()[1] * other.group1()[1])
                        - (self.group2()[0] * other.group1()[0])
                        - (self.group1()[2] * other.group2()[2])
                        - (self.group1()[0] * other.group2()[0])
                        - (self.group1()[1] * other.group2()[1])),
                ])),
            // e1234
            (-(self.group1()[2] * other.group0()[2])
                - (self.group1()[1] * other.group0()[1])
                - (self.group1()[0] * other.group0()[0])
                - (self.group0()[2] * other.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])),
        );
    }
}
impl Wedge<DipoleInversion> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       22        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       19       24        0
    //  no simd       25       30        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                - (swizzle!(other.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * other.group0()[1])
                        + (self.group1()[3] * other.group1()[0])
                        + (self.group1()[0] * other.group1()[3])
                        + (self.group0()[1] * other.group2()[2])),
                    ((self.group2()[0] * other.group0()[2])
                        + (self.group1()[3] * other.group1()[1])
                        + (self.group1()[1] * other.group1()[3])
                        + (self.group0()[2] * other.group2()[0])),
                    ((self.group2()[1] * other.group0()[0])
                        + (self.group1()[3] * other.group1()[2])
                        + (self.group1()[2] * other.group1()[3])
                        + (self.group0()[0] * other.group2()[1])),
                    (-(self.group2()[1] * other.group1()[1])
                        - (self.group2()[0] * other.group1()[0])
                        - (self.group1()[0] * other.group2()[0])
                        - (self.group1()[1] * other.group2()[1])),
                ])),
            // e1234
            (-(self.group1()[2] * other.group0()[2])
                - (self.group1()[1] * other.group0()[1])
                - (self.group1()[0] * other.group0()[0])
                - (self.group0()[2] * other.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])),
        );
    }
}
impl Wedge<DualNum> for DipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]])),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] * other.group0()[0]),
                (self.group1()[1] * other.group0()[0]),
                (self.group1()[2] * other.group0()[0]),
                0.0,
            ]),
        );
    }
}
impl Wedge<FlatPoint> for DipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group0()[3]) + (self.group0()[1] * other.group0()[2])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group0()[2] * other.group0()[0])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group0()[0] * other.group0()[1])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl Wedge<Flector> for DipoleInversion {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group0()[3]) + (self.group0()[1] * other.group0()[2])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group0()[2] * other.group0()[0])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group0()[0] * other.group0()[1])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl Wedge<Line> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: Line) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (-(self.group1()[2] * other.group0()[2])
                - (self.group1()[1] * other.group0()[1])
                - (self.group1()[0] * other.group0()[0])
                - (self.group0()[2] * other.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])),
        );
    }
}
impl Wedge<Motor> for DipoleInversion {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       13        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (self.group0()[0] * other.group1()[3]),
                (self.group0()[1] * other.group1()[3]),
                (self.group0()[2] * other.group1()[3]),
                ((self.group2()[3] * other.group1()[3])
                    - (self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] * other.group1()[3]),
                (self.group1()[1] * other.group1()[3]),
                (self.group1()[2] * other.group1()[3]),
                0.0,
            ]),
        );
    }
}
impl Wedge<MultiVector> for DipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       52        0
    //    simd3        3        6        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       46       63        0
    //  no simd       64       90        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            (Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]])),
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other.group0()[0])),
            // e415, e425, e435, e321
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group1(), 0, 1, 2, 2))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group1()[3]) + (self.group0()[0] * other[e1])),
                    ((self.group2()[1] * other.group1()[3]) + (self.group0()[1] * other[e1])),
                    ((self.group2()[2] * other.group1()[3]) + (self.group0()[2] * other[e1])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
            // e423, e431, e412
            ((Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (swizzle!(self.group0(), 1, 2, 0) * Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]))
                - (swizzle!(self.group0(), 2, 0, 1) * Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]))),
            // e235, e315, e125
            ((Simd32x3::from(other[e1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + Simd32x3::from([
                    ((self.group2()[2] * other.group1()[1]) - (self.group2()[1] * other.group1()[2])),
                    (-(self.group2()[2] * other.group1()[0]) + (self.group2()[0] * other.group1()[2])),
                    ((self.group2()[1] * other.group1()[0]) - (self.group2()[0] * other.group1()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            ((self.group3() * Simd32x4::from(other.group0()[0]))
                - (swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group10()[0]]))
                - (swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * other.group4()[1])
                        + (self.group1()[3] * other.group10()[2])
                        + (self.group1()[0] * other.group3()[3])
                        + (self.group0()[1] * other.group3()[2])),
                    ((self.group2()[0] * other.group4()[2])
                        + (self.group1()[3] * other.group10()[1])
                        + (self.group1()[1] * other.group3()[3])
                        + (self.group0()[2] * other.group3()[0])),
                    ((self.group2()[1] * other.group4()[0])
                        + (self.group1()[3] * other.group10()[0])
                        + (self.group1()[2] * other.group3()[3])
                        + (self.group0()[0] * other.group3()[1])),
                    (-(self.group2()[1] * other.group10()[1])
                        - (self.group2()[0] * other.group10()[2])
                        - (self.group1()[0] * other.group3()[0])
                        - (self.group1()[1] * other.group3()[1])),
                ])),
            // e1234
            ((self.group2()[3] * other.group0()[0])
                - (self.group1()[2] * other.group4()[2])
                - (self.group1()[1] * other.group4()[1])
                - (self.group1()[0] * other.group4()[0])
                - (self.group0()[2] * other.group10()[0])
                - (self.group0()[0] * other.group10()[2])
                - (self.group0()[1] * other.group10()[1])),
            // e12, e31, e23
            (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[2], self.group1()[1], self.group1()[0]])),
        );
    }
}
impl Wedge<RoundPoint> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        2        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       24       38        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            ((Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (swizzle!(self.group0(), 1, 2, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]))
                - (swizzle!(self.group0(), 2, 0, 1) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group0()[3]) + (self.group0()[0] * other[e2])),
                    ((self.group2()[1] * other.group0()[3]) + (self.group0()[1] * other[e2])),
                    ((self.group2()[2] * other.group0()[3]) + (self.group0()[2] * other[e2])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
            // e235, e315, e125, e12345
            ((swizzle!(other.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[3]]))
                + (Simd32x4::from(other[e2]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]))
                + Simd32x4::from([
                    ((self.group2()[1] * other.group0()[2]) * -1.0),
                    ((self.group2()[2] * other.group0()[0]) * -1.0),
                    ((self.group2()[0] * other.group0()[1]) * -1.0),
                    ((self.group3()[2] * other.group0()[2]) + (self.group3()[1] * other.group0()[1]) + (self.group3()[0] * other.group0()[0])),
                ])),
        );
    }
}
impl Wedge<Scalar> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other[scalar])),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(other[scalar])),
            // e15, e25, e35, e1234
            (self.group2() * Simd32x4::from(other[scalar])),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(other[scalar])),
        );
    }
}
impl Wedge<VersorEven> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       20        0
    //    simd3        2        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       21       27        0
    //  no simd       37       45        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            ((Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (swizzle!(self.group0(), 1, 2, 0) * Simd32x3::from([other.group3()[2], other.group3()[0], other.group3()[1]]))
                - (swizzle!(self.group0(), 2, 0, 1) * Simd32x3::from([other.group3()[1], other.group3()[2], other.group3()[0]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group3(), 0, 1, 2, 2))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group3()[3]) + (self.group0()[0] * other.group2()[3])),
                    ((self.group2()[1] * other.group3()[3]) + (self.group0()[1] * other.group2()[3])),
                    ((self.group2()[2] * other.group3()[3]) + (self.group0()[2] * other.group2()[3])),
                    (-(self.group1()[0] * other.group3()[0]) - (self.group1()[1] * other.group3()[1])),
                ])),
            // e235, e315, e125, e12345
            ((swizzle!(other.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[3]]))
                + (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]))
                - (swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group0()[2]]))
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
        );
    }
}
impl Wedge<VersorOdd> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       26        0
    //    simd3        0        1        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       30       45        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(other.group0()[3])),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(other.group0()[3])),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] * other.group0()[3]),
                (self.group2()[1] * other.group0()[3]),
                (self.group2()[2] * other.group0()[3]),
                ((self.group2()[3] * other.group0()[3])
                    - (self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e4235, e4315, e4125, e3215
            ((self.group3() * Simd32x4::from(other.group0()[3]))
                - (swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                - (swizzle!(other.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * other.group0()[1])
                        + (self.group1()[3] * other.group1()[0])
                        + (self.group1()[0] * other.group1()[3])
                        + (self.group0()[1] * other.group2()[2])),
                    ((self.group2()[0] * other.group0()[2])
                        + (self.group1()[3] * other.group1()[1])
                        + (self.group1()[1] * other.group1()[3])
                        + (self.group0()[2] * other.group2()[0])),
                    ((self.group2()[1] * other.group0()[0])
                        + (self.group1()[3] * other.group1()[2])
                        + (self.group1()[2] * other.group1()[3])
                        + (self.group0()[0] * other.group2()[1])),
                    (-(self.group2()[1] * other.group1()[1])
                        - (self.group2()[0] * other.group1()[0])
                        - (self.group1()[0] * other.group2()[0])
                        - (self.group1()[1] * other.group2()[1])),
                ])),
        );
    }
}
impl InfixWedge for DualNum {}
impl Wedge<Circle> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Circle) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
        );
    }
}
impl Wedge<CircleRotor> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
        );
    }
}
impl Wedge<Dipole> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(self.group0()[0]) * other.group0()),
            // e235, e315, e125
            (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
        );
    }
}
impl Wedge<DipoleInversion> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]])),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[0] * other.group1()[0]),
                (self.group0()[0] * other.group1()[1]),
                (self.group0()[0] * other.group1()[2]),
                0.0,
            ]),
        );
    }
}
impl Wedge<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1       11        0
    //  no simd        1       25        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, ((self.group0()[0] * other[e35]) + (self.group0()[1] * other.group0()[0]))]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (self.group0()[0] * other.group0()[0]),
            // e15, e25, e35, e45
            (Simd32x4::from(self.group0()[0]) * other.group1() * Simd32x4::from(-1.0)),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group4()[0]),
                (self.group0()[0] * other.group4()[1]),
                (self.group0()[0] * other.group4()[2]),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from(self.group0()[0]) * swizzle!(other.group10(), 2, 1, 0)),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group6()[0], other.group6()[1], other.group6()[2], other.group5()[3]]) * Simd32x4::from(-1.0)),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<RoundPoint> for DualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ (Simd32x4::from(self.group0()[0]) * other.group0() * Simd32x4::from(-1.0)));
    }
}
impl Wedge<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* e5, e12345 */ (self.group0() * Simd32x2::from(other[scalar])));
    }
}
impl Wedge<Sphere> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self.group0()[0] * other[e4315]));
    }
}
impl Wedge<VersorEven> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from(self.group0()[0]) * other.group3() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
        );
    }
}
impl Wedge<VersorOdd> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (self.group0()[0] * other.group0()[0]),
                (self.group0()[0] * other.group0()[1]),
                (self.group0()[0] * other.group0()[2]),
                ((self.group0()[0] * other.group2()[3]) + (self.group0()[1] * other.group0()[3])),
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]])),
        );
    }
}
impl InfixWedge for FlatPoint {}
impl Wedge<Circle> for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Circle) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (-(self.group0()[3] * other.group1()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<CircleRotor> for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (-(self.group0()[3] * other.group1()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Dipole> for FlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group1()[0]) + (self.group0()[2] * other.group0()[1])),
                    ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group0()[2])),
                    ((self.group0()[3] * other.group1()[2]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<DipoleInversion> for FlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group1()[0]) + (self.group0()[2] * other.group0()[1])),
                    ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group0()[2])),
                    ((self.group0()[3] * other.group1()[2]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<MultiVector> for FlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       26        0
    //  no simd       17       32        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (-(self.group0()[3] * other.group5()[3])
                    - (self.group0()[2] * other.group6()[2])
                    - (self.group0()[0] * other.group6()[0])
                    - (self.group0()[1] * other.group6()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            (self.group0() * Simd32x4::from(other.group0()[0])),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((self.group0()[0] * other.group1()[3]) - (self.group0()[3] * other.group1()[0])),
                ((self.group0()[1] * other.group1()[3]) - (self.group0()[3] * other.group1()[1])),
                ((self.group0()[2] * other.group1()[3]) - (self.group0()[3] * other.group1()[2])),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([
                (-(self.group0()[1] * other.group1()[2]) + (self.group0()[2] * other.group1()[1])),
                ((self.group0()[0] * other.group1()[2]) - (self.group0()[2] * other.group1()[0])),
                (-(self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
            ]),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group10()[0]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group10()[2]) + (self.group0()[2] * other.group4()[1])),
                    ((self.group0()[3] * other.group10()[1]) + (self.group0()[0] * other.group4()[2])),
                    ((self.group0()[3] * other.group10()[0]) + (self.group0()[1] * other.group4()[0])),
                    (-(self.group0()[0] * other.group10()[2]) - (self.group0()[1] * other.group10()[1])),
                ])),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<RoundPoint> for FlatPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       12        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            ((Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e235, e315, e125
            Simd32x3::from([
                (-(self.group0()[1] * other.group0()[2]) + (self.group0()[2] * other.group0()[1])),
                ((self.group0()[0] * other.group0()[2]) - (self.group0()[2] * other.group0()[0])),
                (-(self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0])),
            ]),
        );
    }
}
impl Wedge<Scalar> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * Simd32x4::from(other[scalar])));
    }
}
impl Wedge<VersorEven> for FlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        9       16        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (-(Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group1()[3]]))
                + Simd32x4::from([
                    (self.group0()[0] * other.group3()[3]),
                    (self.group0()[1] * other.group3()[3]),
                    (self.group0()[2] * other.group3()[3]),
                    (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-(self.group0()[1] * other.group3()[2]) + (self.group0()[2] * other.group3()[1])),
                ((self.group0()[0] * other.group3()[2]) - (self.group0()[2] * other.group3()[0])),
                (-(self.group0()[0] * other.group3()[1]) + (self.group0()[1] * other.group3()[0])),
                0.0,
            ]),
        );
    }
}
impl Wedge<VersorOdd> for FlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (self.group0() * Simd32x4::from(other.group0()[3])),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group1()[0]) + (self.group0()[2] * other.group0()[1])),
                    ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group0()[2])),
                    ((self.group0()[3] * other.group1()[2]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl InfixWedge for Flector {}
impl Wedge<Circle> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Circle) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (-(self.group0()[3] * other.group1()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<CircleRotor> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (-(self.group0()[3] * other.group1()[3]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Dipole> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group1()[0]) + (self.group0()[2] * other.group0()[1])),
                    ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group0()[2])),
                    ((self.group0()[3] * other.group1()[2]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<DipoleInversion> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group1()[0]) + (self.group0()[2] * other.group0()[1])),
                    ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group0()[2])),
                    ((self.group0()[3] * other.group1()[2]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       25       40        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                ((self.group1()[3] * other.group1()[3]) + (self.group1()[2] * other.group1()[2]) + (self.group1()[1] * other.group1()[1]) + (self.group1()[0] * other.group1()[0])
                    - (self.group0()[3] * other.group5()[3])
                    - (self.group0()[2] * other.group6()[2])
                    - (self.group0()[0] * other.group6()[0])
                    - (self.group0()[1] * other.group6()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            (self.group0() * Simd32x4::from(other.group0()[0])),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((self.group0()[0] * other.group1()[3]) - (self.group0()[3] * other.group1()[0])),
                ((self.group0()[1] * other.group1()[3]) - (self.group0()[3] * other.group1()[1])),
                ((self.group0()[2] * other.group1()[3]) - (self.group0()[3] * other.group1()[2])),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([
                (-(self.group0()[1] * other.group1()[2]) + (self.group0()[2] * other.group1()[1])),
                ((self.group0()[0] * other.group1()[2]) - (self.group0()[2] * other.group1()[0])),
                (-(self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
            ]),
            // e4235, e4315, e4125, e3215
            ((self.group1() * Simd32x4::from(other.group0()[0]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group10()[0]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group10()[2]) + (self.group0()[2] * other.group4()[1])),
                    ((self.group0()[3] * other.group10()[1]) + (self.group0()[0] * other.group4()[2])),
                    ((self.group0()[3] * other.group10()[0]) + (self.group0()[1] * other.group4()[0])),
                    (-(self.group0()[0] * other.group10()[2]) - (self.group0()[1] * other.group10()[1])),
                ])),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<RoundPoint> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       16        0
    //  no simd        9       19        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group0()[0]) * -1.0),
                    ((self.group0()[3] * other.group0()[1]) * -1.0),
                    ((self.group0()[3] * other.group0()[2]) * -1.0),
                    ((self.group1()[2] * other.group0()[2]) + (self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1])),
                ])),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-(self.group0()[1] * other.group0()[2]) + (self.group0()[2] * other.group0()[1])),
                ((self.group0()[0] * other.group0()[2]) - (self.group0()[2] * other.group0()[0])),
                (-(self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0])),
                0.0,
            ]),
        );
    }
}
impl Wedge<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            (self.group0() * Simd32x4::from(other[scalar])),
            // e4235, e4315, e4125, e3215
            (self.group1() * Simd32x4::from(other[scalar])),
        );
    }
}
impl Wedge<VersorEven> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       16       20        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x4::from(other.group3()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                - (Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group1()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    ((self.group1()[2] * other.group3()[2]) + (self.group1()[1] * other.group3()[1]) + (self.group1()[0] * other.group3()[0])
                        - (self.group0()[2] * other.group0()[2])
                        - (self.group0()[0] * other.group0()[0])
                        - (self.group0()[1] * other.group0()[1])),
                ])),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-(self.group0()[1] * other.group3()[2]) + (self.group0()[2] * other.group3()[1])),
                ((self.group0()[0] * other.group3()[2]) - (self.group0()[2] * other.group3()[0])),
                (-(self.group0()[0] * other.group3()[1]) + (self.group0()[1] * other.group3()[0])),
                0.0,
            ]),
        );
    }
}
impl Wedge<VersorOdd> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (self.group0() * Simd32x4::from(other.group0()[3])),
            // e4235, e4315, e4125, e3215
            ((self.group1() * Simd32x4::from(other.group0()[3]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group1()[0]) + (self.group0()[2] * other.group0()[1])),
                    ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group0()[2])),
                    ((self.group0()[3] * other.group1()[2]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl InfixWedge for Line {}
impl Wedge<Dipole> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (-(self.group1()[2] * other.group0()[2])
                - (self.group1()[1] * other.group0()[1])
                - (self.group1()[0] * other.group0()[0])
                - (self.group0()[2] * other.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])),
        );
    }
}
impl Wedge<DipoleInversion> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (-(self.group1()[2] * other.group0()[2])
                - (self.group1()[1] * other.group0()[1])
                - (self.group1()[0] * other.group0()[0])
                - (self.group0()[2] * other.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])),
        );
    }
}
impl Wedge<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       13       24        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (-(self.group1()[2] * other.group4()[2])
                    - (self.group1()[1] * other.group4()[1])
                    - (self.group1()[0] * other.group4()[0])
                    - (self.group0()[2] * other.group10()[0])
                    - (self.group0()[0] * other.group10()[2])
                    - (self.group0()[1] * other.group10()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group0()[0]),
                (self.group0()[1] * other.group0()[0]),
                (self.group0()[2] * other.group0()[0]),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(other.group0()[0])),
            // e4235, e4315, e4125, e3215
            ((swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[0] * other.group1()[3]) - (self.group0()[1] * other.group1()[2])),
                    (-(self.group1()[1] * other.group1()[3]) - (self.group0()[2] * other.group1()[0])),
                    (-(self.group1()[2] * other.group1()[3]) - (self.group0()[0] * other.group1()[1])),
                    ((self.group1()[0] * other.group1()[0]) + (self.group1()[1] * other.group1()[1])),
                ])),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<RoundPoint> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[0] * other.group0()[3]) - (self.group0()[1] * other.group0()[2])),
                    (-(self.group1()[1] * other.group0()[3]) - (self.group0()[2] * other.group0()[0])),
                    (-(self.group1()[2] * other.group0()[3]) - (self.group0()[0] * other.group0()[1])),
                    ((self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl Wedge<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e415, e425, e435
            (self.group0() * Simd32x3::from(other[scalar])),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(other[scalar])),
        );
    }
}
impl Wedge<VersorEven> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            ((swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[0] * other.group3()[3]) - (self.group0()[1] * other.group3()[2])),
                    (-(self.group1()[1] * other.group3()[3]) - (self.group0()[2] * other.group3()[0])),
                    (-(self.group1()[2] * other.group3()[3]) - (self.group0()[0] * other.group3()[1])),
                    ((self.group1()[0] * other.group3()[0]) + (self.group1()[1] * other.group3()[1])),
                ])),
        );
    }
}
impl Wedge<VersorOdd> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
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
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] * other.group0()[3]),
                (self.group1()[1] * other.group0()[3]),
                (self.group1()[2] * other.group0()[3]),
                0.0,
            ]),
        );
    }
}
impl InfixWedge for Motor {}
impl Wedge<Circle> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Circle) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
        );
    }
}
impl Wedge<CircleRotor> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
        );
    }
}
impl Wedge<Dipole> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (self.group1()[3] * other.group0()[0]),
                (self.group1()[3] * other.group0()[1]),
                (self.group1()[3] * other.group0()[2]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[3] * other.group1()[0]),
                (self.group1()[3] * other.group1()[1]),
                (self.group1()[3] * other.group1()[2]),
                0.0,
            ]),
        );
    }
}
impl Wedge<DipoleInversion> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       13        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (self.group1()[3] * other.group0()[0]),
                (self.group1()[3] * other.group0()[1]),
                (self.group1()[3] * other.group0()[2]),
                ((self.group1()[3] * other.group2()[3])
                    - (self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[3] * other.group1()[0]),
                (self.group1()[3] * other.group1()[1]),
                (self.group1()[3] * other.group1()[2]),
                0.0,
            ]),
        );
    }
}
impl Wedge<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        1        2        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       17       29        0
    //  no simd       25       45        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                ((self.group1()[3] * other[e35]) - (self.group1()[2] * other.group4()[2]) - (self.group1()[1] * other.group4()[1]) - (self.group1()[0] * other.group4()[0])
                    + (self.group0()[3] * other.group0()[0])
                    - (self.group0()[2] * other.group10()[0])
                    - (self.group0()[0] * other.group10()[2])
                    - (self.group0()[1] * other.group10()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (self.group1()[3] * other.group0()[0]),
            // e15, e25, e35, e45
            (Simd32x4::from(self.group1()[3]) * other.group1() * Simd32x4::from(-1.0)),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((self.group0()[0] * other.group0()[0]) + (self.group1()[3] * other.group4()[0])),
                ((self.group0()[1] * other.group0()[0]) + (self.group1()[3] * other.group4()[1])),
                ((self.group0()[2] * other.group0()[0]) + (self.group1()[3] * other.group4()[2])),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            ((Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group1()[3]) * swizzle!(other.group10(), 2, 1, 0))),
            // e4235, e4315, e4125, e3215
            (-(Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group6()[0], other.group6()[1], other.group6()[2], other.group5()[3]]))
                + (swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[0] * other.group1()[3]) - (self.group0()[1] * other.group1()[2])),
                    (-(self.group1()[1] * other.group1()[3]) - (self.group0()[2] * other.group1()[0])),
                    (-(self.group1()[2] * other.group1()[3]) - (self.group0()[0] * other.group1()[1])),
                    ((self.group1()[0] * other.group1()[0]) + (self.group1()[1] * other.group1()[1])),
                ])),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<RoundPoint> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        8       20        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from(self.group1()[3]) * other.group0() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            ((swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[0] * other.group0()[3]) - (self.group0()[1] * other.group0()[2])),
                    (-(self.group1()[1] * other.group0()[3]) - (self.group0()[2] * other.group0()[0])),
                    (-(self.group1()[2] * other.group0()[3]) - (self.group0()[0] * other.group0()[1])),
                    ((self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl Wedge<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (self.group0() * Simd32x4::from(other[scalar])),
            // e235, e315, e125, e5
            (self.group1() * Simd32x4::from(other[scalar])),
        );
    }
}
impl Wedge<Sphere> for Motor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self.group1()[3] * other[e4315]));
    }
}
impl Wedge<VersorEven> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from(self.group1()[3]) * other.group3() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (-(Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                + (swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    (-(self.group1()[0] * other.group3()[3]) - (self.group0()[1] * other.group3()[2])),
                    (-(self.group1()[1] * other.group3()[3]) - (self.group0()[2] * other.group3()[0])),
                    (-(self.group1()[2] * other.group3()[3]) - (self.group0()[0] * other.group3()[1])),
                    ((self.group1()[0] * other.group3()[0]) + (self.group1()[1] * other.group3()[1])),
                ])),
        );
    }
}
impl Wedge<VersorOdd> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       16       21        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            ((self.group0() * Simd32x4::from(other.group0()[3]))
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]))
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
            // e235, e315, e125, e5
            Simd32x4::from([
                ((self.group1()[0] * other.group0()[3]) + (self.group1()[3] * other.group1()[0])),
                ((self.group1()[1] * other.group0()[3]) + (self.group1()[3] * other.group1()[1])),
                ((self.group1()[2] * other.group0()[3]) + (self.group1()[3] * other.group1()[2])),
                (self.group1()[3] * other.group0()[3]),
            ]),
        );
    }
}
impl InfixWedge for MultiVector {}
impl Wedge<AntiScalar> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self.group0()[0] * other[e12345]));
    }
}
impl Wedge<Circle> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       22        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       27        0
    //  no simd       24       40        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x4::from(self.group0()[0]) * other.group1()),
            // e423, e431, e412
            (Simd32x3::from(self.group0()[0]) * other.group0()),
            // e235, e315, e125
            (Simd32x3::from(self.group0()[0]) * other.group2()),
            // e4235, e4315, e4125, e3215
            (-(Simd32x4::from(self[e1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group2()[0]) + (self.group1()[2] * other.group1()[1])),
                    ((self.group1()[3] * other.group2()[1]) + (self.group1()[0] * other.group1()[2])),
                    ((self.group1()[3] * other.group2()[2]) + (self.group1()[1] * other.group1()[0])),
                    (-(self.group1()[0] * other.group2()[0]) - (self.group1()[1] * other.group2()[1])),
                ])),
            // e1234
            ((self.group1()[3] * other.group1()[3]) + (self.group1()[2] * other.group0()[2]) + (self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1])),
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<CircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       23        0
    //    simd3        0        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       25       41        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            (Simd32x4::from(self.group0()[0]) * other.group1()),
            // e423, e431, e412
            (Simd32x3::from(self.group0()[0]) * other.group0()),
            // e235, e315, e125
            (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]])),
            // e4235, e4315, e4125, e3215
            (-(Simd32x4::from(self[e1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group2()[0]) + (self.group1()[2] * other.group1()[1])),
                    ((self.group1()[3] * other.group2()[1]) + (self.group1()[0] * other.group1()[2])),
                    ((self.group1()[3] * other.group2()[2]) + (self.group1()[1] * other.group1()[0])),
                    (-(self.group1()[0] * other.group2()[0]) - (self.group1()[1] * other.group2()[1])),
                ])),
            // e1234
            ((self.group1()[3] * other.group1()[3]) + (self.group1()[2] * other.group0()[2]) + (self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1])),
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<Dipole> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       44        0
    //    simd3        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       40       55        0
    //  no simd       54       80        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            (Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (Simd32x3::from(self.group0()[0]) * other.group0()),
            // e415, e425, e435, e321
            (-(swizzle!(self.group1(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self[e1] * other.group0()[0]) + (self.group1()[3] * other.group2()[0])),
                    ((self[e1] * other.group0()[1]) + (self.group1()[3] * other.group2()[1])),
                    ((self[e1] * other.group0()[2]) + (self.group1()[3] * other.group2()[2])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
            // e423, e431, e412
            ((Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (swizzle!(other.group0(), 2, 0, 1) * Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]))
                + (swizzle!(other.group0(), 1, 2, 0) * Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]))),
            // e235, e315, e125
            ((Simd32x3::from(self[e1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (swizzle!(other.group2(), 2, 0, 1) * Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]))
                - (swizzle!(other.group2(), 1, 2, 0) * Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]))),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group10()[2] * other.group1()[3]) - (self.group4()[2] * other.group2()[1])
                        + (self.group4()[1] * other.group2()[2])
                        + (self.group3()[3] * other.group1()[0])
                        + (self.group3()[2] * other.group0()[1])),
                    ((self.group10()[1] * other.group1()[3]) + (self.group4()[2] * other.group2()[0]) - (self.group4()[0] * other.group2()[2])
                        + (self.group3()[3] * other.group1()[1])
                        + (self.group3()[0] * other.group0()[2])),
                    ((self.group10()[0] * other.group1()[3]) - (self.group4()[1] * other.group2()[0])
                        + (self.group4()[0] * other.group2()[1])
                        + (self.group3()[3] * other.group1()[2])
                        + (self.group3()[1] * other.group0()[0])),
                    (-(self.group10()[2] * other.group2()[0])
                        - (self.group10()[1] * other.group2()[1])
                        - (self.group10()[0] * other.group2()[2])
                        - (self.group3()[0] * other.group1()[0])
                        - (self.group3()[1] * other.group1()[1])),
                ])),
            // e1234
            (-(self.group10()[2] * other.group0()[0])
                - (self.group10()[1] * other.group0()[1])
                - (self.group10()[0] * other.group0()[2])
                - (self.group4()[2] * other.group1()[2])
                - (self.group4()[0] * other.group1()[0])
                - (self.group4()[1] * other.group1()[1])),
            // e12, e31, e23
            (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
    }
}
impl Wedge<DipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       52        0
    //    simd3        3        6        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       46       63        0
    //  no simd       64       90        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            (Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]])),
            // e41, e42, e43
            (Simd32x3::from(self.group0()[0]) * other.group0()),
            // e415, e425, e435, e321
            (-(swizzle!(self.group1(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self[e1] * other.group0()[0]) + (self.group1()[3] * other.group2()[0])),
                    ((self[e1] * other.group0()[1]) + (self.group1()[3] * other.group2()[1])),
                    ((self[e1] * other.group0()[2]) + (self.group1()[3] * other.group2()[2])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
            // e423, e431, e412
            ((Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (swizzle!(other.group0(), 2, 0, 1) * Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]))
                + (swizzle!(other.group0(), 1, 2, 0) * Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]))),
            // e235, e315, e125
            ((Simd32x3::from(self[e1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + Simd32x3::from([
                    ((self.group1()[1] * other.group2()[2]) - (self.group1()[2] * other.group2()[1])),
                    (-(self.group1()[0] * other.group2()[2]) + (self.group1()[2] * other.group2()[0])),
                    ((self.group1()[0] * other.group2()[1]) - (self.group1()[1] * other.group2()[0])),
                ])),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(other.group2(), 1, 2, 0, 0) * Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group10()[2]]))
                + (Simd32x4::from(self.group0()[0]) * other.group3())
                - (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group10()[2] * other.group1()[3])
                        + (self.group4()[1] * other.group2()[2])
                        + (self.group3()[3] * other.group1()[0])
                        + (self.group3()[2] * other.group0()[1])),
                    ((self.group10()[1] * other.group1()[3])
                        + (self.group4()[2] * other.group2()[0])
                        + (self.group3()[3] * other.group1()[1])
                        + (self.group3()[0] * other.group0()[2])),
                    ((self.group10()[0] * other.group1()[3])
                        + (self.group4()[0] * other.group2()[1])
                        + (self.group3()[3] * other.group1()[2])
                        + (self.group3()[1] * other.group0()[0])),
                    (-(self.group10()[1] * other.group2()[1])
                        - (self.group10()[0] * other.group2()[2])
                        - (self.group3()[1] * other.group1()[1])
                        - (self.group3()[0] * other.group1()[0])),
                ])),
            // e1234
            (-(self.group10()[2] * other.group0()[0])
                - (self.group10()[1] * other.group0()[1])
                - (self.group10()[0] * other.group0()[2])
                - (self.group4()[2] * other.group1()[2])
                - (self.group4()[1] * other.group1()[1])
                + (self.group0()[0] * other.group2()[3])
                - (self.group4()[0] * other.group1()[0])),
            // e12, e31, e23
            (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])),
        );
    }
}
impl Wedge<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        9        0
    //  no simd        1       17        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, ((self.group0()[0] * other.group0()[1]) + (self[e35] * other.group0()[0]))]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (self.group0()[0] * other.group0()[0]),
            // e15, e25, e35, e45
            (self.group1() * Simd32x4::from(other.group0()[0])),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group4()[0] * other.group0()[0]),
                (self.group4()[1] * other.group0()[0]),
                (self.group4()[2] * other.group0()[0]),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (swizzle!(self.group10(), 2, 1, 0) * Simd32x3::from(other.group0()[0])),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], self.group5()[3]])),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<FlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       14       26        0
    //  no simd       17       32        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (-(self.group6()[2] * other.group0()[2])
                    - (self.group6()[1] * other.group0()[1])
                    - (self.group5()[3] * other.group0()[3])
                    - (self.group6()[0] * other.group0()[0])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            (Simd32x4::from(self.group0()[0]) * other.group0()),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group1()[0] * other.group0()[3]) + (self.group1()[3] * other.group0()[0])),
                (-(self.group1()[1] * other.group0()[3]) + (self.group1()[3] * other.group0()[1])),
                (-(self.group1()[2] * other.group0()[3]) + (self.group1()[3] * other.group0()[2])),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([
                ((self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1])),
                (-(self.group1()[0] * other.group0()[2]) + (self.group1()[2] * other.group0()[0])),
                ((self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0])),
            ]),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(other.group0(), 1, 2, 0, 0) * Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group10()[2]]))
                + Simd32x4::from([
                    ((self.group10()[2] * other.group0()[3]) + (self.group4()[1] * other.group0()[2])),
                    ((self.group10()[1] * other.group0()[3]) + (self.group4()[2] * other.group0()[0])),
                    ((self.group10()[0] * other.group0()[3]) + (self.group4()[0] * other.group0()[1])),
                    (-(self.group10()[0] * other.group0()[2]) - (self.group10()[1] * other.group0()[1])),
                ])),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       28        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       25       40        0
    fn wedge(self, other: Flector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (-(self.group6()[2] * other.group0()[2])
                    - (self.group6()[1] * other.group0()[1])
                    - (self.group6()[0] * other.group0()[0])
                    - (self.group5()[3] * other.group0()[3])
                    + (self.group1()[3] * other.group1()[3])
                    + (self.group1()[2] * other.group1()[2])
                    + (self.group1()[0] * other.group1()[0])
                    + (self.group1()[1] * other.group1()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            (Simd32x4::from(self.group0()[0]) * other.group0()),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (-(self.group1()[0] * other.group0()[3]) + (self.group1()[3] * other.group0()[0])),
                (-(self.group1()[1] * other.group0()[3]) + (self.group1()[3] * other.group0()[1])),
                (-(self.group1()[2] * other.group0()[3]) + (self.group1()[3] * other.group0()[2])),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([
                ((self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1])),
                (-(self.group1()[0] * other.group0()[2]) + (self.group1()[2] * other.group0()[0])),
                ((self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0])),
            ]),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(other.group0(), 1, 2, 0, 0) * Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group10()[2]]))
                + (Simd32x4::from(self.group0()[0]) * other.group1())
                + Simd32x4::from([
                    ((self.group10()[2] * other.group0()[3]) + (self.group4()[1] * other.group0()[2])),
                    ((self.group10()[1] * other.group0()[3]) + (self.group4()[2] * other.group0()[0])),
                    ((self.group10()[0] * other.group0()[3]) + (self.group4()[0] * other.group0()[1])),
                    (-(self.group10()[1] * other.group0()[1]) - (self.group10()[0] * other.group0()[2])),
                ])),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       19        0
    //  no simd       13       24        0
    fn wedge(self, other: Line) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (-(self.group10()[2] * other.group0()[0])
                    - (self.group10()[1] * other.group0()[1])
                    - (self.group10()[0] * other.group0()[2])
                    - (self.group4()[2] * other.group1()[2])
                    - (self.group4()[0] * other.group1()[0])
                    - (self.group4()[1] * other.group1()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group0()[0]),
                (self.group0()[0] * other.group0()[1]),
                (self.group0()[0] * other.group0()[2]),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from(self.group0()[0]) * other.group1()),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group1()[0]) + (self.group1()[2] * other.group0()[1])),
                    ((self.group1()[3] * other.group1()[1]) + (self.group1()[0] * other.group0()[2])),
                    ((self.group1()[3] * other.group1()[2]) + (self.group1()[1] * other.group0()[0])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        1        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       17       28        0
    //  no simd       25       41        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (-(self.group10()[2] * other.group0()[0]) - (self.group10()[1] * other.group0()[1]) - (self.group10()[0] * other.group0()[2]) + (self[e35] * other.group1()[3])
                    - (self.group4()[2] * other.group1()[2])
                    - (self.group4()[1] * other.group1()[1])
                    + (self.group0()[0] * other.group0()[3])
                    - (self.group4()[0] * other.group1()[0])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (self.group0()[0] * other.group1()[3]),
            // e15, e25, e35, e45
            (self.group1() * Simd32x4::from(other.group1()[3])),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                ((self.group0()[0] * other.group0()[0]) + (self.group4()[0] * other.group1()[3])),
                ((self.group0()[0] * other.group0()[1]) + (self.group4()[1] * other.group1()[3])),
                ((self.group0()[0] * other.group0()[2]) + (self.group4()[2] * other.group1()[3])),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            ((Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (swizzle!(self.group10(), 2, 1, 0) * Simd32x3::from(other.group1()[3]))),
            // e4235, e4315, e4125, e3215
            ((Simd32x4::from(other.group1()[3]) * Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], self.group5()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group1()[0]) + (self.group1()[2] * other.group0()[1])),
                    ((self.group1()[3] * other.group1()[1]) + (self.group1()[0] * other.group0()[2])),
                    ((self.group1()[3] * other.group1()[2]) + (self.group1()[1] * other.group0()[0])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       99      117        0
    //    simd3       16       18        0
    //    simd4       16       18        0
    // Totals...
    // yes simd      131      153        0
    //  no simd      211      243        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self.group0()[0] * other.group0()[0]),
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
            ]),
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group0()[0]) * other.group1()) + (self.group1() * Simd32x4::from(other.group0()[0]))),
            // e5
            ((self.group0()[0] * other[e1]) + (self[e1] * other.group0()[0])),
            // e15, e25, e35, e45
            ((self.group3() * Simd32x4::from(other.group0()[0])) - (Simd32x4::from(self[e1]) * other.group1())
                + (Simd32x4::from(self.group0()[0]) * other.group3())
                + (self.group1() * Simd32x4::from(other[e1]))),
            // e41, e42, e43
            ((self.group4() * Simd32x3::from(other.group0()[0]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from(self.group0()[0]) * other.group4())
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))),
            // e415, e425, e435, e321
            ((self.group5() * Simd32x4::from(other.group0()[0]))
                - (swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group10()[2]]))
                + (Simd32x4::from(self.group0()[0]) * other.group5())
                - (swizzle!(self.group1(), 0, 1, 2, 2) * Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group10()[0]]))
                + Simd32x4::from([
                    ((self.group4()[0] * other[e1]) + (self.group3()[0] * other.group1()[3]) + (self[e1] * other.group4()[0]) + (self.group1()[3] * other.group3()[0])),
                    ((self.group4()[1] * other[e1]) + (self.group3()[1] * other.group1()[3]) + (self[e1] * other.group4()[1]) + (self.group1()[3] * other.group3()[1])),
                    ((self.group4()[2] * other[e1]) + (self.group3()[2] * other.group1()[3]) + (self[e1] * other.group4()[2]) + (self.group1()[3] * other.group3()[2])),
                    (-(self.group10()[1] * other.group1()[1])
                        - (self.group10()[0] * other.group1()[2])
                        - (self.group1()[1] * other.group10()[1])
                        - (self.group1()[0] * other.group10()[2])),
                ])),
            // e423, e431, e412
            ((swizzle!(self.group10(), 2, 1, 0) * Simd32x3::from(other.group1()[3])) + (self.group6() * Simd32x3::from(other.group0()[0]))
                - (swizzle!(self.group4(), 2, 0, 1) * Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]))
                + (swizzle!(self.group4(), 1, 2, 0) * Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]))
                + (Simd32x3::from(self.group1()[3]) * swizzle!(other.group10(), 2, 1, 0))
                + (swizzle!(other.group4(), 1, 2, 0) * Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]))
                + (Simd32x3::from(self.group0()[0]) * other.group6())
                - (swizzle!(other.group4(), 2, 0, 1) * Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]))),
            // e235, e315, e125
            ((swizzle!(self.group10(), 2, 1, 0) * Simd32x3::from(other[e1]))
                + (self.group7() * Simd32x3::from(other.group0()[0]))
                + (Simd32x3::from(self[e1]) * swizzle!(other.group10(), 2, 1, 0))
                + (Simd32x3::from(self.group0()[0]) * other.group7())
                + Simd32x3::from([
                    ((self.group3()[2] * other.group1()[1]) - (self.group3()[1] * other.group1()[2]) - (self.group1()[2] * other.group3()[1])
                        + (self.group1()[1] * other.group3()[2])),
                    (-(self.group3()[2] * other.group1()[0]) + (self.group3()[0] * other.group1()[2]) + (self.group1()[2] * other.group3()[0])
                        - (self.group1()[0] * other.group3()[2])),
                    ((self.group3()[1] * other.group1()[0]) - (self.group3()[0] * other.group1()[1]) - (self.group1()[1] * other.group3()[0])
                        + (self.group1()[0] * other.group3()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            ((self.group8() * Simd32x4::from(other.group0()[0]))
                + (Simd32x4::from(other[e1]) * Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], self.group5()[3]]))
                + (swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group7()[2]]))
                - (swizzle!(other.group3(), 1, 2, 0, 0) * Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group10()[2]]))
                - (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group10()[0]]))
                - (Simd32x4::from(self[e1]) * Simd32x4::from([other.group6()[0], other.group6()[1], other.group6()[2], other.group5()[3]]))
                + (Simd32x4::from(self.group0()[0]) * other.group8())
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group5()[2], other.group5()[0], other.group5()[1], other.group7()[2]]))
                + Simd32x4::from([
                    ((self.group10()[2] * other.group3()[3]) - (self.group7()[0] * other.group1()[3]) - (self.group5()[1] * other.group1()[2])
                        + (self.group4()[1] * other.group3()[2])
                        + (self.group3()[3] * other.group10()[2])
                        + (self.group3()[2] * other.group4()[1])
                        + (self.group1()[3] * other.group7()[0])
                        + (self.group1()[2] * other.group5()[1])),
                    ((self.group10()[1] * other.group3()[3]) - (self.group7()[1] * other.group1()[3]) - (self.group5()[2] * other.group1()[0])
                        + (self.group4()[2] * other.group3()[0])
                        + (self.group3()[3] * other.group10()[1])
                        + (self.group3()[0] * other.group4()[2])
                        + (self.group1()[3] * other.group7()[1])
                        + (self.group1()[0] * other.group5()[2])),
                    ((self.group10()[0] * other.group3()[3]) - (self.group7()[2] * other.group1()[3]) - (self.group5()[0] * other.group1()[1])
                        + (self.group4()[0] * other.group3()[1])
                        + (self.group3()[3] * other.group10()[0])
                        + (self.group3()[1] * other.group4()[0])
                        + (self.group1()[3] * other.group7()[2])
                        + (self.group1()[1] * other.group5()[0])),
                    (-(self.group10()[1] * other.group3()[1]) - (self.group10()[0] * other.group3()[2])
                        + (self.group7()[1] * other.group1()[1])
                        + (self.group7()[0] * other.group1()[0])
                        - (self.group3()[1] * other.group10()[1])
                        - (self.group3()[0] * other.group10()[2])
                        - (self.group1()[1] * other.group7()[1])
                        - (self.group1()[0] * other.group7()[0])),
                ])),
            // e1234
            (-(self.group10()[2] * other.group4()[0]) - (self.group10()[1] * other.group4()[1]) - (self.group10()[0] * other.group4()[2]) + (self[e35] * other.group0()[0])
                - (self.group6()[2] * other.group1()[2])
                - (self.group6()[1] * other.group1()[1])
                - (self.group6()[0] * other.group1()[0])
                - (self.group5()[3] * other.group1()[3])
                - (self.group4()[2] * other.group10()[0])
                - (self.group4()[1] * other.group10()[1])
                - (self.group4()[0] * other.group10()[2])
                + (self.group1()[3] * other.group5()[3])
                + (self.group1()[2] * other.group6()[2])
                + (self.group1()[1] * other.group6()[1])
                + (self.group0()[0] * other[e35])
                + (self.group1()[0] * other.group6()[0])),
            // e12, e31, e23
            ((self.group10() * Simd32x3::from(other.group0()[0]))
                + (Simd32x3::from(self.group0()[0]) * other.group10())
                + Simd32x3::from([
                    (-(self.group1()[1] * other.group1()[0]) + (self.group1()[0] * other.group1()[1])),
                    ((self.group1()[2] * other.group1()[0]) - (self.group1()[0] * other.group1()[2])),
                    (-(self.group1()[2] * other.group1()[1]) + (self.group1()[1] * other.group1()[2])),
                ])),
        );
    }
}
impl Wedge<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn wedge(self, other: Plane) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                ((self.group1()[3] * other.group0()[3]) + (self.group1()[2] * other.group0()[2]) + (self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
            (Simd32x4::from(self.group0()[0]) * other.group0()),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<RoundPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       38        0
    //    simd3        4        6        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       29       50        0
    //  no simd       49       80        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                ((self[e35] * other[e2])
                    + (self.group8()[3] * other.group0()[3])
                    + (self.group8()[2] * other.group0()[2])
                    + (self.group8()[0] * other.group0()[0])
                    + (self.group8()[1] * other.group0()[1])),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[0]) * other.group0()),
            // e5
            (self.group0()[0] * other[e2]),
            // e15, e25, e35, e45
            ((self.group1() * Simd32x4::from(other[e2])) - (Simd32x4::from(self[e1]) * other.group0())),
            // e41, e42, e43
            (-(Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e415, e425, e435, e321
            (-(swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group10()[2]]))
                + Simd32x4::from([
                    ((self.group4()[0] * other[e2]) + (self.group3()[0] * other.group0()[3])),
                    ((self.group4()[1] * other[e2]) + (self.group3()[1] * other.group0()[3])),
                    ((self.group4()[2] * other[e2]) + (self.group3()[2] * other.group0()[3])),
                    (-(self.group10()[0] * other.group0()[2]) - (self.group10()[1] * other.group0()[1])),
                ])),
            // e423, e431, e412
            ((swizzle!(self.group10(), 2, 1, 0) * Simd32x3::from(other.group0()[3]))
                + (swizzle!(self.group4(), 1, 2, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]))
                - (swizzle!(self.group4(), 2, 0, 1) * Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]))),
            // e235, e315, e125
            ((swizzle!(self.group10(), 2, 1, 0) * Simd32x3::from(other[e2]))
                + Simd32x3::from([
                    (-(self.group3()[1] * other.group0()[2]) + (self.group3()[2] * other.group0()[1])),
                    ((self.group3()[0] * other.group0()[2]) - (self.group3()[2] * other.group0()[0])),
                    (-(self.group3()[0] * other.group0()[1]) + (self.group3()[1] * other.group0()[0])),
                ])),
            // e4235, e4315, e4125, e3215
            ((Simd32x4::from(other[e2]) * Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], self.group5()[3]]))
                + (swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group7()[2]]))
                + Simd32x4::from([
                    (-(self.group7()[0] * other.group0()[3]) - (self.group5()[1] * other.group0()[2])),
                    (-(self.group7()[1] * other.group0()[3]) - (self.group5()[2] * other.group0()[0])),
                    (-(self.group7()[2] * other.group0()[3]) - (self.group5()[0] * other.group0()[1])),
                    ((self.group7()[1] * other.group0()[1]) + (self.group7()[0] * other.group0()[0])),
                ])),
            // e1234
            (-(self.group6()[2] * other.group0()[2]) - (self.group6()[1] * other.group0()[1]) - (self.group5()[3] * other.group0()[3]) - (self.group6()[0] * other.group0()[0])),
            // e12, e31, e23
            Simd32x3::from([
                ((self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0])),
                (-(self.group1()[0] * other.group0()[2]) + (self.group1()[2] * other.group0()[0])),
                ((self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1])),
            ]),
        );
    }
}
impl Wedge<Scalar> for MultiVector {
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
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            (self.group0() * Simd32x2::from(other[scalar])),
            // e1, e2, e3, e4
            (self.group1() * Simd32x4::from(other[scalar])),
            // e5
            (self[e1] * other[scalar]),
            // e15, e25, e35, e45
            (self.group3() * Simd32x4::from(other[scalar])),
            // e41, e42, e43
            (self.group4() * Simd32x3::from(other[scalar])),
            // e415, e425, e435, e321
            (self.group5() * Simd32x4::from(other[scalar])),
            // e423, e431, e412
            (self.group6() * Simd32x3::from(other[scalar])),
            // e235, e315, e125
            (self.group7() * Simd32x3::from(other[scalar])),
            // e4235, e4315, e4125, e3215
            (self.group8() * Simd32x4::from(other[scalar])),
            // e1234
            (self[e35] * other[scalar]),
            // e12, e31, e23
            (self.group10() * Simd32x3::from(other[scalar])),
        );
    }
}
impl Wedge<Sphere> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                ((self[e1] * other[e4315])
                    + (self.group1()[3] * other.group0()[3])
                    + (self.group1()[2] * other.group0()[2])
                    + (self.group1()[0] * other.group0()[0])
                    + (self.group1()[1] * other.group0()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
            (Simd32x4::from(self.group0()[0]) * other.group0()),
            // e1234
            (self.group0()[0] * other[e4315]),
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<VersorEven> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       61        0
    //    simd3        6        8        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       57       78        0
    //  no simd       90      121        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[0]) * other.group3()),
            // e5
            (self.group0()[0] * other.group2()[3]),
            // e15, e25, e35, e45
            ((self.group1() * Simd32x4::from(other.group2()[3])) - (Simd32x4::from(self[e1]) * other.group3())),
            // e41, e42, e43
            (-(Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))),
            // e415, e425, e435, e321
            (-(swizzle!(other.group3(), 0, 1, 2, 0) * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group10()[2]]))
                + (Simd32x4::from(self.group0()[0]) * other.group1())
                + Simd32x4::from([
                    ((self.group4()[0] * other.group2()[3]) + (self.group3()[0] * other.group3()[3])),
                    ((self.group4()[1] * other.group2()[3]) + (self.group3()[1] * other.group3()[3])),
                    ((self.group4()[2] * other.group2()[3]) + (self.group3()[2] * other.group3()[3])),
                    (-(self.group10()[1] * other.group3()[1]) - (self.group10()[0] * other.group3()[2])),
                ])),
            // e423, e431, e412
            ((swizzle!(self.group10(), 2, 1, 0) * Simd32x3::from(other.group3()[3]))
                - (swizzle!(self.group4(), 2, 0, 1) * Simd32x3::from([other.group3()[1], other.group3()[2], other.group3()[0]]))
                + (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + (swizzle!(self.group4(), 1, 2, 0) * Simd32x3::from([other.group3()[2], other.group3()[0], other.group3()[1]]))),
            // e235, e315, e125
            ((swizzle!(self.group10(), 2, 1, 0) * Simd32x3::from(other.group2()[3]))
                + (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]))
                + Simd32x3::from([
                    ((self.group3()[2] * other.group3()[1]) - (self.group3()[1] * other.group3()[2])),
                    (-(self.group3()[2] * other.group3()[0]) + (self.group3()[0] * other.group3()[2])),
                    ((self.group3()[1] * other.group3()[0]) - (self.group3()[0] * other.group3()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            ((Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group6()[0], self.group6()[1], self.group6()[2], self.group5()[3]]))
                + (swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group5()[2], self.group5()[0], self.group5()[1], self.group7()[2]]))
                - (Simd32x4::from(self[e1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[2]]))
                + Simd32x4::from([
                    (-(self.group7()[0] * other.group3()[3]) - (self.group5()[1] * other.group3()[2])
                        + (self.group1()[3] * other.group2()[0])
                        + (self.group1()[2] * other.group1()[1])),
                    (-(self.group7()[1] * other.group3()[3]) - (self.group5()[2] * other.group3()[0])
                        + (self.group1()[3] * other.group2()[1])
                        + (self.group1()[0] * other.group1()[2])),
                    (-(self.group7()[2] * other.group3()[3]) - (self.group5()[0] * other.group3()[1])
                        + (self.group1()[3] * other.group2()[2])
                        + (self.group1()[1] * other.group1()[0])),
                    ((self.group7()[1] * other.group3()[1]) + (self.group7()[0] * other.group3()[0])
                        - (self.group1()[0] * other.group2()[0])
                        - (self.group1()[1] * other.group2()[1])),
                ])),
            // e1234
            (-(self.group6()[2] * other.group3()[2]) - (self.group6()[1] * other.group3()[1]) - (self.group6()[0] * other.group3()[0]) - (self.group5()[3] * other.group3()[3])
                + (self.group1()[3] * other.group1()[3])
                + (self.group1()[2] * other.group0()[2])
                + (self.group1()[0] * other.group0()[0])
                + (self.group1()[1] * other.group0()[1])),
            // e12, e31, e23
            Simd32x3::from([
                ((self.group1()[0] * other.group3()[1]) - (self.group1()[1] * other.group3()[0])),
                (-(self.group1()[0] * other.group3()[2]) + (self.group1()[2] * other.group3()[0])),
                ((self.group1()[1] * other.group3()[2]) - (self.group1()[2] * other.group3()[1])),
            ]),
        );
    }
}
impl Wedge<VersorOdd> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       62        0
    //    simd3        6        8        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       57       79        0
    //  no simd       90      122        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self.group0()[0] * other.group0()[3]),
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
            ]),
            // e1, e2, e3, e4
            (self.group1() * Simd32x4::from(other.group0()[3])),
            // e5
            (self[e1] * other.group0()[3]),
            // e15, e25, e35, e45
            ((Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                + (self.group3() * Simd32x4::from(other.group0()[3]))),
            // e41, e42, e43
            ((Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])) + (self.group4() * Simd32x3::from(other.group0()[3]))),
            // e415, e425, e435, e321
            ((self.group5() * Simd32x4::from(other.group0()[3])) - (swizzle!(self.group1(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self[e1] * other.group0()[0]) + (self.group1()[3] * other.group2()[0])),
                    ((self[e1] * other.group0()[1]) + (self.group1()[3] * other.group2()[1])),
                    ((self[e1] * other.group0()[2]) + (self.group1()[3] * other.group2()[2])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
            // e423, e431, e412
            ((self.group6() * Simd32x3::from(other.group0()[3]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + Simd32x3::from([
                    (-(self.group1()[1] * other.group0()[2]) + (self.group1()[2] * other.group0()[1])),
                    ((self.group1()[0] * other.group0()[2]) - (self.group1()[2] * other.group0()[0])),
                    (-(self.group1()[0] * other.group0()[1]) + (self.group1()[1] * other.group0()[0])),
                ])),
            // e235, e315, e125
            ((self.group7() * Simd32x3::from(other.group0()[3]))
                + (Simd32x3::from(self[e1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + Simd32x3::from([
                    ((self.group1()[1] * other.group2()[2]) - (self.group1()[2] * other.group2()[1])),
                    (-(self.group1()[0] * other.group2()[2]) + (self.group1()[2] * other.group2()[0])),
                    ((self.group1()[0] * other.group2()[1]) - (self.group1()[1] * other.group2()[0])),
                ])),
            // e4235, e4315, e4125, e3215
            ((self.group8() * Simd32x4::from(other.group0()[3]))
                - (swizzle!(other.group2(), 1, 2, 0, 0) * Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group10()[2]]))
                + (Simd32x4::from(self.group0()[0]) * other.group3())
                - (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group10()[2] * other.group1()[3])
                        + (self.group4()[1] * other.group2()[2])
                        + (self.group3()[3] * other.group1()[0])
                        + (self.group3()[2] * other.group0()[1])),
                    ((self.group10()[1] * other.group1()[3])
                        + (self.group4()[2] * other.group2()[0])
                        + (self.group3()[3] * other.group1()[1])
                        + (self.group3()[0] * other.group0()[2])),
                    ((self.group10()[0] * other.group1()[3])
                        + (self.group4()[0] * other.group2()[1])
                        + (self.group3()[3] * other.group1()[2])
                        + (self.group3()[1] * other.group0()[0])),
                    (-(self.group10()[1] * other.group2()[1])
                        - (self.group10()[0] * other.group2()[2])
                        - (self.group3()[1] * other.group1()[1])
                        - (self.group3()[0] * other.group1()[0])),
                ])),
            // e1234
            (-(self.group10()[2] * other.group0()[0]) - (self.group10()[1] * other.group0()[1]) - (self.group10()[0] * other.group0()[2]) + (self[e35] * other.group0()[3])
                - (self.group4()[2] * other.group1()[2])
                - (self.group4()[1] * other.group1()[1])
                + (self.group0()[0] * other.group2()[3])
                - (self.group4()[0] * other.group1()[0])),
            // e12, e31, e23
            ((Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[2], other.group1()[1], other.group1()[0]])) + (self.group10() * Simd32x3::from(other.group0()[3]))),
        );
    }
}
impl InfixWedge for Plane {}
impl Wedge<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                ((self.group0()[3] * other.group1()[3]) + (self.group0()[2] * other.group1()[2]) + (self.group0()[0] * other.group1()[0]) + (self.group0()[1] * other.group1()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
            (self.group0() * Simd32x4::from(other.group0()[0])),
            // e1234
            0.0,
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<RoundPoint> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            ((self.group0()[3] * other.group0()[3]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (self.group0() * Simd32x4::from(other[scalar])));
    }
}
impl Wedge<VersorEven> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            ((self.group0()[3] * other.group3()[3]) + (self.group0()[2] * other.group3()[2]) + (self.group0()[0] * other.group3()[0]) + (self.group0()[1] * other.group3()[1])),
        );
    }
}
impl Wedge<VersorOdd> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (self.group0() * Simd32x4::from(other.group0()[3])));
    }
}
impl InfixWedge for RoundPoint {}
impl Wedge<Circle> for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (-(Simd32x4::from(self[e2]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group2()[0]) + (self.group0()[2] * other.group1()[1])),
                    ((self.group0()[3] * other.group2()[1]) + (self.group0()[0] * other.group1()[2])),
                    ((self.group0()[3] * other.group2()[2]) + (self.group0()[1] * other.group1()[0])),
                    (-(self.group0()[0] * other.group2()[0]) - (self.group0()[1] * other.group2()[1])),
                ])),
            // e1234
            ((self.group0()[3] * other.group1()[3]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<CircleRotor> for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (-(Simd32x4::from(self[e2]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group2()[0]) + (self.group0()[2] * other.group1()[1])),
                    ((self.group0()[3] * other.group2()[1]) + (self.group0()[0] * other.group1()[2])),
                    ((self.group0()[3] * other.group2()[2]) + (self.group0()[1] * other.group1()[0])),
                    (-(self.group0()[0] * other.group2()[0]) - (self.group0()[1] * other.group2()[1])),
                ])),
            // e1234
            ((self.group0()[3] * other.group1()[3]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Dipole> for RoundPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd3        4        6        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       20       30        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            ((Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (swizzle!(other.group0(), 2, 0, 1) * Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]))
                + (swizzle!(other.group0(), 1, 2, 0) * Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group0(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self[e2] * other.group0()[0]) + (self.group0()[3] * other.group2()[0])),
                    ((self[e2] * other.group0()[1]) + (self.group0()[3] * other.group2()[1])),
                    ((self[e2] * other.group0()[2]) + (self.group0()[3] * other.group2()[2])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
            // e235, e315, e125
            ((Simd32x3::from(self[e2]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (swizzle!(other.group2(), 2, 0, 1) * Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]))
                - (swizzle!(other.group2(), 1, 2, 0) * Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]))),
        );
    }
}
impl Wedge<DipoleInversion> for RoundPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        2        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       24       38        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            ((Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (swizzle!(other.group0(), 2, 0, 1) * Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]))
                + (swizzle!(other.group0(), 1, 2, 0) * Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group0(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self[e2] * other.group0()[0]) + (self.group0()[3] * other.group2()[0])),
                    ((self[e2] * other.group0()[1]) + (self.group0()[3] * other.group2()[1])),
                    ((self[e2] * other.group0()[2]) + (self.group0()[3] * other.group2()[2])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
            // e235, e315, e125, e12345
            ((Simd32x4::from(self[e2]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]))
                + (swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[3]]))
                + Simd32x4::from([
                    ((self.group0()[2] * other.group2()[1]) * -1.0),
                    ((self.group0()[0] * other.group2()[2]) * -1.0),
                    ((self.group0()[1] * other.group2()[0]) * -1.0),
                    ((self.group0()[2] * other.group3()[2]) + (self.group0()[0] * other.group3()[0]) + (self.group0()[1] * other.group3()[1])),
                ])),
        );
    }
}
impl Wedge<DualNum> for RoundPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ (self.group0() * Simd32x4::from(other.group0()[0])));
    }
}
impl Wedge<FlatPoint> for RoundPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       12        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            (-(Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e235, e315, e125
            Simd32x3::from([
                ((self.group0()[1] * other.group0()[2]) - (self.group0()[2] * other.group0()[1])),
                (-(self.group0()[0] * other.group0()[2]) + (self.group0()[2] * other.group0()[0])),
                ((self.group0()[0] * other.group0()[1]) - (self.group0()[1] * other.group0()[0])),
            ]),
        );
    }
}
impl Wedge<Flector> for RoundPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       16        0
    //  no simd        9       19        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                + Simd32x4::from([
                    ((self.group0()[0] * other.group0()[3]) * -1.0),
                    ((self.group0()[1] * other.group0()[3]) * -1.0),
                    ((self.group0()[2] * other.group0()[3]) * -1.0),
                    ((self.group0()[2] * other.group1()[2]) + (self.group0()[0] * other.group1()[0]) + (self.group0()[1] * other.group1()[1])),
                ])),
            // e235, e315, e125, e5
            Simd32x4::from([
                ((self.group0()[1] * other.group0()[2]) - (self.group0()[2] * other.group0()[1])),
                (-(self.group0()[0] * other.group0()[2]) + (self.group0()[2] * other.group0()[0])),
                ((self.group0()[0] * other.group0()[1]) - (self.group0()[1] * other.group0()[0])),
                0.0,
            ]),
        );
    }
}
impl Wedge<Line> for RoundPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Line) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group1()[0]) + (self.group0()[2] * other.group0()[1])),
                    ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group0()[2])),
                    ((self.group0()[3] * other.group1()[2]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<Motor> for RoundPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (self.group0() * Simd32x4::from(other.group1()[3])),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group1()[0]) + (self.group0()[2] * other.group0()[1])),
                    ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group0()[2])),
                    ((self.group0()[3] * other.group1()[2]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<MultiVector> for RoundPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       38        0
    //    simd3        4        6        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       29       50        0
    //  no simd       49       80        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                ((self[e2] * other[e35])
                    + (self.group0()[3] * other.group8()[3])
                    + (self.group0()[2] * other.group8()[2])
                    + (self.group0()[0] * other.group8()[0])
                    + (self.group0()[1] * other.group8()[1])),
            ]),
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group0()[0])),
            // e5
            (self[e2] * other.group0()[0]),
            // e15, e25, e35, e45
            ((self.group0() * Simd32x4::from(other[e1])) - (Simd32x4::from(self[e2]) * other.group1())),
            // e41, e42, e43
            (-(Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group0(), 0, 1, 2, 2) * Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group10()[0]]))
                + Simd32x4::from([
                    ((self[e2] * other.group4()[0]) + (self.group0()[3] * other.group3()[0])),
                    ((self[e2] * other.group4()[1]) + (self.group0()[3] * other.group3()[1])),
                    ((self[e2] * other.group4()[2]) + (self.group0()[3] * other.group3()[2])),
                    (-(self.group0()[0] * other.group10()[2]) - (self.group0()[1] * other.group10()[1])),
                ])),
            // e423, e431, e412
            ((Simd32x3::from(self.group0()[3]) * swizzle!(other.group10(), 2, 1, 0))
                - (swizzle!(other.group4(), 2, 0, 1) * Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]))
                + (swizzle!(other.group4(), 1, 2, 0) * Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]))),
            // e235, e315, e125
            ((Simd32x3::from(self[e2]) * swizzle!(other.group10(), 2, 1, 0))
                + Simd32x3::from([
                    ((self.group0()[1] * other.group3()[2]) - (self.group0()[2] * other.group3()[1])),
                    (-(self.group0()[0] * other.group3()[2]) + (self.group0()[2] * other.group3()[0])),
                    ((self.group0()[0] * other.group3()[1]) - (self.group0()[1] * other.group3()[0])),
                ])),
            // e4235, e4315, e4125, e3215
            (-(Simd32x4::from(self[e2]) * Simd32x4::from([other.group6()[0], other.group6()[1], other.group6()[2], other.group5()[3]]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group5()[2], other.group5()[0], other.group5()[1], other.group7()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group7()[0]) + (self.group0()[2] * other.group5()[1])),
                    ((self.group0()[3] * other.group7()[1]) + (self.group0()[0] * other.group5()[2])),
                    ((self.group0()[3] * other.group7()[2]) + (self.group0()[1] * other.group5()[0])),
                    (-(self.group0()[0] * other.group7()[0]) - (self.group0()[1] * other.group7()[1])),
                ])),
            // e1234
            ((self.group0()[3] * other.group5()[3]) + (self.group0()[2] * other.group6()[2]) + (self.group0()[0] * other.group6()[0]) + (self.group0()[1] * other.group6()[1])),
            // e12, e31, e23
            Simd32x3::from([
                ((self.group0()[0] * other.group1()[1]) - (self.group0()[1] * other.group1()[0])),
                (-(self.group0()[0] * other.group1()[2]) + (self.group0()[2] * other.group1()[0])),
                ((self.group0()[1] * other.group1()[2]) - (self.group0()[2] * other.group1()[1])),
            ]),
        );
    }
}
impl Wedge<Plane> for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            ((self.group0()[3] * other.group0()[3]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<RoundPoint> for RoundPoint {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        4        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        6        0
    //  no simd       10       20        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            (-(Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e23, e31, e12, e45
            ((swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other[e2]]))
                - (swizzle!(other.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self[e2]]))),
            // e15, e25, e35
            ((Simd32x3::from(other[e2]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self[e2]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
        );
    }
}
impl Wedge<Scalar> for RoundPoint {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ (self.group0() * Simd32x4::from(other[scalar])), /* e5 */ (self[e2] * other[scalar]));
    }
}
impl Wedge<Sphere> for RoundPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            ((self[e2] * other[e4315])
                + (self.group0()[3] * other.group0()[3])
                + (self.group0()[2] * other.group0()[2])
                + (self.group0()[0] * other.group0()[0])
                + (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<VersorEven> for RoundPoint {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        1        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       25       43        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (-(Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))),
            // e23, e31, e12, e45
            ((swizzle!(self.group0(), 1, 2, 0, 3) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[3]]))
                - (swizzle!(other.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self[e2]]))),
            // e15, e25, e35, e1234
            ((self.group0() * Simd32x4::from([other.group2()[3], other.group2()[3], other.group2()[3], other.group1()[3]]))
                + Simd32x4::from([
                    ((self[e2] * other.group3()[0]) * -1.0),
                    ((self[e2] * other.group3()[1]) * -1.0),
                    ((self[e2] * other.group3()[2]) * -1.0),
                    ((self.group0()[2] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            (-(Simd32x4::from(self[e2]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group2()[0]) + (self.group0()[2] * other.group1()[1])),
                    ((self.group0()[3] * other.group2()[1]) + (self.group0()[0] * other.group1()[2])),
                    ((self.group0()[3] * other.group2()[2]) + (self.group0()[1] * other.group1()[0])),
                    (-(self.group0()[0] * other.group2()[0]) - (self.group0()[1] * other.group2()[1])),
                ])),
        );
    }
}
impl Wedge<VersorOdd> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       15       31        0
    //  no simd       24       43        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group3()[3]]))
                + (swizzle!(self.group0(), 2, 0, 1, 2) * Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group3()[2]]))
                + Simd32x4::from([
                    ((self.group0()[1] * other.group0()[2]) * -1.0),
                    ((self.group0()[2] * other.group0()[0]) * -1.0),
                    ((self.group0()[0] * other.group0()[1]) * -1.0),
                    ((self[e2] * other.group2()[3]) + (self.group0()[0] * other.group3()[0]) + (self.group0()[1] * other.group3()[1])),
                ])),
            // e415, e425, e435, e321
            (-(swizzle!(self.group0(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self[e2] * other.group0()[0]) + (self.group0()[3] * other.group2()[0])),
                    ((self[e2] * other.group0()[1]) + (self.group0()[3] * other.group2()[1])),
                    ((self[e2] * other.group0()[2]) + (self.group0()[3] * other.group2()[2])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
            // e235, e315, e125, e5
            Simd32x4::from([
                ((self[e2] * other.group1()[0]) + (self.group0()[1] * other.group2()[2]) - (self.group0()[2] * other.group2()[1])),
                ((self[e2] * other.group1()[1]) - (self.group0()[0] * other.group2()[2]) + (self.group0()[2] * other.group2()[0])),
                ((self[e2] * other.group1()[2]) + (self.group0()[0] * other.group2()[1]) - (self.group0()[1] * other.group2()[0])),
                (self[e2] * other.group0()[3]),
            ]),
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group0()[3])),
        );
    }
}
impl InfixWedge for Scalar {}
impl Wedge<AntiScalar> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[scalar] * other[e12345]));
    }
}
impl Wedge<Circle> for Scalar {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: Circle) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[scalar]) * other.group0()),
            // e415, e425, e435, e321
            (Simd32x4::from(self[scalar]) * other.group1()),
            // e235, e315, e125
            (Simd32x3::from(self[scalar]) * other.group2()),
        );
    }
}
impl Wedge<CircleRotor> for Scalar {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self[scalar]) * other.group0()),
            // e415, e425, e435, e321
            (Simd32x4::from(self[scalar]) * other.group1()),
            // e235, e315, e125, e12345
            (Simd32x4::from(self[scalar]) * other.group2()),
        );
    }
}
impl Wedge<Dipole> for Scalar {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group0()),
            // e23, e31, e12, e45
            (Simd32x4::from(self[scalar]) * other.group1()),
            // e15, e25, e35
            (Simd32x3::from(self[scalar]) * other.group2()),
        );
    }
}
impl Wedge<DipoleInversion> for Scalar {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group0()),
            // e23, e31, e12, e45
            (Simd32x4::from(self[scalar]) * other.group1()),
            // e15, e25, e35, e1234
            (Simd32x4::from(self[scalar]) * other.group2()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[scalar]) * other.group3()),
        );
    }
}
impl Wedge<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* e5, e12345 */ (Simd32x2::from(self[scalar]) * other.group0()));
    }
}
impl Wedge<FlatPoint> for Scalar {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ (Simd32x4::from(self[scalar]) * other.group0()));
    }
}
impl Wedge<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from(self[scalar]) * other.group0()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[scalar]) * other.group1()),
        );
    }
}
impl Wedge<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e415, e425, e435
            (Simd32x3::from(self[scalar]) * other.group0()),
            // e235, e315, e125
            (Simd32x3::from(self[scalar]) * other.group1()),
        );
    }
}
impl Wedge<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (Simd32x4::from(self[scalar]) * other.group0()),
            // e235, e315, e125, e5
            (Simd32x4::from(self[scalar]) * other.group1()),
        );
    }
}
impl Wedge<MultiVector> for Scalar {
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
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            (Simd32x2::from(self[scalar]) * other.group0()),
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]) * other.group1()),
            // e5
            (self[scalar] * other[e1]),
            // e15, e25, e35, e45
            (Simd32x4::from(self[scalar]) * other.group3()),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group4()),
            // e415, e425, e435, e321
            (Simd32x4::from(self[scalar]) * other.group5()),
            // e423, e431, e412
            (Simd32x3::from(self[scalar]) * other.group6()),
            // e235, e315, e125
            (Simd32x3::from(self[scalar]) * other.group7()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[scalar]) * other.group8()),
            // e1234
            (self[scalar] * other[e35]),
            // e12, e31, e23
            (Simd32x3::from(self[scalar]) * other.group10()),
        );
    }
}
impl Wedge<Plane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (Simd32x4::from(self[scalar]) * other.group0()));
    }
}
impl Wedge<RoundPoint> for Scalar {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ (Simd32x4::from(self[scalar]) * other.group0()), /* e5 */ (self[scalar] * other[e2]));
    }
}
impl Wedge<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[scalar] * other[scalar]));
    }
}
impl Wedge<Sphere> for Scalar {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[scalar]) * other.group0()),
            // e1234
            (self[scalar] * other[e4315]),
        );
    }
}
impl Wedge<VersorEven> for Scalar {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from(self[scalar]) * other.group0()),
            // e415, e425, e435, e321
            (Simd32x4::from(self[scalar]) * other.group1()),
            // e235, e315, e125, e5
            (Simd32x4::from(self[scalar]) * other.group2()),
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]) * other.group3()),
        );
    }
}
impl Wedge<VersorOdd> for Scalar {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (Simd32x4::from(self[scalar]) * other.group0()),
            // e23, e31, e12, e45
            (Simd32x4::from(self[scalar]) * other.group1()),
            // e15, e25, e35, e1234
            (Simd32x4::from(self[scalar]) * other.group2()),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self[scalar]) * other.group3()),
        );
    }
}
impl InfixWedge for Sphere {}
impl Wedge<DualNum> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e4315] * other.group0()[0]));
    }
}
impl Wedge<Motor> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self[e4315] * other.group1()[3]));
    }
}
impl Wedge<MultiVector> for Sphere {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4       10        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                ((self[e4315] * other[e1])
                    + (self.group0()[3] * other.group1()[3])
                    + (self.group0()[2] * other.group1()[2])
                    + (self.group0()[0] * other.group1()[0])
                    + (self.group0()[1] * other.group1()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
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
            (self.group0() * Simd32x4::from(other.group0()[0])),
            // e1234
            (self[e4315] * other.group0()[0]),
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<RoundPoint> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            ((self[e4315] * other[e2])
                + (self.group0()[3] * other.group0()[3])
                + (self.group0()[2] * other.group0()[2])
                + (self.group0()[0] * other.group0()[0])
                + (self.group0()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Scalar> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x4::from(other[scalar])),
            // e1234
            (self[e4315] * other[scalar]),
        );
    }
}
impl Wedge<VersorEven> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            ((self[e4315] * other.group2()[3])
                + (self.group0()[3] * other.group3()[3])
                + (self.group0()[2] * other.group3()[2])
                + (self.group0()[0] * other.group3()[0])
                + (self.group0()[1] * other.group3()[1])),
        );
    }
}
impl Wedge<VersorOdd> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (self.group0() * Simd32x4::from(other.group0()[3])),
            // e1234
            (self[e4315] * other.group0()[3]),
        );
    }
}
impl InfixWedge for VersorEven {}
impl Wedge<Circle> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn wedge(self, other: Circle) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (-(Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[2]]))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group2()[0]) + (self.group3()[2] * other.group1()[1])),
                    ((self.group3()[3] * other.group2()[1]) + (self.group3()[0] * other.group1()[2])),
                    ((self.group3()[3] * other.group2()[2]) + (self.group3()[1] * other.group1()[0])),
                    (-(self.group3()[1] * other.group2()[1]) - (self.group3()[0] * other.group2()[0])),
                ])),
            // e1234
            ((self.group3()[3] * other.group1()[3]) + (self.group3()[2] * other.group0()[2]) + (self.group3()[0] * other.group0()[0]) + (self.group3()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<CircleRotor> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (-(Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[2]]))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group2()[0]) + (self.group3()[2] * other.group1()[1])),
                    ((self.group3()[3] * other.group2()[1]) + (self.group3()[0] * other.group1()[2])),
                    ((self.group3()[3] * other.group2()[2]) + (self.group3()[1] * other.group1()[0])),
                    (-(self.group3()[1] * other.group2()[1]) - (self.group3()[0] * other.group2()[0])),
                ])),
            // e1234
            ((self.group3()[3] * other.group1()[3]) + (self.group3()[2] * other.group0()[2]) + (self.group3()[0] * other.group0()[0]) + (self.group3()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<Dipole> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        0
    //    simd3        2        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       22       31        0
    //  no simd       29       40        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            ((Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (swizzle!(other.group0(), 2, 0, 1) * Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[0]]))
                + (swizzle!(other.group0(), 1, 2, 0) * Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group3(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group2()[0]) + (self.group2()[3] * other.group0()[0])),
                    ((self.group3()[3] * other.group2()[1]) + (self.group2()[3] * other.group0()[1])),
                    ((self.group3()[3] * other.group2()[2]) + (self.group2()[3] * other.group0()[2])),
                    (-(self.group3()[0] * other.group1()[0]) - (self.group3()[1] * other.group1()[1])),
                ])),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (-(self.group3()[2] * other.group2()[1]) + (self.group2()[3] * other.group1()[0]) + (self.group3()[1] * other.group2()[2])),
                ((self.group3()[2] * other.group2()[0]) + (self.group2()[3] * other.group1()[1]) - (self.group3()[0] * other.group2()[2])),
                (-(self.group3()[1] * other.group2()[0]) + (self.group2()[3] * other.group1()[2]) + (self.group3()[0] * other.group2()[1])),
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
        );
    }
}
impl Wedge<DipoleInversion> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       20        0
    //    simd3        2        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       21       27        0
    //  no simd       37       45        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            ((Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (swizzle!(other.group0(), 2, 0, 1) * Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[0]]))
                + (swizzle!(other.group0(), 1, 2, 0) * Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group3(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group2()[0]) + (self.group2()[3] * other.group0()[0])),
                    ((self.group3()[3] * other.group2()[1]) + (self.group2()[3] * other.group0()[1])),
                    ((self.group3()[3] * other.group2()[2]) + (self.group2()[3] * other.group0()[2])),
                    (-(self.group3()[0] * other.group1()[0]) - (self.group3()[1] * other.group1()[1])),
                ])),
            // e235, e315, e125, e12345
            (-(swizzle!(other.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group0()[2]]))
                + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]))
                + (swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[3]]))
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
        );
    }
}
impl Wedge<DualNum> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (self.group3() * Simd32x4::from(other.group0()[0])),
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]])),
        );
    }
}
impl Wedge<FlatPoint> for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        9       16        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (-(Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group1()[3]]))
                + Simd32x4::from([
                    (self.group3()[3] * other.group0()[0]),
                    (self.group3()[3] * other.group0()[1]),
                    (self.group3()[3] * other.group0()[2]),
                    (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
            // e235, e315, e125, e5
            Simd32x4::from([
                ((self.group3()[1] * other.group0()[2]) - (self.group3()[2] * other.group0()[1])),
                (-(self.group3()[0] * other.group0()[2]) + (self.group3()[2] * other.group0()[0])),
                ((self.group3()[0] * other.group0()[1]) - (self.group3()[1] * other.group0()[0])),
                0.0,
            ]),
        );
    }
}
impl Wedge<Flector> for VersorEven {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       16       20        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            (-(Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group1()[3]]))
                + (Simd32x4::from(self.group3()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    ((self.group3()[2] * other.group1()[2]) + (self.group3()[1] * other.group1()[1]) + (self.group3()[0] * other.group1()[0])
                        - (self.group0()[2] * other.group0()[2])
                        - (self.group0()[0] * other.group0()[0])
                        - (self.group0()[1] * other.group0()[1])),
                ])),
            // e235, e315, e125, e5
            Simd32x4::from([
                ((self.group3()[1] * other.group0()[2]) - (self.group3()[2] * other.group0()[1])),
                (-(self.group3()[0] * other.group0()[2]) + (self.group3()[2] * other.group0()[0])),
                ((self.group3()[0] * other.group0()[1]) - (self.group3()[1] * other.group0()[0])),
                0.0,
            ]),
        );
    }
}
impl Wedge<Line> for VersorEven {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: Line) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group1()[0]) + (self.group3()[2] * other.group0()[1])),
                    ((self.group3()[3] * other.group1()[1]) + (self.group3()[0] * other.group0()[2])),
                    ((self.group3()[3] * other.group1()[2]) + (self.group3()[1] * other.group0()[0])),
                    (-(self.group3()[0] * other.group1()[0]) - (self.group3()[1] * other.group1()[1])),
                ])),
        );
    }
}
impl Wedge<Motor> for VersorEven {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (self.group3() * Simd32x4::from(other.group1()[3])),
            // e4235, e4315, e4125, e3215
            ((other.group1() * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group1()[3]]))
                - (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group3()[2] * other.group0()[1]) + (self.group0()[0] * other.group1()[3])),
                    ((self.group0()[1] * other.group1()[3]) + (self.group3()[0] * other.group0()[2])),
                    ((self.group3()[1] * other.group0()[0]) + (self.group0()[2] * other.group1()[3])),
                    (-(self.group3()[1] * other.group1()[1]) - (self.group3()[0] * other.group1()[0])),
                ])),
        );
    }
}
impl Wedge<MultiVector> for VersorEven {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       61        0
    //    simd3        6        8        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       57       78        0
    //  no simd       90      121        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
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
            ]),
            // e1, e2, e3, e4
            (self.group3() * Simd32x4::from(other.group0()[0])),
            // e5
            (self.group2()[3] * other.group0()[0]),
            // e15, e25, e35, e45
            (-(Simd32x4::from(self.group2()[3]) * other.group1()) + (self.group3() * Simd32x4::from(other[e1]))),
            // e41, e42, e43
            (-(Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]))
                + (Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))),
            // e415, e425, e435, e321
            (-(swizzle!(self.group3(), 0, 1, 2, 2) * Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group10()[0]]))
                + (self.group1() * Simd32x4::from(other.group0()[0]))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group3()[0]) + (self.group2()[3] * other.group4()[0])),
                    ((self.group3()[3] * other.group3()[1]) + (self.group2()[3] * other.group4()[1])),
                    ((self.group3()[3] * other.group3()[2]) + (self.group2()[3] * other.group4()[2])),
                    (-(self.group3()[1] * other.group10()[1]) - (self.group3()[0] * other.group10()[2])),
                ])),
            // e423, e431, e412
            ((Simd32x3::from(self.group3()[3]) * swizzle!(other.group10(), 2, 1, 0))
                + (swizzle!(other.group4(), 1, 2, 0) * Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]))
                + (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (swizzle!(other.group4(), 2, 0, 1) * Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[0]]))),
            // e235, e315, e125
            ((Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]))
                + (Simd32x3::from(self.group2()[3]) * swizzle!(other.group10(), 2, 1, 0))
                + Simd32x3::from([
                    (-(self.group3()[2] * other.group3()[1]) + (self.group3()[1] * other.group3()[2])),
                    ((self.group3()[2] * other.group3()[0]) - (self.group3()[0] * other.group3()[2])),
                    (-(self.group3()[1] * other.group3()[0]) + (self.group3()[0] * other.group3()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group5()[2], other.group5()[0], other.group5()[1], other.group7()[2]]))
                - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group6()[0], other.group6()[1], other.group6()[2], other.group5()[3]]))
                + (swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[2]]))
                + (Simd32x4::from(other[e1]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group7()[0]) + (self.group3()[2] * other.group5()[1])
                        - (self.group2()[0] * other.group1()[3])
                        - (self.group1()[1] * other.group1()[2])),
                    ((self.group3()[3] * other.group7()[1]) + (self.group3()[0] * other.group5()[2])
                        - (self.group2()[1] * other.group1()[3])
                        - (self.group1()[2] * other.group1()[0])),
                    ((self.group3()[3] * other.group7()[2]) + (self.group3()[1] * other.group5()[0])
                        - (self.group2()[2] * other.group1()[3])
                        - (self.group1()[0] * other.group1()[1])),
                    (-(self.group3()[1] * other.group7()[1]) - (self.group3()[0] * other.group7()[0])
                        + (self.group2()[1] * other.group1()[1])
                        + (self.group2()[0] * other.group1()[0])),
                ])),
            // e1234
            ((self.group3()[3] * other.group5()[3]) + (self.group3()[2] * other.group6()[2]) + (self.group3()[1] * other.group6()[1]) + (self.group3()[0] * other.group6()[0])
                - (self.group1()[3] * other.group1()[3])
                - (self.group0()[2] * other.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])),
            // e12, e31, e23
            Simd32x3::from([
                ((self.group3()[0] * other.group1()[1]) - (self.group3()[1] * other.group1()[0])),
                (-(self.group3()[0] * other.group1()[2]) + (self.group3()[2] * other.group1()[0])),
                ((self.group3()[1] * other.group1()[2]) - (self.group3()[2] * other.group1()[1])),
            ]),
        );
    }
}
impl Wedge<Plane> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            ((self.group3()[3] * other.group0()[3]) + (self.group3()[2] * other.group0()[2]) + (self.group3()[0] * other.group0()[0]) + (self.group3()[1] * other.group0()[1])),
        );
    }
}
impl Wedge<RoundPoint> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd3        1        2        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       11       21        0
    //  no simd       25       40        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (-(Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]))
                + (Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e23, e31, e12, e45
            ((swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other[e2]]))
                - (swizzle!(other.group0(), 1, 2, 0, 3) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[3]]))),
            // e15, e25, e35, e1234
            (-(other.group0() * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group1()[3]]))
                + Simd32x4::from([
                    (self.group3()[0] * other[e2]),
                    (self.group3()[1] * other[e2]),
                    (self.group3()[2] * other[e2]),
                    (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            ((swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[2]]))
                + (Simd32x4::from(other[e2]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + Simd32x4::from([
                    (-(self.group2()[0] * other.group0()[3]) - (self.group1()[1] * other.group0()[2])),
                    (-(self.group2()[1] * other.group0()[3]) - (self.group1()[2] * other.group0()[0])),
                    (-(self.group2()[2] * other.group0()[3]) - (self.group1()[0] * other.group0()[1])),
                    ((self.group2()[1] * other.group0()[1]) + (self.group2()[0] * other.group0()[0])),
                ])),
        );
    }
}
impl Wedge<Scalar> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            (self.group0() * Simd32x4::from(other[scalar])),
            // e415, e425, e435, e321
            (self.group1() * Simd32x4::from(other[scalar])),
            // e235, e315, e125, e5
            (self.group2() * Simd32x4::from(other[scalar])),
            // e1, e2, e3, e4
            (self.group3() * Simd32x4::from(other[scalar])),
        );
    }
}
impl Wedge<Sphere> for VersorEven {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            ((self.group3()[3] * other.group0()[3])
                + (self.group3()[2] * other.group0()[2])
                + (self.group3()[1] * other.group0()[1])
                + (self.group2()[3] * other[e4315])
                + (self.group3()[0] * other.group0()[0])),
        );
    }
}
impl Wedge<VersorEven> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       22        0
    //    simd3        1        2        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       25       32        0
    //  no simd       48       60        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (-(Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]))
                + (Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))),
            // e23, e31, e12, e45
            ((swizzle!(self.group3(), 1, 2, 0, 3) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[3]]))
                - (swizzle!(other.group3(), 1, 2, 0, 3) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[3]]))),
            // e15, e25, e35, e1234
            (-(other.group3() * Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group1()[3]]))
                + (self.group3() * Simd32x4::from([other.group2()[3], other.group2()[3], other.group2()[3], other.group1()[3]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    ((self.group3()[2] * other.group0()[2]) + (self.group3()[1] * other.group0()[1]) + (self.group3()[0] * other.group0()[0])
                        - (self.group0()[2] * other.group3()[2])
                        - (self.group0()[0] * other.group3()[0])
                        - (self.group0()[1] * other.group3()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            ((other.group2() * Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group1()[3]]))
                - (swizzle!(self.group3(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[2]]))
                - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                + (swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[2]]))
                + Simd32x4::from([
                    ((self.group3()[2] * other.group1()[1]) - (self.group2()[0] * other.group3()[3]) + (self.group0()[0] * other.group2()[3])
                        - (self.group1()[1] * other.group3()[2])),
                    ((self.group3()[0] * other.group1()[2]) - (self.group2()[1] * other.group3()[3]) - (self.group1()[2] * other.group3()[0])
                        + (self.group0()[1] * other.group2()[3])),
                    ((self.group3()[1] * other.group1()[0]) - (self.group2()[2] * other.group3()[3]) + (self.group0()[2] * other.group2()[3])
                        - (self.group1()[0] * other.group3()[1])),
                    (-(self.group3()[1] * other.group2()[1]) - (self.group3()[0] * other.group2()[0])
                        + (self.group2()[1] * other.group3()[1])
                        + (self.group2()[0] * other.group3()[0])),
                ])),
        );
    }
}
impl Wedge<VersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       37        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       30       43        0
    //  no simd       45       61        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x4::from(self.group3()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group3()[3]]))
                + (swizzle!(other.group0(), 1, 3, 0, 3) * Simd32x4::from([self.group3()[2], self.group0()[1], self.group3()[1], self.group0()[3]]))
                - (swizzle!(other.group0(), 2, 0, 1, 2) * Simd32x4::from([self.group3()[1], self.group3()[2], self.group3()[0], self.group2()[2]]))
                + Simd32x4::from([
                    (self.group0()[0] * other.group0()[3]),
                    (self.group3()[0] * other.group0()[2]),
                    (self.group0()[2] * other.group0()[3]),
                    ((self.group3()[2] * other.group3()[2])
                        + (self.group3()[1] * other.group3()[1])
                        + (self.group3()[0] * other.group3()[0])
                        + (self.group2()[3] * other.group2()[3])
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
            // e415, e425, e435, e321
            (-(swizzle!(self.group3(), 0, 1, 2, 2) * swizzle!(other.group1(), 3, 3, 3, 2))
                + (self.group1() * Simd32x4::from(other.group0()[3]))
                + Simd32x4::from([
                    ((self.group3()[3] * other.group2()[0]) + (self.group2()[3] * other.group0()[0])),
                    ((self.group3()[3] * other.group2()[1]) + (self.group2()[3] * other.group0()[1])),
                    ((self.group3()[3] * other.group2()[2]) + (self.group2()[3] * other.group0()[2])),
                    (-(self.group3()[1] * other.group1()[1]) - (self.group3()[0] * other.group1()[0])),
                ])),
            // e235, e315, e125, e5
            Simd32x4::from([
                (-(self.group3()[2] * other.group2()[1])
                    + (self.group3()[1] * other.group2()[2])
                    + (self.group2()[0] * other.group0()[3])
                    + (self.group2()[3] * other.group1()[0])),
                ((self.group3()[2] * other.group2()[0]) - (self.group3()[0] * other.group2()[2]) + (self.group2()[1] * other.group0()[3]) + (self.group2()[3] * other.group1()[1])),
                (-(self.group3()[1] * other.group2()[0])
                    + (self.group3()[0] * other.group2()[1])
                    + (self.group2()[2] * other.group0()[3])
                    + (self.group2()[3] * other.group1()[2])),
                (self.group2()[3] * other.group0()[3]),
            ]),
            // e1, e2, e3, e4
            (self.group3() * Simd32x4::from(other.group0()[3])),
        );
    }
}
impl InfixWedge for VersorOdd {}
impl Wedge<AntiScalar> for VersorOdd {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ (self.group0()[3] * other[e12345]));
    }
}
impl Wedge<Circle> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       20        0
    fn wedge(self, other: Circle) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self.group0()[3]) * other.group0()),
            // e415, e425, e435, e321
            (Simd32x4::from(self.group0()[3]) * other.group1()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self.group0()[3] * other.group2()[0]),
                (self.group0()[3] * other.group2()[1]),
                (self.group0()[3] * other.group2()[2]),
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
        );
    }
}
impl Wedge<CircleRotor> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       10       21        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self.group0()[3]) * other.group0()),
            // e415, e425, e435, e321
            (Simd32x4::from(self.group0()[3]) * other.group1()),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self.group0()[3] * other.group2()[0]),
                (self.group0()[3] * other.group2()[1]),
                (self.group0()[3] * other.group2()[2]),
                (-(self.group2()[2] * other.group0()[2])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[0] * other.group1()[0])
                    + (self.group0()[3] * other.group2()[3])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])),
            ]),
        );
    }
}
impl Wedge<Dipole> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       29        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       22       32        0
    //  no simd       25       40        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self.group0()[3]) * other.group0()),
            // e23, e31, e12, e45
            (Simd32x4::from(self.group0()[3]) * other.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group0()[3] * other.group2()[0]),
                (self.group0()[3] * other.group2()[1]),
                (self.group0()[3] * other.group2()[2]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * other.group0()[1])
                        + (self.group1()[3] * other.group1()[0])
                        + (self.group1()[0] * other.group1()[3])
                        + (self.group0()[1] * other.group2()[2])
                        - (self.group0()[2] * other.group2()[1])),
                    ((self.group2()[0] * other.group0()[2]) + (self.group1()[3] * other.group1()[1]) + (self.group1()[1] * other.group1()[3])
                        - (self.group0()[0] * other.group2()[2])
                        + (self.group0()[2] * other.group2()[0])),
                    ((self.group2()[1] * other.group0()[0])
                        + (self.group1()[3] * other.group1()[2])
                        + (self.group1()[2] * other.group1()[3])
                        + (self.group0()[0] * other.group2()[1])
                        - (self.group0()[1] * other.group2()[0])),
                    (-(self.group2()[1] * other.group1()[1])
                        - (self.group2()[0] * other.group1()[0])
                        - (self.group1()[2] * other.group2()[2])
                        - (self.group1()[0] * other.group2()[0])
                        - (self.group1()[1] * other.group2()[1])),
                ])),
        );
    }
}
impl Wedge<DipoleInversion> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       26        0
    //    simd3        0        1        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       30       45        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self.group0()[3]) * other.group0()),
            // e23, e31, e12, e45
            (Simd32x4::from(self.group0()[3]) * other.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group0()[3] * other.group2()[0]),
                (self.group0()[3] * other.group2()[1]),
                (self.group0()[3] * other.group2()[2]),
                (-(self.group1()[2] * other.group0()[2]) - (self.group1()[1] * other.group0()[1]) - (self.group1()[0] * other.group0()[0])
                    + (self.group0()[3] * other.group2()[3])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + (Simd32x4::from(self.group0()[3]) * other.group3())
                - (swizzle!(other.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * other.group0()[1])
                        + (self.group1()[3] * other.group1()[0])
                        + (self.group1()[0] * other.group1()[3])
                        + (self.group0()[1] * other.group2()[2])),
                    ((self.group2()[0] * other.group0()[2])
                        + (self.group1()[3] * other.group1()[1])
                        + (self.group1()[1] * other.group1()[3])
                        + (self.group0()[2] * other.group2()[0])),
                    ((self.group2()[1] * other.group0()[0])
                        + (self.group1()[3] * other.group1()[2])
                        + (self.group1()[2] * other.group1()[3])
                        + (self.group0()[0] * other.group2()[1])),
                    (-(self.group2()[1] * other.group1()[1])
                        - (self.group2()[0] * other.group1()[0])
                        - (self.group1()[1] * other.group2()[1])
                        - (self.group1()[0] * other.group2()[0])),
                ])),
        );
    }
}
impl Wedge<DualNum> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                (self.group0()[0] * other.group0()[0]),
                (self.group0()[1] * other.group0()[0]),
                (self.group0()[2] * other.group0()[0]),
                ((self.group0()[3] * other.group0()[1]) + (self.group2()[3] * other.group0()[0])),
            ]),
            // e235, e315, e125, e5
            (Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]])),
        );
    }
}
impl Wedge<FlatPoint> for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from(self.group0()[3]) * other.group0()),
            // e4235, e4315, e4125, e3215
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group0()[3]) + (self.group0()[1] * other.group0()[2])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group0()[2] * other.group0()[0])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group0()[0] * other.group0()[1])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
        );
    }
}
impl Wedge<Flector> for VersorOdd {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            (Simd32x4::from(self.group0()[3]) * other.group0()),
            // e4235, e4315, e4125, e3215
            ((Simd32x4::from(self.group0()[3]) * other.group1())
                - (swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group0()[3]) + (self.group0()[1] * other.group0()[2])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group0()[2] * other.group0()[0])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group0()[0] * other.group0()[1])),
                    (-(self.group1()[1] * other.group0()[1]) - (self.group1()[0] * other.group0()[0])),
                ])),
        );
    }
}
impl Wedge<Line> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn wedge(self, other: Line) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
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
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[3] * other.group1()[0]),
                (self.group0()[3] * other.group1()[1]),
                (self.group0()[3] * other.group1()[2]),
                0.0,
            ]),
        );
    }
}
impl Wedge<Motor> for VersorOdd {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       16       21        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            ((Simd32x4::from(other.group1()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]))
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
            // e235, e315, e125, e5
            Simd32x4::from([
                ((self.group0()[3] * other.group1()[0]) + (self.group1()[0] * other.group1()[3])),
                ((self.group0()[3] * other.group1()[1]) + (self.group1()[1] * other.group1()[3])),
                ((self.group0()[3] * other.group1()[2]) + (self.group1()[2] * other.group1()[3])),
                (self.group0()[3] * other.group1()[3]),
            ]),
        );
    }
}
impl Wedge<MultiVector> for VersorOdd {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       62        0
    //    simd3        6        8        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       57       79        0
    //  no simd       90      122        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                (self.group0()[3] * other.group0()[0]),
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
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[3]) * other.group1()),
            // e5
            (self.group0()[3] * other[e1]),
            // e15, e25, e35, e45
            ((Simd32x4::from(self.group0()[3]) * other.group3())
                + (Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))),
            // e41, e42, e43
            ((Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])) + (Simd32x3::from(self.group0()[3]) * other.group4())),
            // e415, e425, e435, e321
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group1(), 0, 1, 2, 2))
                + (self.group0() * Simd32x4::from([other[e1], other[e1], other[e1], other.group5()[3]]))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group1()[3]) + (self.group0()[3] * other.group5()[0])),
                    ((self.group2()[1] * other.group1()[3]) + (self.group0()[3] * other.group5()[1])),
                    ((self.group2()[2] * other.group1()[3]) + (self.group0()[3] * other.group5()[2])),
                    (-(self.group1()[1] * other.group1()[1]) - (self.group1()[0] * other.group1()[0])),
                ])),
            // e423, e431, e412
            ((Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[3]) * other.group6())
                + Simd32x3::from([
                    ((self.group0()[1] * other.group1()[2]) - (self.group0()[2] * other.group1()[1])),
                    (-(self.group0()[0] * other.group1()[2]) + (self.group0()[2] * other.group1()[0])),
                    ((self.group0()[0] * other.group1()[1]) - (self.group0()[1] * other.group1()[0])),
                ])),
            // e235, e315, e125
            ((Simd32x3::from(self.group0()[3]) * other.group7())
                + (Simd32x3::from(other[e1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + Simd32x3::from([
                    ((self.group2()[2] * other.group1()[1]) - (self.group2()[1] * other.group1()[2])),
                    (-(self.group2()[2] * other.group1()[0]) + (self.group2()[0] * other.group1()[2])),
                    ((self.group2()[1] * other.group1()[0]) - (self.group2()[0] * other.group1()[1])),
                ])),
            // e4235, e4315, e4125, e3215
            ((self.group3() * Simd32x4::from(other.group0()[0]))
                - (swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group10()[0]]))
                + (Simd32x4::from(self.group0()[3]) * other.group8())
                - (swizzle!(other.group3(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * other.group4()[1])
                        + (self.group1()[3] * other.group10()[2])
                        + (self.group1()[0] * other.group3()[3])
                        + (self.group0()[1] * other.group3()[2])),
                    ((self.group2()[0] * other.group4()[2])
                        + (self.group1()[3] * other.group10()[1])
                        + (self.group1()[1] * other.group3()[3])
                        + (self.group0()[2] * other.group3()[0])),
                    ((self.group2()[1] * other.group4()[0])
                        + (self.group1()[3] * other.group10()[0])
                        + (self.group1()[2] * other.group3()[3])
                        + (self.group0()[0] * other.group3()[1])),
                    (-(self.group2()[1] * other.group10()[1])
                        - (self.group2()[0] * other.group10()[2])
                        - (self.group1()[1] * other.group3()[1])
                        - (self.group1()[0] * other.group3()[0])),
                ])),
            // e1234
            ((self.group2()[3] * other.group0()[0]) - (self.group1()[2] * other.group4()[2]) - (self.group1()[1] * other.group4()[1]) - (self.group1()[0] * other.group4()[0])
                + (self.group0()[3] * other[e35])
                - (self.group0()[2] * other.group10()[0])
                - (self.group0()[0] * other.group10()[2])
                - (self.group0()[1] * other.group10()[1])),
            // e12, e31, e23
            ((Simd32x3::from(self.group0()[3]) * other.group10()) + (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[2], self.group1()[1], self.group1()[0]]))),
        );
    }
}
impl Wedge<Plane> for VersorOdd {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ (Simd32x4::from(self.group0()[3]) * other.group0()));
    }
}
impl Wedge<RoundPoint> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       15       31        0
    //  no simd       24       43        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group3()[3]]))
                + (swizzle!(other.group0(), 2, 0, 1, 2) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group3()[2]]))
                + Simd32x4::from([
                    ((self.group0()[2] * other.group0()[1]) * -1.0),
                    ((self.group0()[0] * other.group0()[2]) * -1.0),
                    ((self.group0()[1] * other.group0()[0]) * -1.0),
                    ((self.group3()[1] * other.group0()[1]) + (self.group2()[3] * other[e2]) + (self.group3()[0] * other.group0()[0])),
                ])),
            // e415, e425, e435, e321
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group0(), 0, 1, 2, 2))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group0()[3]) + (self.group0()[0] * other[e2])),
                    ((self.group2()[1] * other.group0()[3]) + (self.group0()[1] * other[e2])),
                    ((self.group2()[2] * other.group0()[3]) + (self.group0()[2] * other[e2])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
            // e235, e315, e125, e5
            Simd32x4::from([
                ((self.group2()[2] * other.group0()[1]) + (self.group1()[0] * other[e2]) - (self.group2()[1] * other.group0()[2])),
                (-(self.group2()[2] * other.group0()[0]) + (self.group1()[1] * other[e2]) + (self.group2()[0] * other.group0()[2])),
                ((self.group2()[1] * other.group0()[0]) + (self.group1()[2] * other[e2]) - (self.group2()[0] * other.group0()[1])),
                (self.group0()[3] * other[e2]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[3]) * other.group0()),
        );
    }
}
impl Wedge<Scalar> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (self.group0() * Simd32x4::from(other[scalar])),
            // e23, e31, e12, e45
            (self.group1() * Simd32x4::from(other[scalar])),
            // e15, e25, e35, e1234
            (self.group2() * Simd32x4::from(other[scalar])),
            // e4235, e4315, e4125, e3215
            (self.group3() * Simd32x4::from(other[scalar])),
        );
    }
}
impl Wedge<Sphere> for VersorOdd {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            (Simd32x4::from(self.group0()[3]) * other.group0()),
            // e1234
            (self.group0()[3] * other[e4315]),
        );
    }
}
impl Wedge<VersorEven> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       33        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       30       40        0
    //  no simd       48       61        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            ((Simd32x4::from(other.group3()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group3()[3]]))
                + (Simd32x4::from(self.group0()[3]) * other.group0())
                + (swizzle!(other.group3(), 2, 0, 1, 2) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group3()[2]]))
                - (swizzle!(self.group0(), 2, 0, 1, 2) * Simd32x4::from([other.group3()[1], other.group3()[2], other.group3()[0], other.group2()[2]]))
                + Simd32x4::from([
                    0.0,
                    0.0,
                    0.0,
                    ((self.group3()[1] * other.group3()[1]) + (self.group3()[0] * other.group3()[0]) + (self.group2()[3] * other.group2()[3])
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
            // e415, e425, e435, e321
            (-(swizzle!(self.group1(), 3, 3, 3, 2) * swizzle!(other.group3(), 0, 1, 2, 2))
                + (self.group0() * Simd32x4::from([other.group2()[3], other.group2()[3], other.group2()[3], other.group1()[3]]))
                + Simd32x4::from([
                    ((self.group2()[0] * other.group3()[3]) + (self.group0()[3] * other.group1()[0])),
                    ((self.group2()[1] * other.group3()[3]) + (self.group0()[3] * other.group1()[1])),
                    ((self.group2()[2] * other.group3()[3]) + (self.group0()[3] * other.group1()[2])),
                    (-(self.group1()[1] * other.group3()[1]) - (self.group1()[0] * other.group3()[0])),
                ])),
            // e235, e315, e125, e5
            Simd32x4::from([
                ((self.group2()[2] * other.group3()[1]) - (self.group2()[1] * other.group3()[2]) + (self.group0()[3] * other.group2()[0]) + (self.group1()[0] * other.group2()[3])),
                (-(self.group2()[2] * other.group3()[0])
                    + (self.group2()[0] * other.group3()[2])
                    + (self.group0()[3] * other.group2()[1])
                    + (self.group1()[1] * other.group2()[3])),
                ((self.group2()[1] * other.group3()[0]) - (self.group2()[0] * other.group3()[1]) + (self.group0()[3] * other.group2()[2]) + (self.group1()[2] * other.group2()[3])),
                (self.group0()[3] * other.group2()[3]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[3]) * other.group3()),
        );
    }
}
impl Wedge<VersorOdd> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       29        0
    //    simd4        7        8        0
    // Totals...
    // yes simd       27       37        0
    //  no simd       48       61        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                ((self.group0()[0] * other.group0()[3]) + (self.group0()[3] * other.group0()[0])),
                ((self.group0()[1] * other.group0()[3]) + (self.group0()[3] * other.group0()[1])),
                ((self.group0()[2] * other.group0()[3]) + (self.group0()[3] * other.group0()[2])),
                (self.group0()[3] * other.group0()[3]),
            ]),
            // e23, e31, e12, e45
            ((Simd32x4::from(self.group0()[3]) * other.group1()) + (self.group1() * Simd32x4::from(other.group0()[3]))),
            // e15, e25, e35, e1234
            ((Simd32x4::from(self.group0()[3]) * other.group2())
                + (self.group2() * Simd32x4::from(other.group0()[3]))
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
            // e4235, e4315, e4125, e3215
            ((self.group3() * Simd32x4::from(other.group0()[3]))
                - (swizzle!(self.group2(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + (Simd32x4::from(self.group0()[3]) * other.group3())
                - (swizzle!(other.group2(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group2()[2] * other.group0()[1])
                        + (self.group1()[3] * other.group1()[0])
                        + (self.group1()[0] * other.group1()[3])
                        + (self.group0()[1] * other.group2()[2])),
                    ((self.group2()[0] * other.group0()[2])
                        + (self.group1()[3] * other.group1()[1])
                        + (self.group1()[1] * other.group1()[3])
                        + (self.group0()[2] * other.group2()[0])),
                    ((self.group2()[1] * other.group0()[0])
                        + (self.group1()[3] * other.group1()[2])
                        + (self.group1()[2] * other.group1()[3])
                        + (self.group0()[0] * other.group2()[1])),
                    (-(self.group2()[1] * other.group1()[1])
                        - (self.group2()[0] * other.group1()[0])
                        - (self.group1()[1] * other.group2()[1])
                        - (self.group1()[0] * other.group2()[0])),
                ])),
        );
    }
}
