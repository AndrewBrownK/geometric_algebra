// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 512
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         5       9       0
//  Average:         7      13       0
//  Maximum:       131     153       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         5      12       0
//  Average:        11      20       0
//  Maximum:       211     243       0
impl std::ops::Div<wedge> for AntiCircleRotor {
    type Output = wedge_partial<AntiCircleRotor>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       35        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       26       39        0
    //  no simd       35       51        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] * self.group2()[3]) + (self.group0()[0] * other.group2()[3]),
                (other.group0()[1] * self.group2()[3]) + (self.group0()[1] * other.group2()[3]),
                (other.group0()[2] * self.group2()[3]) + (self.group0()[2] * other.group2()[3]),
                other.group2()[3] * self.group2()[3],
            ]),
            // e23, e31, e12, e45
            (Simd32x4::from(other.group2()[3]) * self.group1()) + (Simd32x4::from(self.group2()[3]) * other.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] * self.group2()[3]) + (other.group2()[3] * self.group2()[0]),
                (other.group2()[1] * self.group2()[3]) + (other.group2()[3] * self.group2()[1]),
                (other.group2()[2] * self.group2()[3]) + (other.group2()[3] * self.group2()[2]),
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group2()[2]) + (self.group0()[1] * other.group2()[2]) + (other.group1()[0] * self.group1()[3]) + (other.group1()[3] * self.group1()[0]),
                (other.group0()[2] * self.group2()[0]) + (self.group0()[2] * other.group2()[0]) + (other.group1()[1] * self.group1()[3]) + (other.group1()[3] * self.group1()[1]),
                (other.group0()[0] * self.group2()[1]) + (self.group0()[0] * other.group2()[1]) + (other.group1()[2] * self.group1()[3]) + (other.group1()[3] * self.group1()[2]),
                -(other.group1()[1] * self.group2()[1]) - (other.group1()[2] * self.group2()[2]) - (other.group2()[1] * self.group1()[1]) - (other.group2()[2] * self.group1()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiDipoleInversion> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       43        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       33       46        0
    //  no simd       39       55        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[1] * other.group3()[2]) + (other.group0()[0] * self.group2()[3]) + (self.group1()[0] * other.group2()[3]) - (self.group0()[2] * other.group3()[1]),
                (self.group0()[2] * other.group3()[0]) + (other.group0()[1] * self.group2()[3]) + (self.group1()[1] * other.group2()[3]) - (self.group0()[0] * other.group3()[2]),
                (self.group0()[0] * other.group3()[1]) + (other.group0()[2] * self.group2()[3]) + (self.group1()[2] * other.group2()[3]) - (self.group0()[1] * other.group3()[0]),
                -(self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[3] * other.group1()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group3()[3]) + (self.group2()[3] * other.group1()[0]),
                (self.group0()[1] * other.group3()[3]) + (self.group2()[3] * other.group1()[1]),
                (self.group0()[2] * other.group3()[3]) + (self.group2()[3] * other.group1()[2]),
                -(self.group1()[1] * other.group3()[1]) - (self.group1()[2] * other.group3()[2]),
            ]) + (Simd32x4::from([other.group2()[3], other.group2()[3], other.group2()[3], other.group1()[3]]) * self.group2())
                - (crate::swizzle!(self.group1(), 3, 3, 3, 0) * crate::swizzle!(other.group3(), 0, 1, 2, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] * other.group3()[3]) + (self.group2()[2] * other.group3()[1]) + (self.group2()[3] * other.group2()[0]) - (self.group2()[1] * other.group3()[2]),
                (self.group1()[1] * other.group3()[3]) + (self.group2()[0] * other.group3()[2]) + (self.group2()[3] * other.group2()[1]) - (self.group2()[2] * other.group3()[0]),
                (self.group1()[2] * other.group3()[3]) + (self.group2()[1] * other.group3()[0]) + (self.group2()[3] * other.group2()[2]) - (self.group2()[0] * other.group3()[1]),
                self.group2()[3] * other.group3()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
        );
    }
}
impl Wedge<AntiDualNum> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other.group0()[1]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from(other.group0()[1]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other.group0()[1], other.group0()[1], other.group0()[1], other.group0()[0]]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl Wedge<AntiFlatPoint> for AntiCircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, self.group2()[3] * other.group0()[3]]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                self.group2()[3] * other.group0()[0],
                self.group2()[3] * other.group0()[1],
                self.group2()[3] * other.group0()[2],
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group1()[3] * other.group0()[3]),
            ]),
        );
    }
}
impl Wedge<AntiFlector> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       32        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       18       33        0
    //  no simd       21       36        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[1] * other.group1()[2]) - (self.group0()[2] * other.group1()[1]),
                (self.group0()[2] * other.group1()[0]) - (self.group0()[0] * other.group1()[2]),
                (self.group0()[0] * other.group1()[1]) - (self.group0()[1] * other.group1()[0]),
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group1()[3] * other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group0()[0] * other.group1()[3],
                self.group0()[1] * other.group1()[3],
                self.group0()[2] * other.group1()[3],
                (self.group2()[3] * other.group0()[3]) - (self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2]),
            ]) - (crate::swizzle!(self.group1(), 3, 3, 3, 0) * crate::swizzle!(other.group1(), 0, 1, 2, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] * other.group1()[3]) + (self.group2()[2] * other.group1()[1]) + (self.group2()[3] * other.group0()[0]) - (self.group2()[1] * other.group1()[2]),
                (self.group1()[1] * other.group1()[3]) + (self.group2()[0] * other.group1()[2]) + (self.group2()[3] * other.group0()[1]) - (self.group2()[2] * other.group1()[0]),
                (self.group1()[2] * other.group1()[3]) + (self.group2()[1] * other.group1()[0]) + (self.group2()[3] * other.group0()[2]) - (self.group2()[0] * other.group1()[1]),
                self.group2()[3] * other.group1()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[3] * other.group1()[0], self.group2()[3] * other.group1()[1], self.group2()[3] * other.group1()[2], 0.0]),
        );
    }
}
impl Wedge<AntiLine> for AntiCircleRotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       24        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([other.group0()[0] * self.group2()[3], other.group0()[1] * self.group2()[3], other.group0()[2] * self.group2()[3], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other.group1()[0] * self.group2()[3],
                other.group1()[1] * self.group2()[3],
                other.group1()[2] * self.group2()[3],
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group1()[2]) + (other.group0()[0] * self.group1()[3]) - (self.group0()[2] * other.group1()[1]),
                (self.group0()[2] * other.group1()[0]) + (other.group0()[1] * self.group1()[3]) - (self.group0()[0] * other.group1()[2]),
                (self.group0()[0] * other.group1()[1]) + (other.group0()[2] * self.group1()[3]) - (self.group0()[1] * other.group1()[0]),
                -(other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2]),
            ]),
        );
    }
}
impl Wedge<AntiMotor> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       20       36        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[0] * other.group0()[3]) + (self.group2()[3] * other.group0()[0]),
                (self.group1()[1] * other.group0()[3]) + (self.group2()[3] * other.group0()[1]),
                (self.group1()[2] * other.group0()[3]) + (self.group2()[3] * other.group0()[2]),
                self.group1()[3] * other.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group2()[0] * other.group0()[3]) + (self.group2()[3] * other.group1()[0]),
                (self.group2()[1] * other.group0()[3]) + (self.group2()[3] * other.group1()[1]),
                (self.group2()[2] * other.group0()[3]) + (self.group2()[3] * other.group1()[2]),
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group1()[3] * other.group0()[0],
                self.group1()[3] * other.group0()[1],
                self.group1()[3] * other.group0()[2],
                -(self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[2] * other.group0()[2]),
            ]) + (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group2()[3]]) * crate::swizzle!(other.group1(), 2, 0, 1, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiPlane> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       14       28        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group0()[0] * other.group0()[3],
                self.group0()[1] * other.group0()[3],
                self.group0()[2] * other.group0()[3],
                -(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2]),
            ]) - (crate::swizzle!(self.group1(), 3, 3, 3, 0) * crate::swizzle!(other.group0(), 0, 1, 2, 0)),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] * other.group0()[3]) + (self.group2()[2] * other.group0()[1]) - (self.group2()[1] * other.group0()[2]),
                (self.group1()[1] * other.group0()[3]) + (self.group2()[0] * other.group0()[2]) - (self.group2()[2] * other.group0()[0]),
                (self.group1()[2] * other.group0()[3]) + (self.group2()[1] * other.group0()[0]) - (self.group2()[0] * other.group0()[1]),
                0.0,
            ]),
            // e1, e2, e3, e5
            Simd32x4::from(self.group2()[3]) * other.group0(),
        );
    }
}
impl Wedge<AntiScalar> for AntiCircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self.group2()[3] * other[e12345]);
    }
}
impl Wedge<Circle> for AntiCircleRotor {
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
            Simd32x3::from(self.group2()[3]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self.group2()[3]) * other.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                other.group2()[0] * self.group2()[3],
                other.group2()[1] * self.group2()[3],
                other.group2()[2] * self.group2()[3],
                -(self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[3] * other.group1()[3]),
            ]),
        );
    }
}
impl Wedge<CircleRotor> for AntiCircleRotor {
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
            Simd32x3::from(self.group2()[3]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self.group2()[3]) * other.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                self.group2()[3] * other.group2()[0],
                self.group2()[3] * other.group2()[1],
                self.group2()[3] * other.group2()[2],
                (self.group2()[3] * other.group2()[3])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[3] * other.group1()[3]),
            ]),
        );
    }
}
impl Wedge<Dipole> for AntiCircleRotor {
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
            Simd32x3::from(self.group2()[3]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self.group2()[3]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other.group2()[0] * self.group2()[3],
                other.group2()[1] * self.group2()[3],
                other.group2()[2] * self.group2()[3],
                -(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group2()[2]) + (other.group0()[1] * self.group2()[2]) + (self.group1()[0] * other.group1()[3]) + (self.group1()[3] * other.group1()[0])
                    - (self.group0()[2] * other.group2()[1]),
                (self.group0()[2] * other.group2()[0]) + (other.group0()[2] * self.group2()[0]) + (self.group1()[1] * other.group1()[3]) + (self.group1()[3] * other.group1()[1])
                    - (self.group0()[0] * other.group2()[2]),
                (self.group0()[0] * other.group2()[1]) + (other.group0()[0] * self.group2()[1]) + (self.group1()[2] * other.group1()[3]) + (self.group1()[3] * other.group1()[2])
                    - (self.group0()[1] * other.group2()[0]),
                -(other.group2()[0] * self.group1()[0])
                    - (other.group2()[1] * self.group1()[1])
                    - (other.group2()[2] * self.group1()[2])
                    - (self.group2()[1] * other.group1()[1])
                    - (self.group2()[2] * other.group1()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<DipoleInversion> for AntiCircleRotor {
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
            Simd32x3::from(self.group2()[3]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self.group2()[3]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group2()[3] * other.group2()[0],
                self.group2()[3] * other.group2()[1],
                self.group2()[3] * other.group2()[2],
                (self.group2()[3] * other.group2()[3])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group2()[2]) + (self.group1()[0] * other.group1()[3]) + (self.group1()[3] * other.group1()[0]) + (self.group2()[3] * other.group3()[0]),
                (self.group0()[2] * other.group2()[0]) + (self.group1()[1] * other.group1()[3]) + (self.group1()[3] * other.group1()[1]) + (self.group2()[3] * other.group3()[1]),
                (self.group0()[0] * other.group2()[1]) + (self.group1()[2] * other.group1()[3]) + (self.group1()[3] * other.group1()[2]) + (self.group2()[3] * other.group3()[2]),
                -(self.group1()[1] * other.group2()[1]) - (self.group1()[2] * other.group2()[2]) - (self.group2()[1] * other.group1()[1]) - (self.group2()[2] * other.group1()[2]),
            ]) + (Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group3()[3]]) * crate::swizzle!(self.group2(), 2, 0, 1, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<DualNum> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                other.group0()[0] * self.group1()[0],
                other.group0()[0] * self.group1()[1],
                other.group0()[0] * self.group1()[2],
                other.group0()[1] * self.group2()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0] * self.group2()[0], other.group0()[0] * self.group2()[1], other.group0()[0] * self.group2()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group2()[3]]),
        );
    }
}
impl Wedge<FlatPoint> for AntiCircleRotor {
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
            Simd32x4::from(self.group2()[3]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group0()[2]) + (self.group1()[0] * other.group0()[3]),
                (self.group0()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3]),
                (self.group0()[0] * other.group0()[1]) + (self.group1()[2] * other.group0()[3]),
                -(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2]),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<Flector> for AntiCircleRotor {
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
            Simd32x4::from(self.group2()[3]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group0()[2]) + (self.group1()[0] * other.group0()[3]),
                (self.group0()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3]),
                (self.group0()[0] * other.group0()[1]) + (self.group1()[2] * other.group0()[3]),
                -(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(self.group2()[3]) * other.group1())
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<Line> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn wedge(self, other: Line) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                other.group0()[0] * self.group2()[3],
                other.group0()[1] * self.group2()[3],
                other.group0()[2] * self.group2()[3],
                -(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0] * self.group2()[3], other.group1()[1] * self.group2()[3], other.group1()[2] * self.group2()[3], 0.0]),
        );
    }
}
impl Wedge<Motor> for AntiCircleRotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       12       20        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self.group0()[0] * other.group1()[3],
                self.group0()[1] * other.group1()[3],
                self.group0()[2] * other.group1()[3],
                -(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(self.group2()[3]) * other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] * other.group1()[3]) + (self.group2()[3] * other.group1()[0]),
                (self.group1()[1] * other.group1()[3]) + (self.group2()[3] * other.group1()[1]),
                (self.group1()[2] * other.group1()[3]) + (self.group2()[3] * other.group1()[2]),
                self.group2()[3] * other.group1()[3],
            ]),
        );
    }
}
impl Wedge<MultiVector> for AntiCircleRotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       50        0
    //    simd3        7       10        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       48       68        0
    //  no simd       80      112        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                other.group0()[0] * self.group2()[3],
                (other.group0()[1] * self.group2()[3])
                    - (self.group0()[0] * other.group8()[0])
                    - (self.group0()[1] * other.group8()[1])
                    - (self.group0()[2] * other.group8()[2])
                    - (other.group7()[0] * self.group2()[0])
                    - (other.group7()[1] * self.group2()[1])
                    - (other.group7()[2] * self.group2()[2])
                    - (self.group1()[0] * other.group6()[0])
                    - (self.group1()[1] * other.group6()[1])
                    - (self.group1()[2] * other.group6()[2])
                    - (self.group1()[3] * other.group6()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group2()[3]) * other.group1(),
            // e5
            self.group2()[3] * other[e1],
            // e15, e25, e35, e45
            (Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                + (Simd32x4::from(self.group2()[3]) * other.group3()),
            // e41, e42, e43
            (Simd32x3::from(other.group0()[0]) * self.group0()) + (Simd32x3::from(self.group2()[3]) * other.group4()),
            // e23, e31, e12
            (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])) + (Simd32x3::from(self.group2()[3]) * other.group5()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other[e1]) + (self.group2()[3] * other.group6()[0]),
                (self.group0()[1] * other[e1]) + (self.group2()[3] * other.group6()[1]),
                (self.group0()[2] * other[e1]) + (self.group2()[3] * other.group6()[2]),
                -(self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2]),
            ]) + (Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[3], other.group6()[3]]) * self.group2())
                - (crate::swizzle!(self.group1(), 3, 3, 3, 0) * crate::swizzle!(other.group1(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from(self.group2()[3]) * other.group7())
                + (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (self.group2()[2] * other.group1()[1]) - (self.group2()[1] * other.group1()[2]),
                (self.group2()[0] * other.group1()[2]) - (self.group2()[2] * other.group1()[0]),
                (self.group2()[1] * other.group1()[0]) - (self.group2()[0] * other.group1()[1]),
            ]) + (Simd32x3::from(self.group2()[3]) * other.group8())
                + (Simd32x3::from(other[e1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group3()[2]) + (other.group5()[0] * self.group1()[3]) + (self.group1()[0] * other.group3()[3]) + (self.group2()[3] * other.group9()[0]),
                (self.group0()[2] * other.group3()[0]) + (other.group5()[1] * self.group1()[3]) + (self.group1()[1] * other.group3()[3]) + (self.group2()[3] * other.group9()[1]),
                (self.group0()[0] * other.group3()[1]) + (other.group5()[2] * self.group1()[3]) + (self.group1()[2] * other.group3()[3]) + (self.group2()[3] * other.group9()[2]),
                -(other.group5()[1] * self.group2()[1]) - (other.group5()[2] * self.group2()[2]) - (self.group1()[1] * other.group3()[1]) - (self.group1()[2] * other.group3()[2]),
            ]) + (Simd32x4::from([other.group4()[1], other.group4()[2], other.group4()[0], other.group9()[3]]) * crate::swizzle!(self.group2(), 2, 0, 1, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group5()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0)),
            // e1234
            (self.group2()[3] * other[e45])
                - (self.group0()[0] * other.group5()[0])
                - (self.group0()[1] * other.group5()[1])
                - (self.group0()[2] * other.group5()[2])
                - (other.group4()[0] * self.group1()[0])
                - (other.group4()[1] * self.group1()[1])
                - (other.group4()[2] * self.group1()[2]),
        );
    }
}
impl Wedge<Plane> for AntiCircleRotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self.group2()[3]) * other.group0());
    }
}
impl Wedge<RoundPoint> for AntiCircleRotor {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        2        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       20       35        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other[e2]) + (self.group2()[0] * other.group0()[3]),
                (self.group0()[1] * other[e2]) + (self.group2()[1] * other.group0()[3]),
                (self.group0()[2] * other[e2]) + (self.group2()[2] * other.group0()[3]),
                -(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2]),
            ]) - (crate::swizzle!(self.group1(), 3, 3, 3, 0) * crate::swizzle!(other.group0(), 0, 1, 2, 0)),
            // e235, e315, e125, e4
            Simd32x4::from([
                (self.group1()[0] * other[e2]) + (self.group2()[2] * other.group0()[1]) - (self.group2()[1] * other.group0()[2]),
                (self.group1()[1] * other[e2]) + (self.group2()[0] * other.group0()[2]) - (self.group2()[2] * other.group0()[0]),
                (self.group1()[2] * other[e2]) + (self.group2()[1] * other.group0()[0]) - (self.group2()[0] * other.group0()[1]),
                self.group2()[3] * other.group0()[3],
            ]),
            // e1, e2, e3, e5
            Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other[e2]]),
        );
    }
}
impl Wedge<Scalar> for AntiCircleRotor {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(other[scalar]) * self.group2(),
        );
    }
}
impl Wedge<Sphere> for AntiCircleRotor {
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
            Simd32x4::from(self.group2()[3]) * other.group0(),
            // e1234
            self.group2()[3] * other[e4315],
        );
    }
}
impl Wedge<VersorEven> for AntiCircleRotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       31       44        0
    //  no simd       40       56        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[1] * other.group3()[2]) + (self.group1()[0] * other.group3()[3]) - (self.group0()[2] * other.group3()[1]),
                (self.group0()[2] * other.group3()[0]) + (self.group1()[1] * other.group3()[3]) - (self.group0()[0] * other.group3()[2]),
                (self.group0()[0] * other.group3()[1]) + (self.group1()[2] * other.group3()[3]) - (self.group0()[1] * other.group3()[0]),
                -(self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(self.group2()[3]) * other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group2()[3]) + (self.group2()[3] * other.group1()[0]),
                (self.group0()[1] * other.group2()[3]) + (self.group2()[3] * other.group1()[1]),
                (self.group0()[2] * other.group2()[3]) + (self.group2()[3] * other.group1()[2]),
                -(self.group1()[1] * other.group3()[1]) - (self.group1()[2] * other.group3()[2]),
            ]) + (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group1()[3]]) * self.group2())
                - (crate::swizzle!(self.group1(), 3, 3, 3, 0) * crate::swizzle!(other.group3(), 0, 1, 2, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] * other.group2()[3]) + (self.group2()[2] * other.group3()[1]) + (self.group2()[3] * other.group2()[0]) - (self.group2()[1] * other.group3()[2]),
                (self.group1()[1] * other.group2()[3]) + (self.group2()[0] * other.group3()[2]) + (self.group2()[3] * other.group2()[1]) - (self.group2()[2] * other.group3()[0]),
                (self.group1()[2] * other.group2()[3]) + (self.group2()[1] * other.group3()[0]) + (self.group2()[3] * other.group2()[2]) - (self.group2()[0] * other.group3()[1]),
                self.group2()[3] * other.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group2()[3]) * other.group3(),
        );
    }
}
impl Wedge<VersorOdd> for AntiCircleRotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       25       38        0
    //  no simd       40       56        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]) + (self.group2()[3] * other.group0()[0]),
                (self.group0()[1] * other.group0()[3]) + (self.group2()[3] * other.group0()[1]),
                (self.group0()[2] * other.group0()[3]) + (self.group2()[3] * other.group0()[2]),
                self.group2()[3] * other.group0()[3],
            ]),
            // e23, e31, e12, e45
            (Simd32x4::from(self.group2()[3]) * other.group1()) + (Simd32x4::from(other.group0()[3]) * self.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group2()[3] * other.group2()[0],
                self.group2()[3] * other.group2()[1],
                self.group2()[3] * other.group2()[2],
                -(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2]),
            ]) + (Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group2()[3]]) * self.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group2()[2]) + (self.group1()[0] * other.group1()[3]) + (self.group1()[3] * other.group1()[0]) + (self.group2()[3] * other.group3()[0]),
                (self.group0()[2] * other.group2()[0]) + (self.group1()[1] * other.group1()[3]) + (self.group1()[3] * other.group1()[1]) + (self.group2()[3] * other.group3()[1]),
                (self.group0()[0] * other.group2()[1]) + (self.group1()[2] * other.group1()[3]) + (self.group1()[3] * other.group1()[2]) + (self.group2()[3] * other.group3()[2]),
                -(self.group1()[1] * other.group2()[1]) - (self.group1()[2] * other.group2()[2]) - (self.group2()[1] * other.group1()[1]) - (self.group2()[2] * other.group1()[2]),
            ]) + (Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group3()[3]]) * crate::swizzle!(self.group2(), 2, 0, 1, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0)),
        );
    }
}
impl std::ops::Div<wedge> for AntiDipoleInversion {
    type Output = wedge_partial<AntiDipoleInversion>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       43        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       33       46        0
    //  no simd       39       55        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[1] * self.group3()[2]) + (self.group0()[0] * other.group2()[3]) + (other.group1()[0] * self.group2()[3]) - (other.group0()[2] * self.group3()[1]),
                (other.group0()[2] * self.group3()[0]) + (self.group0()[1] * other.group2()[3]) + (other.group1()[1] * self.group2()[3]) - (other.group0()[0] * self.group3()[2]),
                (other.group0()[0] * self.group3()[1]) + (self.group0()[2] * other.group2()[3]) + (other.group1()[2] * self.group2()[3]) - (other.group0()[1] * self.group3()[0]),
                -(other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])
                    - (other.group1()[3] * self.group1()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self.group3()[3]) + (other.group2()[3] * self.group1()[0]),
                (other.group0()[1] * self.group3()[3]) + (other.group2()[3] * self.group1()[1]),
                (other.group0()[2] * self.group3()[3]) + (other.group2()[3] * self.group1()[2]),
                -(other.group1()[1] * self.group3()[1]) - (other.group1()[2] * self.group3()[2]),
            ]) + (Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group1()[3]]) * other.group2())
                - (crate::swizzle!(other.group1(), 3, 3, 3, 0) * crate::swizzle!(self.group3(), 0, 1, 2, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] * self.group3()[3]) + (other.group2()[2] * self.group3()[1]) + (other.group2()[3] * self.group2()[0]) - (other.group2()[1] * self.group3()[2]),
                (other.group1()[1] * self.group3()[3]) + (other.group2()[0] * self.group3()[2]) + (other.group2()[3] * self.group2()[1]) - (other.group2()[2] * self.group3()[0]),
                (other.group1()[2] * self.group3()[3]) + (other.group2()[1] * self.group3()[0]) + (other.group2()[3] * self.group2()[2]) - (other.group2()[0] * self.group3()[1]),
                other.group2()[3] * self.group3()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl Wedge<AntiDipoleInversion> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd3        1        2        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       19       26        0
    //  no simd       48       60        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self.group2()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))
                - (Simd32x3::from(other.group2()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]])),
            // e23, e31, e12, e45
            (Simd32x4::from([self.group3()[1], self.group3()[2], self.group3()[0], self.group2()[3]]) * crate::swizzle!(other.group3(), 2, 0, 1, 3))
                - (Simd32x4::from([other.group3()[1], other.group3()[2], other.group3()[0], other.group2()[3]]) * crate::swizzle!(self.group3(), 2, 0, 1, 3)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (other.group0()[1] * self.group3()[1]) + (other.group0()[2] * self.group3()[2]) + (other.group1()[3] * self.group2()[3])
                    - (self.group0()[1] * other.group3()[1])
                    - (self.group0()[2] * other.group3()[2])
                    - (other.group2()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group0()[0]]) * crate::swizzle!(self.group3(), 0, 1, 2, 0))
                - (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group0()[0]]) * crate::swizzle!(other.group3(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[1] * self.group3()[2]) - (other.group3()[2] * self.group1()[1]),
                (other.group1()[2] * self.group3()[0]) - (other.group3()[0] * self.group1()[2]),
                (other.group1()[0] * self.group3()[1]) - (other.group3()[1] * self.group1()[0]),
                (other.group3()[3] * self.group1()[3]) - (other.group2()[2] * self.group3()[2]),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[0]]) * crate::swizzle!(other.group3(), 3, 3, 3, 0))
                + (Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group3()[1]]) * crate::swizzle!(self.group2(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[2]]) * crate::swizzle!(other.group3(), 1, 2, 0, 2))
                - (Simd32x4::from(self.group3()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[1]]) * crate::swizzle!(other.group2(), 3, 3, 3, 1)),
        );
    }
}
impl Wedge<AntiDualNum> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       16        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                other.group0()[1] * self.group0()[0],
                other.group0()[1] * self.group0()[1],
                other.group0()[1] * self.group0()[2],
                other.group0()[0] * self.group3()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(other.group0()[1]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other.group0()[1]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[1]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl Wedge<AntiFlatPoint> for AntiDipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group2()[3] * other.group0()[0],
                self.group2()[3] * other.group0()[1],
                self.group2()[3] * other.group0()[2],
                -(self.group3()[0] * other.group0()[0]) - (self.group3()[1] * other.group0()[1]) - (self.group3()[2] * other.group0()[2]) - (self.group3()[3] * other.group0()[3]),
            ]),
            // e1234
            self.group2()[3] * other.group0()[3],
        );
    }
}
impl Wedge<AntiFlector> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       24        0
    //    simd3        0        1        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       13       29        0
    //  no simd       25       43        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group2()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group3()[1] * other.group1()[2]) - (self.group3()[2] * other.group1()[1]),
                (self.group3()[2] * other.group1()[0]) - (self.group3()[0] * other.group1()[2]),
                (self.group3()[0] * other.group1()[1]) - (self.group3()[1] * other.group1()[0]),
                self.group2()[3] * other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group3()[0] * other.group1()[3],
                self.group3()[1] * other.group1()[3],
                self.group3()[2] * other.group1()[3],
                (self.group2()[3] * other.group0()[3]) - (self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]),
            ]) - (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group0()[0]]) * crate::swizzle!(other.group1(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group1()[1] * other.group1()[2]) * -1.0,
                (self.group1()[2] * other.group1()[0]) * -1.0,
                (self.group1()[0] * other.group1()[1]) * -1.0,
                (self.group2()[2] * other.group1()[2])
                    - (self.group3()[0] * other.group0()[0])
                    - (self.group3()[1] * other.group0()[1])
                    - (self.group3()[2] * other.group0()[2])
                    - (self.group3()[3] * other.group0()[3]),
            ]) + (Simd32x4::from(other.group1()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[1]]) * crate::swizzle!(self.group2(), 3, 3, 3, 1)),
        );
    }
}
impl Wedge<AntiLine> for AntiDipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       13       24        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self.group2()[3]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group1()[0] * self.group2()[3],
                other.group1()[1] * self.group2()[3],
                other.group1()[2] * self.group2()[3],
                -(other.group0()[0] * self.group3()[0]) - (other.group0()[1] * self.group3()[1]) - (other.group0()[2] * self.group3()[2]),
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group0()[0] * self.group3()[3]) + (other.group1()[2] * self.group3()[1]) - (other.group1()[1] * self.group3()[2]),
                (other.group0()[1] * self.group3()[3]) + (other.group1()[0] * self.group3()[2]) - (other.group1()[2] * self.group3()[0]),
                (other.group0()[2] * self.group3()[3]) + (other.group1()[1] * self.group3()[0]) - (other.group1()[0] * self.group3()[1]),
                -(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2]),
            ]),
        );
    }
}
impl Wedge<AntiMotor> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       31        0
    //  no simd       24       40        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self.group0()[0] * other.group0()[3],
                self.group0()[1] * other.group0()[3],
                self.group0()[2] * other.group0()[3],
                -(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group2()[3] * other.group1()[0],
                self.group2()[3] * other.group1()[1],
                self.group2()[3] * other.group1()[2],
                -(self.group3()[0] * other.group0()[0]) - (self.group3()[1] * other.group0()[1]) - (self.group3()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(other.group0()[3]) * self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] * other.group0()[3]) + (self.group3()[1] * other.group1()[2]) + (self.group3()[3] * other.group0()[0]) - (self.group3()[2] * other.group1()[1]),
                (self.group2()[1] * other.group0()[3]) + (self.group3()[2] * other.group1()[0]) + (self.group3()[3] * other.group0()[1]) - (self.group3()[0] * other.group1()[2]),
                (self.group2()[2] * other.group0()[3]) + (self.group3()[0] * other.group1()[1]) + (self.group3()[3] * other.group0()[2]) - (self.group3()[1] * other.group1()[0]),
                self.group3()[3] * other.group0()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl Wedge<AntiPlane> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       20        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        8       24        0
    //  no simd       17       35        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group2()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group3()[1] * other.group0()[2]) - (self.group3()[2] * other.group0()[1]),
                (self.group3()[2] * other.group0()[0]) - (self.group3()[0] * other.group0()[2]),
                (self.group3()[0] * other.group0()[1]) - (self.group3()[1] * other.group0()[0]),
                self.group2()[3] * other.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group3()[0] * other.group0()[3],
                self.group3()[1] * other.group0()[3],
                self.group3()[2] * other.group0()[3],
                -(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]) - (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group0()[0]]) * crate::swizzle!(other.group0(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group1()[1] * other.group0()[2]) * -1.0,
                (self.group1()[2] * other.group0()[0]) * -1.0,
                (self.group1()[0] * other.group0()[1]) * -1.0,
                (self.group2()[1] * other.group0()[1]) + (self.group2()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<Circle> for AntiDipoleInversion {
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
            Simd32x4::from([
                (other.group2()[0] * self.group2()[3]) + (self.group3()[2] * other.group1()[1]),
                (other.group2()[1] * self.group2()[3]) + (self.group3()[0] * other.group1()[2]),
                (other.group2()[2] * self.group2()[3]) + (self.group3()[1] * other.group1()[0]),
                -(other.group2()[2] * self.group3()[2]) - (self.group3()[3] * other.group1()[3]),
            ]) - (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[0]]) * crate::swizzle!(self.group3(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[1]]) * crate::swizzle!(self.group3(), 1, 2, 0, 1)),
            // e1234
            (other.group0()[0] * self.group3()[0]) + (other.group0()[1] * self.group3()[1]) + (other.group0()[2] * self.group3()[2]) + (self.group2()[3] * other.group1()[3]),
        );
    }
}
impl Wedge<CircleRotor> for AntiDipoleInversion {
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
            Simd32x4::from([
                (self.group2()[3] * other.group2()[0]) + (self.group3()[2] * other.group1()[1]),
                (self.group2()[3] * other.group2()[1]) + (self.group3()[0] * other.group1()[2]),
                (self.group2()[3] * other.group2()[2]) + (self.group3()[1] * other.group1()[0]),
                -(self.group3()[2] * other.group2()[2]) - (self.group3()[3] * other.group1()[3]),
            ]) - (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[0]]) * crate::swizzle!(self.group3(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[1]]) * crate::swizzle!(self.group3(), 1, 2, 0, 1)),
            // e1234
            (other.group0()[0] * self.group3()[0]) + (other.group0()[1] * self.group3()[1]) + (other.group0()[2] * self.group3()[2]) + (self.group2()[3] * other.group1()[3]),
        );
    }
}
impl Wedge<Dipole> for AntiDipoleInversion {
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
            (Simd32x3::from(self.group2()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self.group3()[3]) + (other.group2()[0] * self.group2()[3]),
                (other.group0()[1] * self.group3()[3]) + (other.group2()[1] * self.group2()[3]),
                (other.group0()[2] * self.group3()[3]) + (other.group2()[2] * self.group2()[3]),
                -(self.group3()[1] * other.group1()[1]) - (self.group3()[2] * other.group1()[2]),
            ]) - (crate::swizzle!(self.group3(), 0, 1, 2, 0) * crate::swizzle!(other.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group2()[2] * self.group3()[1]) + (self.group3()[3] * other.group1()[0]) - (other.group2()[1] * self.group3()[2]),
                (other.group2()[0] * self.group3()[2]) + (self.group3()[3] * other.group1()[1]) - (other.group2()[2] * self.group3()[0]),
                (other.group2()[1] * self.group3()[0]) + (self.group3()[3] * other.group1()[2]) - (other.group2()[0] * self.group3()[1]),
                -(self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[3] * other.group1()[3]),
            ]),
        );
    }
}
impl Wedge<DipoleInversion> for AntiDipoleInversion {
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
            (Simd32x3::from(self.group2()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self.group3()[3]) + (self.group2()[3] * other.group2()[0]),
                (other.group0()[1] * self.group3()[3]) + (self.group2()[3] * other.group2()[1]),
                (other.group0()[2] * self.group3()[3]) + (self.group2()[3] * other.group2()[2]),
                -(self.group3()[1] * other.group1()[1]) - (self.group3()[2] * other.group1()[2]),
            ]) - (crate::swizzle!(self.group3(), 0, 1, 2, 0) * crate::swizzle!(other.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (self.group2()[3] * other.group3()[3]) + (self.group3()[2] * other.group3()[2]) + (self.group3()[3] * other.group2()[3])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[3] * other.group1()[3]),
            ]) + (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group3()[1]]) * crate::swizzle!(self.group3(), 3, 3, 3, 1))
                + (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group0()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<DualNum> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       16        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group3()[3] * -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group1()[3] * -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[0] * self.group2()[0] * -1.0,
                other.group0()[0] * self.group2()[1] * -1.0,
                other.group0()[0] * self.group2()[2] * -1.0,
                0.0,
            ]),
        );
    }
}
impl Wedge<FlatPoint> for AntiDipoleInversion {
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
            Simd32x4::from([
                self.group2()[3] * other.group0()[0],
                self.group2()[3] * other.group0()[1],
                self.group2()[3] * other.group0()[2],
                -(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group1()[3] * other.group0()[3]),
            ]) - (Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group0()[0]]) * crate::swizzle!(other.group0(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group3()[1] * other.group0()[2]) - (self.group3()[2] * other.group0()[1]),
                (self.group3()[2] * other.group0()[0]) - (self.group3()[0] * other.group0()[2]),
                (self.group3()[0] * other.group0()[1]) - (self.group3()[1] * other.group0()[0]),
                0.0,
            ]),
        );
    }
}
impl Wedge<Flector> for AntiDipoleInversion {
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (self.group3()[0] * other.group1()[0]) + (self.group3()[1] * other.group1()[1]) + (self.group3()[2] * other.group1()[2])
                    - (self.group0()[1] * other.group0()[1])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group1()[3] * other.group0()[3]),
            ]) + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group0()[0]]) * crate::swizzle!(other.group0(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group3()[1] * other.group0()[2]) - (self.group3()[2] * other.group0()[1]),
                (self.group3()[2] * other.group0()[0]) - (self.group3()[0] * other.group0()[2]),
                (self.group3()[0] * other.group0()[1]) - (self.group3()[1] * other.group0()[0]),
                0.0,
            ]),
        );
    }
}
impl Wedge<Line> for AntiDipoleInversion {
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
            Simd32x4::from([
                (other.group0()[1] * self.group3()[2]) + (other.group1()[0] * self.group2()[3]),
                (other.group0()[2] * self.group3()[0]) + (other.group1()[1] * self.group2()[3]),
                (other.group0()[0] * self.group3()[1]) + (other.group1()[2] * self.group2()[3]),
                -(other.group1()[1] * self.group3()[1]) - (other.group1()[2] * self.group3()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<Motor> for AntiDipoleInversion {
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
            Simd32x4::from(other.group1()[3]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group2()[3] * other.group1()[0]) + (self.group3()[2] * other.group0()[1]),
                (self.group2()[3] * other.group1()[1]) + (self.group3()[0] * other.group0()[2]),
                (self.group2()[3] * other.group1()[2]) + (self.group3()[1] * other.group0()[0]),
                -(self.group3()[1] * other.group1()[1]) - (self.group3()[2] * other.group1()[2]),
            ]) + (Simd32x4::from(other.group1()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<MultiVector> for AntiDipoleInversion {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       56        0
    //    simd3        6        8        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       53       74        0
    //  no simd       89      120        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self.group2()[3] * other.group9()[3])
                    + (self.group3()[0] * other.group9()[0])
                    + (self.group3()[1] * other.group9()[1])
                    + (self.group3()[2] * other.group9()[2])
                    + (self.group3()[3] * other[e45])
                    - (self.group0()[0] * other.group3()[0])
                    - (self.group0()[1] * other.group3()[1])
                    - (self.group0()[2] * other.group3()[2])
                    - (other.group4()[0] * self.group2()[0])
                    - (other.group4()[1] * self.group2()[1])
                    - (other.group4()[2] * self.group2()[2])
                    - (other.group5()[0] * self.group1()[0])
                    - (other.group5()[1] * self.group1()[1])
                    - (other.group5()[2] * self.group1()[2])
                    - (self.group1()[3] * other.group3()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
            // e5
            other.group0()[0] * self.group3()[3],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e1]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]))
                - (Simd32x4::from(self.group3()[3]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(self.group2()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]])),
            // e23, e31, e12
            Simd32x3::from([
                (self.group3()[1] * other.group1()[2]) - (self.group3()[2] * other.group1()[1]),
                (self.group3()[2] * other.group1()[0]) - (self.group3()[0] * other.group1()[2]),
                (self.group3()[0] * other.group1()[1]) - (self.group3()[1] * other.group1()[0]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group4()[0] * self.group3()[3]) + (self.group2()[3] * other.group3()[0]),
                (other.group4()[1] * self.group3()[3]) + (self.group2()[3] * other.group3()[1]),
                (other.group4()[2] * self.group3()[3]) + (self.group2()[3] * other.group3()[2]),
                -(other.group5()[1] * self.group3()[1]) - (other.group5()[2] * self.group3()[2]),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group1())
                - (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group5()[0]]) * crate::swizzle!(self.group3(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from(other.group0()[0]) * self.group0())
                + (Simd32x3::from(self.group2()[3]) * other.group5())
                + (Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]) * crate::swizzle!(other.group4(), 1, 2, 0))
                - (Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[0]]) * crate::swizzle!(other.group4(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (self.group3()[1] * other.group3()[2]) - (self.group3()[2] * other.group3()[1]),
                (self.group3()[2] * other.group3()[0]) - (self.group3()[0] * other.group3()[2]),
                (self.group3()[0] * other.group3()[1]) - (self.group3()[1] * other.group3()[0]),
            ]) + (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]))
                + (Simd32x3::from(self.group3()[3]) * other.group5()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group3()[2] * other.group6()[1]) - (self.group1()[1] * other.group1()[2]) - (self.group2()[0] * other.group1()[3]),
                (self.group3()[0] * other.group6()[2]) - (self.group1()[2] * other.group1()[0]) - (self.group2()[1] * other.group1()[3]),
                (self.group3()[1] * other.group6()[0]) - (self.group1()[0] * other.group1()[1]) - (self.group2()[2] * other.group1()[3]),
                (self.group2()[2] * other.group1()[2]) - (other.group8()[2] * self.group3()[2]) - (self.group3()[3] * other.group6()[3]),
            ]) + (Simd32x4::from(other[e1]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from([other.group8()[0], other.group8()[1], other.group8()[2], other.group1()[0]]) * crate::swizzle!(self.group2(), 3, 3, 3, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[1]]) * crate::swizzle!(other.group1(), 1, 2, 0, 1))
                - (Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], other.group8()[0]]) * crate::swizzle!(self.group3(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group6()[2], other.group6()[0], other.group6()[1], other.group8()[1]]) * crate::swizzle!(self.group3(), 1, 2, 0, 1)),
            // e1234
            (other.group7()[0] * self.group3()[0]) + (other.group7()[1] * self.group3()[1]) + (other.group7()[2] * self.group3()[2]) + (self.group2()[3] * other.group6()[3])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])
                - (self.group0()[2] * other.group1()[2])
                - (self.group1()[3] * other.group1()[3]),
        );
    }
}
impl Wedge<Plane> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (self.group2()[3] * other.group0()[3]) + (self.group3()[0] * other.group0()[0]) + (self.group3()[1] * other.group0()[1]) + (self.group3()[2] * other.group0()[2]),
        );
    }
}
impl Wedge<RoundPoint> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       18        0
    //    simd3        1        2        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       11       24        0
    //  no simd       25       40        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self.group2()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]])),
            // e23, e31, e12, e45
            Simd32x4::from([
                self.group3()[1] * other.group0()[2],
                self.group3()[2] * other.group0()[0],
                self.group3()[0] * other.group0()[1],
                self.group2()[3] * other[e2],
            ]) - (crate::swizzle!(self.group3(), 2, 0, 1, 3) * crate::swizzle!(other.group0(), 1, 2, 0, 3)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group3()[0] * other[e2],
                self.group3()[1] * other[e2],
                self.group3()[2] * other[e2],
                -(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group1()[3] * other.group0()[3]),
            ]) - (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group0()[0]]) * crate::swizzle!(other.group0(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self.group1()[1] * other.group0()[2]) - (self.group2()[0] * other.group0()[3]),
                -(self.group1()[2] * other.group0()[0]) - (self.group2()[1] * other.group0()[3]),
                -(self.group1()[0] * other.group0()[1]) - (self.group2()[2] * other.group0()[3]),
                (self.group2()[1] * other.group0()[1]) + (self.group2()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(other[e2]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<Scalar> for AntiDipoleInversion {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(other[scalar]) * self.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * self.group3(),
        );
    }
}
impl Wedge<Sphere> for AntiDipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self.group2()[3] * other.group0()[3])
                + (self.group3()[0] * other.group0()[0])
                + (self.group3()[1] * other.group0()[1])
                + (self.group3()[2] * other.group0()[2])
                + (self.group3()[3] * other[e4315]),
        );
    }
}
impl Wedge<VersorEven> for AntiDipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       22        0
    //    simd3        1        2        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       22       32        0
    //  no simd       48       60        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self.group2()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))
                - (Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]])),
            // e23, e31, e12, e45
            Simd32x4::from([
                self.group3()[1] * other.group3()[2],
                self.group3()[2] * other.group3()[0],
                self.group3()[0] * other.group3()[1],
                self.group2()[3] * other.group2()[3],
            ]) - (crate::swizzle!(self.group3(), 2, 0, 1, 3) * crate::swizzle!(other.group3(), 1, 2, 0, 3)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (self.group2()[3] * other.group1()[3]) + (self.group3()[1] * other.group0()[1]) + (self.group3()[2] * other.group0()[2])
                    - (self.group0()[1] * other.group3()[1])
                    - (self.group0()[2] * other.group3()[2])
                    - (self.group1()[3] * other.group3()[3]),
            ]) + (Simd32x4::from([other.group2()[3], other.group2()[3], other.group2()[3], other.group0()[0]]) * crate::swizzle!(self.group3(), 0, 1, 2, 0))
                - (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group0()[0]]) * crate::swizzle!(other.group3(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group3()[2] * other.group1()[1]) - (self.group1()[1] * other.group3()[2]) - (self.group2()[0] * other.group3()[3]),
                (self.group3()[0] * other.group1()[2]) - (self.group1()[2] * other.group3()[0]) - (self.group2()[1] * other.group3()[3]),
                (self.group3()[1] * other.group1()[0]) - (self.group1()[0] * other.group3()[1]) - (self.group2()[2] * other.group3()[3]),
                (self.group2()[2] * other.group3()[2]) - (self.group3()[2] * other.group2()[2]) - (self.group3()[3] * other.group1()[3]),
            ]) + (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0))
                + (Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group3()[1]]) * crate::swizzle!(self.group2(), 3, 3, 3, 1))
                - (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[1]]) * crate::swizzle!(self.group3(), 3, 3, 3, 1))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<VersorOdd> for AntiDipoleInversion {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       29       42        0
    //  no simd       44       60        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self.group0()[0] * other.group0()[3],
                self.group0()[1] * other.group0()[3],
                self.group0()[2] * other.group0()[3],
                (self.group3()[1] * other.group3()[1]) + (self.group3()[2] * other.group3()[2]) + (self.group3()[3] * other.group2()[3])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group3()[3]]))
                + (Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group3()[0]]) * crate::swizzle!(self.group3(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group3()[1], self.group3()[2], self.group3()[0], self.group2()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1, 0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group2()[3] * other.group2()[0]) + (self.group3()[3] * other.group0()[0]),
                (self.group2()[3] * other.group2()[1]) + (self.group3()[3] * other.group0()[1]),
                (self.group2()[3] * other.group2()[2]) + (self.group3()[3] * other.group0()[2]),
                -(self.group3()[1] * other.group1()[1]) - (self.group3()[2] * other.group1()[2]),
            ]) + (Simd32x4::from(other.group0()[3]) * self.group1())
                - (crate::swizzle!(self.group3(), 0, 1, 2, 0) * crate::swizzle!(other.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] * other.group0()[3]) + (self.group3()[1] * other.group2()[2]) + (self.group3()[3] * other.group1()[0]) - (self.group3()[2] * other.group2()[1]),
                (self.group2()[1] * other.group0()[3]) + (self.group3()[2] * other.group2()[0]) + (self.group3()[3] * other.group1()[1]) - (self.group3()[0] * other.group2()[2]),
                (self.group2()[2] * other.group0()[3]) + (self.group3()[0] * other.group2()[1]) + (self.group3()[3] * other.group1()[2]) - (self.group3()[1] * other.group2()[0]),
                self.group3()[3] * other.group0()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group2()[3]]),
        );
    }
}
impl std::ops::Div<wedge> for AntiDualNum {
    type Output = wedge_partial<AntiDualNum>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiDualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from(self.group0()[1]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group0()[0]]) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl Wedge<AntiDipoleInversion> for AntiDualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       16        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self.group0()[1] * other.group0()[0],
                self.group0()[1] * other.group0()[1],
                self.group0()[1] * other.group0()[2],
                self.group0()[0] * other.group3()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(self.group0()[1]) * other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group3()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
        );
    }
}
impl Wedge<AntiDualNum> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from([
            (other.group0()[0] * self.group0()[1]) + (other.group0()[1] * self.group0()[0]),
            other.group0()[1] * self.group0()[1],
        ]));
    }
}
impl Wedge<AntiFlatPoint> for AntiDualNum {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self.group0()[1]) * other.group0());
    }
}
impl Wedge<AntiFlector> for AntiDualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        9        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group1()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1] * other.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from(self.group0()[1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1] * other.group1()[0], self.group0()[1] * other.group1()[1], self.group0()[1] * other.group1()[2], 0.0]),
        );
    }
}
impl Wedge<AntiLine> for AntiDualNum {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self.group0()[1]) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(self.group0()[1]) * other.group1(),
        );
    }
}
impl Wedge<AntiMotor> for AntiDualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1] * other.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[1] * other.group0()[0], self.group0()[1] * other.group0()[1], self.group0()[1] * other.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group0()[1] * other.group1()[0],
                self.group0()[1] * other.group1()[1],
                self.group0()[1] * other.group1()[2],
                self.group0()[0] * other.group0()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1] * other.group1()[3]]),
        );
    }
}
impl Wedge<AntiPlane> for AntiDualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1] * other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[1] * other.group0()[0], self.group0()[1] * other.group0()[1], self.group0()[1] * other.group0()[2], 0.0]),
        );
    }
}
impl Wedge<AntiScalar> for AntiDualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self.group0()[1] * other[e12345]);
    }
}
impl Wedge<Circle> for AntiDualNum {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: Circle) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self.group0()[1]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self.group0()[1]) * other.group1(),
            // e235, e315, e125
            Simd32x3::from(self.group0()[1]) * other.group2(),
        );
    }
}
impl Wedge<CircleRotor> for AntiDualNum {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self.group0()[1]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self.group0()[1]) * other.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(self.group0()[1]) * other.group2(),
        );
    }
}
impl Wedge<Dipole> for AntiDualNum {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group0()[1]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self.group0()[1]) * other.group1(),
            // e15, e25, e35
            Simd32x3::from(self.group0()[1]) * other.group2(),
        );
    }
}
impl Wedge<DipoleInversion> for AntiDualNum {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group0()[1]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self.group0()[1]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self.group0()[1]) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self.group0()[1]) * other.group3(),
        );
    }
}
impl Wedge<DualNum> for AntiDualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(self.group0()[1]) * other.group0());
    }
}
impl Wedge<FlatPoint> for AntiDualNum {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self.group0()[1]) * other.group0());
    }
}
impl Wedge<Flector> for AntiDualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self.group0()[1]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self.group0()[1]) * other.group1(),
        );
    }
}
impl Wedge<Line> for AntiDualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: Line) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self.group0()[1]) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self.group0()[1]) * other.group1(),
        );
    }
}
impl Wedge<Motor> for AntiDualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self.group0()[1] * other.group0()[0],
                self.group0()[1] * other.group0()[1],
                self.group0()[1] * other.group0()[2],
                (self.group0()[0] * other.group1()[3]) + (self.group0()[1] * other.group0()[3]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from(self.group0()[1]) * other.group1(),
        );
    }
}
impl Wedge<MultiVector> for AntiDualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       14        0
    //  no simd        2       34        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([self.group0()[1] * other.group0()[0], (self.group0()[0] * other[e1]) + (self.group0()[1] * other.group0()[1])]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[1]) * other.group1(),
            // e5
            self.group0()[1] * other[e1],
            // e15, e25, e35, e45
            Simd32x4::from(self.group0()[1]) * other.group3(),
            // e41, e42, e43
            Simd32x3::from(self.group0()[1]) * other.group4(),
            // e23, e31, e12
            Simd32x3::from(self.group0()[1]) * other.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(self.group0()[1]) * other.group6(),
            // e423, e431, e412
            Simd32x3::from(self.group0()[1]) * other.group7(),
            // e235, e315, e125
            Simd32x3::from(self.group0()[1]) * other.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self.group0()[1]) * other.group9(),
            // e1234
            (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other[e45]),
        );
    }
}
impl Wedge<Plane> for AntiDualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self.group0()[1]) * other.group0());
    }
}
impl Wedge<RoundPoint> for AntiDualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other[e2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1] * other[e2]]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[1]) * other.group0(),
        );
    }
}
impl Wedge<Scalar> for AntiDualNum {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(other[scalar]) * self.group0());
    }
}
impl Wedge<Sphere> for AntiDualNum {
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
            Simd32x4::from(self.group0()[1]) * other.group0(),
            // e1234
            self.group0()[1] * other[e4315],
        );
    }
}
impl Wedge<VersorEven> for AntiDualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self.group0()[1] * other.group0()[0],
                self.group0()[1] * other.group0()[1],
                self.group0()[1] * other.group0()[2],
                (self.group0()[0] * other.group2()[3]) + (self.group0()[1] * other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(self.group0()[1]) * other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(self.group0()[1]) * other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[1]) * other.group3(),
        );
    }
}
impl Wedge<VersorOdd> for AntiDualNum {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self.group0()[1]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self.group0()[1]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group0()[1] * other.group2()[0],
                self.group0()[1] * other.group2()[1],
                self.group0()[1] * other.group2()[2],
                (self.group0()[0] * other.group0()[3]) + (self.group0()[1] * other.group2()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self.group0()[1]) * other.group3(),
        );
    }
}
impl std::ops::Div<wedge> for AntiFlatPoint {
    type Output = wedge_partial<AntiFlatPoint>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiFlatPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group2()[3] * self.group0()[3]]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                other.group2()[3] * self.group0()[0],
                other.group2()[3] * self.group0()[1],
                other.group2()[3] * self.group0()[2],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group1()[3] * self.group0()[3]),
            ]),
        );
    }
}
impl Wedge<AntiDipoleInversion> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3       12        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group2()[3] * self.group0()[0] * -1.0,
                other.group2()[3] * self.group0()[1] * -1.0,
                other.group2()[3] * self.group0()[2] * -1.0,
                (other.group3()[0] * self.group0()[0]) + (other.group3()[1] * self.group0()[1]) + (other.group3()[2] * self.group0()[2]) + (other.group3()[3] * self.group0()[3]),
            ]),
            // e1234
            other.group2()[3] * self.group0()[3] * -1.0,
        );
    }
}
impl Wedge<AntiDualNum> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other.group0()[1]) * self.group0());
    }
}
impl Wedge<AntiFlector> for AntiFlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            0.0,
            0.0,
            0.0,
            (self.group0()[0] * other.group1()[0]) + (self.group0()[1] * other.group1()[1]) + (self.group0()[2] * other.group1()[2]) + (self.group0()[3] * other.group1()[3]),
        ]));
    }
}
impl Wedge<AntiMotor> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other.group0()[3]) * self.group0());
    }
}
impl Wedge<AntiPlane> for AntiFlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            0.0,
            0.0,
            0.0,
            (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[3] * other.group0()[3]),
        ]));
    }
}
impl Wedge<Dipole> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (self.group0()[3] * other.group1()[3]),
        );
    }
}
impl Wedge<DipoleInversion> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (self.group0()[3] * other.group1()[3]),
        );
    }
}
impl Wedge<DualNum> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[0] * self.group0()[0] * -1.0,
                other.group0()[0] * self.group0()[1] * -1.0,
                other.group0()[0] * self.group0()[2] * -1.0,
                0.0,
            ]),
            // e1234
            other.group0()[0] * self.group0()[3] * -1.0,
        );
    }
}
impl Wedge<FlatPoint> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return AntiScalar::from_groups(/* e12345 */ self.group0()[3] * other.group0()[3] * -1.0);
    }
}
impl Wedge<Flector> for AntiFlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn wedge(self, other: Flector) -> Self::Output {
        return AntiScalar::from_groups(/* e12345 */ self.group0()[3] * other.group0()[3] * -1.0);
    }
}
impl Wedge<Motor> for AntiFlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3] * other.group1()[3]]));
    }
}
impl Wedge<MultiVector> for AntiFlatPoint {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       18        0
    //  no simd        6       20        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(other.group4()[0] * self.group0()[0]) - (other.group4()[1] * self.group0()[1]) - (other.group4()[2] * self.group0()[2]) - (self.group0()[3] * other.group3()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0()[0] * other.group1()[3] * -1.0,
                self.group0()[1] * other.group1()[3] * -1.0,
                self.group0()[2] * other.group1()[3] * -1.0,
                (self.group0()[0] * other.group1()[0]) + (self.group0()[1] * other.group1()[1]) + (self.group0()[2] * other.group1()[2]) + (self.group0()[3] * other[e1]),
            ]),
            // e1234
            self.group0()[3] * other.group1()[3] * -1.0,
        );
    }
}
impl Wedge<RoundPoint> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3       12        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0()[0] * other.group0()[3] * -1.0,
                self.group0()[1] * other.group0()[3] * -1.0,
                self.group0()[2] * other.group0()[3] * -1.0,
                (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[3] * other[e2]),
            ]),
            // e1234
            self.group0()[3] * other.group0()[3] * -1.0,
        );
    }
}
impl Wedge<Scalar> for AntiFlatPoint {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(other[scalar]) * self.group0());
    }
}
impl Wedge<VersorEven> for AntiFlatPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3       12        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0()[0] * other.group3()[3] * -1.0,
                self.group0()[1] * other.group3()[3] * -1.0,
                self.group0()[2] * other.group3()[3] * -1.0,
                (self.group0()[0] * other.group3()[0]) + (self.group0()[1] * other.group3()[1]) + (self.group0()[2] * other.group3()[2]) + (self.group0()[3] * other.group2()[3]),
            ]),
            // e1234
            self.group0()[3] * other.group3()[3] * -1.0,
        );
    }
}
impl Wedge<VersorOdd> for AntiFlatPoint {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3] * other.group0()[3]]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                self.group0()[0] * other.group0()[3],
                self.group0()[1] * other.group0()[3],
                self.group0()[2] * other.group0()[3],
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[3] * other.group1()[3]),
            ]),
        );
    }
}
impl std::ops::Div<wedge> for AntiFlector {
    type Output = wedge_partial<AntiFlector>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       32        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       18       33        0
    //  no simd       21       36        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[1] * self.group1()[2]) - (other.group0()[2] * self.group1()[1]),
                (other.group0()[2] * self.group1()[0]) - (other.group0()[0] * self.group1()[2]),
                (other.group0()[0] * self.group1()[1]) - (other.group0()[1] * self.group1()[0]),
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group1()[3] * self.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group0()[0] * self.group1()[3],
                other.group0()[1] * self.group1()[3],
                other.group0()[2] * self.group1()[3],
                (other.group2()[3] * self.group0()[3]) - (other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2]),
            ]) - (crate::swizzle!(other.group1(), 3, 3, 3, 0) * crate::swizzle!(self.group1(), 0, 1, 2, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] * self.group1()[3]) + (other.group2()[2] * self.group1()[1]) + (other.group2()[3] * self.group0()[0]) - (other.group2()[1] * self.group1()[2]),
                (other.group1()[1] * self.group1()[3]) + (other.group2()[0] * self.group1()[2]) + (other.group2()[3] * self.group0()[1]) - (other.group2()[2] * self.group1()[0]),
                (other.group1()[2] * self.group1()[3]) + (other.group2()[1] * self.group1()[0]) + (other.group2()[3] * self.group0()[2]) - (other.group2()[0] * self.group1()[1]),
                other.group2()[3] * self.group1()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group2()[3] * self.group1()[0], other.group2()[3] * self.group1()[1], other.group2()[3] * self.group1()[2], 0.0]),
        );
    }
}
impl Wedge<AntiDipoleInversion> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       25        0
    //    simd3        0        2        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       13       31        0
    //  no simd       25       47        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group2()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group3()[2] * self.group1()[1]) - (other.group3()[1] * self.group1()[2]),
                (other.group3()[0] * self.group1()[2]) - (other.group3()[2] * self.group1()[0]),
                (other.group3()[1] * self.group1()[0]) - (other.group3()[0] * self.group1()[1]),
                other.group2()[3] * self.group1()[3] * -1.0,
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group3()[0] * self.group1()[3]) * -1.0,
                (other.group3()[1] * self.group1()[3]) * -1.0,
                (other.group3()[2] * self.group1()[3]) * -1.0,
                (other.group0()[1] * self.group1()[1]) + (other.group0()[2] * self.group1()[2]) - (other.group2()[3] * self.group0()[3]),
            ]) + (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group0()[0]]) * crate::swizzle!(self.group1(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group1()[1] * self.group1()[2],
                other.group1()[2] * self.group1()[0],
                other.group1()[0] * self.group1()[1],
                (other.group3()[0] * self.group0()[0]) + (other.group3()[1] * self.group0()[1]) + (other.group3()[2] * self.group0()[2]) + (other.group3()[3] * self.group0()[3])
                    - (other.group2()[2] * self.group1()[2]),
            ]) - (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]) * crate::swizzle!(other.group2(), 3, 3, 3, 1)),
        );
    }
}
impl Wedge<AntiDualNum> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        9        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group1()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1] * self.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from(other.group0()[1]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1] * self.group1()[0], other.group0()[1] * self.group1()[1], other.group0()[1] * self.group1()[2], 0.0]),
        );
    }
}
impl Wedge<AntiFlatPoint> for AntiFlector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            0.0,
            0.0,
            0.0,
            -(other.group0()[0] * self.group1()[0]) - (other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]) - (other.group0()[3] * self.group1()[3]),
        ]));
    }
}
impl Wedge<AntiFlector> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       16       20        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group1()[2] * self.group1()[1]) - (other.group1()[1] * self.group1()[2]),
                (other.group1()[0] * self.group1()[2]) - (other.group1()[2] * self.group1()[0]),
                (other.group1()[1] * self.group1()[0]) - (other.group1()[0] * self.group1()[1]),
                0.0,
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (other.group1()[1] * self.group0()[1]) + (other.group1()[2] * self.group0()[2]) + (other.group1()[3] * self.group0()[3])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group0()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[0]]) * crate::swizzle!(other.group1(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[0]]) * crate::swizzle!(self.group1(), 3, 3, 3, 0)),
        );
    }
}
impl Wedge<AntiLine> for AntiFlector {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (other.group0()[0] * self.group1()[3]) + (other.group1()[2] * self.group1()[1]),
                (other.group0()[1] * self.group1()[3]) + (other.group1()[0] * self.group1()[2]),
                (other.group0()[2] * self.group1()[3]) + (other.group1()[1] * self.group1()[0]),
                -(other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]),
            ]) - (Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[0], other.group0()[0]]) * crate::swizzle!(self.group1(), 2, 0, 1, 0)),
        );
    }
}
impl Wedge<AntiMotor> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (self.group1()[1] * other.group1()[2]) + (self.group1()[3] * other.group0()[0]),
                (self.group1()[2] * other.group1()[0]) + (self.group1()[3] * other.group0()[1]),
                (self.group1()[0] * other.group1()[1]) + (self.group1()[3] * other.group0()[2]),
                -(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(other.group0()[3]) * self.group0())
                - (Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[0], other.group0()[0]]) * crate::swizzle!(self.group1(), 2, 0, 1, 0)),
            // e1, e2, e3, e5
            Simd32x4::from(other.group0()[3]) * self.group1(),
        );
    }
}
impl Wedge<AntiPlane> for AntiFlector {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       16        0
    //  no simd        9       19        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1]),
                (self.group1()[2] * other.group0()[0]) - (self.group1()[0] * other.group0()[2]),
                (self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0]),
                0.0,
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                (self.group1()[3] * other.group0()[0]) * -1.0,
                (self.group1()[3] * other.group0()[1]) * -1.0,
                (self.group1()[3] * other.group0()[2]) * -1.0,
                (self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[3] * other.group0()[3]),
            ]) + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[0]]) * crate::swizzle!(other.group0(), 3, 3, 3, 0)),
        );
    }
}
impl Wedge<Circle> for AntiFlector {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd       11       16        0
    fn wedge(self, other: Circle) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group1()[2] * other.group1()[1],
                self.group1()[0] * other.group1()[2],
                self.group1()[1] * other.group1()[0],
                -(other.group2()[2] * self.group1()[2]) - (self.group1()[3] * other.group1()[3]),
            ]) - (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[0]]) * crate::swizzle!(self.group1(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[1]]) * crate::swizzle!(self.group1(), 1, 2, 0, 1)),
            // e1234
            (other.group0()[0] * self.group1()[0]) + (other.group0()[1] * self.group1()[1]) + (other.group0()[2] * self.group1()[2]),
        );
    }
}
impl Wedge<CircleRotor> for AntiFlector {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd       11       16        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group1()[2] * other.group1()[1],
                self.group1()[0] * other.group1()[2],
                self.group1()[1] * other.group1()[0],
                -(self.group1()[2] * other.group2()[2]) - (self.group1()[3] * other.group1()[3]),
            ]) - (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[0]]) * crate::swizzle!(self.group1(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[1]]) * crate::swizzle!(self.group1(), 1, 2, 0, 1)),
            // e1234
            (other.group0()[0] * self.group1()[0]) + (other.group0()[1] * self.group1()[1]) + (other.group0()[2] * self.group1()[2]),
        );
    }
}
impl Wedge<Dipole> for AntiFlector {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       21        0
    //  no simd       17       28        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group0()[0] * self.group1()[3],
                other.group0()[1] * self.group1()[3],
                other.group0()[2] * self.group1()[3],
                -(self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2]),
            ]) - (crate::swizzle!(self.group1(), 0, 1, 2, 0) * crate::swizzle!(other.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group2()[2] * self.group1()[1]) + (self.group1()[3] * other.group1()[0]) - (other.group2()[1] * self.group1()[2]),
                (other.group2()[0] * self.group1()[2]) + (self.group1()[3] * other.group1()[1]) - (other.group2()[2] * self.group1()[0]),
                (other.group2()[1] * self.group1()[0]) + (self.group1()[3] * other.group1()[2]) - (other.group2()[0] * self.group1()[1]),
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (self.group0()[3] * other.group1()[3]),
            ]),
        );
    }
}
impl Wedge<DipoleInversion> for AntiFlector {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       21       35        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group0()[0] * self.group1()[3],
                other.group0()[1] * self.group1()[3],
                other.group0()[2] * self.group1()[3],
                -(self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2]),
            ]) - (crate::swizzle!(self.group1(), 0, 1, 2, 0) * crate::swizzle!(other.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self.group1()[2] * other.group2()[1]) * -1.0,
                (self.group1()[0] * other.group2()[2]) * -1.0,
                (self.group1()[1] * other.group2()[0]) * -1.0,
                (self.group1()[2] * other.group3()[2]) + (self.group1()[3] * other.group2()[3])
                    - (other.group0()[0] * self.group0()[0])
                    - (other.group0()[1] * self.group0()[1])
                    - (other.group0()[2] * self.group0()[2])
                    - (self.group0()[3] * other.group1()[3]),
            ]) + (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group3()[1]]) * crate::swizzle!(self.group1(), 3, 3, 3, 1))
                + (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<DualNum> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       16        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group1()[3] * -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group0()[3] * -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[0] * self.group0()[0] * -1.0,
                other.group0()[0] * self.group0()[1] * -1.0,
                other.group0()[0] * self.group0()[2] * -1.0,
                0.0,
            ]),
        );
    }
}
impl Wedge<FlatPoint> for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       14        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1]),
                (self.group1()[2] * other.group0()[0]) - (self.group1()[0] * other.group0()[2]),
                (self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0]),
                0.0,
            ]),
        );
    }
}
impl Wedge<Flector> for AntiFlector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       16        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self.group1()[0] * other.group0()[3] * -1.0,
                self.group1()[1] * other.group0()[3] * -1.0,
                self.group1()[2] * other.group0()[3] * -1.0,
                (self.group1()[0] * other.group1()[0]) + (self.group1()[1] * other.group1()[1]) + (self.group1()[2] * other.group1()[2]) - (self.group0()[3] * other.group0()[3]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1]),
                (self.group1()[2] * other.group0()[0]) - (self.group1()[0] * other.group0()[2]),
                (self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0]),
                0.0,
            ]),
        );
    }
}
impl Wedge<Line> for AntiFlector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn wedge(self, other: Line) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[1] * self.group1()[2],
                other.group0()[2] * self.group1()[0],
                other.group0()[0] * self.group1()[1],
                -(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<Motor> for AntiFlector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        6       13        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0] * other.group1()[3], self.group1()[1] * other.group1()[3], self.group1()[2] * other.group1()[3], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group1()[2] * other.group0()[1],
                self.group1()[0] * other.group0()[2],
                self.group1()[1] * other.group0()[0],
                (self.group0()[3] * other.group1()[3]) - (self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<MultiVector> for AntiFlector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       54        0
    //    simd3        3        6        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       35       63        0
    //  no simd       50       84        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self.group1()[0] * other.group9()[0]) + (self.group1()[1] * other.group9()[1]) + (self.group1()[2] * other.group9()[2]) + (self.group1()[3] * other[e45])
                    - (other.group4()[0] * self.group0()[0])
                    - (other.group4()[1] * self.group0()[1])
                    - (other.group4()[2] * self.group0()[2])
                    - (self.group0()[3] * other.group3()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0] * self.group1()[0], other.group0()[0] * self.group1()[1], other.group0()[0] * self.group1()[2], 0.0]),
            // e5
            other.group0()[0] * self.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from([
                (self.group1()[0] * other[e1]) - (self.group1()[3] * other.group1()[0]),
                (self.group1()[1] * other[e1]) - (self.group1()[3] * other.group1()[1]),
                (self.group1()[2] * other[e1]) - (self.group1()[3] * other.group1()[2]),
                self.group1()[3] * other.group1()[3] * -1.0,
            ]),
            // e41, e42, e43
            Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([
                (self.group1()[1] * other.group1()[2]) - (self.group1()[2] * other.group1()[1]),
                (self.group1()[2] * other.group1()[0]) - (self.group1()[0] * other.group1()[2]),
                (self.group1()[0] * other.group1()[1]) - (self.group1()[1] * other.group1()[0]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group4()[0] * self.group1()[3],
                other.group4()[1] * self.group1()[3],
                other.group4()[2] * self.group1()[3],
                (other.group0()[0] * self.group0()[3]) - (other.group5()[1] * self.group1()[1]) - (other.group5()[2] * self.group1()[2]),
            ]) - (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group5()[0]]) * crate::swizzle!(self.group1(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]) * crate::swizzle!(other.group4(), 1, 2, 0))
                - (Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]) * crate::swizzle!(other.group4(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (self.group1()[1] * other.group3()[2]) - (self.group1()[2] * other.group3()[1]),
                (self.group1()[2] * other.group3()[0]) - (self.group1()[0] * other.group3()[2]),
                (self.group1()[0] * other.group3()[1]) - (self.group1()[1] * other.group3()[0]),
            ]) + (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self.group1()[3]) * other.group5()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group1()[2] * other.group6()[1]) - (self.group0()[0] * other.group1()[3]),
                (self.group1()[0] * other.group6()[2]) - (self.group0()[1] * other.group1()[3]),
                (self.group1()[1] * other.group6()[0]) - (self.group0()[2] * other.group1()[3]),
                (self.group0()[0] * other.group1()[0]) + (self.group0()[1] * other.group1()[1]) + (self.group0()[2] * other.group1()[2]) + (self.group0()[3] * other[e1])
                    - (other.group8()[2] * self.group1()[2])
                    - (self.group1()[3] * other.group6()[3]),
            ]) - (Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], other.group8()[0]]) * crate::swizzle!(self.group1(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group6()[2], other.group6()[0], other.group6()[1], other.group8()[1]]) * crate::swizzle!(self.group1(), 1, 2, 0, 1)),
            // e1234
            (other.group7()[0] * self.group1()[0]) + (other.group7()[1] * self.group1()[1]) + (other.group7()[2] * self.group1()[2]) - (self.group0()[3] * other.group1()[3]),
        );
    }
}
impl Wedge<Plane> for AntiFlector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn wedge(self, other: Plane) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2]),
        );
    }
}
impl Wedge<RoundPoint> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       26        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        9       28        0
    //  no simd        9       32        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1]),
                (self.group1()[2] * other.group0()[0]) - (self.group1()[0] * other.group0()[2]),
                (self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0]),
                self.group1()[3] * other.group0()[3] * -1.0,
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group1()[0] * other[e2]) - (self.group1()[3] * other.group0()[0]),
                (self.group1()[1] * other[e2]) - (self.group1()[3] * other.group0()[1]),
                (self.group1()[2] * other[e2]) - (self.group1()[3] * other.group0()[2]),
                self.group0()[3] * other.group0()[3] * -1.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0()[0] * other.group0()[3] * -1.0,
                self.group0()[1] * other.group0()[3] * -1.0,
                self.group0()[2] * other.group0()[3] * -1.0,
                (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[3] * other[e2]),
            ]),
        );
    }
}
impl Wedge<Scalar> for AntiFlector {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(other[scalar]) * self.group1(),
        );
    }
}
impl Wedge<Sphere> for AntiFlector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2]) + (self.group1()[3] * other[e4315]),
        );
    }
}
impl Wedge<VersorEven> for AntiFlector {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       22        0
    //    simd3        0        2        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       28        0
    //  no simd       28       44        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group1()[1] * other.group3()[2]) - (self.group1()[2] * other.group3()[1]),
                (self.group1()[2] * other.group3()[0]) - (self.group1()[0] * other.group3()[2]),
                (self.group1()[0] * other.group3()[1]) - (self.group1()[1] * other.group3()[0]),
                self.group1()[3] * other.group3()[3] * -1.0,
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2])])
                + (Simd32x4::from([other.group2()[3], other.group2()[3], other.group2()[3], other.group0()[0]]) * crate::swizzle!(self.group1(), 0, 1, 2, 0))
                - (Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group0()[3]]) * other.group3()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group1()[2] * other.group1()[1]) - (self.group0()[0] * other.group3()[3]),
                (self.group1()[0] * other.group1()[2]) - (self.group0()[1] * other.group3()[3]),
                (self.group1()[1] * other.group1()[0]) - (self.group0()[2] * other.group3()[3]),
                (self.group0()[0] * other.group3()[0]) + (self.group0()[1] * other.group3()[1]) + (self.group0()[2] * other.group3()[2]) + (self.group0()[3] * other.group2()[3])
                    - (self.group1()[2] * other.group2()[2])
                    - (self.group1()[3] * other.group1()[3]),
            ]) - (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[1]]) * crate::swizzle!(self.group1(), 3, 3, 3, 1))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<VersorOdd> for AntiFlector {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       31       40        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (self.group1()[1] * other.group3()[1]) + (self.group1()[2] * other.group3()[2]) + (self.group1()[3] * other.group2()[3])
                    - (self.group0()[1] * other.group0()[1])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group0()[3] * other.group1()[3]),
            ]) + (Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group3()[0]]) * crate::swizzle!(self.group1(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group0()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1, 0)),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, -(self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2])])
                + (Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group0()[3]]) * other.group0())
                - (crate::swizzle!(self.group1(), 0, 1, 2, 0) * crate::swizzle!(other.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]) + (self.group1()[1] * other.group2()[2]) + (self.group1()[3] * other.group1()[0]) - (self.group1()[2] * other.group2()[1]),
                (self.group0()[1] * other.group0()[3]) + (self.group1()[2] * other.group2()[0]) + (self.group1()[3] * other.group1()[1]) - (self.group1()[0] * other.group2()[2]),
                (self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group2()[1]) + (self.group1()[3] * other.group1()[2]) - (self.group1()[1] * other.group2()[0]),
                self.group1()[3] * other.group0()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0] * other.group0()[3], self.group1()[1] * other.group0()[3], self.group1()[2] * other.group0()[3], 0.0]),
        );
    }
}
impl std::ops::Div<wedge> for AntiLine {
    type Output = wedge_partial<AntiLine>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       24        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0] * other.group2()[3], self.group0()[1] * other.group2()[3], self.group0()[2] * other.group2()[3], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group1()[0] * other.group2()[3],
                self.group1()[1] * other.group2()[3],
                self.group1()[2] * other.group2()[3],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group1()[2]) + (self.group0()[0] * other.group1()[3]) - (other.group0()[2] * self.group1()[1]),
                (other.group0()[2] * self.group1()[0]) + (self.group0()[1] * other.group1()[3]) - (other.group0()[0] * self.group1()[2]),
                (other.group0()[0] * self.group1()[1]) + (self.group0()[2] * other.group1()[3]) - (other.group0()[1] * self.group1()[0]),
                -(self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2]),
            ]),
        );
    }
}
impl Wedge<AntiDipoleInversion> for AntiLine {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       13       24        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other.group2()[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group1()[0] * other.group2()[3],
                self.group1()[1] * other.group2()[3],
                self.group1()[2] * other.group2()[3],
                -(self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1]) - (self.group0()[2] * other.group3()[2]),
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self.group0()[0] * other.group3()[3]) + (self.group1()[2] * other.group3()[1]) - (self.group1()[1] * other.group3()[2]),
                (self.group0()[1] * other.group3()[3]) + (self.group1()[0] * other.group3()[2]) - (self.group1()[2] * other.group3()[0]),
                (self.group0()[2] * other.group3()[3]) + (self.group1()[1] * other.group3()[0]) - (self.group1()[0] * other.group3()[1]),
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2]),
            ]),
        );
    }
}
impl Wedge<AntiDualNum> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other.group0()[1]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other.group0()[1]) * self.group1(),
        );
    }
}
impl Wedge<AntiFlector> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (self.group0()[0] * other.group1()[3]) + (self.group1()[2] * other.group1()[1]),
                (self.group0()[1] * other.group1()[3]) + (self.group1()[0] * other.group1()[2]),
                (self.group0()[2] * other.group1()[3]) + (self.group1()[1] * other.group1()[0]),
                -(self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]),
            ]) - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group0()[0]]) * crate::swizzle!(other.group1(), 2, 0, 1, 0)),
        );
    }
}
impl Wedge<AntiLine> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            0.0,
            0.0,
            0.0,
            -(other.group0()[0] * self.group1()[0])
                - (other.group0()[1] * self.group1()[1])
                - (other.group0()[2] * self.group1()[2])
                - (other.group1()[0] * self.group0()[0])
                - (other.group1()[1] * self.group0()[1])
                - (other.group1()[2] * self.group0()[2]),
        ]));
    }
}
impl Wedge<AntiMotor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0] * other.group0()[3], self.group0()[1] * other.group0()[3], self.group0()[2] * other.group0()[3], 0.0]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                self.group1()[0] * other.group0()[3],
                self.group1()[1] * other.group0()[3],
                self.group1()[2] * other.group0()[3],
                -(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2]),
            ]),
        );
    }
}
impl Wedge<AntiPlane> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]) + (self.group1()[2] * other.group0()[1]),
                (self.group0()[1] * other.group0()[3]) + (self.group1()[0] * other.group0()[2]),
                (self.group0()[2] * other.group0()[3]) + (self.group1()[1] * other.group0()[0]),
                -(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]) - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group0()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1, 0)),
        );
    }
}
impl Wedge<Circle> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: Circle) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])
                - (self.group0()[2] * other.group1()[2])
                - (self.group1()[0] * other.group0()[0])
                - (self.group1()[1] * other.group0()[1])
                - (self.group1()[2] * other.group0()[2]),
        );
    }
}
impl Wedge<CircleRotor> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])
                - (self.group0()[2] * other.group1()[2])
                - (self.group1()[0] * other.group0()[0])
                - (self.group1()[1] * other.group0()[1])
                - (self.group1()[2] * other.group0()[2]),
        );
    }
}
impl Wedge<Dipole> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       18        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[0] * other.group1()[3]) + (self.group1()[2] * other.group0()[1]) - (self.group1()[1] * other.group0()[2]),
                (self.group0()[1] * other.group1()[3]) + (self.group1()[0] * other.group0()[2]) - (self.group1()[2] * other.group0()[0]),
                (self.group0()[2] * other.group1()[3]) + (self.group1()[1] * other.group0()[0]) - (self.group1()[0] * other.group0()[1]),
                -(self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2]),
            ]),
            // e1234
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
        );
    }
}
impl Wedge<DipoleInversion> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       18        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[0] * other.group1()[3]) + (self.group1()[2] * other.group0()[1]) - (self.group1()[1] * other.group0()[2]),
                (self.group0()[1] * other.group1()[3]) + (self.group1()[0] * other.group0()[2]) - (self.group1()[2] * other.group0()[0]),
                (self.group0()[2] * other.group1()[3]) + (self.group1()[1] * other.group0()[0]) - (self.group1()[0] * other.group0()[1]),
                -(self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2]),
            ]),
            // e1234
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
        );
    }
}
impl Wedge<DualNum> for AntiLine {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other.group0()[0]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0] * self.group1()[0], other.group0()[0] * self.group1()[1], other.group0()[0] * self.group1()[2], 0.0]),
            // e235, e315, e125
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<FlatPoint> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            self.group0()[0] * other.group0()[3],
            self.group0()[1] * other.group0()[3],
            self.group0()[2] * other.group0()[3],
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
        ]));
    }
}
impl Wedge<Flector> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            self.group0()[0] * other.group0()[3],
            self.group0()[1] * other.group0()[3],
            self.group0()[2] * other.group0()[3],
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
        ]));
    }
}
impl Wedge<Line> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn wedge(self, other: Line) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
        );
    }
}
impl Wedge<Motor> for AntiLine {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group0()[0] * other.group1()[3], self.group0()[1] * other.group1()[3], self.group0()[2] * other.group1()[3], 0.0]),
        );
    }
}
impl Wedge<MultiVector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       33        0
    //    simd3        2        5        0
    // Totals...
    // yes simd       22       38        0
    //  no simd       26       48        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(self.group0()[0] * other.group6()[0])
                    - (self.group0()[1] * other.group6()[1])
                    - (self.group0()[2] * other.group6()[2])
                    - (self.group1()[0] * other.group7()[0])
                    - (self.group1()[1] * other.group7()[1])
                    - (self.group1()[2] * other.group7()[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([other.group0()[0] * self.group1()[0], other.group0()[0] * self.group1()[1], other.group0()[0] * self.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other.group0()[0]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group1()[0] * other.group1()[3],
                self.group1()[1] * other.group1()[3],
                self.group1()[2] * other.group1()[3],
                -(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]),
            ]),
            // e423, e431, e412
            Simd32x3::from(other.group1()[3]) * self.group0(),
            // e235, e315, e125
            (Simd32x3::from(other[e1]) * self.group0()) + (Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]) * crate::swizzle!(self.group1(), 2, 0, 1))
                - (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]) * crate::swizzle!(self.group1(), 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[0] * other.group3()[3]) + (self.group1()[2] * other.group4()[1]) - (self.group1()[1] * other.group4()[2]),
                (self.group0()[1] * other.group3()[3]) + (self.group1()[0] * other.group4()[2]) - (self.group1()[2] * other.group4()[0]),
                (self.group0()[2] * other.group3()[3]) + (self.group1()[1] * other.group4()[0]) - (self.group1()[0] * other.group4()[1]),
                -(self.group0()[0] * other.group3()[0])
                    - (self.group0()[1] * other.group3()[1])
                    - (self.group0()[2] * other.group3()[2])
                    - (self.group1()[0] * other.group5()[0])
                    - (self.group1()[1] * other.group5()[1])
                    - (self.group1()[2] * other.group5()[2]),
            ]),
            // e1234
            -(self.group0()[0] * other.group4()[0]) - (self.group0()[1] * other.group4()[1]) - (self.group0()[2] * other.group4()[2]),
        );
    }
}
impl Wedge<RoundPoint> for AntiLine {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        8       18        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other.group0()[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group1()[0] * other.group0()[3],
                self.group1()[1] * other.group0()[3],
                self.group1()[2] * other.group0()[3],
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]),
            // e235, e315, e125
            (Simd32x3::from(other[e2]) * self.group0()) + (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * crate::swizzle!(self.group1(), 2, 0, 1))
                - (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]) * crate::swizzle!(self.group1(), 1, 2, 0)),
        );
    }
}
impl Wedge<Scalar> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group0(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * self.group1(),
        );
    }
}
impl Wedge<VersorEven> for AntiLine {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       13       24        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other.group3()[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group1()[0] * other.group3()[3],
                self.group1()[1] * other.group3()[3],
                self.group1()[2] * other.group3()[3],
                -(self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1]) - (self.group0()[2] * other.group3()[2]),
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self.group0()[0] * other.group2()[3]) + (self.group1()[2] * other.group3()[1]) - (self.group1()[1] * other.group3()[2]),
                (self.group0()[1] * other.group2()[3]) + (self.group1()[0] * other.group3()[2]) - (self.group1()[2] * other.group3()[0]),
                (self.group0()[2] * other.group2()[3]) + (self.group1()[1] * other.group3()[0]) - (self.group1()[0] * other.group3()[1]),
                -(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2]),
            ]),
        );
    }
}
impl Wedge<VersorOdd> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       24        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0] * other.group0()[3], self.group0()[1] * other.group0()[3], self.group0()[2] * other.group0()[3], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group1()[0] * other.group0()[3],
                self.group1()[1] * other.group0()[3],
                self.group1()[2] * other.group0()[3],
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[0] * other.group1()[3]) + (self.group1()[2] * other.group0()[1]) - (self.group1()[1] * other.group0()[2]),
                (self.group0()[1] * other.group1()[3]) + (self.group1()[0] * other.group0()[2]) - (self.group1()[2] * other.group0()[0]),
                (self.group0()[2] * other.group1()[3]) + (self.group1()[1] * other.group0()[0]) - (self.group1()[0] * other.group0()[1]),
                -(self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2]),
            ]),
        );
    }
}
impl std::ops::Div<wedge> for AntiMotor {
    type Output = wedge_partial<AntiMotor>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       14       27        0
    //  no simd       20       36        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[0] * self.group0()[3]) + (other.group2()[3] * self.group0()[0]),
                (other.group1()[1] * self.group0()[3]) + (other.group2()[3] * self.group0()[1]),
                (other.group1()[2] * self.group0()[3]) + (other.group2()[3] * self.group0()[2]),
                other.group1()[3] * self.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group2()[0] * self.group0()[3]) + (other.group2()[3] * self.group1()[0]),
                (other.group2()[1] * self.group0()[3]) + (other.group2()[3] * self.group1()[1]),
                (other.group2()[2] * self.group0()[3]) + (other.group2()[3] * self.group1()[2]),
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group1()[3] * self.group0()[0],
                other.group1()[3] * self.group0()[1],
                other.group1()[3] * self.group0()[2],
                -(other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])
                    - (other.group2()[0] * self.group0()[0])
                    - (other.group2()[1] * self.group0()[1])
                    - (other.group2()[2] * self.group0()[2]),
            ]) + (Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group2()[3]]) * crate::swizzle!(self.group1(), 2, 0, 1, 3))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiDipoleInversion> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       18       31        0
    //  no simd       24       40        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                other.group0()[0] * self.group0()[3],
                other.group0()[1] * self.group0()[3],
                other.group0()[2] * self.group0()[3],
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]])),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group2()[3] * self.group1()[0],
                other.group2()[3] * self.group1()[1],
                other.group2()[3] * self.group1()[2],
                -(other.group3()[0] * self.group0()[0]) - (other.group3()[1] * self.group0()[1]) - (other.group3()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] * self.group0()[3]) + (other.group3()[1] * self.group1()[2]) + (other.group3()[3] * self.group0()[0]) - (other.group3()[2] * self.group1()[1]),
                (other.group2()[1] * self.group0()[3]) + (other.group3()[2] * self.group1()[0]) + (other.group3()[3] * self.group0()[1]) - (other.group3()[0] * self.group1()[2]),
                (other.group2()[2] * self.group0()[3]) + (other.group3()[0] * self.group1()[1]) + (other.group3()[3] * self.group0()[2]) - (other.group3()[1] * self.group1()[0]),
                other.group3()[3] * self.group0()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
        );
    }
}
impl Wedge<AntiDualNum> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1] * self.group0()[3]]),
            // e23, e31, e12, e45
            Simd32x4::from([other.group0()[1] * self.group0()[0], other.group0()[1] * self.group0()[1], other.group0()[1] * self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other.group0()[1] * self.group1()[0],
                other.group0()[1] * self.group1()[1],
                other.group0()[1] * self.group1()[2],
                other.group0()[0] * self.group0()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1] * self.group1()[3]]),
        );
    }
}
impl Wedge<AntiFlatPoint> for AntiMotor {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self.group0()[3]) * other.group0());
    }
}
impl Wedge<AntiFlector> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (other.group1()[1] * self.group1()[2]) + (other.group1()[3] * self.group0()[0]),
                (other.group1()[2] * self.group1()[0]) + (other.group1()[3] * self.group0()[1]),
                (other.group1()[0] * self.group1()[1]) + (other.group1()[3] * self.group0()[2]),
                -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * other.group0())
                - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group0()[0]]) * crate::swizzle!(other.group1(), 2, 0, 1, 0)),
            // e1, e2, e3, e5
            Simd32x4::from(self.group0()[3]) * other.group1(),
        );
    }
}
impl Wedge<AntiLine> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([other.group0()[0] * self.group0()[3], other.group0()[1] * self.group0()[3], other.group0()[2] * self.group0()[3], 0.0]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                other.group1()[0] * self.group0()[3],
                other.group1()[1] * self.group0()[3],
                other.group1()[2] * self.group0()[3],
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2]),
            ]),
        );
    }
}
impl Wedge<AntiMotor> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       15        0
    //  no simd       16       21        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[0] * self.group0()[3]) + (other.group0()[3] * self.group0()[0]),
                (other.group0()[1] * self.group0()[3]) + (other.group0()[3] * self.group0()[1]),
                (other.group0()[2] * self.group0()[3]) + (other.group0()[3] * self.group0()[2]),
                other.group0()[3] * self.group0()[3],
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(other.group0()[3]) * self.group1())
                + (Simd32x4::from(self.group0()[3]) * other.group1()),
        );
    }
}
impl Wedge<AntiPlane> for AntiMotor {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]) + (self.group1()[2] * other.group0()[1]),
                (self.group0()[1] * other.group0()[3]) + (self.group1()[0] * other.group0()[2]),
                (self.group0()[2] * other.group0()[3]) + (self.group1()[1] * other.group0()[0]),
                -(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]) - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group0()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1, 0)),
            // e1, e2, e3, e5
            Simd32x4::from(self.group0()[3]) * other.group0(),
        );
    }
}
impl Wedge<AntiScalar> for AntiMotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self.group0()[3] * other[e12345]);
    }
}
impl Wedge<Circle> for AntiMotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       16        0
    fn wedge(self, other: Circle) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self.group0()[3]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self.group0()[3]) * other.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                other.group2()[0] * self.group0()[3],
                other.group2()[1] * self.group0()[3],
                other.group2()[2] * self.group0()[3],
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2]),
            ]),
        );
    }
}
impl Wedge<CircleRotor> for AntiMotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       17        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self.group0()[3]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self.group0()[3]) * other.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                self.group0()[3] * other.group2()[0],
                self.group0()[3] * other.group2()[1],
                self.group0()[3] * other.group2()[2],
                (self.group0()[3] * other.group2()[3])
                    - (other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2]),
            ]),
        );
    }
}
impl Wedge<Dipole> for AntiMotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       13       28        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group0()[3]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self.group0()[3]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other.group2()[0] * self.group0()[3],
                other.group2()[1] * self.group0()[3],
                other.group2()[2] * self.group0()[3],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group1()[2]) + (self.group0()[0] * other.group1()[3]),
                (other.group0()[2] * self.group1()[0]) + (self.group0()[1] * other.group1()[3]),
                (other.group0()[0] * self.group1()[1]) + (self.group0()[2] * other.group1()[3]),
                -(other.group2()[0] * self.group0()[0])
                    - (other.group2()[1] * self.group0()[1])
                    - (other.group2()[2] * self.group0()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<DipoleInversion> for AntiMotor {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       18       33        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group0()[3]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self.group0()[3]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group0()[3] * other.group2()[0],
                self.group0()[3] * other.group2()[1],
                self.group0()[3] * other.group2()[2],
                (self.group0()[3] * other.group2()[3]) - (other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group1()[2]) + (self.group0()[3] * other.group3()[0]),
                (other.group0()[2] * self.group1()[0]) + (self.group0()[3] * other.group3()[1]),
                (other.group0()[0] * self.group1()[1]) + (self.group0()[3] * other.group3()[2]),
                -(self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2]),
            ]) + (Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[3], other.group3()[3]]) * self.group0())
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<DualNum> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        9        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                other.group0()[0] * self.group0()[0],
                other.group0()[0] * self.group0()[1],
                other.group0()[0] * self.group0()[2],
                (other.group0()[0] * self.group1()[3]) + (other.group0()[1] * self.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0] * self.group1()[0], other.group0()[0] * self.group1()[1], other.group0()[0] * self.group1()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group0()[3]]),
        );
    }
}
impl Wedge<FlatPoint> for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self.group0()[3]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0()[0] * other.group0()[3],
                self.group0()[1] * other.group0()[3],
                self.group0()[2] * other.group0()[3],
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]),
        );
    }
}
impl Wedge<Flector> for AntiMotor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self.group0()[3]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0()[3] * other.group1()[0],
                self.group0()[3] * other.group1()[1],
                self.group0()[3] * other.group1()[2],
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]) + (Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[3]]) * self.group0()),
        );
    }
}
impl Wedge<Line> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        9        0
    fn wedge(self, other: Line) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                other.group0()[0] * self.group0()[3],
                other.group0()[1] * self.group0()[3],
                other.group0()[2] * self.group0()[3],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0] * self.group0()[3], other.group1()[1] * self.group0()[3], other.group1()[2] * self.group0()[3], 0.0]),
        );
    }
}
impl Wedge<Motor> for AntiMotor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       14        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self.group0()[3] * other.group0()[0],
                self.group0()[3] * other.group0()[1],
                self.group0()[3] * other.group0()[2],
                (self.group0()[3] * other.group0()[3]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[0] * other.group1()[3]) + (self.group0()[3] * other.group1()[0]),
                (self.group0()[1] * other.group1()[3]) + (self.group0()[3] * other.group1()[1]),
                (self.group0()[2] * other.group1()[3]) + (self.group0()[3] * other.group1()[2]),
                self.group0()[3] * other.group1()[3],
            ]),
        );
    }
}
impl Wedge<MultiVector> for AntiMotor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       41        0
    //    simd3        4        7        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       30       53        0
    //  no simd       50       82        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                other.group0()[0] * self.group0()[3],
                (other.group0()[1] * self.group0()[3]) + (self.group1()[3] * other.group1()[3])
                    - (other.group7()[0] * self.group1()[0])
                    - (other.group7()[1] * self.group1()[1])
                    - (other.group7()[2] * self.group1()[2])
                    - (self.group0()[0] * other.group6()[0])
                    - (self.group0()[1] * other.group6()[1])
                    - (self.group0()[2] * other.group6()[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[3]) * other.group1(),
            // e5
            self.group0()[3] * other[e1],
            // e15, e25, e35, e45
            Simd32x4::from([
                (other.group0()[0] * self.group1()[0]) + (self.group0()[3] * other.group3()[0]),
                (other.group0()[0] * self.group1()[1]) + (self.group0()[3] * other.group3()[1]),
                (other.group0()[0] * self.group1()[2]) + (self.group0()[3] * other.group3()[2]),
                self.group0()[3] * other.group3()[3],
            ]),
            // e41, e42, e43
            Simd32x3::from(self.group0()[3]) * other.group4(),
            // e23, e31, e12
            (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])) + (Simd32x3::from(self.group0()[3]) * other.group5()),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group1()[0] * other.group1()[3],
                self.group1()[1] * other.group1()[3],
                self.group1()[2] * other.group1()[3],
                -(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * other.group6()),
            // e423, e431, e412
            (Simd32x3::from(self.group0()[3]) * other.group7()) + (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e235, e315, e125
            Simd32x3::from([
                (self.group1()[2] * other.group1()[1]) - (self.group1()[1] * other.group1()[2]),
                (self.group1()[0] * other.group1()[2]) - (self.group1()[2] * other.group1()[0]),
                (self.group1()[1] * other.group1()[0]) - (self.group1()[0] * other.group1()[1]),
            ]) + (Simd32x3::from(self.group0()[3]) * other.group8())
                + (Simd32x3::from(other[e1]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0()[3] * other.group9()[0],
                self.group0()[3] * other.group9()[1],
                self.group0()[3] * other.group9()[2],
                -(other.group5()[1] * self.group1()[1])
                    - (other.group5()[2] * self.group1()[2])
                    - (self.group0()[0] * other.group3()[0])
                    - (self.group0()[1] * other.group3()[1])
                    - (self.group0()[2] * other.group3()[2]),
            ]) + (Simd32x4::from([other.group4()[1], other.group4()[2], other.group4()[0], other.group0()[0]]) * crate::swizzle!(self.group1(), 2, 0, 1, 3))
                + (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group9()[3]]) * self.group0())
                - (Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group5()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
            // e1234
            (self.group0()[3] * other[e45]) - (other.group4()[0] * self.group0()[0]) - (other.group4()[1] * self.group0()[1]) - (other.group4()[2] * self.group0()[2]),
        );
    }
}
impl Wedge<Plane> for AntiMotor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Plane) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self.group0()[3]) * other.group0());
    }
}
impl Wedge<RoundPoint> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       18        0
    //  no simd        8       24        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group1()[0] * other.group0()[3],
                self.group1()[1] * other.group0()[3],
                self.group1()[2] * other.group0()[3],
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[0] * other[e2]) + (self.group1()[2] * other.group0()[1]) - (self.group1()[1] * other.group0()[2]),
                (self.group0()[1] * other[e2]) + (self.group1()[0] * other.group0()[2]) - (self.group1()[2] * other.group0()[0]),
                (self.group0()[2] * other[e2]) + (self.group1()[1] * other.group0()[0]) - (self.group1()[0] * other.group0()[1]),
                self.group0()[3] * other[e2],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[3]) * other.group0(),
        );
    }
}
impl Wedge<Scalar> for AntiMotor {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(other[scalar]) * self.group1(),
        );
    }
}
impl Wedge<Sphere> for AntiMotor {
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
            Simd32x4::from(self.group0()[3]) * other.group0(),
            // e1234
            self.group0()[3] * other[e4315],
        );
    }
}
impl Wedge<VersorEven> for AntiMotor {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       25       41        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self.group0()[3] * other.group0()[0],
                self.group0()[3] * other.group0()[1],
                self.group0()[3] * other.group0()[2],
                (self.group1()[3] * other.group3()[3])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2]),
            ]) + (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group0()[3]]) * self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group1()[0] * other.group3()[3],
                self.group1()[1] * other.group3()[3],
                self.group1()[2] * other.group3()[3],
                -(self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1]) - (self.group0()[2] * other.group3()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * other.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[0] * other.group2()[3]) + (self.group0()[3] * other.group2()[0]) + (self.group1()[2] * other.group3()[1]) - (self.group1()[1] * other.group3()[2]),
                (self.group0()[1] * other.group2()[3]) + (self.group0()[3] * other.group2()[1]) + (self.group1()[0] * other.group3()[2]) - (self.group1()[2] * other.group3()[0]),
                (self.group0()[2] * other.group2()[3]) + (self.group0()[3] * other.group2()[2]) + (self.group1()[1] * other.group3()[0]) - (self.group1()[0] * other.group3()[1]),
                self.group0()[3] * other.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[3]) * other.group3(),
        );
    }
}
impl Wedge<VersorOdd> for AntiMotor {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       13       26        0
    //  no simd       25       41        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(self.group0()[3]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]) + (self.group0()[3] * other.group1()[0]),
                (self.group0()[1] * other.group0()[3]) + (self.group0()[3] * other.group1()[1]),
                (self.group0()[2] * other.group0()[3]) + (self.group0()[3] * other.group1()[2]),
                self.group0()[3] * other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group1()[0] * other.group0()[3],
                self.group1()[1] * other.group0()[3],
                self.group1()[2] * other.group0()[3],
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * other.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0()[3] * other.group3()[0],
                self.group0()[3] * other.group3()[1],
                self.group0()[3] * other.group3()[2],
                -(self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2]),
            ]) + (Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[3], other.group3()[3]]) * self.group0())
                + (crate::swizzle!(self.group1(), 2, 0, 1, 3) * crate::swizzle!(other.group0(), 1, 2, 0, 3))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
        );
    }
}
impl std::ops::Div<wedge> for AntiPlane {
    type Output = wedge_partial<AntiPlane>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiPlane {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       14        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       14       28        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group0()[0] * self.group0()[3],
                other.group0()[1] * self.group0()[3],
                other.group0()[2] * self.group0()[3],
                -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) - (crate::swizzle!(other.group1(), 3, 3, 3, 0) * crate::swizzle!(self.group0(), 0, 1, 2, 0)),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group1()[0] * self.group0()[3]) + (other.group2()[2] * self.group0()[1]) - (other.group2()[1] * self.group0()[2]),
                (other.group1()[1] * self.group0()[3]) + (other.group2()[0] * self.group0()[2]) - (other.group2()[2] * self.group0()[0]),
                (other.group1()[2] * self.group0()[3]) + (other.group2()[1] * self.group0()[0]) - (other.group2()[0] * self.group0()[1]),
                0.0,
            ]),
            // e1, e2, e3, e5
            Simd32x4::from(other.group2()[3]) * self.group0(),
        );
    }
}
impl Wedge<AntiDipoleInversion> for AntiPlane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       21        0
    //    simd3        0        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        8       26        0
    //  no simd       17       39        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group2()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group3()[2] * self.group0()[1]) - (other.group3()[1] * self.group0()[2]),
                (other.group3()[0] * self.group0()[2]) - (other.group3()[2] * self.group0()[0]),
                (other.group3()[1] * self.group0()[0]) - (other.group3()[0] * self.group0()[1]),
                other.group2()[3] * self.group0()[3] * -1.0,
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group3()[0] * self.group0()[3]) * -1.0,
                (other.group3()[1] * self.group0()[3]) * -1.0,
                (other.group3()[2] * self.group0()[3]) * -1.0,
                (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]),
            ]) + (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group0()[0]]) * crate::swizzle!(self.group0(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group1()[1] * self.group0()[2],
                other.group1()[2] * self.group0()[0],
                other.group1()[0] * self.group0()[1],
                -(other.group2()[1] * self.group0()[1]) - (other.group2()[2] * self.group0()[2]),
            ]) - (Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiDualNum> for AntiPlane {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group0()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1] * self.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1] * self.group0()[0], other.group0()[1] * self.group0()[1], other.group0()[1] * self.group0()[2], 0.0]),
        );
    }
}
impl Wedge<AntiFlatPoint> for AntiPlane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            0.0,
            0.0,
            0.0,
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group0()[3] * self.group0()[3]),
        ]));
    }
}
impl Wedge<AntiFlector> for AntiPlane {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        9       16        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group1()[2] * self.group0()[1]) - (other.group1()[1] * self.group0()[2]),
                (other.group1()[0] * self.group0()[2]) - (other.group1()[2] * self.group0()[0]),
                (other.group1()[1] * self.group0()[0]) - (other.group1()[0] * self.group0()[1]),
                0.0,
            ]),
            // e15, e25, e35, e3215
            Simd32x4::from([
                other.group1()[3] * self.group0()[0],
                other.group1()[3] * self.group0()[1],
                other.group1()[3] * self.group0()[2],
                -(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group0()[3] * self.group0()[3]),
            ]) - (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[0]]) * crate::swizzle!(self.group0(), 3, 3, 3, 0)),
        );
    }
}
impl Wedge<AntiLine> for AntiPlane {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (other.group0()[0] * self.group0()[3]) + (other.group1()[2] * self.group0()[1]),
                (other.group0()[1] * self.group0()[3]) + (other.group1()[0] * self.group0()[2]),
                (other.group0()[2] * self.group0()[3]) + (other.group1()[1] * self.group0()[0]),
                -(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]) - (Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[0], other.group0()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
        );
    }
}
impl Wedge<AntiMotor> for AntiPlane {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([
                (other.group0()[0] * self.group0()[3]) + (other.group1()[2] * self.group0()[1]),
                (other.group0()[1] * self.group0()[3]) + (other.group1()[0] * self.group0()[2]),
                (other.group0()[2] * self.group0()[3]) + (other.group1()[1] * self.group0()[0]),
                -(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]) - (Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[0], other.group0()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
            // e1, e2, e3, e5
            Simd32x4::from(other.group0()[3]) * self.group0(),
        );
    }
}
impl Wedge<AntiPlane> for AntiPlane {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        6       12        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from([
                (other.group0()[2] * self.group0()[1]) - (other.group0()[1] * self.group0()[2]),
                (other.group0()[0] * self.group0()[2]) - (other.group0()[2] * self.group0()[0]),
                (other.group0()[1] * self.group0()[0]) - (other.group0()[0] * self.group0()[1]),
            ]),
            // e15, e25, e35
            (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
        );
    }
}
impl Wedge<Circle> for AntiPlane {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd       11       16        0
    fn wedge(self, other: Circle) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0()[2] * other.group1()[1],
                self.group0()[0] * other.group1()[2],
                self.group0()[1] * other.group1()[0],
                -(other.group2()[2] * self.group0()[2]) - (self.group0()[3] * other.group1()[3]),
            ]) - (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[0]]) * crate::swizzle!(self.group0(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0, 1)),
            // e1234
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]),
        );
    }
}
impl Wedge<CircleRotor> for AntiPlane {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd       11       16        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0()[2] * other.group1()[1],
                self.group0()[0] * other.group1()[2],
                self.group0()[1] * other.group1()[0],
                -(self.group0()[2] * other.group2()[2]) - (self.group0()[3] * other.group1()[3]),
            ]) - (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[0]]) * crate::swizzle!(self.group0(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0, 1)),
            // e1234
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]),
        );
    }
}
impl Wedge<Dipole> for AntiPlane {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       14       24        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group0()[0] * self.group0()[3],
                other.group0()[1] * self.group0()[3],
                other.group0()[2] * self.group0()[3],
                -(self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]),
            ]) - (crate::swizzle!(self.group0(), 0, 1, 2, 0) * crate::swizzle!(other.group1(), 3, 3, 3, 0)),
            // e235, e315, e125
            (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]) * crate::swizzle!(other.group2(), 2, 0, 1))
                - (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * crate::swizzle!(other.group2(), 1, 2, 0)),
        );
    }
}
impl Wedge<DipoleInversion> for AntiPlane {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       13        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       18        0
    //  no simd       17       31        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group0()[0] * self.group0()[3],
                other.group0()[1] * self.group0()[3],
                other.group0()[2] * self.group0()[3],
                -(self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]),
            ]) - (crate::swizzle!(self.group0(), 0, 1, 2, 0) * crate::swizzle!(other.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self.group0()[2] * other.group2()[1]) * -1.0,
                (self.group0()[0] * other.group2()[2]) * -1.0,
                (self.group0()[1] * other.group2()[0]) * -1.0,
                (self.group0()[2] * other.group3()[2]) + (self.group0()[3] * other.group2()[3]),
            ]) + (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group3()[1]]) * crate::swizzle!(self.group0(), 3, 3, 3, 1))
                + (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<DualNum> for AntiPlane {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group0()[3] * -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<FlatPoint> for AntiPlane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       12        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([
                (self.group0()[1] * other.group0()[2]) - (self.group0()[2] * other.group0()[1]),
                (self.group0()[2] * other.group0()[0]) - (self.group0()[0] * other.group0()[2]),
                (self.group0()[0] * other.group0()[1]) - (self.group0()[1] * other.group0()[0]),
            ]),
        );
    }
}
impl Wedge<Flector> for AntiPlane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       15        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self.group0()[0] * other.group0()[3] * -1.0,
                self.group0()[1] * other.group0()[3] * -1.0,
                self.group0()[2] * other.group0()[3] * -1.0,
                (self.group0()[0] * other.group1()[0]) + (self.group0()[1] * other.group1()[1]) + (self.group0()[2] * other.group1()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[1] * other.group0()[2]) - (self.group0()[2] * other.group0()[1]),
                (self.group0()[2] * other.group0()[0]) - (self.group0()[0] * other.group0()[2]),
                (self.group0()[0] * other.group0()[1]) - (self.group0()[1] * other.group0()[0]),
                0.0,
            ]),
        );
    }
}
impl Wedge<Line> for AntiPlane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn wedge(self, other: Line) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[1] * self.group0()[2],
                other.group0()[2] * self.group0()[0],
                other.group0()[0] * self.group0()[1],
                -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<Motor> for AntiPlane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        5       12        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self.group0()[0] * other.group1()[3], self.group0()[1] * other.group1()[3], self.group0()[2] * other.group1()[3], 0.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0()[2] * other.group0()[1],
                self.group0()[0] * other.group0()[2],
                self.group0()[1] * other.group0()[0],
                -(self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<MultiVector> for AntiPlane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       41        0
    //    simd3        2        5        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       21       49        0
    //  no simd       34       68        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (self.group0()[0] * other.group9()[0]) + (self.group0()[1] * other.group9()[1]) + (self.group0()[2] * other.group9()[2]) + (self.group0()[3] * other[e45]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0] * self.group0()[0], other.group0()[0] * self.group0()[1], other.group0()[0] * self.group0()[2], 0.0]),
            // e5
            other.group0()[0] * self.group0()[3],
            // e15, e25, e35, e45
            Simd32x4::from([
                (self.group0()[0] * other[e1]) - (self.group0()[3] * other.group1()[0]),
                (self.group0()[1] * other[e1]) - (self.group0()[3] * other.group1()[1]),
                (self.group0()[2] * other[e1]) - (self.group0()[3] * other.group1()[2]),
                self.group0()[3] * other.group1()[3] * -1.0,
            ]),
            // e41, e42, e43
            Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([
                (self.group0()[1] * other.group1()[2]) - (self.group0()[2] * other.group1()[1]),
                (self.group0()[2] * other.group1()[0]) - (self.group0()[0] * other.group1()[2]),
                (self.group0()[0] * other.group1()[1]) - (self.group0()[1] * other.group1()[0]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group4()[0] * self.group0()[3],
                other.group4()[1] * self.group0()[3],
                other.group4()[2] * self.group0()[3],
                -(other.group5()[1] * self.group0()[1]) - (other.group5()[2] * self.group0()[2]),
            ]) - (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group5()[0]]) * crate::swizzle!(self.group0(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * crate::swizzle!(other.group4(), 1, 2, 0))
                - (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]) * crate::swizzle!(other.group4(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (self.group0()[1] * other.group3()[2]) - (self.group0()[2] * other.group3()[1]),
                (self.group0()[2] * other.group3()[0]) - (self.group0()[0] * other.group3()[2]),
                (self.group0()[0] * other.group3()[1]) - (self.group0()[1] * other.group3()[0]),
            ]) + (Simd32x3::from(self.group0()[3]) * other.group5()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0()[2] * other.group6()[1],
                self.group0()[0] * other.group6()[2],
                self.group0()[1] * other.group6()[0],
                -(other.group8()[2] * self.group0()[2]) - (self.group0()[3] * other.group6()[3]),
            ]) - (Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], other.group8()[0]]) * crate::swizzle!(self.group0(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group6()[2], other.group6()[0], other.group6()[1], other.group8()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0, 1)),
            // e1234
            (other.group7()[0] * self.group0()[0]) + (other.group7()[1] * self.group0()[1]) + (other.group7()[2] * self.group0()[2]),
        );
    }
}
impl Wedge<Plane> for AntiPlane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn wedge(self, other: Plane) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2]),
        );
    }
}
impl Wedge<RoundPoint> for AntiPlane {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd3        1        4        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        6       20        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[1] * other.group0()[2]) - (self.group0()[2] * other.group0()[1]),
                (self.group0()[2] * other.group0()[0]) - (self.group0()[0] * other.group0()[2]),
                (self.group0()[0] * other.group0()[1]) - (self.group0()[1] * other.group0()[0]),
                self.group0()[3] * other.group0()[3] * -1.0,
            ]),
            // e15, e25, e35
            (Simd32x3::from(other[e2]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
        );
    }
}
impl Wedge<Scalar> for AntiPlane {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(other[scalar]) * self.group0());
    }
}
impl Wedge<Sphere> for AntiPlane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[3] * other[e4315]),
        );
    }
}
impl Wedge<VersorEven> for AntiPlane {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       21        0
    //    simd3        0        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        8       26        0
    //  no simd       17       39        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[1] * other.group3()[2]) - (self.group0()[2] * other.group3()[1]),
                (self.group0()[2] * other.group3()[0]) - (self.group0()[0] * other.group3()[2]),
                (self.group0()[0] * other.group3()[1]) - (self.group0()[1] * other.group3()[0]),
                self.group0()[3] * other.group3()[3] * -1.0,
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (self.group0()[3] * other.group3()[0]) * -1.0,
                (self.group0()[3] * other.group3()[1]) * -1.0,
                (self.group0()[3] * other.group3()[2]) * -1.0,
                (self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2]),
            ]) + (Simd32x4::from([other.group2()[3], other.group2()[3], other.group2()[3], other.group0()[0]]) * crate::swizzle!(self.group0(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                self.group0()[2] * other.group1()[1],
                self.group0()[0] * other.group1()[2],
                self.group0()[1] * other.group1()[0],
                -(self.group0()[2] * other.group2()[2]) - (self.group0()[3] * other.group1()[3]),
            ]) - (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[1]]) * crate::swizzle!(self.group0(), 3, 3, 3, 1))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<VersorOdd> for AntiPlane {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       27        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       17       35        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (self.group0()[1] * other.group0()[2]) * -1.0,
                (self.group0()[2] * other.group0()[0]) * -1.0,
                (self.group0()[0] * other.group0()[1]) * -1.0,
                (self.group0()[1] * other.group3()[1]) + (self.group0()[2] * other.group3()[2]) + (self.group0()[3] * other.group2()[3]),
            ]) + (Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group3()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group0()[3] * other.group0()[0],
                self.group0()[3] * other.group0()[1],
                self.group0()[3] * other.group0()[2],
                -(self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]),
            ]) - (crate::swizzle!(self.group0(), 0, 1, 2, 0) * crate::swizzle!(other.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[1] * other.group2()[2]) + (self.group0()[3] * other.group1()[0]) - (self.group0()[2] * other.group2()[1]),
                (self.group0()[2] * other.group2()[0]) + (self.group0()[3] * other.group1()[1]) - (self.group0()[0] * other.group2()[2]),
                (self.group0()[0] * other.group2()[1]) + (self.group0()[3] * other.group1()[2]) - (self.group0()[1] * other.group2()[0]),
                self.group0()[3] * other.group0()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0] * other.group0()[3], self.group0()[1] * other.group0()[3], self.group0()[2] * other.group0()[3], 0.0]),
        );
    }
}
impl std::ops::Div<wedge> for AntiScalar {
    type Output = wedge_partial<AntiScalar>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other.group2()[3] * self[e12345]);
    }
}
impl Wedge<AntiDualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other.group0()[1] * self[e12345]);
    }
}
impl Wedge<AntiMotor> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other.group0()[3] * self[e12345]);
    }
}
impl Wedge<MultiVector> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other.group0()[0] * self[e12345]);
    }
}
impl Wedge<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self[e12345] * other[scalar]);
    }
}
impl Wedge<VersorOdd> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other.group0()[3] * self[e12345]);
    }
}
impl std::ops::Div<wedge> for Circle {
    type Output = wedge_partial<Circle>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       13        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        9       15        0
    //  no simd        9       20        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other.group2()[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other.group2()[3]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                self.group2()[0] * other.group2()[3],
                self.group2()[1] * other.group2()[3],
                self.group2()[2] * other.group2()[3],
                -(other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])
                    - (other.group1()[3] * self.group1()[3]),
            ]),
        );
    }
}
impl Wedge<AntiDipoleInversion> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self.group2()[0] * other.group2()[3]) - (other.group3()[2] * self.group1()[1]),
                -(self.group2()[1] * other.group2()[3]) - (other.group3()[0] * self.group1()[2]),
                -(self.group2()[2] * other.group2()[3]) - (other.group3()[1] * self.group1()[0]),
                (self.group2()[2] * other.group3()[2]) + (other.group3()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[0]]) * crate::swizzle!(other.group3(), 3, 3, 3, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[1]]) * crate::swizzle!(other.group3(), 1, 2, 0, 1)),
            // e1234
            -(self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1]) - (self.group0()[2] * other.group3()[2]) - (other.group2()[3] * self.group1()[3]),
        );
    }
}
impl Wedge<AntiDualNum> for Circle {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other.group0()[1]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other.group0()[1]) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(other.group0()[1]) * self.group2(),
        );
    }
}
impl Wedge<AntiFlector> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       13        0
    //  no simd       11       19        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[2] * self.group1()[1]) * -1.0,
                (other.group1()[0] * self.group1()[2]) * -1.0,
                (other.group1()[1] * self.group1()[0]) * -1.0,
                (self.group2()[2] * other.group1()[2]) + (other.group1()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[0]]) * crate::swizzle!(other.group1(), 3, 3, 3, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[1]]) * crate::swizzle!(other.group1(), 1, 2, 0, 1)),
            // e1234
            -(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]),
        );
    }
}
impl Wedge<AntiLine> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(other.group0()[0] * self.group1()[0])
                - (other.group0()[1] * self.group1()[1])
                - (other.group0()[2] * self.group1()[2])
                - (other.group1()[0] * self.group0()[0])
                - (other.group1()[1] * self.group0()[1])
                - (other.group1()[2] * self.group0()[2]),
        );
    }
}
impl Wedge<AntiMotor> for Circle {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       16        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other.group0()[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other.group0()[3]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                self.group2()[0] * other.group0()[3],
                self.group2()[1] * other.group0()[3],
                self.group2()[2] * other.group0()[3],
                -(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2]),
            ]),
        );
    }
}
impl Wedge<AntiPlane> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       13        0
    //  no simd       11       19        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[2] * self.group1()[1]) * -1.0,
                (other.group0()[0] * self.group1()[2]) * -1.0,
                (other.group0()[1] * self.group1()[0]) * -1.0,
                (self.group2()[2] * other.group0()[2]) + (other.group0()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[0]]) * crate::swizzle!(other.group0(), 3, 3, 3, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0, 1)),
            // e1234
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
        );
    }
}
impl Wedge<Dipole> for Circle {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(self.group0()[0] * other.group2()[0])
                - (self.group0()[1] * other.group2()[1])
                - (self.group0()[2] * other.group2()[2])
                - (self.group2()[0] * other.group0()[0])
                - (self.group2()[1] * other.group0()[1])
                - (self.group2()[2] * other.group0()[2])
                - (self.group1()[0] * other.group1()[0])
                - (self.group1()[1] * other.group1()[1])
                - (self.group1()[2] * other.group1()[2])
                - (self.group1()[3] * other.group1()[3]),
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
            -(self.group0()[0] * other.group2()[0])
                - (self.group0()[1] * other.group2()[1])
                - (self.group0()[2] * other.group2()[2])
                - (self.group2()[0] * other.group0()[0])
                - (self.group2()[1] * other.group0()[1])
                - (self.group2()[2] * other.group0()[2])
                - (self.group1()[0] * other.group1()[0])
                - (self.group1()[1] * other.group1()[1])
                - (self.group1()[2] * other.group1()[2])
                - (self.group1()[3] * other.group1()[3]),
        );
    }
}
impl Wedge<DualNum> for Circle {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[0] * self.group2()[0] * -1.0,
                other.group0()[0] * self.group2()[1] * -1.0,
                other.group0()[0] * self.group2()[2] * -1.0,
                0.0,
            ]),
            // e1234
            other.group0()[0] * self.group1()[3] * -1.0,
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
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group1()[3] * other.group0()[3]),
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
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group1()[3] * other.group0()[3]),
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
            Simd32x4::from(other.group1()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
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
                -(self.group0()[0] * other.group3()[0])
                    - (self.group0()[1] * other.group3()[1])
                    - (self.group0()[2] * other.group3()[2])
                    - (self.group2()[0] * other.group4()[0])
                    - (self.group2()[1] * other.group4()[1])
                    - (self.group2()[2] * other.group4()[2])
                    - (other.group5()[0] * self.group1()[0])
                    - (other.group5()[1] * self.group1()[1])
                    - (other.group5()[2] * self.group1()[2])
                    - (self.group1()[3] * other.group3()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(other.group0()[0]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(other.group0()[0]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other.group0()[0]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self.group2()[0] * other.group1()[3]) - (self.group1()[1] * other.group1()[2]),
                -(self.group2()[1] * other.group1()[3]) - (self.group1()[2] * other.group1()[0]),
                -(self.group2()[2] * other.group1()[3]) - (self.group1()[0] * other.group1()[1]),
                (self.group2()[1] * other.group1()[1]) + (self.group2()[2] * other.group1()[2]),
            ]) + (Simd32x4::from(other[e1]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
            // e1234
            -(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]) - (self.group1()[3] * other.group1()[3]),
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
            Simd32x4::from([
                -(self.group2()[0] * other.group0()[3]) - (self.group1()[1] * other.group0()[2]),
                -(self.group2()[1] * other.group0()[3]) - (self.group1()[2] * other.group0()[0]),
                -(self.group2()[2] * other.group0()[3]) - (self.group1()[0] * other.group0()[1]),
                (self.group2()[1] * other.group0()[1]) + (self.group2()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(other[e2]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
            // e1234
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group1()[3] * other.group0()[3]),
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
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group2(),
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
            Simd32x4::from([
                -(self.group2()[0] * other.group3()[3]) - (self.group1()[1] * other.group3()[2]),
                -(self.group2()[1] * other.group3()[3]) - (self.group1()[2] * other.group3()[0]),
                -(self.group2()[2] * other.group3()[3]) - (self.group1()[0] * other.group3()[1]),
                (self.group2()[1] * other.group3()[1]) + (self.group2()[2] * other.group3()[2]),
            ]) + (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0)),
            // e1234
            -(self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1]) - (self.group0()[2] * other.group3()[2]) - (self.group1()[3] * other.group3()[3]),
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
            Simd32x3::from(other.group0()[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other.group0()[3]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                self.group2()[0] * other.group0()[3],
                self.group2()[1] * other.group0()[3],
                self.group2()[2] * other.group0()[3],
                -(self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[2] * other.group0()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[3] * other.group1()[3]),
            ]),
        );
    }
}
impl std::ops::Div<wedge> for CircleRotor {
    type Output = wedge_partial<CircleRotor>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       14        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       10       21        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other.group2()[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other.group2()[3]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                other.group2()[3] * self.group2()[0],
                other.group2()[3] * self.group2()[1],
                other.group2()[3] * self.group2()[2],
                (other.group2()[3] * self.group2()[3])
                    - (other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])
                    - (other.group1()[3] * self.group1()[3]),
            ]),
        );
    }
}
impl Wedge<AntiDipoleInversion> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        9       14        0
    //  no simd       15       20        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(other.group2()[3] * self.group2()[0]) - (other.group3()[2] * self.group1()[1]),
                -(other.group2()[3] * self.group2()[1]) - (other.group3()[0] * self.group1()[2]),
                -(other.group2()[3] * self.group2()[2]) - (other.group3()[1] * self.group1()[0]),
                (other.group3()[2] * self.group2()[2]) + (other.group3()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[0]]) * crate::swizzle!(other.group3(), 3, 3, 3, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[1]]) * crate::swizzle!(other.group3(), 1, 2, 0, 1)),
            // e1234
            -(self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1]) - (self.group0()[2] * other.group3()[2]) - (other.group2()[3] * self.group1()[3]),
        );
    }
}
impl Wedge<AntiDualNum> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other.group0()[1]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other.group0()[1]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(other.group0()[1]) * self.group2(),
        );
    }
}
impl Wedge<AntiFlector> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       13        0
    //  no simd       11       19        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[2] * self.group1()[1]) * -1.0,
                (other.group1()[0] * self.group1()[2]) * -1.0,
                (other.group1()[1] * self.group1()[0]) * -1.0,
                (other.group1()[2] * self.group2()[2]) + (other.group1()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[0]]) * crate::swizzle!(other.group1(), 3, 3, 3, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[1]]) * crate::swizzle!(other.group1(), 1, 2, 0, 1)),
            // e1234
            -(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]),
        );
    }
}
impl Wedge<AntiLine> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(other.group0()[0] * self.group1()[0])
                - (other.group0()[1] * self.group1()[1])
                - (other.group0()[2] * self.group1()[2])
                - (other.group1()[0] * self.group0()[0])
                - (other.group1()[1] * self.group0()[1])
                - (other.group1()[2] * self.group0()[2]),
        );
    }
}
impl Wedge<AntiMotor> for CircleRotor {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       12        0
    //  no simd        6       17        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other.group0()[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other.group0()[3]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                other.group0()[3] * self.group2()[0],
                other.group0()[3] * self.group2()[1],
                other.group0()[3] * self.group2()[2],
                (other.group0()[3] * self.group2()[3])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2]),
            ]),
        );
    }
}
impl Wedge<AntiPlane> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       11        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        5       13        0
    //  no simd       11       19        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[2] * self.group1()[1]) * -1.0,
                (other.group0()[0] * self.group1()[2]) * -1.0,
                (other.group0()[1] * self.group1()[0]) * -1.0,
                (other.group0()[2] * self.group2()[2]) + (other.group0()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[0]]) * crate::swizzle!(other.group0(), 3, 3, 3, 0))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0, 1)),
            // e1234
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
        );
    }
}
impl Wedge<Dipole> for CircleRotor {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(self.group0()[0] * other.group2()[0])
                - (self.group0()[1] * other.group2()[1])
                - (self.group0()[2] * other.group2()[2])
                - (other.group0()[0] * self.group2()[0])
                - (other.group0()[1] * self.group2()[1])
                - (other.group0()[2] * self.group2()[2])
                - (self.group1()[0] * other.group1()[0])
                - (self.group1()[1] * other.group1()[1])
                - (self.group1()[2] * other.group1()[2])
                - (self.group1()[3] * other.group1()[3]),
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
            -(self.group0()[0] * other.group2()[0])
                - (self.group0()[1] * other.group2()[1])
                - (self.group0()[2] * other.group2()[2])
                - (other.group0()[0] * self.group2()[0])
                - (other.group0()[1] * self.group2()[1])
                - (other.group0()[2] * self.group2()[2])
                - (self.group1()[0] * other.group1()[0])
                - (self.group1()[1] * other.group1()[1])
                - (self.group1()[2] * other.group1()[2])
                - (self.group1()[3] * other.group1()[3]),
        );
    }
}
impl Wedge<DualNum> for CircleRotor {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[0] * self.group2()[0] * -1.0,
                other.group0()[0] * self.group2()[1] * -1.0,
                other.group0()[0] * self.group2()[2] * -1.0,
                0.0,
            ]),
            // e1234
            other.group0()[0] * self.group1()[3] * -1.0,
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
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group1()[3] * other.group0()[3]),
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
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group1()[3] * other.group0()[3]),
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
            Simd32x4::from(other.group1()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
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
                (other.group0()[0] * self.group2()[3])
                    - (self.group0()[0] * other.group3()[0])
                    - (self.group0()[1] * other.group3()[1])
                    - (self.group0()[2] * other.group3()[2])
                    - (other.group4()[0] * self.group2()[0])
                    - (other.group4()[1] * self.group2()[1])
                    - (other.group4()[2] * self.group2()[2])
                    - (other.group5()[0] * self.group1()[0])
                    - (other.group5()[1] * self.group1()[1])
                    - (other.group5()[2] * self.group1()[2])
                    - (self.group1()[3] * other.group3()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(other.group0()[0]) * self.group1(),
            // e423, e431, e412
            Simd32x3::from(other.group0()[0]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self.group1()[1] * other.group1()[2]) - (self.group2()[0] * other.group1()[3]),
                -(self.group1()[2] * other.group1()[0]) - (self.group2()[1] * other.group1()[3]),
                -(self.group1()[0] * other.group1()[1]) - (self.group2()[2] * other.group1()[3]),
                (self.group2()[1] * other.group1()[1]) + (self.group2()[2] * other.group1()[2]),
            ]) + (Simd32x4::from(other[e1]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
            // e1234
            -(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]) - (self.group1()[3] * other.group1()[3]),
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
            Simd32x4::from([
                -(self.group1()[1] * other.group0()[2]) - (self.group2()[0] * other.group0()[3]),
                -(self.group1()[2] * other.group0()[0]) - (self.group2()[1] * other.group0()[3]),
                -(self.group1()[0] * other.group0()[1]) - (self.group2()[2] * other.group0()[3]),
                (self.group2()[1] * other.group0()[1]) + (self.group2()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(other[e2]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
            // e1234
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group1()[3] * other.group0()[3]),
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
            Simd32x3::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(other[scalar]) * self.group2(),
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
            Simd32x4::from([
                -(self.group1()[1] * other.group3()[2]) - (self.group2()[0] * other.group3()[3]),
                -(self.group1()[2] * other.group3()[0]) - (self.group2()[1] * other.group3()[3]),
                -(self.group1()[0] * other.group3()[1]) - (self.group2()[2] * other.group3()[3]),
                (self.group2()[1] * other.group3()[1]) + (self.group2()[2] * other.group3()[2]),
            ]) + (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0)),
            // e1234
            -(self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1]) - (self.group0()[2] * other.group3()[2]) - (self.group1()[3] * other.group3()[3]),
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
            Simd32x3::from(other.group0()[3]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other.group0()[3]) * self.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                self.group2()[0] * other.group0()[3],
                self.group2()[1] * other.group0()[3],
                self.group2()[2] * other.group0()[3],
                (self.group2()[3] * other.group0()[3])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[2] * other.group0()[2]),
            ]),
        );
    }
}
impl std::ops::Div<wedge> for Dipole {
    type Output = wedge_partial<Dipole>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for Dipole {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       29        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       22       32        0
    //  no simd       25       40        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group2()[3]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other.group2()[3]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group2()[0] * other.group2()[3],
                self.group2()[1] * other.group2()[3],
                self.group2()[2] * other.group2()[3],
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group2()[2]) + (self.group0()[1] * other.group2()[2]) + (other.group1()[0] * self.group1()[3]) + (other.group1()[3] * self.group1()[0])
                    - (other.group0()[2] * self.group2()[1]),
                (other.group0()[2] * self.group2()[0]) + (self.group0()[2] * other.group2()[0]) + (other.group1()[1] * self.group1()[3]) + (other.group1()[3] * self.group1()[1])
                    - (other.group0()[0] * self.group2()[2]),
                (other.group0()[0] * self.group2()[1]) + (self.group0()[0] * other.group2()[1]) + (other.group1()[2] * self.group1()[3]) + (other.group1()[3] * self.group1()[2])
                    - (other.group0()[1] * self.group2()[0]),
                -(self.group2()[0] * other.group1()[0])
                    - (self.group2()[1] * other.group1()[1])
                    - (self.group2()[2] * other.group1()[2])
                    - (other.group2()[1] * self.group1()[1])
                    - (other.group2()[2] * self.group1()[2]),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiDipoleInversion> for Dipole {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        0
    //    simd3        2        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       22       31        0
    //  no simd       29       40        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other.group2()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([other.group3()[2], other.group3()[0], other.group3()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group3()[1], other.group3()[2], other.group3()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group3()[3]) + (self.group2()[0] * other.group2()[3]),
                (self.group0()[1] * other.group3()[3]) + (self.group2()[1] * other.group2()[3]),
                (self.group0()[2] * other.group3()[3]) + (self.group2()[2] * other.group2()[3]),
                -(other.group3()[1] * self.group1()[1]) - (other.group3()[2] * self.group1()[2]),
            ]) - (crate::swizzle!(other.group3(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self.group2()[2] * other.group3()[1]) + (other.group3()[3] * self.group1()[0]) - (self.group2()[1] * other.group3()[2]),
                (self.group2()[0] * other.group3()[2]) + (other.group3()[3] * self.group1()[1]) - (self.group2()[2] * other.group3()[0]),
                (self.group2()[1] * other.group3()[0]) + (other.group3()[3] * self.group1()[2]) - (self.group2()[0] * other.group3()[1]),
                -(other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])
                    - (other.group1()[3] * self.group1()[3]),
            ]),
        );
    }
}
impl Wedge<AntiDualNum> for Dipole {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group0()[1]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other.group0()[1]) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(other.group0()[1]) * self.group2(),
        );
    }
}
impl Wedge<AntiFlatPoint> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (other.group0()[3] * self.group1()[3]),
        );
    }
}
impl Wedge<AntiFlector> for Dipole {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       12       21        0
    //  no simd       17       28        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group0()[0] * other.group1()[3],
                self.group0()[1] * other.group1()[3],
                self.group0()[2] * other.group1()[3],
                -(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2]),
            ]) - (crate::swizzle!(other.group1(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self.group2()[2] * other.group1()[1]) + (other.group1()[3] * self.group1()[0]) - (self.group2()[1] * other.group1()[2]),
                (self.group2()[0] * other.group1()[2]) + (other.group1()[3] * self.group1()[1]) - (self.group2()[2] * other.group1()[0]),
                (self.group2()[1] * other.group1()[0]) + (other.group1()[3] * self.group1()[2]) - (self.group2()[0] * other.group1()[1]),
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (other.group0()[3] * self.group1()[3]),
            ]),
        );
    }
}
impl Wedge<AntiLine> for Dipole {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       18        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] * self.group1()[3]) + (other.group1()[2] * self.group0()[1]) - (other.group1()[1] * self.group0()[2]),
                (other.group0()[1] * self.group1()[3]) + (other.group1()[0] * self.group0()[2]) - (other.group1()[2] * self.group0()[0]),
                (other.group0()[2] * self.group1()[3]) + (other.group1()[1] * self.group0()[0]) - (other.group1()[0] * self.group0()[1]),
                -(other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2]),
            ]),
            // e1234
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
        );
    }
}
impl Wedge<AntiMotor> for Dipole {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       13       28        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group0()[3]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other.group0()[3]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group2()[0] * other.group0()[3],
                self.group2()[1] * other.group0()[3],
                self.group2()[2] * other.group0()[3],
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group1()[2]) + (other.group0()[0] * self.group1()[3]),
                (self.group0()[2] * other.group1()[0]) + (other.group0()[1] * self.group1()[3]),
                (self.group0()[0] * other.group1()[1]) + (other.group0()[2] * self.group1()[3]),
                -(self.group2()[0] * other.group0()[0])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[2] * other.group0()[2])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2]),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiPlane> for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd3        3        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       11        0
    //  no simd       14       24        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group0()[0] * other.group0()[3],
                self.group0()[1] * other.group0()[3],
                self.group0()[2] * other.group0()[3],
                -(other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]),
            ]) - (crate::swizzle!(other.group0(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 3, 3, 3, 0)),
            // e235, e315, e125
            (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * crate::swizzle!(self.group2(), 2, 0, 1))
                - (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]) * crate::swizzle!(self.group2(), 1, 2, 0)),
        );
    }
}
impl Wedge<Circle> for Dipole {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: Circle) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(other.group0()[0] * self.group2()[0])
                - (other.group0()[1] * self.group2()[1])
                - (other.group0()[2] * self.group2()[2])
                - (other.group2()[0] * self.group0()[0])
                - (other.group2()[1] * self.group0()[1])
                - (other.group2()[2] * self.group0()[2])
                - (other.group1()[0] * self.group1()[0])
                - (other.group1()[1] * self.group1()[1])
                - (other.group1()[2] * self.group1()[2])
                - (other.group1()[3] * self.group1()[3]),
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
            -(other.group0()[0] * self.group2()[0])
                - (other.group0()[1] * self.group2()[1])
                - (other.group0()[2] * self.group2()[2])
                - (self.group0()[0] * other.group2()[0])
                - (self.group0()[1] * other.group2()[1])
                - (self.group0()[2] * other.group2()[2])
                - (other.group1()[0] * self.group1()[0])
                - (other.group1()[1] * self.group1()[1])
                - (other.group1()[2] * self.group1()[2])
                - (other.group1()[3] * self.group1()[3]),
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
                (other.group0()[1] * self.group2()[2]) + (other.group2()[2] * self.group0()[1]) + (other.group1()[0] * self.group1()[3]) + (other.group1()[3] * self.group1()[0])
                    - (other.group0()[2] * self.group2()[1])
                    - (other.group2()[1] * self.group0()[2]),
                (other.group0()[2] * self.group2()[0]) + (other.group2()[0] * self.group0()[2]) + (other.group1()[1] * self.group1()[3]) + (other.group1()[3] * self.group1()[1])
                    - (other.group0()[0] * self.group2()[2])
                    - (other.group2()[2] * self.group0()[0]),
                (other.group0()[0] * self.group2()[1]) + (other.group2()[1] * self.group0()[0]) + (other.group1()[2] * self.group1()[3]) + (other.group1()[3] * self.group1()[2])
                    - (other.group0()[1] * self.group2()[0])
                    - (other.group2()[0] * self.group0()[1]),
                -(other.group2()[0] * self.group1()[0])
                    - (other.group2()[1] * self.group1()[1])
                    - (other.group2()[2] * self.group1()[2])
                    - (self.group2()[0] * other.group1()[0])
                    - (self.group2()[1] * other.group1()[1])
                    - (self.group2()[2] * other.group1()[2]),
            ]),
            // e1234
            -(other.group0()[0] * self.group1()[0])
                - (other.group0()[1] * self.group1()[1])
                - (other.group0()[2] * self.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])
                - (self.group0()[2] * other.group1()[2]),
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
            Simd32x4::from([
                (self.group0()[1] * other.group2()[2]) + (self.group2()[2] * other.group0()[1]) + (self.group1()[0] * other.group1()[3]) + (self.group1()[3] * other.group1()[0])
                    - (self.group2()[1] * other.group0()[2]),
                (self.group0()[2] * other.group2()[0]) + (self.group2()[0] * other.group0()[2]) + (self.group1()[1] * other.group1()[3]) + (self.group1()[3] * other.group1()[1])
                    - (self.group2()[2] * other.group0()[0]),
                (self.group0()[0] * other.group2()[1]) + (self.group2()[1] * other.group0()[0]) + (self.group1()[2] * other.group1()[3]) + (self.group1()[3] * other.group1()[2])
                    - (self.group2()[0] * other.group0()[1]),
                -(self.group2()[0] * other.group1()[0])
                    - (self.group2()[1] * other.group1()[1])
                    - (self.group2()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group2()[1])
                    - (self.group1()[2] * other.group2()[2]),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0)),
            // e1234
            -(self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])
                - (self.group0()[2] * other.group1()[2])
                - (other.group0()[0] * self.group1()[0])
                - (other.group0()[1] * self.group1()[1])
                - (other.group0()[2] * self.group1()[2]),
        );
    }
}
impl Wedge<DualNum> for Dipole {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0] * self.group2()[0], other.group0()[0] * self.group2()[1], other.group0()[0] * self.group2()[2], 0.0]),
            // e235, e315, e125
            Simd32x3::from(0.0),
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
            Simd32x4::from([
                (self.group0()[1] * other.group0()[2]) + (self.group1()[0] * other.group0()[3]),
                (self.group0()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3]),
                (self.group0()[0] * other.group0()[1]) + (self.group1()[2] * other.group0()[3]),
                -(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2]),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
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
            Simd32x4::from([
                (self.group0()[1] * other.group0()[2]) + (self.group1()[0] * other.group0()[3]),
                (self.group0()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3]),
                (self.group0()[0] * other.group0()[1]) + (self.group1()[2] * other.group0()[3]),
                -(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2]),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
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
            -(self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])
                - (self.group0()[2] * other.group1()[2])
                - (other.group0()[0] * self.group1()[0])
                - (other.group0()[1] * self.group1()[1])
                - (other.group0()[2] * self.group1()[2]),
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
                self.group0()[0] * other.group1()[3],
                self.group0()[1] * other.group1()[3],
                self.group0()[2] * other.group1()[3],
                -(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0] * other.group1()[3], self.group1()[1] * other.group1()[3], self.group1()[2] * other.group1()[3], 0.0]),
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
                -(self.group0()[0] * other.group8()[0])
                    - (self.group0()[1] * other.group8()[1])
                    - (self.group0()[2] * other.group8()[2])
                    - (self.group2()[0] * other.group7()[0])
                    - (self.group2()[1] * other.group7()[1])
                    - (self.group2()[2] * other.group7()[2])
                    - (self.group1()[0] * other.group6()[0])
                    - (self.group1()[1] * other.group6()[1])
                    - (self.group1()[2] * other.group6()[2])
                    - (self.group1()[3] * other.group6()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            Simd32x3::from(other.group0()[0]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other[e1]) + (self.group2()[0] * other.group1()[3]),
                (self.group0()[1] * other[e1]) + (self.group2()[1] * other.group1()[3]),
                (self.group0()[2] * other[e1]) + (self.group2()[2] * other.group1()[3]),
                -(self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2]),
            ]) - (crate::swizzle!(self.group1(), 3, 3, 3, 0) * crate::swizzle!(other.group1(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e235, e315, e125
            (Simd32x3::from(other[e1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]) * crate::swizzle!(self.group2(), 2, 0, 1))
                - (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]) * crate::swizzle!(self.group2(), 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group3()[2]) + (self.group2()[2] * other.group4()[1]) + (other.group5()[0] * self.group1()[3]) + (self.group1()[0] * other.group3()[3])
                    - (self.group2()[1] * other.group4()[2]),
                (self.group0()[2] * other.group3()[0]) + (self.group2()[0] * other.group4()[2]) + (other.group5()[1] * self.group1()[3]) + (self.group1()[1] * other.group3()[3])
                    - (self.group2()[2] * other.group4()[0]),
                (self.group0()[0] * other.group3()[1]) + (self.group2()[1] * other.group4()[0]) + (other.group5()[2] * self.group1()[3]) + (self.group1()[2] * other.group3()[3])
                    - (self.group2()[0] * other.group4()[1]),
                -(self.group2()[0] * other.group5()[0])
                    - (self.group2()[1] * other.group5()[1])
                    - (self.group2()[2] * other.group5()[2])
                    - (self.group1()[1] * other.group3()[1])
                    - (self.group1()[2] * other.group3()[2]),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0)),
            // e1234
            -(self.group0()[0] * other.group5()[0])
                - (self.group0()[1] * other.group5()[1])
                - (self.group0()[2] * other.group5()[2])
                - (other.group4()[0] * self.group1()[0])
                - (other.group4()[1] * self.group1()[1])
                - (other.group4()[2] * self.group1()[2]),
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
            (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other[e2]) + (self.group2()[0] * other.group0()[3]),
                (self.group0()[1] * other[e2]) + (self.group2()[1] * other.group0()[3]),
                (self.group0()[2] * other[e2]) + (self.group2()[2] * other.group0()[3]),
                -(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2]),
            ]) - (crate::swizzle!(self.group1(), 3, 3, 3, 0) * crate::swizzle!(other.group0(), 0, 1, 2, 0)),
            // e235, e315, e125
            (Simd32x3::from(other[e2]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * crate::swizzle!(self.group2(), 2, 0, 1))
                - (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]) * crate::swizzle!(self.group2(), 1, 2, 0)),
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
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35
            Simd32x3::from(other[scalar]) * self.group2(),
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
            (Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([other.group3()[2], other.group3()[0], other.group3()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group3()[1], other.group3()[2], other.group3()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group2()[3]) + (self.group2()[0] * other.group3()[3]),
                (self.group0()[1] * other.group2()[3]) + (self.group2()[1] * other.group3()[3]),
                (self.group0()[2] * other.group2()[3]) + (self.group2()[2] * other.group3()[3]),
                -(self.group1()[1] * other.group3()[1]) - (self.group1()[2] * other.group3()[2]),
            ]) - (crate::swizzle!(self.group1(), 3, 3, 3, 0) * crate::swizzle!(other.group3(), 0, 1, 2, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self.group2()[2] * other.group3()[1]) + (self.group1()[0] * other.group2()[3]) - (self.group2()[1] * other.group3()[2]),
                (self.group2()[0] * other.group3()[2]) + (self.group1()[1] * other.group2()[3]) - (self.group2()[2] * other.group3()[0]),
                (self.group2()[1] * other.group3()[0]) + (self.group1()[2] * other.group2()[3]) - (self.group2()[0] * other.group3()[1]),
                -(self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group2()[0] * other.group0()[0])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[2] * other.group0()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[3] * other.group1()[3]),
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
            Simd32x3::from(other.group0()[3]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other.group0()[3]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group2()[0] * other.group0()[3],
                self.group2()[1] * other.group0()[3],
                self.group2()[2] * other.group0()[3],
                -(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group2()[2]) + (self.group2()[2] * other.group0()[1]) + (self.group1()[0] * other.group1()[3]) + (self.group1()[3] * other.group1()[0])
                    - (self.group2()[1] * other.group0()[2]),
                (self.group0()[2] * other.group2()[0]) + (self.group2()[0] * other.group0()[2]) + (self.group1()[1] * other.group1()[3]) + (self.group1()[3] * other.group1()[1])
                    - (self.group2()[2] * other.group0()[0]),
                (self.group0()[0] * other.group2()[1]) + (self.group2()[1] * other.group0()[0]) + (self.group1()[2] * other.group1()[3]) + (self.group1()[3] * other.group1()[2])
                    - (self.group2()[0] * other.group0()[1]),
                -(self.group2()[0] * other.group1()[0])
                    - (self.group2()[1] * other.group1()[1])
                    - (self.group2()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group2()[1])
                    - (self.group1()[2] * other.group2()[2]),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0)),
        );
    }
}
impl std::ops::Div<wedge> for DipoleInversion {
    type Output = wedge_partial<DipoleInversion>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       26        0
    //    simd3        0        1        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       21       31        0
    //  no simd       30       45        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group2()[3]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other.group2()[3]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other.group2()[3] * self.group2()[0],
                other.group2()[3] * self.group2()[1],
                other.group2()[3] * self.group2()[2],
                (other.group2()[3] * self.group2()[3])
                    - (other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group2()[2]) + (other.group1()[0] * self.group1()[3]) + (other.group1()[3] * self.group1()[0]) + (other.group2()[3] * self.group3()[0]),
                (other.group0()[2] * self.group2()[0]) + (other.group1()[1] * self.group1()[3]) + (other.group1()[3] * self.group1()[1]) + (other.group2()[3] * self.group3()[1]),
                (other.group0()[0] * self.group2()[1]) + (other.group1()[2] * self.group1()[3]) + (other.group1()[3] * self.group1()[2]) + (other.group2()[3] * self.group3()[2]),
                -(other.group1()[1] * self.group2()[1]) - (other.group1()[2] * self.group2()[2]) - (other.group2()[1] * self.group1()[1]) - (other.group2()[2] * self.group1()[2]),
            ]) + (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group3()[3]]) * crate::swizzle!(other.group2(), 2, 0, 1, 3))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiDipoleInversion> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       20        0
    //    simd3        2        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       21       27        0
    //  no simd       37       45        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from(other.group2()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([other.group3()[2], other.group3()[0], other.group3()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group3()[1], other.group3()[2], other.group3()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group3()[3]) + (other.group2()[3] * self.group2()[0]),
                (self.group0()[1] * other.group3()[3]) + (other.group2()[3] * self.group2()[1]),
                (self.group0()[2] * other.group3()[3]) + (other.group2()[3] * self.group2()[2]),
                -(other.group3()[1] * self.group1()[1]) - (other.group3()[2] * self.group1()[2]),
            ]) - (crate::swizzle!(other.group3(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (other.group2()[3] * self.group3()[3]) + (other.group3()[2] * self.group3()[2]) + (other.group3()[3] * self.group2()[3])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])
                    - (other.group1()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group3()[1]]) * crate::swizzle!(other.group3(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group0()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiDualNum> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group0()[1]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other.group0()[1]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other.group0()[1]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other.group0()[1]) * self.group3(),
        );
    }
}
impl Wedge<AntiFlatPoint> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (other.group0()[3] * self.group1()[3]),
        );
    }
}
impl Wedge<AntiFlector> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       10       22        0
    //  no simd       21       35        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group0()[0] * other.group1()[3],
                self.group0()[1] * other.group1()[3],
                self.group0()[2] * other.group1()[3],
                -(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2]),
            ]) - (crate::swizzle!(other.group1(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group1()[2] * self.group2()[1]) * -1.0,
                (other.group1()[0] * self.group2()[2]) * -1.0,
                (other.group1()[1] * self.group2()[0]) * -1.0,
                (other.group1()[2] * self.group3()[2]) + (other.group1()[3] * self.group2()[3])
                    - (self.group0()[0] * other.group0()[0])
                    - (self.group0()[1] * other.group0()[1])
                    - (self.group0()[2] * other.group0()[2])
                    - (other.group0()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group3()[1]]) * crate::swizzle!(other.group1(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiLine> for DipoleInversion {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       18        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] * self.group1()[3]) + (other.group1()[2] * self.group0()[1]) - (other.group1()[1] * self.group0()[2]),
                (other.group0()[1] * self.group1()[3]) + (other.group1()[0] * self.group0()[2]) - (other.group1()[2] * self.group0()[0]),
                (other.group0()[2] * self.group1()[3]) + (other.group1()[1] * self.group0()[0]) - (other.group1()[0] * self.group0()[1]),
                -(other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2]),
            ]),
            // e1234
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
        );
    }
}
impl Wedge<AntiMotor> for DipoleInversion {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        0        1        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       18       33        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group0()[3]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other.group0()[3]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other.group0()[3] * self.group2()[0],
                other.group0()[3] * self.group2()[1],
                other.group0()[3] * self.group2()[2],
                (other.group0()[3] * self.group2()[3]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group1()[2]) + (other.group0()[3] * self.group3()[0]),
                (self.group0()[2] * other.group1()[0]) + (other.group0()[3] * self.group3()[1]),
                (self.group0()[0] * other.group1()[1]) + (other.group0()[3] * self.group3()[2]),
                -(other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2]),
            ]) + (Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group3()[3]]) * other.group0())
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiPlane> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       13        0
    //    simd3        1        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        6       18        0
    //  no simd       17       31        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group0()[0] * other.group0()[3],
                self.group0()[1] * other.group0()[3],
                self.group0()[2] * other.group0()[3],
                -(other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]),
            ]) - (crate::swizzle!(other.group0(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group0()[2] * self.group2()[1]) * -1.0,
                (other.group0()[0] * self.group2()[2]) * -1.0,
                (other.group0()[1] * self.group2()[0]) * -1.0,
                (other.group0()[2] * self.group3()[2]) + (other.group0()[3] * self.group2()[3]),
            ]) + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group3()[1]]) * crate::swizzle!(other.group0(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<Circle> for DipoleInversion {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        9       10        0
    fn wedge(self, other: Circle) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(other.group0()[0] * self.group2()[0])
                - (other.group0()[1] * self.group2()[1])
                - (other.group0()[2] * self.group2()[2])
                - (other.group2()[0] * self.group0()[0])
                - (other.group2()[1] * self.group0()[1])
                - (other.group2()[2] * self.group0()[2])
                - (other.group1()[0] * self.group1()[0])
                - (other.group1()[1] * self.group1()[1])
                - (other.group1()[2] * self.group1()[2])
                - (other.group1()[3] * self.group1()[3]),
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
            -(other.group0()[0] * self.group2()[0])
                - (other.group0()[1] * self.group2()[1])
                - (other.group0()[2] * self.group2()[2])
                - (self.group0()[0] * other.group2()[0])
                - (self.group0()[1] * other.group2()[1])
                - (self.group0()[2] * other.group2()[2])
                - (other.group1()[0] * self.group1()[0])
                - (other.group1()[1] * self.group1()[1])
                - (other.group1()[2] * self.group1()[2])
                - (other.group1()[3] * self.group1()[3]),
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
            Simd32x4::from([
                (other.group0()[1] * self.group2()[2]) + (other.group2()[2] * self.group0()[1]) + (other.group1()[0] * self.group1()[3]) + (other.group1()[3] * self.group1()[0])
                    - (other.group2()[1] * self.group0()[2]),
                (other.group0()[2] * self.group2()[0]) + (other.group2()[0] * self.group0()[2]) + (other.group1()[1] * self.group1()[3]) + (other.group1()[3] * self.group1()[1])
                    - (other.group2()[2] * self.group0()[0]),
                (other.group0()[0] * self.group2()[1]) + (other.group2()[1] * self.group0()[0]) + (other.group1()[2] * self.group1()[3]) + (other.group1()[3] * self.group1()[2])
                    - (other.group2()[0] * self.group0()[1]),
                -(other.group2()[0] * self.group1()[0])
                    - (other.group2()[1] * self.group1()[1])
                    - (other.group2()[2] * self.group1()[2])
                    - (other.group1()[1] * self.group2()[1])
                    - (other.group1()[2] * self.group2()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0)),
            // e1234
            -(other.group0()[0] * self.group1()[0])
                - (other.group0()[1] * self.group1()[1])
                - (other.group0()[2] * self.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])
                - (self.group0()[2] * other.group1()[2]),
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
            Simd32x4::from([
                (other.group0()[1] * self.group2()[2]) + (self.group0()[1] * other.group2()[2]) + (other.group1()[0] * self.group1()[3]) + (other.group1()[3] * self.group1()[0]),
                (other.group0()[2] * self.group2()[0]) + (self.group0()[2] * other.group2()[0]) + (other.group1()[1] * self.group1()[3]) + (other.group1()[3] * self.group1()[1]),
                (other.group0()[0] * self.group2()[1]) + (self.group0()[0] * other.group2()[1]) + (other.group1()[2] * self.group1()[3]) + (other.group1()[3] * self.group1()[2]),
                -(other.group1()[1] * self.group2()[1]) - (other.group1()[2] * self.group2()[2]) - (other.group2()[1] * self.group1()[1]) - (other.group2()[2] * self.group1()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0)),
            // e1234
            -(other.group0()[0] * self.group1()[0])
                - (other.group0()[1] * self.group1()[1])
                - (other.group0()[2] * self.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])
                - (self.group0()[2] * other.group1()[2]),
        );
    }
}
impl Wedge<DualNum> for DipoleInversion {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0] * self.group2()[0], other.group0()[0] * self.group2()[1], other.group0()[0] * self.group2()[2], 0.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group3()[3]]),
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
            Simd32x4::from([
                (self.group0()[1] * other.group0()[2]) + (self.group1()[0] * other.group0()[3]),
                (self.group0()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3]),
                (self.group0()[0] * other.group0()[1]) + (self.group1()[2] * other.group0()[3]),
                -(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2]),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
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
            Simd32x4::from([
                (self.group0()[1] * other.group0()[2]) + (self.group1()[0] * other.group0()[3]),
                (self.group0()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3]),
                (self.group0()[0] * other.group0()[1]) + (self.group1()[2] * other.group0()[3]),
                -(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2]),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
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
            -(self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])
                - (self.group0()[2] * other.group1()[2])
                - (other.group0()[0] * self.group1()[0])
                - (other.group0()[1] * self.group1()[1])
                - (other.group0()[2] * self.group1()[2]),
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
                self.group0()[0] * other.group1()[3],
                self.group0()[1] * other.group1()[3],
                self.group0()[2] * other.group1()[3],
                (self.group2()[3] * other.group1()[3])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0] * other.group1()[3], self.group1()[1] * other.group1()[3], self.group1()[2] * other.group1()[3], 0.0]),
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
                (self.group2()[3] * other[e1])
                    + (self.group3()[0] * other.group1()[0])
                    + (self.group3()[1] * other.group1()[1])
                    + (self.group3()[2] * other.group1()[2])
                    + (self.group3()[3] * other.group1()[3])
                    - (self.group0()[0] * other.group8()[0])
                    - (self.group0()[1] * other.group8()[1])
                    - (self.group0()[2] * other.group8()[2])
                    - (other.group7()[0] * self.group2()[0])
                    - (other.group7()[1] * self.group2()[1])
                    - (other.group7()[2] * self.group2()[2])
                    - (self.group1()[0] * other.group6()[0])
                    - (self.group1()[1] * other.group6()[1])
                    - (self.group1()[2] * other.group6()[2])
                    - (self.group1()[3] * other.group6()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]),
            // e41, e42, e43
            Simd32x3::from(other.group0()[0]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other[e1]) + (self.group2()[0] * other.group1()[3]),
                (self.group0()[1] * other[e1]) + (self.group2()[1] * other.group1()[3]),
                (self.group0()[2] * other[e1]) + (self.group2()[2] * other.group1()[3]),
                -(self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2]),
            ]) - (crate::swizzle!(self.group1(), 3, 3, 3, 0) * crate::swizzle!(other.group1(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (self.group2()[2] * other.group1()[1]) - (self.group2()[1] * other.group1()[2]),
                (self.group2()[0] * other.group1()[2]) - (self.group2()[2] * other.group1()[0]),
                (self.group2()[1] * other.group1()[0]) - (self.group2()[0] * other.group1()[1]),
            ]) + (Simd32x3::from(other[e1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group3()[2]) + (other.group4()[1] * self.group2()[2]) + (other.group5()[0] * self.group1()[3]) + (self.group1()[0] * other.group3()[3]),
                (self.group0()[2] * other.group3()[0]) + (other.group4()[2] * self.group2()[0]) + (other.group5()[1] * self.group1()[3]) + (self.group1()[1] * other.group3()[3]),
                (self.group0()[0] * other.group3()[1]) + (other.group4()[0] * self.group2()[1]) + (other.group5()[2] * self.group1()[3]) + (self.group1()[2] * other.group3()[3]),
                -(other.group5()[1] * self.group2()[1]) - (other.group5()[2] * self.group2()[2]) - (self.group1()[1] * other.group3()[1]) - (self.group1()[2] * other.group3()[2]),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group3())
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group5()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0)),
            // e1234
            (other.group0()[0] * self.group2()[3])
                - (self.group0()[0] * other.group5()[0])
                - (self.group0()[1] * other.group5()[1])
                - (self.group0()[2] * other.group5()[2])
                - (other.group4()[0] * self.group1()[0])
                - (other.group4()[1] * self.group1()[1])
                - (other.group4()[2] * self.group1()[2]),
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
            (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other[e2]) + (self.group2()[0] * other.group0()[3]),
                (self.group0()[1] * other[e2]) + (self.group2()[1] * other.group0()[3]),
                (self.group0()[2] * other[e2]) + (self.group2()[2] * other.group0()[3]),
                -(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2]),
            ]) - (crate::swizzle!(self.group1(), 3, 3, 3, 0) * crate::swizzle!(other.group0(), 0, 1, 2, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (self.group2()[1] * other.group0()[2]) * -1.0,
                (self.group2()[2] * other.group0()[0]) * -1.0,
                (self.group2()[0] * other.group0()[1]) * -1.0,
                (self.group3()[1] * other.group0()[1]) + (self.group3()[2] * other.group0()[2]) + (self.group3()[3] * other.group0()[3]),
            ]) + (Simd32x4::from(other[e2]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]))
                + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
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
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group3(),
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
            (Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([other.group3()[2], other.group3()[0], other.group3()[1]]) * crate::swizzle!(self.group0(), 1, 2, 0))
                - (Simd32x3::from([other.group3()[1], other.group3()[2], other.group3()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group2()[3]) + (self.group2()[0] * other.group3()[3]),
                (self.group0()[1] * other.group2()[3]) + (self.group2()[1] * other.group3()[3]),
                (self.group0()[2] * other.group2()[3]) + (self.group2()[2] * other.group3()[3]),
                -(self.group1()[1] * other.group3()[1]) - (self.group1()[2] * other.group3()[2]),
            ]) - (crate::swizzle!(self.group1(), 3, 3, 3, 0) * crate::swizzle!(other.group3(), 0, 1, 2, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (self.group3()[1] * other.group3()[1]) + (self.group3()[2] * other.group3()[2]) + (self.group3()[3] * other.group3()[3])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group2()[3]]))
                + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group0()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0)),
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
            Simd32x3::from(other.group0()[3]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other.group0()[3]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group2()[0] * other.group0()[3],
                self.group2()[1] * other.group0()[3],
                self.group2()[2] * other.group0()[3],
                (self.group2()[3] * other.group0()[3])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group2()[2]) + (self.group1()[0] * other.group1()[3]) + (self.group1()[3] * other.group1()[0]) + (self.group3()[0] * other.group0()[3]),
                (self.group0()[2] * other.group2()[0]) + (self.group1()[1] * other.group1()[3]) + (self.group1()[3] * other.group1()[1]) + (self.group3()[1] * other.group0()[3]),
                (self.group0()[0] * other.group2()[1]) + (self.group1()[2] * other.group1()[3]) + (self.group1()[3] * other.group1()[2]) + (self.group3()[2] * other.group0()[3]),
                -(self.group1()[1] * other.group2()[1]) - (self.group1()[2] * other.group2()[2]) - (self.group2()[1] * other.group1()[1]) - (self.group2()[2] * other.group1()[2]),
            ]) + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[3]]) * crate::swizzle!(other.group0(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0)),
        );
    }
}
impl std::ops::Div<wedge> for DualNum {
    type Output = wedge_partial<DualNum>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self.group0()[0] * other.group1()[0],
                self.group0()[0] * other.group1()[1],
                self.group0()[0] * other.group1()[2],
                self.group0()[1] * other.group2()[3],
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0] * other.group2()[0], self.group0()[0] * other.group2()[1], self.group0()[0] * other.group2()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group2()[3]]),
        );
    }
}
impl Wedge<AntiDipoleInversion> for DualNum {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group3()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group1()[3]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0] * other.group2()[0], self.group0()[0] * other.group2()[1], self.group0()[0] * other.group2()[2], 0.0]),
        );
    }
}
impl Wedge<AntiDualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(other.group0()[1]) * self.group0());
    }
}
impl Wedge<AntiFlatPoint> for DualNum {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0] * other.group0()[0], self.group0()[0] * other.group0()[1], self.group0()[0] * other.group0()[2], 0.0]),
            // e1234
            self.group0()[0] * other.group0()[3],
        );
    }
}
impl Wedge<AntiFlector> for DualNum {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group1()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group0()[3]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0] * other.group0()[0], self.group0()[0] * other.group0()[1], self.group0()[0] * other.group0()[2], 0.0]),
        );
    }
}
impl Wedge<AntiLine> for DualNum {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self.group0()[0]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0] * other.group1()[0], self.group0()[0] * other.group1()[1], self.group0()[0] * other.group1()[2], 0.0]),
            // e235, e315, e125
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<AntiMotor> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        9        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self.group0()[0] * other.group0()[0],
                self.group0()[0] * other.group0()[1],
                self.group0()[0] * other.group0()[2],
                (self.group0()[0] * other.group1()[3]) + (self.group0()[1] * other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0] * other.group1()[0], self.group0()[0] * other.group1()[1], self.group0()[0] * other.group1()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group0()[3]]),
        );
    }
}
impl Wedge<AntiPlane> for DualNum {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<Circle> for DualNum {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn wedge(self, other: Circle) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0] * other.group2()[0], self.group0()[0] * other.group2()[1], self.group0()[0] * other.group2()[2], 0.0]),
            // e1234
            self.group0()[0] * other.group1()[3],
        );
    }
}
impl Wedge<CircleRotor> for DualNum {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn wedge(self, other: CircleRotor) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0] * other.group2()[0], self.group0()[0] * other.group2()[1], self.group0()[0] * other.group2()[2], 0.0]),
            // e1234
            self.group0()[0] * other.group1()[3],
        );
    }
}
impl Wedge<Dipole> for DualNum {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0] * other.group2()[0], self.group0()[0] * other.group2()[1], self.group0()[0] * other.group2()[2], 0.0]),
            // e235, e315, e125
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<DipoleInversion> for DualNum {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn wedge(self, other: DipoleInversion) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0] * other.group2()[0], self.group0()[0] * other.group2()[1], self.group0()[0] * other.group2()[2], 0.0]),
            // e235, e315, e125, e12345
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group3()[3]]),
        );
    }
}
impl Wedge<FlatPoint> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn wedge(self, other: FlatPoint) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
        );
    }
}
impl Wedge<Flector> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}
impl Wedge<Line> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn wedge(self, other: Line) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            self.group0()[0] * other.group1()[0],
            self.group0()[0] * other.group1()[1],
            self.group0()[0] * other.group1()[2],
            0.0,
        ]));
    }
}
impl Wedge<Motor> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn wedge(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group1()[3]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0] * other.group1()[0], self.group0()[0] * other.group1()[1], self.group0()[0] * other.group1()[2], 0.0]),
        );
    }
}
impl Wedge<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       11        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        1       13        0
    //  no simd        1       17        0
    fn wedge(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (self.group0()[0] * other.group9()[3]) + (self.group0()[1] * other.group0()[0])]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group0()[0]]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other[e1]]),
            // e41, e42, e43
            Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0] * other.group3()[0], self.group0()[0] * other.group3()[1], self.group0()[0] * other.group3()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(self.group0()[0]) * other.group5(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0] * other.group8()[0], self.group0()[0] * other.group8()[1], self.group0()[0] * other.group8()[2], 0.0]),
            // e1234
            self.group0()[0] * other.group6()[3],
        );
    }
}
impl Wedge<Plane> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Plane) -> Self::Output {
        return AntiScalar::from_groups(/* e12345 */ self.group0()[0] * other.group0()[3]);
    }
}
impl Wedge<RoundPoint> for DualNum {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn wedge(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other[e2]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
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
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(other[scalar]) * self.group0());
    }
}
impl Wedge<Sphere> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Sphere) -> Self::Output {
        return AntiScalar::from_groups(/* e12345 */ self.group0()[0] * other.group0()[3]);
    }
}
impl Wedge<VersorEven> for DualNum {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn wedge(self, other: VersorEven) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group2()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group1()[3]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0] * other.group2()[0], self.group0()[0] * other.group2()[1], self.group0()[0] * other.group2()[2], 0.0]),
        );
    }
}
impl Wedge<VersorOdd> for DualNum {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        9        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                self.group0()[0] * other.group1()[0],
                self.group0()[0] * other.group1()[1],
                self.group0()[0] * other.group1()[2],
                (self.group0()[0] * other.group3()[3]) + (self.group0()[1] * other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0] * other.group2()[0], self.group0()[0] * other.group2()[1], self.group0()[0] * other.group2()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group0()[3]]),
        );
    }
}
impl std::ops::Div<wedge> for FlatPoint {
    type Output = wedge_partial<FlatPoint>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for FlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        8       16        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other.group2()[3]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group0()[2]) + (other.group1()[0] * self.group0()[3]),
                (other.group0()[2] * self.group0()[0]) + (other.group1()[1] * self.group0()[3]),
                (other.group0()[0] * self.group0()[1]) + (other.group1()[2] * self.group0()[3]),
                -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiDipoleInversion> for FlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        9       16        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                other.group2()[3] * self.group0()[0],
                other.group2()[3] * self.group0()[1],
                other.group2()[3] * self.group0()[2],
                -(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group1()[3] * self.group0()[3]),
            ]) - (Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[0]]) * crate::swizzle!(self.group0(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group3()[1] * self.group0()[2]) - (other.group3()[2] * self.group0()[1]),
                (other.group3()[2] * self.group0()[0]) - (other.group3()[0] * self.group0()[2]),
                (other.group3()[0] * self.group0()[1]) - (other.group3()[1] * self.group0()[0]),
                0.0,
            ]),
        );
    }
}
impl Wedge<AntiDualNum> for FlatPoint {
    type Output = FlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other.group0()[1]) * self.group0());
    }
}
impl Wedge<AntiFlatPoint> for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        return AntiScalar::from_groups(/* e12345 */ other.group0()[3] * self.group0()[3] * -1.0);
    }
}
impl Wedge<AntiFlector> for FlatPoint {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       14        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) * Simd32x4::from(-1.0),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[1] * self.group0()[2]) - (other.group1()[2] * self.group0()[1]),
                (other.group1()[2] * self.group0()[0]) - (other.group1()[0] * self.group0()[2]),
                (other.group1()[0] * self.group0()[1]) - (other.group1()[1] * self.group0()[0]),
                0.0,
            ]),
        );
    }
}
impl Wedge<AntiLine> for FlatPoint {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            other.group0()[0] * self.group0()[3],
            other.group0()[1] * self.group0()[3],
            other.group0()[2] * self.group0()[3],
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
        ]));
    }
}
impl Wedge<AntiMotor> for FlatPoint {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       10        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other.group0()[3]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[0] * self.group0()[3],
                other.group0()[1] * self.group0()[3],
                other.group0()[2] * self.group0()[3],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]),
        );
    }
}
impl Wedge<AntiPlane> for FlatPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       12        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from([
                (other.group0()[1] * self.group0()[2]) - (other.group0()[2] * self.group0()[1]),
                (other.group0()[2] * self.group0()[0]) - (other.group0()[0] * self.group0()[2]),
                (other.group0()[0] * self.group0()[1]) - (other.group0()[1] * self.group0()[0]),
            ]),
        );
    }
}
impl Wedge<Circle> for FlatPoint {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Circle) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group1()[3] * self.group0()[3]),
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
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group1()[3] * self.group0()[3]),
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
            Simd32x4::from([
                (other.group0()[1] * self.group0()[2]) + (other.group1()[0] * self.group0()[3]),
                (other.group0()[2] * self.group0()[0]) + (other.group1()[1] * self.group0()[3]),
                (other.group0()[0] * self.group0()[1]) + (other.group1()[2] * self.group0()[3]),
                -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
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
            Simd32x4::from([
                (other.group0()[1] * self.group0()[2]) + (other.group1()[0] * self.group0()[3]),
                (other.group0()[2] * self.group0()[0]) + (other.group1()[1] * self.group0()[3]),
                (other.group0()[0] * self.group0()[1]) + (other.group1()[2] * self.group0()[3]),
                -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<DualNum> for FlatPoint {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
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
                -(other.group7()[0] * self.group0()[0]) - (other.group7()[1] * self.group0()[1]) - (other.group7()[2] * self.group0()[2]) - (self.group0()[3] * other.group6()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(other.group0()[0]) * self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group1()[3]) - (self.group0()[3] * other.group1()[0]),
                (self.group0()[1] * other.group1()[3]) - (self.group0()[3] * other.group1()[1]),
                (self.group0()[2] * other.group1()[3]) - (self.group0()[3] * other.group1()[2]),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([
                (self.group0()[2] * other.group1()[1]) - (self.group0()[1] * other.group1()[2]),
                (self.group0()[0] * other.group1()[2]) - (self.group0()[2] * other.group1()[0]),
                (self.group0()[1] * other.group1()[0]) - (self.group0()[0] * other.group1()[1]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group4()[1] * self.group0()[2]) + (other.group5()[0] * self.group0()[3]),
                (other.group4()[2] * self.group0()[0]) + (other.group5()[1] * self.group0()[3]),
                (other.group4()[0] * self.group0()[1]) + (other.group5()[2] * self.group0()[3]),
                -(other.group5()[1] * self.group0()[1]) - (other.group5()[2] * self.group0()[2]),
            ]) - (Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group5()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
            // e1234
            0.0,
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
            (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            Simd32x3::from([
                (self.group0()[2] * other.group0()[1]) - (self.group0()[1] * other.group0()[2]),
                (self.group0()[0] * other.group0()[2]) - (self.group0()[2] * other.group0()[0]),
                (self.group0()[1] * other.group0()[0]) - (self.group0()[0] * other.group0()[1]),
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
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(other[scalar]) * self.group0());
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
            Simd32x4::from([
                self.group0()[0] * other.group3()[3],
                self.group0()[1] * other.group3()[3],
                self.group0()[2] * other.group3()[3],
                -(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[3] * other.group1()[3]),
            ]) - (Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[0]]) * crate::swizzle!(self.group0(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[2] * other.group3()[1]) - (self.group0()[1] * other.group3()[2]),
                (self.group0()[0] * other.group3()[2]) - (self.group0()[2] * other.group3()[0]),
                (self.group0()[1] * other.group3()[0]) - (self.group0()[0] * other.group3()[1]),
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
            Simd32x4::from(other.group0()[3]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[2] * other.group0()[1]) + (self.group0()[3] * other.group1()[0]),
                (self.group0()[0] * other.group0()[2]) + (self.group0()[3] * other.group1()[1]),
                (self.group0()[1] * other.group0()[0]) + (self.group0()[3] * other.group1()[2]),
                -(self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl std::ops::Div<wedge> for Flector {
    type Output = wedge_partial<Flector>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        6       11        0
    //  no simd       12       20        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other.group2()[3]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group0()[2]) + (other.group1()[0] * self.group0()[3]),
                (other.group0()[2] * self.group0()[0]) + (other.group1()[1] * self.group0()[3]),
                (other.group0()[0] * self.group0()[1]) + (other.group1()[2] * self.group0()[3]),
                -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(other.group2()[3]) * self.group1())
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiDipoleInversion> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       14        0
    //  no simd       16       20        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (other.group3()[0] * self.group1()[0]) + (other.group3()[1] * self.group1()[1]) + (other.group3()[2] * self.group1()[2])
                    - (other.group0()[1] * self.group0()[1])
                    - (other.group0()[2] * self.group0()[2])
                    - (other.group1()[3] * self.group0()[3]),
            ]) + (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                - (Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[0]]) * crate::swizzle!(self.group0(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group3()[1] * self.group0()[2]) - (other.group3()[2] * self.group0()[1]),
                (other.group3()[2] * self.group0()[0]) - (other.group3()[0] * self.group0()[2]),
                (other.group3()[0] * self.group0()[1]) - (other.group3()[1] * self.group0()[0]),
                0.0,
            ]),
        );
    }
}
impl Wedge<AntiDualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other.group0()[1]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other.group0()[1]) * self.group1(),
        );
    }
}
impl Wedge<AntiFlatPoint> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        return AntiScalar::from_groups(/* e12345 */ other.group0()[3] * self.group0()[3] * -1.0);
    }
}
impl Wedge<AntiFlector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       16        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                other.group1()[0] * self.group0()[3] * -1.0,
                other.group1()[1] * self.group0()[3] * -1.0,
                other.group1()[2] * self.group0()[3] * -1.0,
                (other.group1()[0] * self.group1()[0]) + (other.group1()[1] * self.group1()[1]) + (other.group1()[2] * self.group1()[2]) - (other.group0()[3] * self.group0()[3]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[1] * self.group0()[2]) - (other.group1()[2] * self.group0()[1]),
                (other.group1()[2] * self.group0()[0]) - (other.group1()[0] * self.group0()[2]),
                (other.group1()[0] * self.group0()[1]) - (other.group1()[1] * self.group0()[0]),
                0.0,
            ]),
        );
    }
}
impl Wedge<AntiLine> for Flector {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            other.group0()[0] * self.group0()[3],
            other.group0()[1] * self.group0()[3],
            other.group0()[2] * self.group0()[3],
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
        ]));
    }
}
impl Wedge<AntiMotor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        6       14        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(other.group0()[3]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[3] * self.group1()[0],
                other.group0()[3] * self.group1()[1],
                other.group0()[3] * self.group1()[2],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]) + (Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[3]]) * other.group0()),
        );
    }
}
impl Wedge<AntiPlane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       15        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                other.group0()[0] * self.group0()[3] * -1.0,
                other.group0()[1] * self.group0()[3] * -1.0,
                other.group0()[2] * self.group0()[3] * -1.0,
                (other.group0()[0] * self.group1()[0]) + (other.group0()[1] * self.group1()[1]) + (other.group0()[2] * self.group1()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[1] * self.group0()[2]) - (other.group0()[2] * self.group0()[1]),
                (other.group0()[2] * self.group0()[0]) - (other.group0()[0] * self.group0()[2]),
                (other.group0()[0] * self.group0()[1]) - (other.group0()[1] * self.group0()[0]),
                0.0,
            ]),
        );
    }
}
impl Wedge<Circle> for Flector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: Circle) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group1()[3] * self.group0()[3]),
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
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group1()[3] * self.group0()[3]),
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
            Simd32x4::from([
                (other.group0()[1] * self.group0()[2]) + (other.group1()[0] * self.group0()[3]),
                (other.group0()[2] * self.group0()[0]) + (other.group1()[1] * self.group0()[3]),
                (other.group0()[0] * self.group0()[1]) + (other.group1()[2] * self.group0()[3]),
                -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
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
            Simd32x4::from([
                (other.group0()[1] * self.group0()[2]) + (other.group1()[0] * self.group0()[3]),
                (other.group0()[2] * self.group0()[0]) + (other.group1()[1] * self.group0()[3]),
                (other.group0()[0] * self.group0()[1]) + (other.group1()[2] * self.group0()[3]),
                -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<DualNum> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
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
                (self.group1()[0] * other.group1()[0]) + (self.group1()[1] * other.group1()[1]) + (self.group1()[2] * other.group1()[2]) + (self.group1()[3] * other.group1()[3])
                    - (other.group7()[0] * self.group0()[0])
                    - (other.group7()[1] * self.group0()[1])
                    - (other.group7()[2] * self.group0()[2])
                    - (self.group0()[3] * other.group6()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(other.group0()[0]) * self.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group1()[3]) - (self.group0()[3] * other.group1()[0]),
                (self.group0()[1] * other.group1()[3]) - (self.group0()[3] * other.group1()[1]),
                (self.group0()[2] * other.group1()[3]) - (self.group0()[3] * other.group1()[2]),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([
                (self.group0()[2] * other.group1()[1]) - (self.group0()[1] * other.group1()[2]),
                (self.group0()[0] * other.group1()[2]) - (self.group0()[2] * other.group1()[0]),
                (self.group0()[1] * other.group1()[0]) - (self.group0()[0] * other.group1()[1]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group4()[1] * self.group0()[2]) + (other.group5()[0] * self.group0()[3]),
                (other.group4()[2] * self.group0()[0]) + (other.group5()[1] * self.group0()[3]),
                (other.group4()[0] * self.group0()[1]) + (other.group5()[2] * self.group0()[3]),
                -(other.group5()[1] * self.group0()[1]) - (other.group5()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group1())
                - (Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group5()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
            // e1234
            0.0,
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
            Simd32x4::from([
                (self.group0()[3] * other.group0()[0]) * -1.0,
                (self.group0()[3] * other.group0()[1]) * -1.0,
                (self.group0()[3] * other.group0()[2]) * -1.0,
                (self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2]) + (self.group1()[3] * other.group0()[3]),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[0]]) * crate::swizzle!(other.group0(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[2] * other.group0()[1]) - (self.group0()[1] * other.group0()[2]),
                (self.group0()[0] * other.group0()[2]) - (self.group0()[2] * other.group0()[0]),
                (self.group0()[1] * other.group0()[0]) - (self.group0()[0] * other.group0()[1]),
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
            Simd32x4::from(other[scalar]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group1(),
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (self.group1()[1] * other.group3()[1]) + (self.group1()[2] * other.group3()[2]) + (self.group1()[3] * other.group3()[3])
                    - (self.group0()[1] * other.group0()[1])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group0()[3] * other.group1()[3]),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[0]]) * crate::swizzle!(other.group3(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[0]]) * crate::swizzle!(self.group0(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[2] * other.group3()[1]) - (self.group0()[1] * other.group3()[2]),
                (self.group0()[0] * other.group3()[2]) - (self.group0()[2] * other.group3()[0]),
                (self.group0()[1] * other.group3()[0]) - (self.group0()[0] * other.group3()[1]),
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
            Simd32x4::from(other.group0()[3]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[3] * other.group1()[0]) + (self.group1()[0] * other.group0()[3]),
                (self.group0()[3] * other.group1()[1]) + (self.group1()[1] * other.group0()[3]),
                (self.group0()[3] * other.group1()[2]) + (self.group1()[2] * other.group0()[3]),
                -(self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[3]]) * crate::swizzle!(other.group0(), 1, 2, 0, 3))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl std::ops::Div<wedge> for Line {
    type Output = wedge_partial<Line>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self.group0()[0] * other.group2()[3],
                self.group0()[1] * other.group2()[3],
                self.group0()[2] * other.group2()[3],
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0] * other.group2()[3], self.group1()[1] * other.group2()[3], self.group1()[2] * other.group2()[3], 0.0]),
        );
    }
}
impl Wedge<AntiDipoleInversion> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self.group0()[1] * other.group3()[2]) - (self.group1()[0] * other.group2()[3]),
                -(self.group0()[2] * other.group3()[0]) - (self.group1()[1] * other.group2()[3]),
                -(self.group0()[0] * other.group3()[1]) - (self.group1()[2] * other.group2()[3]),
                (self.group1()[1] * other.group3()[1]) + (self.group1()[2] * other.group3()[2]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiDualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            Simd32x3::from(other.group0()[1]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other.group0()[1]) * self.group1(),
        );
    }
}
impl Wedge<AntiFlector> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        5       12        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group1()[2]) * -1.0,
                (self.group0()[2] * other.group1()[0]) * -1.0,
                (self.group0()[0] * other.group1()[1]) * -1.0,
                (self.group1()[1] * other.group1()[1]) + (self.group1()[2] * other.group1()[2]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiLine> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
        );
    }
}
impl Wedge<AntiMotor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        9        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                self.group0()[0] * other.group0()[3],
                self.group0()[1] * other.group0()[3],
                self.group0()[2] * other.group0()[3],
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0] * other.group0()[3], self.group1()[1] * other.group0()[3], self.group1()[2] * other.group0()[3], 0.0]),
        );
    }
}
impl Wedge<AntiPlane> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        5       12        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[1] * other.group0()[2]) * -1.0,
                (self.group0()[2] * other.group0()[0]) * -1.0,
                (self.group0()[0] * other.group0()[1]) * -1.0,
                (self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<Dipole> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn wedge(self, other: Dipole) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            -(other.group0()[0] * self.group1()[0])
                - (other.group0()[1] * self.group1()[1])
                - (other.group0()[2] * self.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])
                - (self.group0()[2] * other.group1()[2]),
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
            -(other.group0()[0] * self.group1()[0])
                - (other.group0()[1] * self.group1()[1])
                - (other.group0()[2] * self.group1()[2])
                - (self.group0()[0] * other.group1()[0])
                - (self.group0()[1] * other.group1()[1])
                - (self.group0()[2] * other.group1()[2]),
        );
    }
}
impl Wedge<DualNum> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([
            other.group0()[0] * self.group1()[0] * -1.0,
            other.group0()[0] * self.group1()[1] * -1.0,
            other.group0()[0] * self.group1()[2] * -1.0,
            0.0,
        ]));
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
                -(self.group0()[0] * other.group5()[0])
                    - (self.group0()[1] * other.group5()[1])
                    - (self.group0()[2] * other.group5()[2])
                    - (self.group1()[0] * other.group4()[0])
                    - (self.group1()[1] * other.group4()[1])
                    - (self.group1()[2] * other.group4()[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0] * self.group0()[0], other.group0()[0] * self.group0()[1], other.group0()[0] * self.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(other.group0()[0]) * self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self.group0()[1] * other.group1()[2]) - (self.group1()[0] * other.group1()[3]),
                -(self.group0()[2] * other.group1()[0]) - (self.group1()[1] * other.group1()[3]),
                -(self.group0()[0] * other.group1()[1]) - (self.group1()[2] * other.group1()[3]),
                (self.group1()[1] * other.group1()[1]) + (self.group1()[2] * other.group1()[2]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
            // e1234
            0.0,
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
            Simd32x4::from([
                -(self.group0()[1] * other.group0()[2]) - (self.group1()[0] * other.group0()[3]),
                -(self.group0()[2] * other.group0()[0]) - (self.group1()[1] * other.group0()[3]),
                -(self.group0()[0] * other.group0()[1]) - (self.group1()[2] * other.group0()[3]),
                (self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
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
            Simd32x3::from(other[scalar]) * self.group0(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group1(),
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
            Simd32x4::from([
                -(self.group0()[1] * other.group3()[2]) - (self.group1()[0] * other.group3()[3]),
                -(self.group0()[2] * other.group3()[0]) - (self.group1()[1] * other.group3()[3]),
                -(self.group0()[0] * other.group3()[1]) - (self.group1()[2] * other.group3()[3]),
                (self.group1()[1] * other.group3()[1]) + (self.group1()[2] * other.group3()[2]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0)),
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
                self.group0()[0] * other.group0()[3],
                self.group0()[1] * other.group0()[3],
                self.group0()[2] * other.group0()[3],
                -(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([self.group1()[0] * other.group0()[3], self.group1()[1] * other.group0()[3], self.group1()[2] * other.group0()[3], 0.0]),
        );
    }
}
impl std::ops::Div<wedge> for Motor {
    type Output = wedge_partial<Motor>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       12       20        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                other.group0()[0] * self.group1()[3],
                other.group0()[1] * self.group1()[3],
                other.group0()[2] * self.group1()[3],
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(other.group2()[3]) * self.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] * self.group1()[3]) + (other.group2()[3] * self.group1()[0]),
                (other.group1()[1] * self.group1()[3]) + (other.group2()[3] * self.group1()[1]),
                (other.group1()[2] * self.group1()[3]) + (other.group2()[3] * self.group1()[2]),
                other.group2()[3] * self.group1()[3],
            ]),
        );
    }
}
impl Wedge<AntiDipoleInversion> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]) * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(other.group2()[3] * self.group1()[0]) - (other.group3()[2] * self.group0()[1]),
                -(other.group2()[3] * self.group1()[1]) - (other.group3()[0] * self.group0()[2]),
                -(other.group2()[3] * self.group1()[2]) - (other.group3()[1] * self.group0()[0]),
                (other.group3()[1] * self.group1()[1]) + (other.group3()[2] * self.group1()[2]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0))
                - (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
        );
    }
}
impl Wedge<AntiDualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                other.group0()[1] * self.group0()[0],
                other.group0()[1] * self.group0()[1],
                other.group0()[1] * self.group0()[2],
                (other.group0()[0] * self.group1()[3]) + (other.group0()[1] * self.group0()[3]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from(other.group0()[1]) * self.group1(),
        );
    }
}
impl Wedge<AntiFlatPoint> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3] * self.group1()[3] * -1.0]));
    }
}
impl Wedge<AntiFlector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       16        0
    //  no simd        6       19        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                other.group1()[0] * self.group1()[3] * -1.0,
                other.group1()[1] * self.group1()[3] * -1.0,
                other.group1()[2] * self.group1()[3] * -1.0,
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[2] * self.group0()[1]) * -1.0,
                (other.group1()[0] * self.group0()[2]) * -1.0,
                (other.group1()[1] * self.group0()[0]) * -1.0,
                (other.group1()[1] * self.group1()[1]) + (other.group1()[2] * self.group1()[2]) - (other.group0()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiLine> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group0()[0] * self.group1()[3], other.group0()[1] * self.group1()[3], other.group0()[2] * self.group1()[3], 0.0]),
        );
    }
}
impl Wedge<AntiMotor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       14        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([
                other.group0()[3] * self.group0()[0],
                other.group0()[3] * self.group0()[1],
                other.group0()[3] * self.group0()[2],
                (other.group0()[3] * self.group0()[3]) - (other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[0] * self.group1()[3]) + (other.group0()[3] * self.group1()[0]),
                (other.group0()[1] * self.group1()[3]) + (other.group0()[3] * self.group1()[1]),
                (other.group0()[2] * self.group1()[3]) + (other.group0()[3] * self.group1()[2]),
                other.group0()[3] * self.group1()[3],
            ]),
        );
    }
}
impl Wedge<AntiPlane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       14        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2       15        0
    //  no simd        5       18        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([
                other.group0()[0] * self.group1()[3] * -1.0,
                other.group0()[1] * self.group1()[3] * -1.0,
                other.group0()[2] * self.group1()[3] * -1.0,
                0.0,
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[2] * self.group0()[1]) * -1.0,
                (other.group0()[0] * self.group0()[2]) * -1.0,
                (other.group0()[1] * self.group0()[0]) * -1.0,
                (other.group0()[1] * self.group1()[1]) + (other.group0()[2] * self.group1()[2]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<Circle> for Motor {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: Circle) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0),
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
            Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0),
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
                other.group0()[0] * self.group1()[3],
                other.group0()[1] * self.group1()[3],
                other.group0()[2] * self.group1()[3],
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0] * self.group1()[3], other.group1()[1] * self.group1()[3], other.group1()[2] * self.group1()[3], 0.0]),
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
                other.group0()[0] * self.group1()[3],
                other.group0()[1] * self.group1()[3],
                other.group0()[2] * self.group1()[3],
                (other.group2()[3] * self.group1()[3])
                    - (other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0] * self.group1()[3], other.group1()[1] * self.group1()[3], other.group1()[2] * self.group1()[3], 0.0]),
        );
    }
}
impl Wedge<DualNum> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group1()[3] * -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[0] * self.group1()[0] * -1.0,
                other.group0()[0] * self.group1()[1] * -1.0,
                other.group0()[0] * self.group1()[2] * -1.0,
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
                (other.group0()[0] * self.group0()[3]) + (self.group1()[3] * other[e45])
                    - (other.group4()[0] * self.group1()[0])
                    - (other.group4()[1] * self.group1()[1])
                    - (other.group4()[2] * self.group1()[2])
                    - (other.group5()[0] * self.group0()[0])
                    - (other.group5()[1] * self.group0()[1])
                    - (other.group5()[2] * self.group0()[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group0()[0] * self.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from(self.group1()[3]) * other.group1() * Simd32x4::from(-1.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self.group0()[0]) + (other.group4()[0] * self.group1()[3]),
                (other.group0()[0] * self.group0()[1]) + (other.group4()[1] * self.group1()[3]),
                (other.group0()[0] * self.group0()[2]) + (other.group4()[2] * self.group1()[3]),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])) + (Simd32x3::from(self.group1()[3]) * other.group5()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self.group0()[1] * other.group1()[2]) - (self.group1()[0] * other.group1()[3]),
                -(self.group0()[2] * other.group1()[0]) - (self.group1()[1] * other.group1()[3]),
                -(self.group0()[0] * other.group1()[1]) - (self.group1()[2] * other.group1()[3]),
                (self.group1()[1] * other.group1()[1]) + (self.group1()[2] * other.group1()[2]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0))
                - (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], other.group6()[3]])),
            // e1234
            0.0,
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
            Simd32x4::from(self.group1()[3]) * other.group0() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self.group0()[1] * other.group0()[2]) - (self.group1()[0] * other.group0()[3]),
                -(self.group0()[2] * other.group0()[0]) - (self.group1()[1] * other.group0()[3]),
                -(self.group0()[0] * other.group0()[1]) - (self.group1()[2] * other.group0()[3]),
                (self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
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
            Simd32x4::from(other[scalar]) * self.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * self.group1(),
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
        return AntiScalar::from_groups(/* e12345 */ self.group1()[3] * other[e4315]);
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
            Simd32x4::from(self.group1()[3]) * other.group3() * Simd32x4::from(-1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self.group0()[1] * other.group3()[2]) - (self.group1()[3] * other.group0()[0]),
                -(self.group0()[2] * other.group3()[0]) - (self.group1()[3] * other.group0()[1]),
                -(self.group0()[0] * other.group3()[1]) - (self.group1()[3] * other.group0()[2]),
                (self.group1()[1] * other.group3()[1]) + (self.group1()[2] * other.group3()[2]),
            ]) + (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group1()[3]]) * self.group1()),
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]))
                + (Simd32x4::from(other.group0()[3]) * self.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group1()[0] * other.group0()[3]) + (self.group1()[3] * other.group1()[0]),
                (self.group1()[1] * other.group0()[3]) + (self.group1()[3] * other.group1()[1]),
                (self.group1()[2] * other.group0()[3]) + (self.group1()[3] * other.group1()[2]),
                self.group1()[3] * other.group0()[3],
            ]),
        );
    }
}
impl std::ops::Div<wedge> for MultiVector {
    type Output = wedge_partial<MultiVector>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       50        0
    //    simd3        7       10        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       48       68        0
    //  no simd       80      112        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self.group0()[0] * other.group2()[3],
                (self.group0()[1] * other.group2()[3])
                    - (other.group0()[0] * self.group8()[0])
                    - (other.group0()[1] * self.group8()[1])
                    - (other.group0()[2] * self.group8()[2])
                    - (self.group7()[0] * other.group2()[0])
                    - (self.group7()[1] * other.group2()[1])
                    - (self.group7()[2] * other.group2()[2])
                    - (other.group1()[0] * self.group6()[0])
                    - (other.group1()[1] * self.group6()[1])
                    - (other.group1()[2] * self.group6()[2])
                    - (other.group1()[3] * self.group6()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group2()[3]) * self.group1(),
            // e5
            other.group2()[3] * self[e1],
            // e15, e25, e35, e45
            (Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                + (Simd32x4::from(other.group2()[3]) * self.group3()),
            // e41, e42, e43
            (Simd32x3::from(self.group0()[0]) * other.group0()) + (Simd32x3::from(other.group2()[3]) * self.group4()),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])) + (Simd32x3::from(other.group2()[3]) * self.group5()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self[e1]) + (other.group2()[3] * self.group6()[0]),
                (other.group0()[1] * self[e1]) + (other.group2()[3] * self.group6()[1]),
                (other.group0()[2] * self[e1]) + (other.group2()[3] * self.group6()[2]),
                -(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2]),
            ]) + (Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group6()[3]]) * other.group2())
                - (crate::swizzle!(other.group1(), 3, 3, 3, 0) * crate::swizzle!(self.group1(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from(other.group2()[3]) * self.group7())
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (other.group2()[2] * self.group1()[1]) - (other.group2()[1] * self.group1()[2]),
                (other.group2()[0] * self.group1()[2]) - (other.group2()[2] * self.group1()[0]),
                (other.group2()[1] * self.group1()[0]) - (other.group2()[0] * self.group1()[1]),
            ]) + (Simd32x3::from(other.group2()[3]) * self.group8())
                + (Simd32x3::from(self[e1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group3()[2]) + (self.group5()[0] * other.group1()[3]) + (other.group1()[0] * self.group3()[3]) + (other.group2()[3] * self.group9()[0]),
                (other.group0()[2] * self.group3()[0]) + (self.group5()[1] * other.group1()[3]) + (other.group1()[1] * self.group3()[3]) + (other.group2()[3] * self.group9()[1]),
                (other.group0()[0] * self.group3()[1]) + (self.group5()[2] * other.group1()[3]) + (other.group1()[2] * self.group3()[3]) + (other.group2()[3] * self.group9()[2]),
                -(self.group5()[1] * other.group2()[1]) - (self.group5()[2] * other.group2()[2]) - (other.group1()[1] * self.group3()[1]) - (other.group1()[2] * self.group3()[2]),
            ]) + (Simd32x4::from([self.group4()[1], self.group4()[2], self.group4()[0], self.group9()[3]]) * crate::swizzle!(other.group2(), 2, 0, 1, 3))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group5()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0)),
            // e1234
            (other.group2()[3] * self[e45])
                - (other.group0()[0] * self.group5()[0])
                - (other.group0()[1] * self.group5()[1])
                - (other.group0()[2] * self.group5()[2])
                - (self.group4()[0] * other.group1()[0])
                - (self.group4()[1] * other.group1()[1])
                - (self.group4()[2] * other.group1()[2]),
        );
    }
}
impl Wedge<AntiDipoleInversion> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       39       56        0
    //    simd3        6        8        0
    //    simd4        8       10        0
    // Totals...
    // yes simd       53       74        0
    //  no simd       89      120        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (other.group2()[3] * self.group9()[3])
                    + (other.group3()[0] * self.group9()[0])
                    + (other.group3()[1] * self.group9()[1])
                    + (other.group3()[2] * self.group9()[2])
                    + (other.group3()[3] * self[e45])
                    - (other.group0()[0] * self.group3()[0])
                    - (other.group0()[1] * self.group3()[1])
                    - (other.group0()[2] * self.group3()[2])
                    - (self.group4()[0] * other.group2()[0])
                    - (self.group4()[1] * other.group2()[1])
                    - (self.group4()[2] * other.group2()[2])
                    - (self.group5()[0] * other.group1()[0])
                    - (self.group5()[1] * other.group1()[1])
                    - (self.group5()[2] * other.group1()[2])
                    - (other.group1()[3] * self.group3()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
            // e5
            self.group0()[0] * other.group3()[3],
            // e15, e25, e35, e45
            (Simd32x4::from(other.group3()[3]) * self.group1())
                - (Simd32x4::from(self[e1]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]])),
            // e41, e42, e43
            (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))
                - (Simd32x3::from(other.group2()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])),
            // e23, e31, e12
            Simd32x3::from([
                (other.group3()[2] * self.group1()[1]) - (other.group3()[1] * self.group1()[2]),
                (other.group3()[0] * self.group1()[2]) - (other.group3()[2] * self.group1()[0]),
                (other.group3()[1] * self.group1()[0]) - (other.group3()[0] * self.group1()[1]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group4()[0] * other.group3()[3]) + (other.group2()[3] * self.group3()[0]),
                (self.group4()[1] * other.group3()[3]) + (other.group2()[3] * self.group3()[1]),
                (self.group4()[2] * other.group3()[3]) + (other.group2()[3] * self.group3()[2]),
                -(self.group5()[1] * other.group3()[1]) - (self.group5()[2] * other.group3()[2]),
            ]) + (Simd32x4::from(self.group0()[0]) * other.group1())
                - (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group5()[0]]) * crate::swizzle!(other.group3(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from(self.group0()[0]) * other.group0())
                + (Simd32x3::from(other.group2()[3]) * self.group5())
                + (Simd32x3::from([other.group3()[2], other.group3()[0], other.group3()[1]]) * crate::swizzle!(self.group4(), 1, 2, 0))
                - (Simd32x3::from([other.group3()[1], other.group3()[2], other.group3()[0]]) * crate::swizzle!(self.group4(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (other.group3()[1] * self.group3()[2]) - (other.group3()[2] * self.group3()[1]),
                (other.group3()[2] * self.group3()[0]) - (other.group3()[0] * self.group3()[2]),
                (other.group3()[0] * self.group3()[1]) - (other.group3()[1] * self.group3()[0]),
            ]) + (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]))
                + (Simd32x3::from(other.group3()[3]) * self.group5()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[1] * self.group1()[2]) + (other.group2()[0] * self.group1()[3]) - (other.group3()[2] * self.group6()[1]),
                (other.group1()[2] * self.group1()[0]) + (other.group2()[1] * self.group1()[3]) - (other.group3()[0] * self.group6()[2]),
                (other.group1()[0] * self.group1()[1]) + (other.group2()[2] * self.group1()[3]) - (other.group3()[1] * self.group6()[0]),
                (self.group8()[2] * other.group3()[2]) + (other.group3()[3] * self.group6()[3]) - (other.group2()[2] * self.group1()[2]),
            ]) + (Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group8()[0]]) * crate::swizzle!(other.group3(), 3, 3, 3, 0))
                + (Simd32x4::from([self.group6()[2], self.group6()[0], self.group6()[1], self.group8()[1]]) * crate::swizzle!(other.group3(), 1, 2, 0, 1))
                - (Simd32x4::from(self[e1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([self.group8()[0], self.group8()[1], self.group8()[2], self.group1()[0]]) * crate::swizzle!(other.group2(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[1]]) * crate::swizzle!(self.group1(), 1, 2, 0, 1)),
            // e1234
            (other.group0()[0] * self.group1()[0]) + (other.group0()[1] * self.group1()[1]) + (other.group0()[2] * self.group1()[2]) + (other.group1()[3] * self.group1()[3])
                - (self.group7()[0] * other.group3()[0])
                - (self.group7()[1] * other.group3()[1])
                - (self.group7()[2] * other.group3()[2])
                - (other.group2()[3] * self.group6()[3]),
        );
    }
}
impl Wedge<AntiDualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2       14        0
    //  no simd        2       34        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[1] * self.group0()[0], (other.group0()[0] * self[e1]) + (other.group0()[1] * self.group0()[1])]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[1]) * self.group1(),
            // e5
            other.group0()[1] * self[e1],
            // e15, e25, e35, e45
            Simd32x4::from(other.group0()[1]) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(other.group0()[1]) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(other.group0()[1]) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(other.group0()[1]) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(other.group0()[1]) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(other.group0()[1]) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other.group0()[1]) * self.group9(),
            // e1234
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self[e45]),
        );
    }
}
impl Wedge<AntiFlatPoint> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       16        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(self.group4()[0] * other.group0()[0]) - (self.group4()[1] * other.group0()[1]) - (self.group4()[2] * other.group0()[2]) - (other.group0()[3] * self.group3()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[0] * self.group1()[3],
                other.group0()[1] * self.group1()[3],
                other.group0()[2] * self.group1()[3],
                -(other.group0()[0] * self.group1()[0]) - (other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]) - (other.group0()[3] * self[e1]),
            ]),
            // e1234
            other.group0()[3] * self.group1()[3],
        );
    }
}
impl Wedge<AntiFlector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       53        0
    //    simd3        3        5        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       35       61        0
    //  no simd       50       80        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (other.group1()[0] * self.group9()[0]) + (other.group1()[1] * self.group9()[1]) + (other.group1()[2] * self.group9()[2]) + (other.group1()[3] * self[e45])
                    - (self.group4()[0] * other.group0()[0])
                    - (self.group4()[1] * other.group0()[1])
                    - (self.group4()[2] * other.group0()[2])
                    - (other.group0()[3] * self.group3()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0] * other.group1()[0], self.group0()[0] * other.group1()[1], self.group0()[0] * other.group1()[2], 0.0]),
            // e5
            self.group0()[0] * other.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from([
                (other.group1()[3] * self.group1()[0]) - (other.group1()[0] * self[e1]),
                (other.group1()[3] * self.group1()[1]) - (other.group1()[1] * self[e1]),
                (other.group1()[3] * self.group1()[2]) - (other.group1()[2] * self[e1]),
                other.group1()[3] * self.group1()[3],
            ]),
            // e41, e42, e43
            Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from([
                (other.group1()[2] * self.group1()[1]) - (other.group1()[1] * self.group1()[2]),
                (other.group1()[0] * self.group1()[2]) - (other.group1()[2] * self.group1()[0]),
                (other.group1()[1] * self.group1()[0]) - (other.group1()[0] * self.group1()[1]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group4()[0] * other.group1()[3],
                self.group4()[1] * other.group1()[3],
                self.group4()[2] * other.group1()[3],
                (self.group0()[0] * other.group0()[3]) - (self.group5()[1] * other.group1()[1]) - (self.group5()[2] * other.group1()[2]),
            ]) - (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group5()[0]]) * crate::swizzle!(other.group1(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]) * crate::swizzle!(self.group4(), 1, 2, 0))
                - (Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]) * crate::swizzle!(self.group4(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (other.group1()[1] * self.group3()[2]) - (other.group1()[2] * self.group3()[1]),
                (other.group1()[2] * self.group3()[0]) - (other.group1()[0] * self.group3()[2]),
                (other.group1()[0] * self.group3()[1]) - (other.group1()[1] * self.group3()[0]),
            ]) + (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + (Simd32x3::from(other.group1()[3]) * self.group5()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] * self.group1()[3]) - (other.group1()[2] * self.group6()[1]),
                (other.group0()[1] * self.group1()[3]) - (other.group1()[0] * self.group6()[2]),
                (other.group0()[2] * self.group1()[3]) - (other.group1()[1] * self.group6()[0]),
                (self.group8()[2] * other.group1()[2]) + (other.group1()[3] * self.group6()[3])
                    - (other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group0()[3] * self[e1]),
            ]) + (Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group8()[0]]) * crate::swizzle!(other.group1(), 3, 3, 3, 0))
                + (Simd32x4::from([self.group6()[2], self.group6()[0], self.group6()[1], self.group8()[1]]) * crate::swizzle!(other.group1(), 1, 2, 0, 1)),
            // e1234
            (other.group0()[3] * self.group1()[3]) - (self.group7()[0] * other.group1()[0]) - (self.group7()[1] * other.group1()[1]) - (self.group7()[2] * other.group1()[2]),
        );
    }
}
impl Wedge<AntiLine> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       33        0
    //    simd3        2        5        0
    // Totals...
    // yes simd       22       38        0
    //  no simd       26       48        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                -(other.group0()[0] * self.group6()[0])
                    - (other.group0()[1] * self.group6()[1])
                    - (other.group0()[2] * self.group6()[2])
                    - (other.group1()[0] * self.group7()[0])
                    - (other.group1()[1] * self.group7()[1])
                    - (other.group1()[2] * self.group7()[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([self.group0()[0] * other.group1()[0], self.group0()[0] * other.group1()[1], self.group0()[0] * other.group1()[2], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self.group0()[0]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group1()[0] * self.group1()[3],
                other.group1()[1] * self.group1()[3],
                other.group1()[2] * self.group1()[3],
                -(other.group0()[0] * self.group1()[0]) - (other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]),
            ]),
            // e423, e431, e412
            Simd32x3::from(self.group1()[3]) * other.group0(),
            // e235, e315, e125
            (Simd32x3::from(self[e1]) * other.group0()) + (Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]) * crate::swizzle!(other.group1(), 2, 0, 1))
                - (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]) * crate::swizzle!(other.group1(), 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] * self.group3()[3]) + (other.group1()[2] * self.group4()[1]) - (other.group1()[1] * self.group4()[2]),
                (other.group0()[1] * self.group3()[3]) + (other.group1()[0] * self.group4()[2]) - (other.group1()[2] * self.group4()[0]),
                (other.group0()[2] * self.group3()[3]) + (other.group1()[1] * self.group4()[0]) - (other.group1()[0] * self.group4()[1]),
                -(other.group0()[0] * self.group3()[0])
                    - (other.group0()[1] * self.group3()[1])
                    - (other.group0()[2] * self.group3()[2])
                    - (other.group1()[0] * self.group5()[0])
                    - (other.group1()[1] * self.group5()[1])
                    - (other.group1()[2] * self.group5()[2]),
            ]),
            // e1234
            -(other.group0()[0] * self.group4()[0]) - (other.group0()[1] * self.group4()[1]) - (other.group0()[2] * self.group4()[2]),
        );
    }
}
impl Wedge<AntiMotor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       41        0
    //    simd3        4        7        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       30       53        0
    //  no simd       50       82        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                self.group0()[0] * other.group0()[3],
                (self.group0()[1] * other.group0()[3]) + (other.group1()[3] * self.group1()[3])
                    - (self.group7()[0] * other.group1()[0])
                    - (self.group7()[1] * other.group1()[1])
                    - (self.group7()[2] * other.group1()[2])
                    - (other.group0()[0] * self.group6()[0])
                    - (other.group0()[1] * self.group6()[1])
                    - (other.group0()[2] * self.group6()[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[3]) * self.group1(),
            // e5
            other.group0()[3] * self[e1],
            // e15, e25, e35, e45
            Simd32x4::from([
                (self.group0()[0] * other.group1()[0]) + (other.group0()[3] * self.group3()[0]),
                (self.group0()[0] * other.group1()[1]) + (other.group0()[3] * self.group3()[1]),
                (self.group0()[0] * other.group1()[2]) + (other.group0()[3] * self.group3()[2]),
                other.group0()[3] * self.group3()[3],
            ]),
            // e41, e42, e43
            Simd32x3::from(other.group0()[3]) * self.group4(),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])) + (Simd32x3::from(other.group0()[3]) * self.group5()),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group1()[0] * self.group1()[3],
                other.group1()[1] * self.group1()[3],
                other.group1()[2] * self.group1()[3],
                -(other.group0()[0] * self.group1()[0]) - (other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]),
            ]) + (Simd32x4::from(other.group0()[3]) * self.group6()),
            // e423, e431, e412
            (Simd32x3::from(other.group0()[3]) * self.group7()) + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e235, e315, e125
            Simd32x3::from([
                (other.group1()[2] * self.group1()[1]) - (other.group1()[1] * self.group1()[2]),
                (other.group1()[0] * self.group1()[2]) - (other.group1()[2] * self.group1()[0]),
                (other.group1()[1] * self.group1()[0]) - (other.group1()[0] * self.group1()[1]),
            ]) + (Simd32x3::from(other.group0()[3]) * self.group8())
                + (Simd32x3::from(self[e1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[3] * self.group9()[0],
                other.group0()[3] * self.group9()[1],
                other.group0()[3] * self.group9()[2],
                -(self.group5()[1] * other.group1()[1])
                    - (self.group5()[2] * other.group1()[2])
                    - (other.group0()[0] * self.group3()[0])
                    - (other.group0()[1] * self.group3()[1])
                    - (other.group0()[2] * self.group3()[2]),
            ]) + (Simd32x4::from([self.group4()[1], self.group4()[2], self.group4()[0], self.group0()[0]]) * crate::swizzle!(other.group1(), 2, 0, 1, 3))
                + (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group9()[3]]) * other.group0())
                - (Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group5()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
            // e1234
            (other.group0()[3] * self[e45]) - (self.group4()[0] * other.group0()[0]) - (self.group4()[1] * other.group0()[1]) - (self.group4()[2] * other.group0()[2]),
        );
    }
}
impl Wedge<AntiPlane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       43        0
    //    simd3        2        4        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       21       50        0
    //  no simd       34       67        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([
                0.0,
                (other.group0()[0] * self.group9()[0]) + (other.group0()[1] * self.group9()[1]) + (other.group0()[2] * self.group9()[2]) + (other.group0()[3] * self[e45]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0] * other.group0()[0], self.group0()[0] * other.group0()[1], self.group0()[0] * other.group0()[2], 0.0]),
            // e5
            self.group0()[0] * other.group0()[3],
            // e15, e25, e35, e45
            Simd32x4::from([
                (other.group0()[3] * self.group1()[0]) - (other.group0()[0] * self[e1]),
                (other.group0()[3] * self.group1()[1]) - (other.group0()[1] * self[e1]),
                (other.group0()[3] * self.group1()[2]) - (other.group0()[2] * self[e1]),
                other.group0()[3] * self.group1()[3],
            ]),
            // e41, e42, e43
            Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from([
                (other.group0()[2] * self.group1()[1]) - (other.group0()[1] * self.group1()[2]),
                (other.group0()[0] * self.group1()[2]) - (other.group0()[2] * self.group1()[0]),
                (other.group0()[1] * self.group1()[0]) - (other.group0()[0] * self.group1()[1]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                self.group4()[0] * other.group0()[3],
                self.group4()[1] * other.group0()[3],
                self.group4()[2] * other.group0()[3],
                -(self.group5()[1] * other.group0()[1]) - (self.group5()[2] * other.group0()[2]),
            ]) - (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group5()[0]]) * crate::swizzle!(other.group0(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]) * crate::swizzle!(self.group4(), 1, 2, 0))
                - (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * crate::swizzle!(self.group4(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (other.group0()[1] * self.group3()[2]) - (other.group0()[2] * self.group3()[1]),
                (other.group0()[2] * self.group3()[0]) - (other.group0()[0] * self.group3()[2]),
                (other.group0()[0] * self.group3()[1]) - (other.group0()[1] * self.group3()[0]),
            ]) + (Simd32x3::from(other.group0()[3]) * self.group5()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[2] * self.group6()[1]) * -1.0,
                (other.group0()[0] * self.group6()[2]) * -1.0,
                (other.group0()[1] * self.group6()[0]) * -1.0,
                (self.group8()[2] * other.group0()[2]) + (other.group0()[3] * self.group6()[3]),
            ]) + (Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group8()[0]]) * crate::swizzle!(other.group0(), 3, 3, 3, 0))
                + (Simd32x4::from([self.group6()[2], self.group6()[0], self.group6()[1], self.group8()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0, 1)),
            // e1234
            -(self.group7()[0] * other.group0()[0]) - (self.group7()[1] * other.group0()[1]) - (self.group7()[2] * other.group0()[2]),
        );
    }
}
impl Wedge<AntiScalar> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self.group0()[0] * other[e12345]);
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
                -(other.group0()[0] * self.group3()[0])
                    - (other.group0()[1] * self.group3()[1])
                    - (other.group0()[2] * self.group3()[2])
                    - (other.group2()[0] * self.group4()[0])
                    - (other.group2()[1] * self.group4()[1])
                    - (other.group2()[2] * self.group4()[2])
                    - (self.group5()[0] * other.group1()[0])
                    - (self.group5()[1] * other.group1()[1])
                    - (self.group5()[2] * other.group1()[2])
                    - (other.group1()[3] * self.group3()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(self.group0()[0]) * other.group1(),
            // e423, e431, e412
            Simd32x3::from(self.group0()[0]) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self.group0()[0]) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group2()[0] * self.group1()[3]) + (other.group1()[1] * self.group1()[2]),
                (other.group2()[1] * self.group1()[3]) + (other.group1()[2] * self.group1()[0]),
                (other.group2()[2] * self.group1()[3]) + (other.group1()[0] * self.group1()[1]),
                -(other.group2()[1] * self.group1()[1]) - (other.group2()[2] * self.group1()[2]),
            ]) - (Simd32x4::from(self[e1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
            // e1234
            (other.group0()[0] * self.group1()[0]) + (other.group0()[1] * self.group1()[1]) + (other.group0()[2] * self.group1()[2]) + (other.group1()[3] * self.group1()[3]),
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
                (self.group0()[0] * other.group2()[3])
                    - (other.group0()[0] * self.group3()[0])
                    - (other.group0()[1] * self.group3()[1])
                    - (other.group0()[2] * self.group3()[2])
                    - (self.group4()[0] * other.group2()[0])
                    - (self.group4()[1] * other.group2()[1])
                    - (self.group4()[2] * other.group2()[2])
                    - (self.group5()[0] * other.group1()[0])
                    - (self.group5()[1] * other.group1()[1])
                    - (self.group5()[2] * other.group1()[2])
                    - (other.group1()[3] * self.group3()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(self.group0()[0]) * other.group1(),
            // e423, e431, e412
            Simd32x3::from(self.group0()[0]) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[1] * self.group1()[2]) + (other.group2()[0] * self.group1()[3]),
                (other.group1()[2] * self.group1()[0]) + (other.group2()[1] * self.group1()[3]),
                (other.group1()[0] * self.group1()[1]) + (other.group2()[2] * self.group1()[3]),
                -(other.group2()[1] * self.group1()[1]) - (other.group2()[2] * self.group1()[2]),
            ]) - (Simd32x4::from(self[e1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
            // e1234
            (other.group0()[0] * self.group1()[0]) + (other.group0()[1] * self.group1()[1]) + (other.group0()[2] * self.group1()[2]) + (other.group1()[3] * self.group1()[3]),
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
                -(other.group0()[0] * self.group8()[0])
                    - (other.group0()[1] * self.group8()[1])
                    - (other.group0()[2] * self.group8()[2])
                    - (other.group2()[0] * self.group7()[0])
                    - (other.group2()[1] * self.group7()[1])
                    - (other.group2()[2] * self.group7()[2])
                    - (other.group1()[0] * self.group6()[0])
                    - (other.group1()[1] * self.group6()[1])
                    - (other.group1()[2] * self.group6()[2])
                    - (other.group1()[3] * self.group6()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]),
            // e41, e42, e43
            Simd32x3::from(self.group0()[0]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self[e1]) + (other.group2()[0] * self.group1()[3]),
                (other.group0()[1] * self[e1]) + (other.group2()[1] * self.group1()[3]),
                (other.group0()[2] * self[e1]) + (other.group2()[2] * self.group1()[3]),
                -(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2]),
            ]) - (crate::swizzle!(other.group1(), 3, 3, 3, 0) * crate::swizzle!(self.group1(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e235, e315, e125
            (Simd32x3::from(self[e1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]) * crate::swizzle!(other.group2(), 2, 0, 1))
                - (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]) * crate::swizzle!(other.group2(), 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group3()[2]) + (other.group2()[2] * self.group4()[1]) + (self.group5()[0] * other.group1()[3]) + (other.group1()[0] * self.group3()[3])
                    - (other.group2()[1] * self.group4()[2]),
                (other.group0()[2] * self.group3()[0]) + (other.group2()[0] * self.group4()[2]) + (self.group5()[1] * other.group1()[3]) + (other.group1()[1] * self.group3()[3])
                    - (other.group2()[2] * self.group4()[0]),
                (other.group0()[0] * self.group3()[1]) + (other.group2()[1] * self.group4()[0]) + (self.group5()[2] * other.group1()[3]) + (other.group1()[2] * self.group3()[3])
                    - (other.group2()[0] * self.group4()[1]),
                -(other.group2()[0] * self.group5()[0])
                    - (other.group2()[1] * self.group5()[1])
                    - (other.group2()[2] * self.group5()[2])
                    - (other.group1()[1] * self.group3()[1])
                    - (other.group1()[2] * self.group3()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0)),
            // e1234
            -(other.group0()[0] * self.group5()[0])
                - (other.group0()[1] * self.group5()[1])
                - (other.group0()[2] * self.group5()[2])
                - (self.group4()[0] * other.group1()[0])
                - (self.group4()[1] * other.group1()[1])
                - (self.group4()[2] * other.group1()[2]),
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
                (other.group2()[3] * self[e1])
                    + (other.group3()[0] * self.group1()[0])
                    + (other.group3()[1] * self.group1()[1])
                    + (other.group3()[2] * self.group1()[2])
                    + (other.group3()[3] * self.group1()[3])
                    - (other.group0()[0] * self.group8()[0])
                    - (other.group0()[1] * self.group8()[1])
                    - (other.group0()[2] * self.group8()[2])
                    - (self.group7()[0] * other.group2()[0])
                    - (self.group7()[1] * other.group2()[1])
                    - (self.group7()[2] * other.group2()[2])
                    - (other.group1()[0] * self.group6()[0])
                    - (other.group1()[1] * self.group6()[1])
                    - (other.group1()[2] * self.group6()[2])
                    - (other.group1()[3] * self.group6()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]),
            // e41, e42, e43
            Simd32x3::from(self.group0()[0]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self[e1]) + (other.group2()[0] * self.group1()[3]),
                (other.group0()[1] * self[e1]) + (other.group2()[1] * self.group1()[3]),
                (other.group0()[2] * self[e1]) + (other.group2()[2] * self.group1()[3]),
                -(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2]),
            ]) - (crate::swizzle!(other.group1(), 3, 3, 3, 0) * crate::swizzle!(self.group1(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (other.group2()[2] * self.group1()[1]) - (other.group2()[1] * self.group1()[2]),
                (other.group2()[0] * self.group1()[2]) - (other.group2()[2] * self.group1()[0]),
                (other.group2()[1] * self.group1()[0]) - (other.group2()[0] * self.group1()[1]),
            ]) + (Simd32x3::from(self[e1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group3()[2]) + (self.group4()[1] * other.group2()[2]) + (self.group5()[0] * other.group1()[3]) + (other.group1()[0] * self.group3()[3]),
                (other.group0()[2] * self.group3()[0]) + (self.group4()[2] * other.group2()[0]) + (self.group5()[1] * other.group1()[3]) + (other.group1()[1] * self.group3()[3]),
                (other.group0()[0] * self.group3()[1]) + (self.group4()[0] * other.group2()[1]) + (self.group5()[2] * other.group1()[3]) + (other.group1()[2] * self.group3()[3]),
                -(self.group5()[1] * other.group2()[1]) - (self.group5()[2] * other.group2()[2]) - (other.group1()[1] * self.group3()[1]) - (other.group1()[2] * self.group3()[2]),
            ]) + (Simd32x4::from(self.group0()[0]) * other.group3())
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group5()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0)),
            // e1234
            (self.group0()[0] * other.group2()[3])
                - (other.group0()[0] * self.group5()[0])
                - (other.group0()[1] * self.group5()[1])
                - (other.group0()[2] * self.group5()[2])
                - (self.group4()[0] * other.group1()[0])
                - (self.group4()[1] * other.group1()[1])
                - (self.group4()[2] * other.group1()[2]),
        );
    }
}
impl Wedge<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       16        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        1       19        0
    //  no simd        1       25        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[0] * self.group9()[3]) + (other.group0()[1] * self.group0()[0])]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group0()[0]]),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self[e1] * -1.0]),
            // e41, e42, e43
            Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0] * self.group3()[0], other.group0()[0] * self.group3()[1], other.group0()[0] * self.group3()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(other.group0()[0]) * self.group5(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[0] * self.group8()[0] * -1.0,
                other.group0()[0] * self.group8()[1] * -1.0,
                other.group0()[0] * self.group8()[2] * -1.0,
                0.0,
            ]),
            // e1234
            other.group0()[0] * self.group6()[3] * -1.0,
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
                -(self.group7()[0] * other.group0()[0]) - (self.group7()[1] * other.group0()[1]) - (self.group7()[2] * other.group0()[2]) - (other.group0()[3] * self.group6()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(self.group0()[0]) * other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self.group1()[3]) - (other.group0()[3] * self.group1()[0]),
                (other.group0()[1] * self.group1()[3]) - (other.group0()[3] * self.group1()[1]),
                (other.group0()[2] * self.group1()[3]) - (other.group0()[3] * self.group1()[2]),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([
                (other.group0()[2] * self.group1()[1]) - (other.group0()[1] * self.group1()[2]),
                (other.group0()[0] * self.group1()[2]) - (other.group0()[2] * self.group1()[0]),
                (other.group0()[1] * self.group1()[0]) - (other.group0()[0] * self.group1()[1]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group4()[1] * other.group0()[2]) + (self.group5()[0] * other.group0()[3]),
                (self.group4()[2] * other.group0()[0]) + (self.group5()[1] * other.group0()[3]),
                (self.group4()[0] * other.group0()[1]) + (self.group5()[2] * other.group0()[3]),
                -(self.group5()[1] * other.group0()[1]) - (self.group5()[2] * other.group0()[2]),
            ]) - (Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group5()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
            // e1234
            0.0,
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
                (other.group1()[0] * self.group1()[0]) + (other.group1()[1] * self.group1()[1]) + (other.group1()[2] * self.group1()[2]) + (other.group1()[3] * self.group1()[3])
                    - (self.group7()[0] * other.group0()[0])
                    - (self.group7()[1] * other.group0()[1])
                    - (self.group7()[2] * other.group0()[2])
                    - (other.group0()[3] * self.group6()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(self.group0()[0]) * other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self.group1()[3]) - (other.group0()[3] * self.group1()[0]),
                (other.group0()[1] * self.group1()[3]) - (other.group0()[3] * self.group1()[1]),
                (other.group0()[2] * self.group1()[3]) - (other.group0()[3] * self.group1()[2]),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([
                (other.group0()[2] * self.group1()[1]) - (other.group0()[1] * self.group1()[2]),
                (other.group0()[0] * self.group1()[2]) - (other.group0()[2] * self.group1()[0]),
                (other.group0()[1] * self.group1()[0]) - (other.group0()[0] * self.group1()[1]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group4()[1] * other.group0()[2]) + (self.group5()[0] * other.group0()[3]),
                (self.group4()[2] * other.group0()[0]) + (self.group5()[1] * other.group0()[3]),
                (self.group4()[0] * other.group0()[1]) + (self.group5()[2] * other.group0()[3]),
                -(self.group5()[1] * other.group0()[1]) - (self.group5()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(self.group0()[0]) * other.group1())
                - (Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group5()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
            // e1234
            0.0,
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
                -(other.group0()[0] * self.group5()[0])
                    - (other.group0()[1] * self.group5()[1])
                    - (other.group0()[2] * self.group5()[2])
                    - (other.group1()[0] * self.group4()[0])
                    - (other.group1()[1] * self.group4()[1])
                    - (other.group1()[2] * self.group4()[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([self.group0()[0] * other.group0()[0], self.group0()[0] * other.group0()[1], self.group0()[0] * other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(self.group0()[0]) * other.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group1()[2]) + (other.group1()[0] * self.group1()[3]),
                (other.group0()[2] * self.group1()[0]) + (other.group1()[1] * self.group1()[3]),
                (other.group0()[0] * self.group1()[1]) + (other.group1()[2] * self.group1()[3]),
                -(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
            // e1234
            0.0,
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
                (self.group0()[0] * other.group0()[3]) + (other.group1()[3] * self[e45])
                    - (self.group4()[0] * other.group1()[0])
                    - (self.group4()[1] * other.group1()[1])
                    - (self.group4()[2] * other.group1()[2])
                    - (self.group5()[0] * other.group0()[0])
                    - (self.group5()[1] * other.group0()[1])
                    - (self.group5()[2] * other.group0()[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            self.group0()[0] * other.group1()[3],
            // e15, e25, e35, e45
            Simd32x4::from(other.group1()[3]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[0] * other.group0()[0]) + (self.group4()[0] * other.group1()[3]),
                (self.group0()[0] * other.group0()[1]) + (self.group4()[1] * other.group1()[3]),
                (self.group0()[0] * other.group0()[2]) + (self.group4()[2] * other.group1()[3]),
                0.0,
            ]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])) + (Simd32x3::from(other.group1()[3]) * self.group5()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group1()[2]) + (other.group1()[0] * self.group1()[3]),
                (other.group0()[2] * self.group1()[0]) + (other.group1()[1] * self.group1()[3]),
                (other.group0()[0] * self.group1()[1]) + (other.group1()[2] * self.group1()[3]),
                -(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2]),
            ]) + (Simd32x4::from(other.group1()[3]) * Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group6()[3]]))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
            // e1234
            0.0,
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
                other.group0()[0] * self.group0()[0],
                (other.group0()[0] * self.group0()[1])
                    + (other.group0()[1] * self.group0()[0])
                    + (other.group1()[0] * self.group9()[0])
                    + (other.group1()[1] * self.group9()[1])
                    + (other.group1()[2] * self.group9()[2])
                    + (other.group1()[3] * self.group9()[3])
                    + (other.group9()[0] * self.group1()[0])
                    + (other.group9()[1] * self.group1()[1])
                    + (other.group9()[2] * self.group1()[2])
                    + (other.group9()[3] * self.group1()[3])
                    + (other[e1] * self[e45])
                    + (other[e45] * self[e1])
                    - (other.group4()[0] * self.group8()[0])
                    - (other.group4()[1] * self.group8()[1])
                    - (other.group4()[2] * self.group8()[2])
                    - (other.group5()[0] * self.group6()[0])
                    - (other.group5()[1] * self.group6()[1])
                    - (other.group5()[2] * self.group6()[2])
                    - (other.group7()[0] * self.group3()[0])
                    - (other.group7()[1] * self.group3()[1])
                    - (other.group7()[2] * self.group3()[2])
                    - (other.group8()[0] * self.group4()[0])
                    - (other.group8()[1] * self.group4()[1])
                    - (other.group8()[2] * self.group4()[2])
                    - (self.group5()[0] * other.group6()[0])
                    - (self.group5()[1] * other.group6()[1])
                    - (self.group5()[2] * other.group6()[2])
                    - (self.group7()[0] * other.group3()[0])
                    - (self.group7()[1] * other.group3()[1])
                    - (self.group7()[2] * other.group3()[2])
                    - (other.group3()[3] * self.group6()[3])
                    - (other.group6()[3] * self.group3()[3]),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from(other.group0()[0]) * self.group1()) + (Simd32x4::from(self.group0()[0]) * other.group1()),
            // e5
            (other.group0()[0] * self[e1]) + (self.group0()[0] * other[e1]),
            // e15, e25, e35, e45
            (Simd32x4::from(other.group0()[0]) * self.group3()) + (Simd32x4::from(self.group0()[0]) * other.group3()) + (Simd32x4::from(other[e1]) * self.group1())
                - (Simd32x4::from(self[e1]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(other.group0()[0]) * self.group4())
                + (Simd32x3::from(self.group0()[0]) * other.group4())
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])),
            // e23, e31, e12
            Simd32x3::from([
                (other.group1()[2] * self.group1()[1]) - (other.group1()[1] * self.group1()[2]),
                (other.group1()[0] * self.group1()[2]) - (other.group1()[2] * self.group1()[0]),
                (other.group1()[1] * self.group1()[0]) - (other.group1()[0] * self.group1()[1]),
            ]) + (Simd32x3::from(other.group0()[0]) * self.group5())
                + (Simd32x3::from(self.group0()[0]) * other.group5()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group4()[0] * self[e1]) + (self.group4()[0] * other[e1]) + (other.group1()[3] * self.group3()[0]) + (other.group3()[0] * self.group1()[3]),
                (other.group4()[1] * self[e1]) + (self.group4()[1] * other[e1]) + (other.group1()[3] * self.group3()[1]) + (other.group3()[1] * self.group1()[3]),
                (other.group4()[2] * self[e1]) + (self.group4()[2] * other[e1]) + (other.group1()[3] * self.group3()[2]) + (other.group3()[2] * self.group1()[3]),
                -(other.group5()[1] * self.group1()[1]) - (other.group5()[2] * self.group1()[2]) - (self.group5()[1] * other.group1()[1]) - (self.group5()[2] * other.group1()[2]),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group6())
                + (Simd32x4::from(self.group0()[0]) * other.group6())
                - (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group5()[0]]) * crate::swizzle!(self.group1(), 0, 1, 2, 0))
                - (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group5()[0]]) * crate::swizzle!(other.group1(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from(other.group0()[0]) * self.group7())
                + (Simd32x3::from(self.group0()[0]) * other.group7())
                + (Simd32x3::from(other.group1()[3]) * self.group5())
                + (Simd32x3::from(self.group1()[3]) * other.group5())
                + (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]) * crate::swizzle!(self.group4(), 1, 2, 0))
                + (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]) * crate::swizzle!(other.group4(), 1, 2, 0))
                - (Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]) * crate::swizzle!(self.group4(), 2, 0, 1))
                - (Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]) * crate::swizzle!(other.group4(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (other.group1()[1] * self.group3()[2]) + (other.group3()[2] * self.group1()[1]) - (other.group1()[2] * self.group3()[1]) - (other.group3()[1] * self.group1()[2]),
                (other.group1()[2] * self.group3()[0]) + (other.group3()[0] * self.group1()[2]) - (other.group1()[0] * self.group3()[2]) - (other.group3()[2] * self.group1()[0]),
                (other.group1()[0] * self.group3()[1]) + (other.group3()[1] * self.group1()[0]) - (other.group1()[1] * self.group3()[0]) - (other.group3()[0] * self.group1()[1]),
            ]) + (Simd32x3::from(other.group0()[0]) * self.group8())
                + (Simd32x3::from(self.group0()[0]) * other.group8())
                + (Simd32x3::from(other[e1]) * self.group5())
                + (Simd32x3::from(self[e1]) * other.group5()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group4()[1] * self.group3()[2])
                    + (other.group5()[0] * self.group3()[3])
                    + (other.group8()[0] * self.group1()[3])
                    + (self.group4()[1] * other.group3()[2])
                    + (self.group5()[0] * other.group3()[3])
                    + (other.group6()[1] * self.group1()[2])
                    - (self.group8()[0] * other.group1()[3])
                    - (other.group1()[2] * self.group6()[1]),
                (other.group4()[2] * self.group3()[0])
                    + (other.group5()[1] * self.group3()[3])
                    + (other.group8()[1] * self.group1()[3])
                    + (self.group4()[2] * other.group3()[0])
                    + (self.group5()[1] * other.group3()[3])
                    + (other.group6()[2] * self.group1()[0])
                    - (self.group8()[1] * other.group1()[3])
                    - (other.group1()[0] * self.group6()[2]),
                (other.group4()[0] * self.group3()[1])
                    + (other.group5()[2] * self.group3()[3])
                    + (other.group8()[2] * self.group1()[3])
                    + (self.group4()[0] * other.group3()[1])
                    + (self.group5()[2] * other.group3()[3])
                    + (other.group6()[0] * self.group1()[1])
                    - (self.group8()[2] * other.group1()[3])
                    - (other.group1()[1] * self.group6()[0]),
                (self.group8()[1] * other.group1()[1]) + (self.group8()[2] * other.group1()[2])
                    - (other.group5()[1] * self.group3()[1])
                    - (other.group5()[2] * self.group3()[2])
                    - (other.group8()[1] * self.group1()[1])
                    - (other.group8()[2] * self.group1()[2])
                    - (self.group5()[1] * other.group3()[1])
                    - (self.group5()[2] * other.group3()[2]),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group9())
                + (Simd32x4::from(self.group0()[0]) * other.group9())
                + (Simd32x4::from(other[e1]) * Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group6()[3]]))
                + (Simd32x4::from([self.group6()[2], self.group6()[0], self.group6()[1], self.group8()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0))
                - (Simd32x4::from(self[e1]) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], other.group6()[3]]))
                - (Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group5()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group5()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group6()[2], other.group6()[0], other.group6()[1], other.group8()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
            // e1234
            (other.group0()[0] * self[e45])
                + (self.group0()[0] * other[e45])
                + (other.group7()[0] * self.group1()[0])
                + (other.group7()[1] * self.group1()[1])
                + (other.group7()[2] * self.group1()[2])
                + (other.group6()[3] * self.group1()[3])
                - (other.group4()[0] * self.group5()[0])
                - (other.group4()[1] * self.group5()[1])
                - (other.group4()[2] * self.group5()[2])
                - (other.group5()[0] * self.group4()[0])
                - (other.group5()[1] * self.group4()[1])
                - (other.group5()[2] * self.group4()[2])
                - (self.group7()[0] * other.group1()[0])
                - (self.group7()[1] * other.group1()[1])
                - (self.group7()[2] * other.group1()[2])
                - (other.group1()[3] * self.group6()[3]),
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
                (self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2]) + (self.group1()[3] * other.group0()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self.group0()[0]) * other.group0(),
            // e1234
            0.0,
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
                (self.group9()[0] * other.group0()[0])
                    + (self.group9()[1] * other.group0()[1])
                    + (self.group9()[2] * other.group0()[2])
                    + (self.group9()[3] * other.group0()[3])
                    + (self[e45] * other[e2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[0]) * other.group0(),
            // e5
            self.group0()[0] * other[e2],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e2]) * self.group1()) - (Simd32x4::from(self[e1]) * other.group0()),
            // e41, e42, e43
            (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])),
            // e23, e31, e12
            Simd32x3::from([
                (self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1]),
                (self.group1()[2] * other.group0()[0]) - (self.group1()[0] * other.group0()[2]),
                (self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group4()[0] * other[e2]) + (self.group3()[0] * other.group0()[3]),
                (self.group4()[1] * other[e2]) + (self.group3()[1] * other.group0()[3]),
                (self.group4()[2] * other[e2]) + (self.group3()[2] * other.group0()[3]),
                -(self.group5()[1] * other.group0()[1]) - (self.group5()[2] * other.group0()[2]),
            ]) - (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group5()[0]]) * crate::swizzle!(other.group0(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from(other.group0()[3]) * self.group5())
                + (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]) * crate::swizzle!(self.group4(), 1, 2, 0))
                - (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * crate::swizzle!(self.group4(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (self.group3()[2] * other.group0()[1]) - (self.group3()[1] * other.group0()[2]),
                (self.group3()[0] * other.group0()[2]) - (self.group3()[2] * other.group0()[0]),
                (self.group3()[1] * other.group0()[0]) - (self.group3()[0] * other.group0()[1]),
            ]) + (Simd32x3::from(other[e2]) * self.group5()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(self.group8()[0] * other.group0()[3]) - (self.group6()[1] * other.group0()[2]),
                -(self.group8()[1] * other.group0()[3]) - (self.group6()[2] * other.group0()[0]),
                -(self.group8()[2] * other.group0()[3]) - (self.group6()[0] * other.group0()[1]),
                (self.group8()[1] * other.group0()[1]) + (self.group8()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(other[e2]) * Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group6()[3]]))
                + (Simd32x4::from([self.group6()[2], self.group6()[0], self.group6()[1], self.group8()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
            // e1234
            -(self.group7()[0] * other.group0()[0]) - (self.group7()[1] * other.group0()[1]) - (self.group7()[2] * other.group0()[2]) - (self.group6()[3] * other.group0()[3]),
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
            Simd32x2::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group1(),
            // e5
            self[e1] * other[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(other[scalar]) * self.group3(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group4(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group6(),
            // e423, e431, e412
            Simd32x3::from(other[scalar]) * self.group7(),
            // e235, e315, e125
            Simd32x3::from(other[scalar]) * self.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group9(),
            // e1234
            self[e45] * other[scalar],
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
                (self.group1()[0] * other.group0()[0])
                    + (self.group1()[1] * other.group0()[1])
                    + (self.group1()[2] * other.group0()[2])
                    + (self.group1()[3] * other.group0()[3])
                    + (self[e1] * other[e4315]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self.group0()[0]) * other.group0(),
            // e1234
            self.group0()[0] * other[e4315],
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
                (self.group0()[0] * other.group0()[3])
                    + (self.group9()[0] * other.group3()[0])
                    + (self.group9()[1] * other.group3()[1])
                    + (self.group9()[2] * other.group3()[2])
                    + (self.group9()[3] * other.group3()[3])
                    + (other.group2()[3] * self[e45])
                    - (self.group4()[0] * other.group2()[0])
                    - (self.group4()[1] * other.group2()[1])
                    - (self.group4()[2] * other.group2()[2])
                    - (self.group5()[0] * other.group1()[0])
                    - (self.group5()[1] * other.group1()[1])
                    - (self.group5()[2] * other.group1()[2])
                    - (self.group3()[0] * other.group0()[0])
                    - (self.group3()[1] * other.group0()[1])
                    - (self.group3()[2] * other.group0()[2])
                    - (self.group3()[3] * other.group1()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[0]) * other.group3(),
            // e5
            self.group0()[0] * other.group2()[3],
            // e15, e25, e35, e45
            (Simd32x4::from(other.group2()[3]) * self.group1()) - (Simd32x4::from(self[e1]) * other.group3()),
            // e41, e42, e43
            (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))
                - (Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])),
            // e23, e31, e12
            Simd32x3::from([
                (self.group1()[1] * other.group3()[2]) - (self.group1()[2] * other.group3()[1]),
                (self.group1()[2] * other.group3()[0]) - (self.group1()[0] * other.group3()[2]),
                (self.group1()[0] * other.group3()[1]) - (self.group1()[1] * other.group3()[0]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group4()[0] * other.group2()[3]) + (self.group3()[0] * other.group3()[3]),
                (self.group4()[1] * other.group2()[3]) + (self.group3()[1] * other.group3()[3]),
                (self.group4()[2] * other.group2()[3]) + (self.group3()[2] * other.group3()[3]),
                -(self.group5()[1] * other.group3()[1]) - (self.group5()[2] * other.group3()[2]),
            ]) + (Simd32x4::from(self.group0()[0]) * other.group1())
                - (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group5()[0]]) * crate::swizzle!(other.group3(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + (Simd32x3::from(other.group3()[3]) * self.group5())
                + (Simd32x3::from([other.group3()[2], other.group3()[0], other.group3()[1]]) * crate::swizzle!(self.group4(), 1, 2, 0))
                - (Simd32x3::from([other.group3()[1], other.group3()[2], other.group3()[0]]) * crate::swizzle!(self.group4(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (self.group3()[2] * other.group3()[1]) - (self.group3()[1] * other.group3()[2]),
                (self.group3()[0] * other.group3()[2]) - (self.group3()[2] * other.group3()[0]),
                (self.group3()[1] * other.group3()[0]) - (self.group3()[0] * other.group3()[1]),
            ]) + (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]))
                + (Simd32x3::from(other.group2()[3]) * self.group5()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group1()[2] * other.group1()[1]) + (self.group1()[3] * other.group2()[0]) - (self.group8()[0] * other.group3()[3]) - (self.group6()[1] * other.group3()[2]),
                (self.group1()[0] * other.group1()[2]) + (self.group1()[3] * other.group2()[1]) - (self.group8()[1] * other.group3()[3]) - (self.group6()[2] * other.group3()[0]),
                (self.group1()[1] * other.group1()[0]) + (self.group1()[3] * other.group2()[2]) - (self.group8()[2] * other.group3()[3]) - (self.group6()[0] * other.group3()[1]),
                (self.group8()[1] * other.group3()[1]) + (self.group8()[2] * other.group3()[2]) - (self.group1()[1] * other.group2()[1]) - (self.group1()[2] * other.group2()[2]),
            ]) + (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], self.group6()[3]]))
                + (Simd32x4::from([self.group6()[2], self.group6()[0], self.group6()[1], self.group8()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0))
                - (Simd32x4::from(self[e1]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
            // e1234
            (self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2]) + (self.group1()[3] * other.group1()[3])
                - (self.group7()[0] * other.group3()[0])
                - (self.group7()[1] * other.group3()[1])
                - (self.group7()[2] * other.group3()[2])
                - (self.group6()[3] * other.group3()[3]),
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
                self.group0()[0] * other.group0()[3],
                (self.group0()[1] * other.group0()[3])
                    + (self.group1()[0] * other.group3()[0])
                    + (self.group1()[1] * other.group3()[1])
                    + (self.group1()[2] * other.group3()[2])
                    + (self.group1()[3] * other.group3()[3])
                    + (other.group2()[3] * self[e1])
                    - (self.group7()[0] * other.group2()[0])
                    - (self.group7()[1] * other.group2()[1])
                    - (self.group7()[2] * other.group2()[2])
                    - (self.group8()[0] * other.group0()[0])
                    - (self.group8()[1] * other.group0()[1])
                    - (self.group8()[2] * other.group0()[2])
                    - (self.group6()[0] * other.group1()[0])
                    - (self.group6()[1] * other.group1()[1])
                    - (self.group6()[2] * other.group1()[2])
                    - (self.group6()[3] * other.group1()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[3]) * self.group1(),
            // e5
            other.group0()[3] * self[e1],
            // e15, e25, e35, e45
            (Simd32x4::from(self.group0()[0]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group1()[3]]))
                + (Simd32x4::from(other.group0()[3]) * self.group3()),
            // e41, e42, e43
            (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])) + (Simd32x3::from(other.group0()[3]) * self.group4()),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])) + (Simd32x3::from(other.group0()[3]) * self.group5()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group1()[3] * other.group2()[0]) + (other.group0()[0] * self[e1]),
                (self.group1()[3] * other.group2()[1]) + (other.group0()[1] * self[e1]),
                (self.group1()[3] * other.group2()[2]) + (other.group0()[2] * self[e1]),
                -(self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2]),
            ]) + (Simd32x4::from(other.group0()[3]) * self.group6())
                - (crate::swizzle!(self.group1(), 0, 1, 2, 0) * crate::swizzle!(other.group1(), 3, 3, 3, 0)),
            // e423, e431, e412
            Simd32x3::from([
                (self.group1()[2] * other.group0()[1]) - (self.group1()[1] * other.group0()[2]),
                (self.group1()[0] * other.group0()[2]) - (self.group1()[2] * other.group0()[0]),
                (self.group1()[1] * other.group0()[0]) - (self.group1()[0] * other.group0()[1]),
            ]) + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from(other.group0()[3]) * self.group7()),
            // e235, e315, e125
            Simd32x3::from([
                (self.group1()[1] * other.group2()[2]) - (self.group1()[2] * other.group2()[1]),
                (self.group1()[2] * other.group2()[0]) - (self.group1()[0] * other.group2()[2]),
                (self.group1()[0] * other.group2()[1]) - (self.group1()[1] * other.group2()[0]),
            ]) + (Simd32x3::from(other.group0()[3]) * self.group8())
                + (Simd32x3::from(self[e1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group4()[1] * other.group2()[2]) + (self.group5()[0] * other.group1()[3]) + (self.group3()[3] * other.group1()[0]) + (self.group9()[0] * other.group0()[3]),
                (self.group4()[2] * other.group2()[0]) + (self.group5()[1] * other.group1()[3]) + (self.group3()[3] * other.group1()[1]) + (self.group9()[1] * other.group0()[3]),
                (self.group4()[0] * other.group2()[1]) + (self.group5()[2] * other.group1()[3]) + (self.group3()[3] * other.group1()[2]) + (self.group9()[2] * other.group0()[3]),
                -(self.group5()[1] * other.group2()[1]) - (self.group5()[2] * other.group2()[2]) - (self.group3()[1] * other.group1()[1]) - (self.group3()[2] * other.group1()[2]),
            ]) + (Simd32x4::from(self.group0()[0]) * other.group3())
                + (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group9()[3]]) * crate::swizzle!(other.group0(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group5()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0)),
            // e1234
            (self.group0()[0] * other.group2()[3]) + (other.group0()[3] * self[e45])
                - (self.group4()[0] * other.group1()[0])
                - (self.group4()[1] * other.group1()[1])
                - (self.group4()[2] * other.group1()[2])
                - (self.group5()[0] * other.group0()[0])
                - (self.group5()[1] * other.group0()[1])
                - (self.group5()[2] * other.group0()[2]),
        );
    }
}
impl std::ops::Div<wedge> for Plane {
    type Output = wedge_partial<Plane>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other.group2()[3]) * self.group0());
    }
}
impl Wedge<AntiDipoleInversion> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (other.group2()[3] * self.group0()[3]) + (other.group3()[0] * self.group0()[0]) + (other.group3()[1] * self.group0()[1]) + (other.group3()[2] * self.group0()[2]),
        );
    }
}
impl Wedge<AntiDualNum> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other.group0()[1]) * self.group0());
    }
}
impl Wedge<AntiFlector> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (other.group1()[0] * self.group0()[0]) + (other.group1()[1] * self.group0()[1]) + (other.group1()[2] * self.group0()[2]),
        );
    }
}
impl Wedge<AntiMotor> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other.group0()[3]) * self.group0());
    }
}
impl Wedge<AntiPlane> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return AntiScalar::from_groups(
            // e12345
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]),
        );
    }
}
impl Wedge<DualNum> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return AntiScalar::from_groups(/* e12345 */ other.group0()[0] * self.group0()[3]);
    }
}
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
                (other.group1()[0] * self.group0()[0]) + (other.group1()[1] * self.group0()[1]) + (other.group1()[2] * self.group0()[2]) + (other.group1()[3] * self.group0()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other.group0()[0]) * self.group0(),
            // e1234
            0.0,
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
            (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[3] * other.group0()[3]),
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
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other[scalar]) * self.group0());
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
            (self.group0()[0] * other.group3()[0]) + (self.group0()[1] * other.group3()[1]) + (self.group0()[2] * other.group3()[2]) + (self.group0()[3] * other.group3()[3]),
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
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(other.group0()[3]) * self.group0());
    }
}
impl std::ops::Div<wedge> for RoundPoint {
    type Output = wedge_partial<RoundPoint>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for RoundPoint {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        2        3        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       20       35        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self[e2]) + (other.group2()[0] * self.group0()[3]),
                (other.group0()[1] * self[e2]) + (other.group2()[1] * self.group0()[3]),
                (other.group0()[2] * self[e2]) + (other.group2()[2] * self.group0()[3]),
                -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) - (crate::swizzle!(other.group1(), 3, 3, 3, 0) * crate::swizzle!(self.group0(), 0, 1, 2, 0)),
            // e235, e315, e125, e4
            Simd32x4::from([
                (other.group1()[0] * self[e2]) + (other.group2()[2] * self.group0()[1]) - (other.group2()[1] * self.group0()[2]),
                (other.group1()[1] * self[e2]) + (other.group2()[0] * self.group0()[2]) - (other.group2()[2] * self.group0()[0]),
                (other.group1()[2] * self[e2]) + (other.group2()[1] * self.group0()[0]) - (other.group2()[0] * self.group0()[1]),
                other.group2()[3] * self.group0()[3],
            ]),
            // e1, e2, e3, e5
            Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self[e2]]),
        );
    }
}
impl Wedge<AntiDipoleInversion> for RoundPoint {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       25        0
    //    simd3        1        2        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       11       31        0
    //  no simd       25       47        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))
                - (Simd32x3::from(other.group2()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group3()[1] * self.group0()[2]) * -1.0,
                (other.group3()[2] * self.group0()[0]) * -1.0,
                (other.group3()[0] * self.group0()[1]) * -1.0,
                (other.group2()[3] * self[e2]) * -1.0,
            ]) + (crate::swizzle!(other.group3(), 2, 0, 1, 3) * crate::swizzle!(self.group0(), 1, 2, 0, 3)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group3()[0] * self[e2]) * -1.0,
                (other.group3()[1] * self[e2]) * -1.0,
                (other.group3()[2] * self[e2]) * -1.0,
                (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]) + (other.group1()[3] * self.group0()[3]),
            ]) + (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group0()[0]]) * crate::swizzle!(self.group0(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[1] * self.group0()[2]) + (other.group2()[0] * self.group0()[3]),
                (other.group1()[2] * self.group0()[0]) + (other.group2()[1] * self.group0()[3]),
                (other.group1()[0] * self.group0()[1]) + (other.group2()[2] * self.group0()[3]),
                -(other.group2()[1] * self.group0()[1]) - (other.group2()[2] * self.group0()[2]),
            ]) - (Simd32x4::from(self[e2]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiDualNum> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self[e2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1] * self[e2]]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[1]) * self.group0(),
        );
    }
}
impl Wedge<AntiFlatPoint> for RoundPoint {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[0] * self.group0()[3],
                other.group0()[1] * self.group0()[3],
                other.group0()[2] * self.group0()[3],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group0()[3] * self[e2]),
            ]),
            // e1234
            other.group0()[3] * self.group0()[3],
        );
    }
}
impl Wedge<AntiFlector> for RoundPoint {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        9       22        0
    //  no simd        9       24        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[2] * self.group0()[1]) - (other.group1()[1] * self.group0()[2]),
                (other.group1()[0] * self.group0()[2]) - (other.group1()[2] * self.group0()[0]),
                (other.group1()[1] * self.group0()[0]) - (other.group1()[0] * self.group0()[1]),
                other.group1()[3] * self.group0()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group1()[3] * self.group0()[0]) - (other.group1()[0] * self[e2]),
                (other.group1()[3] * self.group0()[1]) - (other.group1()[1] * self[e2]),
                (other.group1()[3] * self.group0()[2]) - (other.group1()[2] * self[e2]),
                other.group0()[3] * self.group0()[3],
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[0] * self.group0()[3],
                other.group0()[1] * self.group0()[3],
                other.group0()[2] * self.group0()[3],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group0()[3] * self[e2]),
            ]),
        );
    }
}
impl Wedge<AntiLine> for RoundPoint {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        8       18        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(self.group0()[3]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group1()[0] * self.group0()[3],
                other.group1()[1] * self.group0()[3],
                other.group1()[2] * self.group0()[3],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]),
            // e235, e315, e125
            (Simd32x3::from(self[e2]) * other.group0()) + (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]) * crate::swizzle!(other.group1(), 2, 0, 1))
                - (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * crate::swizzle!(other.group1(), 1, 2, 0)),
        );
    }
}
impl Wedge<AntiMotor> for RoundPoint {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       18        0
    //  no simd        8       24        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group1()[0] * self.group0()[3],
                other.group1()[1] * self.group0()[3],
                other.group1()[2] * self.group0()[3],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[0] * self[e2]) + (other.group1()[2] * self.group0()[1]) - (other.group1()[1] * self.group0()[2]),
                (other.group0()[1] * self[e2]) + (other.group1()[0] * self.group0()[2]) - (other.group1()[2] * self.group0()[0]),
                (other.group0()[2] * self[e2]) + (other.group1()[1] * self.group0()[0]) - (other.group1()[0] * self.group0()[1]),
                other.group0()[3] * self[e2],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[3]) * self.group0(),
        );
    }
}
impl Wedge<AntiPlane> for RoundPoint {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        6       16        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[2] * self.group0()[1]) - (other.group0()[1] * self.group0()[2]),
                (other.group0()[0] * self.group0()[2]) - (other.group0()[2] * self.group0()[0]),
                (other.group0()[1] * self.group0()[0]) - (other.group0()[0] * self.group0()[1]),
                other.group0()[3] * self.group0()[3],
            ]),
            // e15, e25, e35
            (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self[e2]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
        );
    }
}
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
            Simd32x4::from([
                (other.group2()[0] * self.group0()[3]) + (other.group1()[1] * self.group0()[2]),
                (other.group2()[1] * self.group0()[3]) + (other.group1()[2] * self.group0()[0]),
                (other.group2()[2] * self.group0()[3]) + (other.group1()[0] * self.group0()[1]),
                -(other.group2()[1] * self.group0()[1]) - (other.group2()[2] * self.group0()[2]),
            ]) - (Simd32x4::from(self[e2]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
            // e1234
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]) + (other.group1()[3] * self.group0()[3]),
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
            Simd32x4::from([
                (other.group1()[1] * self.group0()[2]) + (other.group2()[0] * self.group0()[3]),
                (other.group1()[2] * self.group0()[0]) + (other.group2()[1] * self.group0()[3]),
                (other.group1()[0] * self.group0()[1]) + (other.group2()[2] * self.group0()[3]),
                -(other.group2()[1] * self.group0()[1]) - (other.group2()[2] * self.group0()[2]),
            ]) - (Simd32x4::from(self[e2]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
            // e1234
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]) + (other.group1()[3] * self.group0()[3]),
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
            (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self[e2]) + (other.group2()[0] * self.group0()[3]),
                (other.group0()[1] * self[e2]) + (other.group2()[1] * self.group0()[3]),
                (other.group0()[2] * self[e2]) + (other.group2()[2] * self.group0()[3]),
                -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) - (crate::swizzle!(other.group1(), 3, 3, 3, 0) * crate::swizzle!(self.group0(), 0, 1, 2, 0)),
            // e235, e315, e125
            (Simd32x3::from(self[e2]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]) * crate::swizzle!(other.group2(), 2, 0, 1))
                - (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * crate::swizzle!(other.group2(), 1, 2, 0)),
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
            (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self[e2]) + (other.group2()[0] * self.group0()[3]),
                (other.group0()[1] * self[e2]) + (other.group2()[1] * self.group0()[3]),
                (other.group0()[2] * self[e2]) + (other.group2()[2] * self.group0()[3]),
                -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) - (crate::swizzle!(other.group1(), 3, 3, 3, 0) * crate::swizzle!(self.group0(), 0, 1, 2, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group2()[1] * self.group0()[2]) * -1.0,
                (other.group2()[2] * self.group0()[0]) * -1.0,
                (other.group2()[0] * self.group0()[1]) * -1.0,
                (other.group3()[1] * self.group0()[1]) + (other.group3()[2] * self.group0()[2]) + (other.group3()[3] * self.group0()[3]),
            ]) + (Simd32x4::from(self[e2]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]))
                + (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<DualNum> for RoundPoint {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn wedge(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self[e2] * -1.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
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
            (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e235, e315, e125
            Simd32x3::from([
                (other.group0()[2] * self.group0()[1]) - (other.group0()[1] * self.group0()[2]),
                (other.group0()[0] * self.group0()[2]) - (other.group0()[2] * self.group0()[0]),
                (other.group0()[1] * self.group0()[0]) - (other.group0()[0] * self.group0()[1]),
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
            Simd32x4::from([
                (other.group0()[3] * self.group0()[0]) * -1.0,
                (other.group0()[3] * self.group0()[1]) * -1.0,
                (other.group0()[3] * self.group0()[2]) * -1.0,
                (other.group1()[1] * self.group0()[1]) + (other.group1()[2] * self.group0()[2]) + (other.group1()[3] * self.group0()[3]),
            ]) + (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]) * crate::swizzle!(self.group0(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[2] * self.group0()[1]) - (other.group0()[1] * self.group0()[2]),
                (other.group0()[0] * self.group0()[2]) - (other.group0()[2] * self.group0()[0]),
                (other.group0()[1] * self.group0()[0]) - (other.group0()[0] * self.group0()[1]),
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
            Simd32x4::from([
                (other.group0()[1] * self.group0()[2]) + (other.group1()[0] * self.group0()[3]),
                (other.group0()[2] * self.group0()[0]) + (other.group1()[1] * self.group0()[3]),
                (other.group0()[0] * self.group0()[1]) + (other.group1()[2] * self.group0()[3]),
                -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
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
            Simd32x4::from(other.group1()[3]) * self.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group0()[2]) + (other.group1()[0] * self.group0()[3]),
                (other.group0()[2] * self.group0()[0]) + (other.group1()[1] * self.group0()[3]),
                (other.group0()[0] * self.group0()[1]) + (other.group1()[2] * self.group0()[3]),
                -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
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
                (other.group9()[0] * self.group0()[0])
                    + (other.group9()[1] * self.group0()[1])
                    + (other.group9()[2] * self.group0()[2])
                    + (other.group9()[3] * self.group0()[3])
                    + (other[e45] * self[e2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[0]) * self.group0(),
            // e5
            other.group0()[0] * self[e2],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e1]) * self.group0()) - (Simd32x4::from(self[e2]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12
            Simd32x3::from([
                (other.group1()[2] * self.group0()[1]) - (other.group1()[1] * self.group0()[2]),
                (other.group1()[0] * self.group0()[2]) - (other.group1()[2] * self.group0()[0]),
                (other.group1()[1] * self.group0()[0]) - (other.group1()[0] * self.group0()[1]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group4()[0] * self[e2]) + (other.group3()[0] * self.group0()[3]),
                (other.group4()[1] * self[e2]) + (other.group3()[1] * self.group0()[3]),
                (other.group4()[2] * self[e2]) + (other.group3()[2] * self.group0()[3]),
                -(other.group5()[1] * self.group0()[1]) - (other.group5()[2] * self.group0()[2]),
            ]) - (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group5()[0]]) * crate::swizzle!(self.group0(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from(self.group0()[3]) * other.group5())
                + (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * crate::swizzle!(other.group4(), 1, 2, 0))
                - (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]) * crate::swizzle!(other.group4(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (other.group3()[2] * self.group0()[1]) - (other.group3()[1] * self.group0()[2]),
                (other.group3()[0] * self.group0()[2]) - (other.group3()[2] * self.group0()[0]),
                (other.group3()[1] * self.group0()[0]) - (other.group3()[0] * self.group0()[1]),
            ]) + (Simd32x3::from(self[e2]) * other.group5()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group8()[0] * self.group0()[3]) + (other.group6()[1] * self.group0()[2]),
                (other.group8()[1] * self.group0()[3]) + (other.group6()[2] * self.group0()[0]),
                (other.group8()[2] * self.group0()[3]) + (other.group6()[0] * self.group0()[1]),
                -(other.group8()[1] * self.group0()[1]) - (other.group8()[2] * self.group0()[2]),
            ]) - (Simd32x4::from(self[e2]) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], other.group6()[3]]))
                - (Simd32x4::from([other.group6()[2], other.group6()[0], other.group6()[1], other.group8()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
            // e1234
            (other.group7()[0] * self.group0()[0]) + (other.group7()[1] * self.group0()[1]) + (other.group7()[2] * self.group0()[2]) + (other.group6()[3] * self.group0()[3]),
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
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]) + (other.group0()[3] * self.group0()[3]),
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
            (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12, e45
            (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other[e2]]) * crate::swizzle!(self.group0(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self[e2]]) * crate::swizzle!(other.group0(), 1, 2, 0, 3)),
            // e15, e25, e35
            (Simd32x3::from(other[e2]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self[e2]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
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
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0(), /* e5 */ self[e2] * other[scalar]);
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
            (self.group0()[0] * other.group0()[0])
                + (self.group0()[1] * other.group0()[1])
                + (self.group0()[2] * other.group0()[2])
                + (self.group0()[3] * other.group0()[3])
                + (self[e2] * other[e4315]),
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
            (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))
                - (Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12, e45
            (Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[3]]) * crate::swizzle!(self.group0(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self[e2]]) * crate::swizzle!(other.group3(), 1, 2, 0, 3)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                (other.group3()[0] * self[e2]) * -1.0,
                (other.group3()[1] * self[e2]) * -1.0,
                (other.group3()[2] * self[e2]) * -1.0,
                (self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2]) + (self.group0()[3] * other.group1()[3]),
            ]) + (Simd32x4::from([other.group2()[3], other.group2()[3], other.group2()[3], other.group0()[0]]) * crate::swizzle!(self.group0(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (self.group0()[2] * other.group1()[1]) + (self.group0()[3] * other.group2()[0]),
                (self.group0()[0] * other.group1()[2]) + (self.group0()[3] * other.group2()[1]),
                (self.group0()[1] * other.group1()[0]) + (self.group0()[3] * other.group2()[2]),
                -(self.group0()[1] * other.group2()[1]) - (self.group0()[2] * other.group2()[2]),
            ]) - (Simd32x4::from(self[e2]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
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
            Simd32x4::from([
                (self.group0()[1] * other.group0()[2]) * -1.0,
                (self.group0()[2] * other.group0()[0]) * -1.0,
                (self.group0()[0] * other.group0()[1]) * -1.0,
                (self.group0()[2] * other.group3()[2]) + (self.group0()[3] * other.group3()[3]) + (other.group2()[3] * self[e2]),
            ]) + (Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group3()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1, 0))
                + (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group3()[1]]) * crate::swizzle!(self.group0(), 3, 3, 3, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group0()[3] * other.group2()[0]) + (other.group0()[0] * self[e2]),
                (self.group0()[3] * other.group2()[1]) + (other.group0()[1] * self[e2]),
                (self.group0()[3] * other.group2()[2]) + (other.group0()[2] * self[e2]),
                -(self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]),
            ]) - (crate::swizzle!(self.group0(), 0, 1, 2, 0) * crate::swizzle!(other.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group0()[1] * other.group2()[2]) + (other.group1()[0] * self[e2]) - (self.group0()[2] * other.group2()[1]),
                (self.group0()[2] * other.group2()[0]) + (other.group1()[1] * self[e2]) - (self.group0()[0] * other.group2()[2]),
                (self.group0()[0] * other.group2()[1]) + (other.group1()[2] * self[e2]) - (self.group0()[1] * other.group2()[0]),
                other.group0()[3] * self[e2],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[3]) * self.group0(),
        );
    }
}
impl std::ops::Div<wedge> for Scalar {
    type Output = wedge_partial<Scalar>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for Scalar {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       11        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, scalar
            Simd32x4::from(self[scalar]) * other.group2(),
        );
    }
}
impl Wedge<AntiDipoleInversion> for Scalar {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       15        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from(self[scalar]) * other.group2(),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * other.group3(),
        );
    }
}
impl Wedge<AntiDualNum> for Scalar {
    type Output = AntiDualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(self[scalar]) * other.group0());
    }
}
impl Wedge<AntiFlatPoint> for Scalar {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(self[scalar]) * other.group0());
    }
}
impl Wedge<AntiFlector> for Scalar {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from(self[scalar]) * other.group0(),
            // e1, e2, e3, e5
            Simd32x4::from(self[scalar]) * other.group1(),
        );
    }
}
impl Wedge<AntiLine> for Scalar {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        use crate::elements::*;
        return AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group0(),
            // e15, e25, e35
            Simd32x3::from(self[scalar]) * other.group1(),
        );
    }
}
impl Wedge<AntiMotor> for Scalar {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group0(),
            // e15, e25, e35, e3215
            Simd32x4::from(self[scalar]) * other.group1(),
        );
    }
}
impl Wedge<AntiPlane> for Scalar {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(self[scalar]) * other.group0());
    }
}
impl Wedge<AntiScalar> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other[e12345] * self[scalar]);
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
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * other.group2(),
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
            Simd32x3::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from(self[scalar]) * other.group2(),
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
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35
            Simd32x3::from(self[scalar]) * other.group2(),
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
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[scalar]) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group3(),
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
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(self[scalar]) * other.group0());
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
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(self[scalar]) * other.group0());
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
            Simd32x4::from(self[scalar]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group1(),
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
            Simd32x3::from(self[scalar]) * other.group0(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * other.group1(),
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
            Simd32x4::from(self[scalar]) * other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * other.group1(),
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
            Simd32x2::from(self[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group1(),
            // e5
            other[e1] * self[scalar],
            // e15, e25, e35, e45
            Simd32x4::from(self[scalar]) * other.group3(),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group4(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group5(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group6(),
            // e423, e431, e412
            Simd32x3::from(self[scalar]) * other.group7(),
            // e235, e315, e125
            Simd32x3::from(self[scalar]) * other.group8(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group9(),
            // e1234
            other[e45] * self[scalar],
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
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self[scalar]) * other.group0());
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
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[scalar]) * other.group0(), /* e5 */ other[e2] * self[scalar]);
    }
}
impl Wedge<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]);
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
            Simd32x4::from(self[scalar]) * other.group0(),
            // e1234
            self[scalar] * other[e4315],
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
            Simd32x4::from(self[scalar]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self[scalar]) * other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(self[scalar]) * other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group3(),
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
            Simd32x4::from(self[scalar]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self[scalar]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(self[scalar]) * other.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(self[scalar]) * other.group3(),
        );
    }
}
impl std::ops::Div<wedge> for Sphere {
    type Output = wedge_partial<Sphere>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other.group2()[3]) * self.group0(),
            // e1234
            other.group2()[3] * self[e4315],
        );
    }
}
impl Wedge<AntiDipoleInversion> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        5        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other.group2()[3] * self.group0()[3])
                + (other.group3()[0] * self.group0()[0])
                + (other.group3()[1] * self.group0()[1])
                + (other.group3()[2] * self.group0()[2])
                + (other.group3()[3] * self[e4315]),
        );
    }
}
impl Wedge<AntiDualNum> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other.group0()[1]) * self.group0(),
            // e1234
            other.group0()[1] * self[e4315],
        );
    }
}
impl Wedge<AntiFlector> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other.group1()[0] * self.group0()[0]) + (other.group1()[1] * self.group0()[1]) + (other.group1()[2] * self.group0()[2]) + (other.group1()[3] * self[e4315]),
        );
    }
}
impl Wedge<AntiMotor> for Sphere {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other.group0()[3]) * self.group0(),
            // e1234
            other.group0()[3] * self[e4315],
        );
    }
}
impl Wedge<AntiPlane> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        4        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(
            // e12345
            (other.group0()[0] * self.group0()[0]) + (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]) + (other.group0()[3] * self[e4315]),
        );
    }
}
impl Wedge<DualNum> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return AntiScalar::from_groups(/* e12345 */ other.group0()[0] * self.group0()[3]);
    }
}
impl Wedge<Motor> for Sphere {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ other.group1()[3] * self[e4315]);
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
                (other.group1()[0] * self.group0()[0])
                    + (other.group1()[1] * self.group0()[1])
                    + (other.group1()[2] * self.group0()[2])
                    + (other.group1()[3] * self.group0()[3])
                    + (other[e1] * self[e4315]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other.group0()[0]) * self.group0(),
            // e1234
            other.group0()[0] * self[e4315],
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
            (other.group0()[0] * self.group0()[0])
                + (other.group0()[1] * self.group0()[1])
                + (other.group0()[2] * self.group0()[2])
                + (other.group0()[3] * self.group0()[3])
                + (other[e2] * self[e4315]),
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
            Simd32x4::from(other[scalar]) * self.group0(),
            // e1234
            other[scalar] * self[e4315],
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
            (self.group0()[0] * other.group3()[0])
                + (self.group0()[1] * other.group3()[1])
                + (self.group0()[2] * other.group3()[2])
                + (self.group0()[3] * other.group3()[3])
                + (other.group2()[3] * self[e4315]),
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
            Simd32x4::from(other.group0()[3]) * self.group0(),
            // e1234
            other.group0()[3] * self[e4315],
        );
    }
}
impl std::ops::Div<wedge> for VersorEven {
    type Output = wedge_partial<VersorEven>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       31       44        0
    //  no simd       40       56        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[1] * self.group3()[2]) + (other.group1()[0] * self.group3()[3]) - (other.group0()[2] * self.group3()[1]),
                (other.group0()[2] * self.group3()[0]) + (other.group1()[1] * self.group3()[3]) - (other.group0()[0] * self.group3()[2]),
                (other.group0()[0] * self.group3()[1]) + (other.group1()[2] * self.group3()[3]) - (other.group0()[1] * self.group3()[0]),
                -(other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])
                    - (other.group1()[3] * self.group1()[3])
                    - (other.group2()[0] * self.group0()[0])
                    - (other.group2()[1] * self.group0()[1])
                    - (other.group2()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(other.group2()[3]) * self.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self.group2()[3]) + (other.group2()[3] * self.group1()[0]),
                (other.group0()[1] * self.group2()[3]) + (other.group2()[3] * self.group1()[1]),
                (other.group0()[2] * self.group2()[3]) + (other.group2()[3] * self.group1()[2]),
                -(other.group1()[1] * self.group3()[1]) - (other.group1()[2] * self.group3()[2]),
            ]) + (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group1()[3]]) * other.group2())
                - (crate::swizzle!(other.group1(), 3, 3, 3, 0) * crate::swizzle!(self.group3(), 0, 1, 2, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] * self.group2()[3]) + (other.group2()[2] * self.group3()[1]) + (other.group2()[3] * self.group2()[0]) - (other.group2()[1] * self.group3()[2]),
                (other.group1()[1] * self.group2()[3]) + (other.group2()[0] * self.group3()[2]) + (other.group2()[3] * self.group2()[1]) - (other.group2()[2] * self.group3()[0]),
                (other.group1()[2] * self.group2()[3]) + (other.group2()[1] * self.group3()[0]) + (other.group2()[3] * self.group2()[2]) - (other.group2()[0] * self.group3()[1]),
                other.group2()[3] * self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group2()[3]) * self.group3(),
        );
    }
}
impl Wedge<AntiDipoleInversion> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       26        0
    //    simd3        1        2        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       22       36        0
    //  no simd       48       64        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))
                - (Simd32x3::from(other.group2()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]])),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group3()[1] * self.group3()[2]) * -1.0,
                (other.group3()[2] * self.group3()[0]) * -1.0,
                (other.group3()[0] * self.group3()[1]) * -1.0,
                (other.group2()[3] * self.group2()[3]) * -1.0,
            ]) + (crate::swizzle!(other.group3(), 2, 0, 1, 3) * crate::swizzle!(self.group3(), 1, 2, 0, 3)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (other.group0()[1] * self.group3()[1]) + (other.group0()[2] * self.group3()[2]) + (other.group1()[3] * self.group3()[3])
                    - (other.group2()[3] * self.group1()[3])
                    - (other.group3()[1] * self.group0()[1])
                    - (other.group3()[2] * self.group0()[2]),
            ]) + (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group0()[0]]) * crate::swizzle!(self.group3(), 0, 1, 2, 0))
                - (Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[0]]) * crate::swizzle!(other.group3(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[1] * self.group3()[2]) + (other.group2()[0] * self.group3()[3]) - (other.group3()[2] * self.group1()[1]),
                (other.group1()[2] * self.group3()[0]) + (other.group2()[1] * self.group3()[3]) - (other.group3()[0] * self.group1()[2]),
                (other.group1()[0] * self.group3()[1]) + (other.group2()[2] * self.group3()[3]) - (other.group3()[1] * self.group1()[0]),
                (other.group3()[2] * self.group2()[2]) + (other.group3()[3] * self.group1()[3]) - (other.group2()[2] * self.group3()[2]),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[1]]) * crate::swizzle!(other.group3(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0))
                - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group3()[1]]) * crate::swizzle!(other.group2(), 3, 3, 3, 1)),
        );
    }
}
impl Wedge<AntiDualNum> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                other.group0()[1] * self.group0()[0],
                other.group0()[1] * self.group0()[1],
                other.group0()[1] * self.group0()[2],
                (other.group0()[0] * self.group2()[3]) + (other.group0()[1] * self.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from(other.group0()[1]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other.group0()[1]) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[1]) * self.group3(),
        );
    }
}
impl Wedge<AntiFlatPoint> for VersorEven {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        return Sphere::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[0] * self.group3()[3],
                other.group0()[1] * self.group3()[3],
                other.group0()[2] * self.group3()[3],
                -(other.group0()[0] * self.group3()[0]) - (other.group0()[1] * self.group3()[1]) - (other.group0()[2] * self.group3()[2]) - (other.group0()[3] * self.group2()[3]),
            ]),
            // e1234
            other.group0()[3] * self.group3()[3],
        );
    }
}
impl Wedge<AntiFlector> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       21        0
    //    simd3        0        1        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       28       40        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group1()[2] * self.group3()[1]) - (other.group1()[1] * self.group3()[2]),
                (other.group1()[0] * self.group3()[2]) - (other.group1()[2] * self.group3()[0]),
                (other.group1()[1] * self.group3()[0]) - (other.group1()[0] * self.group3()[1]),
                other.group1()[3] * self.group3()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, -(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2])])
                + (Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[3], other.group0()[3]]) * self.group3())
                - (Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[0]]) * crate::swizzle!(other.group1(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] * self.group3()[3]) - (other.group1()[2] * self.group1()[1]),
                (other.group0()[1] * self.group3()[3]) - (other.group1()[0] * self.group1()[2]),
                (other.group0()[2] * self.group3()[3]) - (other.group1()[1] * self.group1()[0]),
                (other.group1()[2] * self.group2()[2]) + (other.group1()[3] * self.group1()[3])
                    - (other.group0()[0] * self.group3()[0])
                    - (other.group0()[1] * self.group3()[1])
                    - (other.group0()[2] * self.group3()[2])
                    - (other.group0()[3] * self.group2()[3]),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[1]]) * crate::swizzle!(other.group1(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiLine> for VersorEven {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       13       24        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(self.group3()[3]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group1()[0] * self.group3()[3],
                other.group1()[1] * self.group3()[3],
                other.group1()[2] * self.group3()[3],
                -(other.group0()[0] * self.group3()[0]) - (other.group0()[1] * self.group3()[1]) - (other.group0()[2] * self.group3()[2]),
            ]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group0()[0] * self.group2()[3]) + (other.group1()[2] * self.group3()[1]) - (other.group1()[1] * self.group3()[2]),
                (other.group0()[1] * self.group2()[3]) + (other.group1()[0] * self.group3()[2]) - (other.group1()[2] * self.group3()[0]),
                (other.group0()[2] * self.group2()[3]) + (other.group1()[1] * self.group3()[0]) - (other.group1()[0] * self.group3()[1]),
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2]),
            ]),
        );
    }
}
impl Wedge<AntiMotor> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       19       32        0
    //  no simd       25       41        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                other.group0()[3] * self.group0()[0],
                other.group0()[3] * self.group0()[1],
                other.group0()[3] * self.group0()[2],
                (other.group1()[3] * self.group3()[3])
                    - (other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2]),
            ]) + (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group0()[3]]) * other.group0()),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group1()[0] * self.group3()[3],
                other.group1()[1] * self.group3()[3],
                other.group1()[2] * self.group3()[3],
                -(other.group0()[0] * self.group3()[0]) - (other.group0()[1] * self.group3()[1]) - (other.group0()[2] * self.group3()[2]),
            ]) + (Simd32x4::from(other.group0()[3]) * self.group1()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[0] * self.group2()[3]) + (other.group0()[3] * self.group2()[0]) + (other.group1()[2] * self.group3()[1]) - (other.group1()[1] * self.group3()[2]),
                (other.group0()[1] * self.group2()[3]) + (other.group0()[3] * self.group2()[1]) + (other.group1()[0] * self.group3()[2]) - (other.group1()[2] * self.group3()[0]),
                (other.group0()[2] * self.group2()[3]) + (other.group0()[3] * self.group2()[2]) + (other.group1()[1] * self.group3()[0]) - (other.group1()[0] * self.group3()[1]),
                other.group0()[3] * self.group2()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[3]) * self.group3(),
        );
    }
}
impl Wedge<AntiPlane> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       20        0
    //    simd3        0        1        0
    //    simd4        3        3        0
    // Totals...
    // yes simd        8       24        0
    //  no simd       17       35        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[2] * self.group3()[1]) - (other.group0()[1] * self.group3()[2]),
                (other.group0()[0] * self.group3()[2]) - (other.group0()[2] * self.group3()[0]),
                (other.group0()[1] * self.group3()[0]) - (other.group0()[0] * self.group3()[1]),
                other.group0()[3] * self.group3()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other.group0()[3] * self.group3()[0],
                other.group0()[3] * self.group3()[1],
                other.group0()[3] * self.group3()[2],
                -(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]) - (Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[0]]) * crate::swizzle!(other.group0(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[2] * self.group1()[1]) * -1.0,
                (other.group0()[0] * self.group1()[2]) * -1.0,
                (other.group0()[1] * self.group1()[0]) * -1.0,
                (other.group0()[2] * self.group2()[2]) + (other.group0()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[1]]) * crate::swizzle!(other.group0(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
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
            Simd32x4::from([
                (other.group2()[0] * self.group3()[3]) + (other.group1()[1] * self.group3()[2]),
                (other.group2()[1] * self.group3()[3]) + (other.group1()[2] * self.group3()[0]),
                (other.group2()[2] * self.group3()[3]) + (other.group1()[0] * self.group3()[1]),
                -(other.group2()[1] * self.group3()[1]) - (other.group2()[2] * self.group3()[2]),
            ]) - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0)),
            // e1234
            (other.group0()[0] * self.group3()[0]) + (other.group0()[1] * self.group3()[1]) + (other.group0()[2] * self.group3()[2]) + (other.group1()[3] * self.group3()[3]),
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
            Simd32x4::from([
                (other.group1()[1] * self.group3()[2]) + (other.group2()[0] * self.group3()[3]),
                (other.group1()[2] * self.group3()[0]) + (other.group2()[1] * self.group3()[3]),
                (other.group1()[0] * self.group3()[1]) + (other.group2()[2] * self.group3()[3]),
                -(other.group2()[1] * self.group3()[1]) - (other.group2()[2] * self.group3()[2]),
            ]) - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0)),
            // e1234
            (other.group0()[0] * self.group3()[0]) + (other.group0()[1] * self.group3()[1]) + (other.group0()[2] * self.group3()[2]) + (other.group1()[3] * self.group3()[3]),
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
            (Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self.group2()[3]) + (other.group2()[0] * self.group3()[3]),
                (other.group0()[1] * self.group2()[3]) + (other.group2()[1] * self.group3()[3]),
                (other.group0()[2] * self.group2()[3]) + (other.group2()[2] * self.group3()[3]),
                -(other.group1()[1] * self.group3()[1]) - (other.group1()[2] * self.group3()[2]),
            ]) - (crate::swizzle!(other.group1(), 3, 3, 3, 0) * crate::swizzle!(self.group3(), 0, 1, 2, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                (other.group2()[2] * self.group3()[1]) + (other.group1()[0] * self.group2()[3]) - (other.group2()[1] * self.group3()[2]),
                (other.group2()[0] * self.group3()[2]) + (other.group1()[1] * self.group2()[3]) - (other.group2()[2] * self.group3()[0]),
                (other.group2()[1] * self.group3()[0]) + (other.group1()[2] * self.group2()[3]) - (other.group2()[0] * self.group3()[1]),
                -(other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (other.group2()[0] * self.group0()[0])
                    - (other.group2()[1] * self.group0()[1])
                    - (other.group2()[2] * self.group0()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])
                    - (other.group1()[3] * self.group1()[3]),
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
            (Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]) * crate::swizzle!(other.group0(), 1, 2, 0))
                - (Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[0] * self.group2()[3]) + (other.group2()[0] * self.group3()[3]),
                (other.group0()[1] * self.group2()[3]) + (other.group2()[1] * self.group3()[3]),
                (other.group0()[2] * self.group2()[3]) + (other.group2()[2] * self.group3()[3]),
                -(other.group1()[1] * self.group3()[1]) - (other.group1()[2] * self.group3()[2]),
            ]) - (crate::swizzle!(other.group1(), 3, 3, 3, 0) * crate::swizzle!(self.group3(), 0, 1, 2, 0)),
            // e235, e315, e125, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (other.group3()[1] * self.group3()[1]) + (other.group3()[2] * self.group3()[2]) + (other.group3()[3] * self.group3()[3])
                    - (other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])
                    - (other.group1()[3] * self.group1()[3])
                    - (other.group2()[1] * self.group0()[1])
                    - (other.group2()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group2()[3]]))
                + (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group0()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<DualNum> for VersorEven {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       16        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group2()[3] * -1.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group1()[3] * -1.0]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[0] * self.group2()[0] * -1.0,
                other.group0()[0] * self.group2()[1] * -1.0,
                other.group0()[0] * self.group2()[2] * -1.0,
                0.0,
            ]),
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
            Simd32x4::from([
                other.group0()[0] * self.group3()[3],
                other.group0()[1] * self.group3()[3],
                other.group0()[2] * self.group3()[3],
                -(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group0()[3] * self.group1()[3]),
            ]) - (Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group0()[0]]) * crate::swizzle!(other.group0(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[2] * self.group3()[1]) - (other.group0()[1] * self.group3()[2]),
                (other.group0()[0] * self.group3()[2]) - (other.group0()[2] * self.group3()[0]),
                (other.group0()[1] * self.group3()[0]) - (other.group0()[0] * self.group3()[1]),
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (other.group1()[1] * self.group3()[1]) + (other.group1()[2] * self.group3()[2]) + (other.group1()[3] * self.group3()[3])
                    - (other.group0()[1] * self.group0()[1])
                    - (other.group0()[2] * self.group0()[2])
                    - (other.group0()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]) * crate::swizzle!(self.group3(), 3, 3, 3, 0))
                - (Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group0()[0]]) * crate::swizzle!(other.group0(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[2] * self.group3()[1]) - (other.group0()[1] * self.group3()[2]),
                (other.group0()[0] * self.group3()[2]) - (other.group0()[2] * self.group3()[0]),
                (other.group0()[1] * self.group3()[0]) - (other.group0()[0] * self.group3()[1]),
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
            Simd32x4::from([
                (other.group0()[1] * self.group3()[2]) + (other.group1()[0] * self.group3()[3]),
                (other.group0()[2] * self.group3()[0]) + (other.group1()[1] * self.group3()[3]),
                (other.group0()[0] * self.group3()[1]) + (other.group1()[2] * self.group3()[3]),
                -(other.group1()[1] * self.group3()[1]) - (other.group1()[2] * self.group3()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0)),
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
            Simd32x4::from(other.group1()[3]) * self.group3(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group3()[2]) + (other.group1()[3] * self.group0()[0]),
                (other.group0()[2] * self.group3()[0]) + (other.group1()[3] * self.group0()[1]),
                (other.group0()[0] * self.group3()[1]) + (other.group1()[3] * self.group0()[2]),
                -(other.group1()[1] * self.group3()[1]) - (other.group1()[2] * self.group3()[2]),
            ]) + (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group1()[3]]) * other.group1())
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0)),
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
                (other.group0()[0] * self.group0()[3])
                    + (other.group9()[0] * self.group3()[0])
                    + (other.group9()[1] * self.group3()[1])
                    + (other.group9()[2] * self.group3()[2])
                    + (other.group9()[3] * self.group3()[3])
                    + (self.group2()[3] * other[e45])
                    - (other.group4()[0] * self.group2()[0])
                    - (other.group4()[1] * self.group2()[1])
                    - (other.group4()[2] * self.group2()[2])
                    - (other.group5()[0] * self.group1()[0])
                    - (other.group5()[1] * self.group1()[1])
                    - (other.group5()[2] * self.group1()[2])
                    - (other.group3()[0] * self.group0()[0])
                    - (other.group3()[1] * self.group0()[1])
                    - (other.group3()[2] * self.group0()[2])
                    - (other.group3()[3] * self.group1()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[0]) * self.group3(),
            // e5
            other.group0()[0] * self.group2()[3],
            // e15, e25, e35, e45
            (Simd32x4::from(other[e1]) * self.group3()) - (Simd32x4::from(self.group2()[3]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]])),
            // e23, e31, e12
            Simd32x3::from([
                (other.group1()[2] * self.group3()[1]) - (other.group1()[1] * self.group3()[2]),
                (other.group1()[0] * self.group3()[2]) - (other.group1()[2] * self.group3()[0]),
                (other.group1()[1] * self.group3()[0]) - (other.group1()[0] * self.group3()[1]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group4()[0] * self.group2()[3]) + (other.group3()[0] * self.group3()[3]),
                (other.group4()[1] * self.group2()[3]) + (other.group3()[1] * self.group3()[3]),
                (other.group4()[2] * self.group2()[3]) + (other.group3()[2] * self.group3()[3]),
                -(other.group5()[1] * self.group3()[1]) - (other.group5()[2] * self.group3()[2]),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group1())
                - (Simd32x4::from([other.group3()[3], other.group3()[3], other.group3()[3], other.group5()[0]]) * crate::swizzle!(self.group3(), 0, 1, 2, 0)),
            // e423, e431, e412
            (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self.group3()[3]) * other.group5())
                + (Simd32x3::from([self.group3()[2], self.group3()[0], self.group3()[1]]) * crate::swizzle!(other.group4(), 1, 2, 0))
                - (Simd32x3::from([self.group3()[1], self.group3()[2], self.group3()[0]]) * crate::swizzle!(other.group4(), 2, 0, 1)),
            // e235, e315, e125
            Simd32x3::from([
                (other.group3()[2] * self.group3()[1]) - (other.group3()[1] * self.group3()[2]),
                (other.group3()[0] * self.group3()[2]) - (other.group3()[2] * self.group3()[0]),
                (other.group3()[1] * self.group3()[0]) - (other.group3()[0] * self.group3()[1]),
            ]) + (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]))
                + (Simd32x3::from(self.group2()[3]) * other.group5()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group8()[0] * self.group3()[3]) + (other.group6()[1] * self.group3()[2]) - (other.group1()[2] * self.group1()[1]) - (other.group1()[3] * self.group2()[0]),
                (other.group8()[1] * self.group3()[3]) + (other.group6()[2] * self.group3()[0]) - (other.group1()[0] * self.group1()[2]) - (other.group1()[3] * self.group2()[1]),
                (other.group8()[2] * self.group3()[3]) + (other.group6()[0] * self.group3()[1]) - (other.group1()[1] * self.group1()[0]) - (other.group1()[3] * self.group2()[2]),
                (other.group1()[1] * self.group2()[1]) + (other.group1()[2] * self.group2()[2]) - (other.group8()[1] * self.group3()[1]) - (other.group8()[2] * self.group3()[2]),
            ]) + (Simd32x4::from(other[e1]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0))
                - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group7()[0], other.group7()[1], other.group7()[2], other.group6()[3]]))
                - (Simd32x4::from([other.group6()[2], other.group6()[0], other.group6()[1], other.group8()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0)),
            // e1234
            (other.group7()[0] * self.group3()[0]) + (other.group7()[1] * self.group3()[1]) + (other.group7()[2] * self.group3()[2]) + (other.group6()[3] * self.group3()[3])
                - (other.group1()[0] * self.group0()[0])
                - (other.group1()[1] * self.group0()[1])
                - (other.group1()[2] * self.group0()[2])
                - (other.group1()[3] * self.group1()[3]),
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
            (other.group0()[0] * self.group3()[0]) + (other.group0()[1] * self.group3()[1]) + (other.group0()[2] * self.group3()[2]) + (other.group0()[3] * self.group3()[3]),
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
            (Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]])),
            // e23, e31, e12, e45
            (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other[e2]]) * crate::swizzle!(self.group3(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[3]]) * crate::swizzle!(other.group0(), 1, 2, 0, 3)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                self.group3()[0] * other[e2],
                self.group3()[1] * other[e2],
                self.group3()[2] * other[e2],
                -(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group0()[3] * self.group1()[3]),
            ]) - (Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[0]]) * crate::swizzle!(other.group0(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                -(other.group0()[2] * self.group1()[1]) - (other.group0()[3] * self.group2()[0]),
                -(other.group0()[0] * self.group1()[2]) - (other.group0()[3] * self.group2()[1]),
                -(other.group0()[1] * self.group1()[0]) - (other.group0()[3] * self.group2()[2]),
                (other.group0()[1] * self.group2()[1]) + (other.group0()[2] * self.group2()[2]),
            ]) + (Simd32x4::from(other[e2]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
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
            Simd32x4::from(other[scalar]) * self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(other[scalar]) * self.group1(),
            // e235, e315, e125, e5
            Simd32x4::from(other[scalar]) * self.group2(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group3(),
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
            (other.group0()[0] * self.group3()[0])
                + (other.group0()[1] * self.group3()[1])
                + (other.group0()[2] * self.group3()[2])
                + (other.group0()[3] * self.group3()[3])
                + (self.group2()[3] * other[e4315]),
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
            (Simd32x3::from(self.group3()[3]) * Simd32x3::from([other.group3()[0], other.group3()[1], other.group3()[2]]))
                - (Simd32x3::from(other.group3()[3]) * Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]])),
            // e23, e31, e12, e45
            (Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[3]]) * crate::swizzle!(self.group3(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[3]]) * crate::swizzle!(other.group3(), 1, 2, 0, 3)),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (other.group0()[1] * self.group3()[1]) + (other.group0()[2] * self.group3()[2]) + (other.group1()[3] * self.group3()[3])
                    - (other.group3()[1] * self.group0()[1])
                    - (other.group3()[2] * self.group0()[2])
                    - (other.group3()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([other.group2()[3], other.group2()[3], other.group2()[3], other.group0()[0]]) * crate::swizzle!(self.group3(), 0, 1, 2, 0))
                - (Simd32x4::from([self.group2()[3], self.group2()[3], self.group2()[3], self.group0()[0]]) * crate::swizzle!(other.group3(), 0, 1, 2, 0)),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group1()[1] * self.group3()[2]) + (other.group2()[3] * self.group0()[0]) - (other.group3()[2] * self.group1()[1]) - (other.group3()[3] * self.group2()[0]),
                (other.group1()[2] * self.group3()[0]) + (other.group2()[3] * self.group0()[1]) - (other.group3()[0] * self.group1()[2]) - (other.group3()[3] * self.group2()[1]),
                (other.group1()[0] * self.group3()[1]) + (other.group2()[3] * self.group0()[2]) - (other.group3()[1] * self.group1()[0]) - (other.group3()[3] * self.group2()[2]),
                (other.group3()[1] * self.group2()[1]) + (other.group3()[2] * self.group2()[2]) - (other.group2()[1] * self.group3()[1]) - (other.group2()[2] * self.group3()[2]),
            ]) + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group2()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group3()[3], self.group3()[3], self.group3()[3], self.group1()[3]]) * other.group2())
                - (Simd32x4::from(self.group2()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group2()[0]]) * crate::swizzle!(self.group3(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<VersorOdd> for VersorEven {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       33        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       30       40        0
    //  no simd       48       61        0
    fn wedge(self, other: VersorOdd) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (self.group2()[3] * other.group2()[3]) + (self.group3()[2] * other.group3()[2]) + (self.group3()[3] * other.group3()[3])
                    - (self.group0()[0] * other.group2()[0])
                    - (self.group0()[1] * other.group2()[1])
                    - (self.group0()[2] * other.group2()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])
                    - (self.group1()[3] * other.group1()[3])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(other.group0()[3]) * self.group0())
                + (Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group3()[0]]) * crate::swizzle!(self.group3(), 2, 0, 1, 0))
                + (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group3()[1]]) * crate::swizzle!(self.group3(), 3, 3, 3, 1))
                - (Simd32x4::from([self.group3()[1], self.group3()[2], self.group3()[0], self.group2()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1, 0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (self.group2()[3] * other.group0()[0]) + (self.group3()[3] * other.group2()[0]),
                (self.group2()[3] * other.group0()[1]) + (self.group3()[3] * other.group2()[1]),
                (self.group2()[3] * other.group0()[2]) + (self.group3()[3] * other.group2()[2]),
                -(self.group3()[1] * other.group1()[1]) - (self.group3()[2] * other.group1()[2]),
            ]) + (Simd32x4::from(other.group0()[3]) * self.group1())
                - (crate::swizzle!(self.group3(), 0, 1, 2, 0) * crate::swizzle!(other.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (self.group2()[0] * other.group0()[3]) + (self.group2()[3] * other.group1()[0]) + (self.group3()[1] * other.group2()[2]) - (self.group3()[2] * other.group2()[1]),
                (self.group2()[1] * other.group0()[3]) + (self.group2()[3] * other.group1()[1]) + (self.group3()[2] * other.group2()[0]) - (self.group3()[0] * other.group2()[2]),
                (self.group2()[2] * other.group0()[3]) + (self.group2()[3] * other.group1()[2]) + (self.group3()[0] * other.group2()[1]) - (self.group3()[1] * other.group2()[0]),
                self.group2()[3] * other.group0()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[3]) * self.group3(),
        );
    }
}
impl std::ops::Div<wedge> for VersorOdd {
    type Output = wedge_partial<VersorOdd>;
    fn div(self, _rhs: wedge) -> Self::Output {
        wedge_partial(self)
    }
}
impl Wedge<AntiCircleRotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       25       38        0
    //  no simd       40       56        0
    fn wedge(self, other: AntiCircleRotor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([
                (other.group0()[0] * self.group0()[3]) + (other.group2()[3] * self.group0()[0]),
                (other.group0()[1] * self.group0()[3]) + (other.group2()[3] * self.group0()[1]),
                (other.group0()[2] * self.group0()[3]) + (other.group2()[3] * self.group0()[2]),
                other.group2()[3] * self.group0()[3],
            ]),
            // e23, e31, e12, e45
            (Simd32x4::from(other.group2()[3]) * self.group1()) + (Simd32x4::from(self.group0()[3]) * other.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other.group2()[3] * self.group2()[0],
                other.group2()[3] * self.group2()[1],
                other.group2()[3] * self.group2()[2],
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2]),
            ]) + (Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group2()[3]]) * other.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group2()[2]) + (other.group1()[0] * self.group1()[3]) + (other.group1()[3] * self.group1()[0]) + (other.group2()[3] * self.group3()[0]),
                (other.group0()[2] * self.group2()[0]) + (other.group1()[1] * self.group1()[3]) + (other.group1()[3] * self.group1()[1]) + (other.group2()[3] * self.group3()[1]),
                (other.group0()[0] * self.group2()[1]) + (other.group1()[2] * self.group1()[3]) + (other.group1()[3] * self.group1()[2]) + (other.group2()[3] * self.group3()[2]),
                -(other.group1()[1] * self.group2()[1]) - (other.group1()[2] * self.group2()[2]) - (other.group2()[1] * self.group1()[1]) - (other.group2()[2] * self.group1()[2]),
            ]) + (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group3()[3]]) * crate::swizzle!(other.group2(), 2, 0, 1, 3))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiDipoleInversion> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       36        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       29       42        0
    //  no simd       44       60        0
    fn wedge(self, other: AntiDipoleInversion) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                other.group0()[0] * self.group0()[3],
                other.group0()[1] * self.group0()[3],
                other.group0()[2] * self.group0()[3],
                (other.group3()[1] * self.group3()[1]) + (other.group3()[2] * self.group3()[2]) + (other.group3()[3] * self.group2()[3])
                    - (other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])
                    - (other.group1()[3] * self.group1()[3])
                    - (other.group2()[1] * self.group0()[1])
                    - (other.group2()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(other.group2()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group3()[3]]))
                + (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group3()[0]]) * crate::swizzle!(other.group3(), 2, 0, 1, 0))
                - (Simd32x4::from([other.group3()[1], other.group3()[2], other.group3()[0], other.group2()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group2()[3] * self.group2()[0]) + (other.group3()[3] * self.group0()[0]),
                (other.group2()[3] * self.group2()[1]) + (other.group3()[3] * self.group0()[1]),
                (other.group2()[3] * self.group2()[2]) + (other.group3()[3] * self.group0()[2]),
                -(other.group3()[1] * self.group1()[1]) - (other.group3()[2] * self.group1()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * other.group1())
                - (crate::swizzle!(other.group3(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] * self.group0()[3]) + (other.group3()[1] * self.group2()[2]) + (other.group3()[3] * self.group1()[0]) - (other.group3()[2] * self.group2()[1]),
                (other.group2()[1] * self.group0()[3]) + (other.group3()[2] * self.group2()[0]) + (other.group3()[3] * self.group1()[1]) - (other.group3()[0] * self.group2()[2]),
                (other.group2()[2] * self.group0()[3]) + (other.group3()[0] * self.group2()[1]) + (other.group3()[3] * self.group1()[2]) - (other.group3()[1] * self.group2()[0]),
                other.group3()[3] * self.group0()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
        );
    }
}
impl Wedge<AntiDualNum> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        1        8        0
    //  no simd        1       17        0
    fn wedge(self, other: AntiDualNum) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other.group0()[1]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other.group0()[1]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other.group0()[1] * self.group2()[0],
                other.group0()[1] * self.group2()[1],
                other.group0()[1] * self.group2()[2],
                (other.group0()[0] * self.group0()[3]) + (other.group0()[1] * self.group2()[3]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other.group0()[1]) * self.group3(),
        );
    }
}
impl Wedge<AntiFlatPoint> for VersorOdd {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn wedge(self, other: AntiFlatPoint) -> Self::Output {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3] * self.group0()[3]]),
            // e235, e315, e125, e12345
            Simd32x4::from([
                other.group0()[0] * self.group0()[3],
                other.group0()[1] * self.group0()[3],
                other.group0()[2] * self.group0()[3],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group0()[3] * self.group1()[3]),
            ]),
        );
    }
}
impl Wedge<AntiFlector> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       24        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       31       40        0
    fn wedge(self, other: AntiFlector) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (other.group1()[1] * self.group3()[1]) + (other.group1()[2] * self.group3()[2]) + (other.group1()[3] * self.group2()[3])
                    - (other.group0()[1] * self.group0()[1])
                    - (other.group0()[2] * self.group0()[2])
                    - (other.group0()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group3()[0]]) * crate::swizzle!(other.group1(), 2, 0, 1, 0))
                - (Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[0], other.group0()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, -(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2])])
                + (Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[3], other.group0()[3]]) * self.group0())
                - (crate::swizzle!(other.group1(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[0] * self.group0()[3]) + (other.group1()[1] * self.group2()[2]) + (other.group1()[3] * self.group1()[0]) - (other.group1()[2] * self.group2()[1]),
                (other.group0()[1] * self.group0()[3]) + (other.group1()[2] * self.group2()[0]) + (other.group1()[3] * self.group1()[1]) - (other.group1()[0] * self.group2()[2]),
                (other.group0()[2] * self.group0()[3]) + (other.group1()[0] * self.group2()[1]) + (other.group1()[3] * self.group1()[2]) - (other.group1()[1] * self.group2()[0]),
                other.group1()[3] * self.group0()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0] * self.group0()[3], other.group1()[1] * self.group0()[3], other.group1()[2] * self.group0()[3], 0.0]),
        );
    }
}
impl Wedge<AntiLine> for VersorOdd {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       24        0
    fn wedge(self, other: AntiLine) -> Self::Output {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([other.group0()[0] * self.group0()[3], other.group0()[1] * self.group0()[3], other.group0()[2] * self.group0()[3], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other.group1()[0] * self.group0()[3],
                other.group1()[1] * self.group0()[3],
                other.group1()[2] * self.group0()[3],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[0] * self.group1()[3]) + (other.group1()[2] * self.group0()[1]) - (other.group1()[1] * self.group0()[2]),
                (other.group0()[1] * self.group1()[3]) + (other.group1()[0] * self.group0()[2]) - (other.group1()[2] * self.group0()[0]),
                (other.group0()[2] * self.group1()[3]) + (other.group1()[1] * self.group0()[0]) - (other.group1()[0] * self.group0()[1]),
                -(other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2]),
            ]),
        );
    }
}
impl Wedge<AntiMotor> for VersorOdd {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       13       26        0
    //  no simd       25       41        0
    fn wedge(self, other: AntiMotor) -> Self::Output {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(other.group0()[3]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (other.group0()[0] * self.group0()[3]) + (other.group0()[3] * self.group1()[0]),
                (other.group0()[1] * self.group0()[3]) + (other.group0()[3] * self.group1()[1]),
                (other.group0()[2] * self.group0()[3]) + (other.group0()[3] * self.group1()[2]),
                other.group0()[3] * self.group1()[3],
            ]),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other.group1()[0] * self.group0()[3],
                other.group1()[1] * self.group0()[3],
                other.group1()[2] * self.group0()[3],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(other.group0()[3]) * self.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                other.group0()[3] * self.group3()[0],
                other.group0()[3] * self.group3()[1],
                other.group0()[3] * self.group3()[2],
                -(other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2]),
            ]) + (Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group3()[3]]) * other.group0())
                + (crate::swizzle!(other.group1(), 2, 0, 1, 3) * crate::swizzle!(self.group0(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<AntiPlane> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       27        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       11       29        0
    //  no simd       17       35        0
    fn wedge(self, other: AntiPlane) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                (other.group0()[1] * self.group0()[2]) * -1.0,
                (other.group0()[2] * self.group0()[0]) * -1.0,
                (other.group0()[0] * self.group0()[1]) * -1.0,
                (other.group0()[1] * self.group3()[1]) + (other.group0()[2] * self.group3()[2]) + (other.group0()[3] * self.group2()[3]),
            ]) + (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group3()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1, 0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                other.group0()[3] * self.group0()[0],
                other.group0()[3] * self.group0()[1],
                other.group0()[3] * self.group0()[2],
                -(other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]),
            ]) - (crate::swizzle!(other.group0(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[1] * self.group2()[2]) + (other.group0()[3] * self.group1()[0]) - (other.group0()[2] * self.group2()[1]),
                (other.group0()[2] * self.group2()[0]) + (other.group0()[3] * self.group1()[1]) - (other.group0()[0] * self.group2()[2]),
                (other.group0()[0] * self.group2()[1]) + (other.group0()[3] * self.group1()[2]) - (other.group0()[1] * self.group2()[0]),
                other.group0()[3] * self.group0()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0] * self.group0()[3], other.group0()[1] * self.group0()[3], other.group0()[2] * self.group0()[3], 0.0]),
        );
    }
}
impl Wedge<AntiScalar> for VersorOdd {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn wedge(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e12345 */ self.group0()[3] * other[e12345]);
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
            Simd32x3::from(self.group0()[3]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self.group0()[3]) * other.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                other.group2()[0] * self.group0()[3],
                other.group2()[1] * self.group0()[3],
                other.group2()[2] * self.group0()[3],
                -(other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (other.group2()[0] * self.group0()[0])
                    - (other.group2()[1] * self.group0()[1])
                    - (other.group2()[2] * self.group0()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])
                    - (other.group1()[3] * self.group1()[3]),
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
            Simd32x3::from(self.group0()[3]) * other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(self.group0()[3]) * other.group1(),
            // e235, e315, e125, e12345
            Simd32x4::from([
                other.group2()[0] * self.group0()[3],
                other.group2()[1] * self.group0()[3],
                other.group2()[2] * self.group0()[3],
                (other.group2()[3] * self.group0()[3])
                    - (other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])
                    - (other.group1()[3] * self.group1()[3])
                    - (other.group2()[0] * self.group0()[0])
                    - (other.group2()[1] * self.group0()[1])
                    - (other.group2()[2] * self.group0()[2]),
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
            Simd32x3::from(self.group0()[3]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self.group0()[3]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other.group2()[0] * self.group0()[3],
                other.group2()[1] * self.group0()[3],
                other.group2()[2] * self.group0()[3],
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group2()[2]) + (other.group2()[2] * self.group0()[1]) + (other.group1()[0] * self.group1()[3]) + (other.group1()[3] * self.group1()[0])
                    - (other.group2()[1] * self.group0()[2]),
                (other.group0()[2] * self.group2()[0]) + (other.group2()[0] * self.group0()[2]) + (other.group1()[1] * self.group1()[3]) + (other.group1()[3] * self.group1()[1])
                    - (other.group2()[2] * self.group0()[0]),
                (other.group0()[0] * self.group2()[1]) + (other.group2()[1] * self.group0()[0]) + (other.group1()[2] * self.group1()[3]) + (other.group1()[3] * self.group1()[2])
                    - (other.group2()[0] * self.group0()[1]),
                -(other.group2()[0] * self.group1()[0])
                    - (other.group2()[1] * self.group1()[1])
                    - (other.group2()[2] * self.group1()[2])
                    - (other.group1()[1] * self.group2()[1])
                    - (other.group1()[2] * self.group2()[2]),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0)),
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
            Simd32x3::from(self.group0()[3]) * other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(self.group0()[3]) * other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([
                other.group2()[0] * self.group0()[3],
                other.group2()[1] * self.group0()[3],
                other.group2()[2] * self.group0()[3],
                (other.group2()[3] * self.group0()[3])
                    - (other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2]),
            ]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[1] * self.group2()[2]) + (other.group1()[0] * self.group1()[3]) + (other.group1()[3] * self.group1()[0]) + (other.group3()[0] * self.group0()[3]),
                (other.group0()[2] * self.group2()[0]) + (other.group1()[1] * self.group1()[3]) + (other.group1()[3] * self.group1()[1]) + (other.group3()[1] * self.group0()[3]),
                (other.group0()[0] * self.group2()[1]) + (other.group1()[2] * self.group1()[3]) + (other.group1()[3] * self.group1()[2]) + (other.group3()[2] * self.group0()[3]),
                -(other.group1()[1] * self.group2()[1]) - (other.group1()[2] * self.group2()[2]) - (other.group2()[1] * self.group1()[1]) - (other.group2()[2] * self.group1()[2]),
            ]) + (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[3]]) * crate::swizzle!(self.group0(), 1, 2, 0, 3))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0)),
        );
    }
}
impl Wedge<DualNum> for VersorOdd {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        9        0
    fn wedge(self, other: DualNum) -> Self::Output {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([
                other.group0()[0] * self.group1()[0],
                other.group0()[0] * self.group1()[1],
                other.group0()[0] * self.group1()[2],
                (other.group0()[0] * self.group3()[3]) + (other.group0()[1] * self.group0()[3]),
            ]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0] * self.group2()[0], other.group0()[0] * self.group2()[1], other.group0()[0] * self.group2()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group0()[3]]),
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
            Simd32x4::from(self.group0()[3]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[2] * self.group0()[1]) + (other.group0()[3] * self.group1()[0]),
                (other.group0()[0] * self.group0()[2]) + (other.group0()[3] * self.group1()[1]),
                (other.group0()[1] * self.group0()[0]) + (other.group0()[3] * self.group1()[2]),
                -(other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
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
            Simd32x4::from(self.group0()[3]) * other.group0(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[3] * self.group1()[0]) + (other.group1()[0] * self.group0()[3]),
                (other.group0()[3] * self.group1()[1]) + (other.group1()[1] * self.group0()[3]),
                (other.group0()[3] * self.group1()[2]) + (other.group1()[2] * self.group0()[3]),
                -(other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]),
            ]) + (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[3]]) * crate::swizzle!(self.group0(), 1, 2, 0, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
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
                other.group0()[0] * self.group0()[3],
                other.group0()[1] * self.group0()[3],
                other.group0()[2] * self.group0()[3],
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2]),
            ]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0] * self.group0()[3], other.group1()[1] * self.group0()[3], other.group1()[2] * self.group0()[3], 0.0]),
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(other.group1()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group2()[3]]))
                + (Simd32x4::from(self.group0()[3]) * other.group0()),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group1()[0] * self.group0()[3]) + (other.group1()[3] * self.group1()[0]),
                (other.group1()[1] * self.group0()[3]) + (other.group1()[3] * self.group1()[1]),
                (other.group1()[2] * self.group0()[3]) + (other.group1()[3] * self.group1()[2]),
                other.group1()[3] * self.group0()[3],
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
                other.group0()[0] * self.group0()[3],
                (other.group0()[1] * self.group0()[3])
                    + (other.group1()[0] * self.group3()[0])
                    + (other.group1()[1] * self.group3()[1])
                    + (other.group1()[2] * self.group3()[2])
                    + (other.group1()[3] * self.group3()[3])
                    + (self.group2()[3] * other[e1])
                    - (other.group7()[0] * self.group2()[0])
                    - (other.group7()[1] * self.group2()[1])
                    - (other.group7()[2] * self.group2()[2])
                    - (other.group8()[0] * self.group0()[0])
                    - (other.group8()[1] * self.group0()[1])
                    - (other.group8()[2] * self.group0()[2])
                    - (other.group6()[0] * self.group1()[0])
                    - (other.group6()[1] * self.group1()[1])
                    - (other.group6()[2] * self.group1()[2])
                    - (other.group6()[3] * self.group1()[3]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[3]) * other.group1(),
            // e5
            self.group0()[3] * other[e1],
            // e15, e25, e35, e45
            (Simd32x4::from(other.group0()[0]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group1()[3]]))
                + (Simd32x4::from(self.group0()[3]) * other.group3()),
            // e41, e42, e43
            (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])) + (Simd32x3::from(self.group0()[3]) * other.group4()),
            // e23, e31, e12
            (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])) + (Simd32x3::from(self.group0()[3]) * other.group5()),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group1()[3] * self.group2()[0]) + (self.group0()[0] * other[e1]),
                (other.group1()[3] * self.group2()[1]) + (self.group0()[1] * other[e1]),
                (other.group1()[3] * self.group2()[2]) + (self.group0()[2] * other[e1]),
                -(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * other.group6())
                - (crate::swizzle!(other.group1(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 3, 3, 3, 0)),
            // e423, e431, e412
            Simd32x3::from([
                (other.group1()[2] * self.group0()[1]) - (other.group1()[1] * self.group0()[2]),
                (other.group1()[0] * self.group0()[2]) - (other.group1()[2] * self.group0()[0]),
                (other.group1()[1] * self.group0()[0]) - (other.group1()[0] * self.group0()[1]),
            ]) + (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[3]) * other.group7()),
            // e235, e315, e125
            Simd32x3::from([
                (other.group1()[1] * self.group2()[2]) - (other.group1()[2] * self.group2()[1]),
                (other.group1()[2] * self.group2()[0]) - (other.group1()[0] * self.group2()[2]),
                (other.group1()[0] * self.group2()[1]) - (other.group1()[1] * self.group2()[0]),
            ]) + (Simd32x3::from(self.group0()[3]) * other.group8())
                + (Simd32x3::from(other[e1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group4()[1] * self.group2()[2]) + (other.group5()[0] * self.group1()[3]) + (other.group3()[3] * self.group1()[0]) + (other.group9()[0] * self.group0()[3]),
                (other.group4()[2] * self.group2()[0]) + (other.group5()[1] * self.group1()[3]) + (other.group3()[3] * self.group1()[1]) + (other.group9()[1] * self.group0()[3]),
                (other.group4()[0] * self.group2()[1]) + (other.group5()[2] * self.group1()[3]) + (other.group3()[3] * self.group1()[2]) + (other.group9()[2] * self.group0()[3]),
                -(other.group5()[1] * self.group2()[1]) - (other.group5()[2] * self.group2()[2]) - (other.group3()[1] * self.group1()[1]) - (other.group3()[2] * self.group1()[2]),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group3())
                + (Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group9()[3]]) * crate::swizzle!(self.group0(), 1, 2, 0, 3))
                - (Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group5()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group3(), 1, 2, 0, 0)),
            // e1234
            (other.group0()[0] * self.group2()[3]) + (self.group0()[3] * other[e45])
                - (other.group4()[0] * self.group1()[0])
                - (other.group4()[1] * self.group1()[1])
                - (other.group4()[2] * self.group1()[2])
                - (other.group5()[0] * self.group0()[0])
                - (other.group5()[1] * self.group0()[1])
                - (other.group5()[2] * self.group0()[2]),
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
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(self.group0()[3]) * other.group0());
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
            Simd32x4::from([
                (other.group0()[1] * self.group0()[2]) * -1.0,
                (other.group0()[2] * self.group0()[0]) * -1.0,
                (other.group0()[0] * self.group0()[1]) * -1.0,
                (other.group0()[2] * self.group3()[2]) + (other.group0()[3] * self.group3()[3]) + (self.group2()[3] * other[e2]),
            ]) + (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group3()[0]]) * crate::swizzle!(other.group0(), 2, 0, 1, 0))
                + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group3()[1]]) * crate::swizzle!(other.group0(), 3, 3, 3, 1)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group0()[3] * self.group2()[0]) + (self.group0()[0] * other[e2]),
                (other.group0()[3] * self.group2()[1]) + (self.group0()[1] * other[e2]),
                (other.group0()[3] * self.group2()[2]) + (self.group0()[2] * other[e2]),
                -(other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]),
            ]) - (crate::swizzle!(other.group0(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group0()[1] * self.group2()[2]) + (self.group1()[0] * other[e2]) - (other.group0()[2] * self.group2()[1]),
                (other.group0()[2] * self.group2()[0]) + (self.group1()[1] * other[e2]) - (other.group0()[0] * self.group2()[2]),
                (other.group0()[0] * self.group2()[1]) + (self.group1()[2] * other[e2]) - (other.group0()[1] * self.group2()[0]),
                self.group0()[3] * other[e2],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[3]) * other.group0(),
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
            Simd32x4::from(other[scalar]) * self.group0(),
            // e23, e31, e12, e45
            Simd32x4::from(other[scalar]) * self.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from(other[scalar]) * self.group2(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(other[scalar]) * self.group3(),
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
            Simd32x4::from(self.group0()[3]) * other.group0(),
            // e1234
            self.group0()[3] * other[e4315],
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
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (other.group2()[3] * self.group2()[3]) + (other.group3()[2] * self.group3()[2]) + (other.group3()[3] * self.group3()[3])
                    - (other.group0()[0] * self.group2()[0])
                    - (other.group0()[1] * self.group2()[1])
                    - (other.group0()[2] * self.group2()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])
                    - (other.group1()[3] * self.group1()[3])
                    - (other.group2()[1] * self.group0()[1])
                    - (other.group2()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * other.group0())
                + (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group3()[0]]) * crate::swizzle!(other.group3(), 2, 0, 1, 0))
                + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group3()[1]]) * crate::swizzle!(other.group3(), 3, 3, 3, 1))
                - (Simd32x4::from([other.group3()[1], other.group3()[2], other.group3()[0], other.group2()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
            // e415, e425, e435, e321
            Simd32x4::from([
                (other.group2()[3] * self.group0()[0]) + (other.group3()[3] * self.group2()[0]),
                (other.group2()[3] * self.group0()[1]) + (other.group3()[3] * self.group2()[1]),
                (other.group2()[3] * self.group0()[2]) + (other.group3()[3] * self.group2()[2]),
                -(other.group3()[1] * self.group1()[1]) - (other.group3()[2] * self.group1()[2]),
            ]) + (Simd32x4::from(self.group0()[3]) * other.group1())
                - (crate::swizzle!(other.group3(), 0, 1, 2, 0) * crate::swizzle!(self.group1(), 3, 3, 3, 0)),
            // e235, e315, e125, e5
            Simd32x4::from([
                (other.group2()[0] * self.group0()[3]) + (other.group2()[3] * self.group1()[0]) + (other.group3()[1] * self.group2()[2]) - (other.group3()[2] * self.group2()[1]),
                (other.group2()[1] * self.group0()[3]) + (other.group2()[3] * self.group1()[1]) + (other.group3()[2] * self.group2()[0]) - (other.group3()[0] * self.group2()[2]),
                (other.group2()[2] * self.group0()[3]) + (other.group2()[3] * self.group1()[2]) + (other.group3()[0] * self.group2()[1]) - (other.group3()[1] * self.group2()[0]),
                other.group2()[3] * self.group0()[3],
            ]),
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[3]) * other.group3(),
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
                (other.group0()[0] * self.group0()[3]) + (other.group0()[3] * self.group0()[0]),
                (other.group0()[1] * self.group0()[3]) + (other.group0()[3] * self.group0()[1]),
                (other.group0()[2] * self.group0()[3]) + (other.group0()[3] * self.group0()[2]),
                other.group0()[3] * self.group0()[3],
            ]),
            // e23, e31, e12, e45
            (Simd32x4::from(other.group0()[3]) * self.group1()) + (Simd32x4::from(self.group0()[3]) * other.group1()),
            // e15, e25, e35, e1234
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                -(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(other.group0()[3]) * self.group2())
                + (Simd32x4::from(self.group0()[3]) * other.group2()),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([
                (other.group0()[3] * self.group3()[0]) + (other.group1()[0] * self.group1()[3]) + (other.group1()[3] * self.group1()[0]) + (other.group3()[0] * self.group0()[3]),
                (other.group0()[3] * self.group3()[1]) + (other.group1()[1] * self.group1()[3]) + (other.group1()[3] * self.group1()[1]) + (other.group3()[1] * self.group0()[3]),
                (other.group0()[3] * self.group3()[2]) + (other.group1()[2] * self.group1()[3]) + (other.group1()[3] * self.group1()[2]) + (other.group3()[2] * self.group0()[3]),
                -(other.group1()[1] * self.group2()[1]) - (other.group1()[2] * self.group2()[2]) - (other.group2()[1] * self.group1()[1]) - (other.group2()[2] * self.group1()[2]),
            ]) + (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[3]]) * crate::swizzle!(self.group0(), 1, 2, 0, 3))
                + (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[3]]) * crate::swizzle!(other.group0(), 1, 2, 0, 3))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group2(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group2(), 1, 2, 0, 0)),
        );
    }
}
