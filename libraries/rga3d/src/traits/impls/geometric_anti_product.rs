// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 117
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         0       4       0
//  Average:         6      11       0
//  Maximum:       103     118       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         0       8       0
//  Average:        10      18       0
//  Maximum:       177     192       0
impl std::ops::Div<geometric_anti_product> for AntiScalar {
    type Output = geometric_anti_product_partial<AntiScalar>;
    fn div(self, _rhs: geometric_anti_product) -> Self::Output {
        geometric_anti_product_partial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234]);
    }
}
impl GeometricAntiProduct<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[e1234]) * other.group0());
    }
}
impl GeometricAntiProduct<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group1(),
        );
    }
}
impl GeometricAntiProduct<Horizon> for AntiScalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e1234] * other[e321]);
    }
}
impl GeometricAntiProduct<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group1(),
        );
    }
}
impl GeometricAntiProduct<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e1234]) * other.group1(),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e1234]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group4(),
        );
    }
}
impl GeometricAntiProduct<Origin> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e1234] * other[e4]);
    }
}
impl GeometricAntiProduct<Plane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e1234]) * other.group0());
    }
}
impl GeometricAntiProduct<Point> for AntiScalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e1234]) * other.group0());
    }
}
impl GeometricAntiProduct<Scalar> for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e1234] * other[scalar]);
    }
}
impl std::ops::Div<geometric_anti_product> for DualNum {
    type Output = geometric_anti_product_partial<DualNum>;
    fn div(self, _rhs: geometric_anti_product) -> Self::Output {
        geometric_anti_product_partial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[e1234]) * self.group0());
    }
}
impl GeometricAntiProduct<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (other.group0()[0] * self.group0()[1]) + (other.group0()[1] * self.group0()[0]),
            other.group0()[1] * self.group0()[1],
        ]));
    }
}
impl GeometricAntiProduct<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       12        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group1()[0]) + (self.group0()[1] * other.group0()[0]),
                (self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group0()[1]),
                (self.group0()[0] * other.group1()[2]) + (self.group0()[1] * other.group0()[2]),
                self.group0()[1] * other.group0()[3],
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                self.group0()[1] * other.group1()[0],
                self.group0()[1] * other.group1()[1],
                self.group0()[1] * other.group1()[2],
                (self.group0()[0] * other.group0()[3]) + (self.group0()[1] * other.group1()[3]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Horizon> for DualNum {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self.group0()[1] * other[e321]);
    }
}
impl GeometricAntiProduct<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        3        0
    // no simd        3        9        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self.group0()[1]) * other.group0(),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[0]) * other.group0()) + (Simd32x3::from(self.group0()[1]) * other.group1()),
        );
    }
}
impl GeometricAntiProduct<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        3        0
    // no simd        4       12        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self.group0()[1]) * other.group0(),
            // e23, e31, e12, scalar
            (Simd32x4::from(self.group0()[0]) * other.group0()) + (Simd32x4::from(self.group0()[1]) * other.group1()),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        6       18        0
    //  no simd        8       24        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0]), self.group0()[1] * other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group4()[0]) + (self.group0()[1] * other.group1()[0]),
                (self.group0()[0] * other.group4()[1]) + (self.group0()[1] * other.group1()[1]),
                (self.group0()[0] * other.group4()[2]) + (self.group0()[1] * other.group1()[2]),
                self.group0()[1] * other.group1()[3],
            ]),
            // e41, e42, e43
            Simd32x3::from(self.group0()[1]) * other.group2(),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[0]) * other.group2()) + (Simd32x3::from(self.group0()[1]) * other.group3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                self.group0()[1] * other.group4()[0],
                self.group0()[1] * other.group4()[1],
                self.group0()[1] * other.group4()[2],
                (self.group0()[0] * other.group1()[3]) + (self.group0()[1] * other.group4()[3]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Origin> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1] * other[e4]]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other[e4]]),
        );
    }
}
impl GeometricAntiProduct<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0] * other.group0()[0], self.group0()[0] * other.group0()[1], self.group0()[0] * other.group0()[2], 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from(self.group0()[1]) * other.group0(),
        );
    }
}
impl GeometricAntiProduct<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self.group0()[1]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[0] * other.group0()[3]]),
        );
    }
}
impl GeometricAntiProduct<Scalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.group0()[1] * other[scalar]);
    }
}
impl std::ops::Div<geometric_anti_product> for Flector {
    type Output = geometric_anti_product_partial<Flector>;
    fn div(self, _rhs: geometric_anti_product) -> Self::Output {
        geometric_anti_product_partial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group1(),
        );
    }
}
impl GeometricAntiProduct<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       12        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[1] * self.group0()[0]) - (other.group0()[0] * self.group1()[0]),
                (other.group0()[1] * self.group0()[1]) - (other.group0()[0] * self.group1()[1]),
                (other.group0()[1] * self.group0()[2]) - (other.group0()[0] * self.group1()[2]),
                other.group0()[1] * self.group0()[3],
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                other.group0()[1] * self.group1()[0],
                other.group0()[1] * self.group1()[1],
                other.group0()[1] * self.group1()[2],
                (other.group0()[1] * self.group1()[3]) - (other.group0()[0] * self.group0()[3]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       16       24        0
    //  no simd       40       48        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(other.group1()[0] * self.group0()[3]) - (other.group1()[2] * self.group1()[1]),
                -(other.group1()[0] * self.group1()[2]) - (other.group1()[1] * self.group0()[3]),
                -(other.group1()[1] * self.group1()[0]) - (other.group1()[2] * self.group0()[3]),
                (other.group1()[1] * self.group1()[1]) + (other.group1()[2] * self.group1()[2]),
            ]) + (crate::swizzle!(other.group1(), 1, 2, 0, 0) * crate::swizzle!(self.group1(), 2, 0, 1, 0))
                - (Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[2] * self.group1()[1]) - (other.group1()[2] * self.group0()[1]),
                (other.group0()[0] * self.group1()[2]) - (other.group1()[0] * self.group0()[2]),
                (other.group0()[1] * self.group1()[0]) - (other.group1()[1] * self.group0()[0]),
                (other.group1()[3] * self.group0()[3]) - (other.group0()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]) * crate::swizzle!(self.group0(), 3, 3, 3, 0))
                + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[2]]) * crate::swizzle!(other.group1(), 3, 3, 3, 2))
                + (crate::swizzle!(other.group1(), 1, 2, 0, 1) * crate::swizzle!(self.group0(), 2, 0, 1, 1))
                - (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[2]]) * crate::swizzle!(self.group1(), 3, 3, 3, 2))
                - (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]) * crate::swizzle!(other.group0(), 3, 3, 3, 1))
                - (crate::swizzle!(other.group0(), 1, 2, 0, 0) * crate::swizzle!(self.group1(), 2, 0, 1, 0)),
        );
    }
}
impl GeometricAntiProduct<Horizon> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]),
        );
    }
}
impl GeometricAntiProduct<Line> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       22       30        0
    //  no simd       28       36        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[0] * self.group1()[3]) + (other.group0()[2] * self.group0()[1]) + (other.group1()[1] * self.group1()[2])
                    - (other.group0()[1] * self.group0()[2])
                    - (other.group1()[0] * self.group0()[3]),
                (other.group0()[0] * self.group0()[2]) + (other.group0()[1] * self.group1()[3]) + (other.group1()[2] * self.group1()[0])
                    - (other.group0()[2] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[3]),
                (other.group0()[1] * self.group0()[0]) + (other.group0()[2] * self.group1()[3]) + (other.group1()[0] * self.group1()[1])
                    - (other.group0()[0] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[3]),
                -(other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]),
            ]) - (crate::swizzle!(other.group1(), 2, 0, 1).extend_to_4(other.group0()[0]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group0()[0] * self.group0()[3]) - (other.group0()[1] * self.group1()[2]),
                (other.group0()[1] * self.group0()[3]) - (other.group0()[2] * self.group1()[0]),
                (other.group0()[2] * self.group0()[3]) - (other.group0()[0] * self.group1()[1]),
                (other.group1()[1] * self.group1()[1]) + (other.group1()[2] * self.group1()[2])
                    - (other.group0()[0] * self.group0()[0])
                    - (other.group0()[1] * self.group0()[1])
                    - (other.group0()[2] * self.group0()[2]),
            ]) + (crate::swizzle!(other.group0(), 2, 0, 1).extend_to_4(other.group1()[0]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       16        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       20       24        0
    //  no simd       44       48        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] * other.group0()[2]) + (self.group1()[2] * other.group1()[1]) + (self.group1()[3] * other.group0()[0]) - (self.group0()[3] * other.group1()[0]),
                (self.group0()[2] * other.group0()[0]) + (self.group1()[0] * other.group1()[2]) + (self.group1()[3] * other.group0()[1]) - (self.group0()[3] * other.group1()[1]),
                (self.group0()[2] * other.group0()[3]) + (self.group1()[1] * other.group1()[0]) + (self.group1()[3] * other.group0()[2]) - (self.group0()[3] * other.group1()[2]),
                0.0,
            ]) + (crate::swizzle!(self.group0(), 0, 1, 0, 3) * crate::swizzle!(other.group0(), 3, 3, 1, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[3], other.group0()[2]]) * crate::swizzle!(self.group1(), 1, 2, 2, 2))
                - (Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[1], other.group0()[1]]) * crate::swizzle!(self.group1(), 0, 1, 0, 1)),
            // e423, e431, e412, e321
            Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (self.group1()[2] * other.group1()[2]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[3] * other.group1()[3]),
            ]) + (Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[3]]) * other.group0())
                + (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[3], other.group1()[1]]) * crate::swizzle!(self.group1(), 1, 2, 2, 1))
                + (Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[1], other.group1()[0]]) * crate::swizzle!(self.group1(), 0, 1, 0, 0))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       51        0
    //    simd2        4        4        0
    //    simd3        6        6        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       50       66        0
    //  no simd       81       97        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self.group0()[3] * other.group4()[3]) - (self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2]),
                0.0,
            ]) + (Simd32x2::from(other.group4()[0]) * Simd32x2::from([self.group0()[0], self.group1()[0]]))
                + (Simd32x2::from(other.group4()[1]) * Simd32x2::from([self.group0()[1], self.group1()[1]]))
                + (Simd32x2::from(other.group4()[2]) * Simd32x2::from([self.group0()[2], self.group1()[2]]))
                - (Simd32x2::from(other.group1()[3]) * Simd32x2::from([self.group1()[3], self.group0()[3]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group2()[0] * self.group1()[3]) + (other.group2()[2] * self.group0()[1]) + (other.group3()[1] * self.group1()[2])
                    - (other.group2()[1] * self.group0()[2])
                    - (other.group3()[0] * self.group0()[3]),
                (other.group2()[0] * self.group0()[2]) + (other.group2()[1] * self.group1()[3]) + (other.group3()[2] * self.group1()[0])
                    - (other.group2()[2] * self.group0()[0])
                    - (other.group3()[1] * self.group0()[3]),
                (other.group2()[1] * self.group0()[0]) + (other.group2()[2] * self.group1()[3]) + (other.group3()[0] * self.group1()[1])
                    - (other.group2()[0] * self.group0()[1])
                    - (other.group3()[2] * self.group0()[3]),
                (other.group2()[2] * self.group1()[2]) * -1.0,
            ]) + (Simd32x4::from(other.group0()[1]) * self.group0())
                - (crate::swizzle!(other.group0(), 0, 0).extend_to_4(other.group0()[0], other.group2()[0]) * crate::swizzle!(self.group1(), 0, 1, 2, 0))
                - (crate::swizzle!(other.group3(), 2, 0, 1).extend_to_4(other.group2()[1]) * crate::swizzle!(self.group1(), 1, 2, 0, 1)),
            // e41, e42, e43
            Simd32x3::from([
                (self.group1()[2] * other.group4()[1]) - (self.group1()[1] * other.group4()[2]),
                (self.group1()[0] * other.group4()[2]) - (self.group1()[2] * other.group4()[0]),
                (self.group1()[1] * other.group4()[0]) - (self.group1()[0] * other.group4()[1]),
            ]) - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])),
            // e23, e31, e12
            Simd32x3::from([
                (self.group0()[2] * other.group4()[1]) + (self.group1()[1] * other.group1()[2]) - (self.group0()[1] * other.group4()[2]) - (self.group1()[2] * other.group1()[1]),
                (self.group0()[0] * other.group4()[2]) + (self.group1()[2] * other.group1()[0]) - (self.group0()[2] * other.group4()[0]) - (self.group1()[0] * other.group1()[2]),
                (self.group0()[1] * other.group4()[0]) + (self.group1()[0] * other.group1()[1]) - (self.group0()[0] * other.group4()[1]) - (self.group1()[1] * other.group1()[0]),
            ]) + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group2()[0] * self.group0()[3]) - (other.group2()[1] * self.group1()[2]),
                (other.group2()[1] * self.group0()[3]) - (other.group2()[2] * self.group1()[0]),
                (other.group2()[2] * self.group0()[3]) - (other.group2()[0] * self.group1()[1]),
                (other.group3()[1] * self.group1()[1]) + (other.group3()[2] * self.group1()[2])
                    - (other.group0()[0] * self.group0()[3])
                    - (other.group2()[0] * self.group0()[0])
                    - (other.group2()[1] * self.group0()[1])
                    - (other.group2()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(other.group0()[1]) * self.group1())
                + (crate::swizzle!(other.group2(), 2, 0, 1).extend_to_4(other.group3()[0]) * crate::swizzle!(self.group1(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<Origin> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e4]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e4]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from(-1.0),
        );
    }
}
impl GeometricAntiProduct<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       20       28        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(self.group0()[3] * other.group0()[0]) - (self.group1()[1] * other.group0()[2]),
                -(self.group0()[3] * other.group0()[1]) - (self.group1()[2] * other.group0()[0]),
                -(self.group0()[3] * other.group0()[2]) - (self.group1()[0] * other.group0()[1]),
                (self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2]),
            ]) + (crate::swizzle!(self.group1(), 2, 0, 1, 0) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(self.group0()[1] * other.group0()[2]) - (self.group1()[3] * other.group0()[0]),
                -(self.group0()[2] * other.group0()[0]) - (self.group1()[3] * other.group0()[1]),
                -(self.group0()[0] * other.group0()[1]) - (self.group1()[3] * other.group0()[2]),
                (self.group0()[2] * other.group0()[2]) + (self.group0()[3] * other.group0()[3]),
            ]) + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[1]]) * crate::swizzle!(other.group0(), 3, 3, 3, 1))
                + (crate::swizzle!(self.group0(), 2, 0, 1, 0) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self.group0()[3] * other.group0()[0]) + (self.group1()[1] * other.group0()[2]),
                (self.group0()[3] * other.group0()[1]) + (self.group1()[2] * other.group0()[0]),
                (self.group0()[3] * other.group0()[2]) + (self.group1()[0] * other.group0()[1]),
                -(self.group1()[2] * other.group0()[2]) - (self.group1()[3] * other.group0()[3]),
            ]) - (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[0]]) * crate::swizzle!(other.group0(), 3, 3, 3, 0))
                - (crate::swizzle!(self.group1(), 2, 0, 1, 1) * crate::swizzle!(other.group0(), 1, 2, 0, 1)),
        );
    }
}
impl GeometricAntiProduct<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                self.group1()[0] * other[scalar] * -1.0,
                self.group1()[1] * other[scalar] * -1.0,
                self.group1()[2] * other[scalar] * -1.0,
                0.0,
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3] * other[scalar] * -1.0]),
        );
    }
}
impl std::ops::Div<geometric_anti_product> for Horizon {
    type Output = geometric_anti_product_partial<Horizon>;
    fn div(self, _rhs: geometric_anti_product) -> Self::Output {
        geometric_anti_product_partial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[e1234] * self[e321]);
    }
}
impl GeometricAntiProduct<DualNum> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other.group0()[1] * self[e321]);
    }
}
impl GeometricAntiProduct<Flector> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) * Simd32x4::from(-1.0),
        );
    }
}
impl GeometricAntiProduct<Line> for Horizon {
    type Output = Point;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0] * self[e321], other.group0()[1] * self[e321], other.group0()[2] * self[e321], 0.0]),
        );
    }
}
impl GeometricAntiProduct<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0] * self[e321], other.group0()[1] * self[e321], other.group0()[2] * self[e321], 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3] * self[e321]]),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other.group1()[3] * self[e321] * -1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group2()[0] * self[e321], other.group2()[1] * self[e321], other.group2()[2] * self[e321], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1] * self[e321]]),
        );
    }
}
impl GeometricAntiProduct<Origin> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self[e321] * other[e4] * -1.0);
    }
}
impl GeometricAntiProduct<Plane> for Horizon {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0),
        );
    }
}
impl GeometricAntiProduct<Point> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other.group0()[3] * self[e321] * -1.0);
    }
}
impl std::ops::Div<geometric_anti_product> for Line {
    type Output = geometric_anti_product_partial<Line>;
    fn div(self, _rhs: geometric_anti_product) -> Self::Output {
        geometric_anti_product_partial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group1(),
        );
    }
}
impl GeometricAntiProduct<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        3        0
    // no simd        3        9        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other.group0()[1]) * self.group0(),
            // e23, e31, e12
            (Simd32x3::from(other.group0()[0]) * self.group0()) + (Simd32x3::from(other.group0()[1]) * self.group1()),
        );
    }
}
impl GeometricAntiProduct<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       22       30        0
    //  no simd       28       36        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group1()[3]) + (self.group0()[1] * other.group0()[2]) + (self.group1()[0] * other.group0()[3]) + (self.group1()[1] * other.group1()[2])
                    - (self.group0()[2] * other.group0()[1]),
                (self.group0()[1] * other.group1()[3]) + (self.group0()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3]) + (self.group1()[2] * other.group1()[0])
                    - (self.group0()[0] * other.group0()[2]),
                (self.group0()[0] * other.group0()[1]) + (self.group0()[2] * other.group1()[3]) + (self.group1()[0] * other.group1()[1]) + (self.group1()[2] * other.group0()[3])
                    - (self.group0()[1] * other.group0()[0]),
                -(self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]),
            ]) - (crate::swizzle!(self.group1(), 2, 0, 1).extend_to_4(self.group0()[0]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]) + (self.group0()[1] * other.group1()[2]),
                (self.group0()[1] * other.group0()[3]) + (self.group0()[2] * other.group1()[0]),
                (self.group0()[0] * other.group1()[1]) + (self.group0()[2] * other.group0()[3]),
                -(self.group0()[0] * other.group0()[0])
                    - (self.group0()[1] * other.group0()[1])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2]),
            ]) - (crate::swizzle!(self.group0(), 2, 0, 1).extend_to_4(self.group1()[0]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<Horizon> for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0] * other[e321], self.group0()[1] * other[e321], self.group0()[2] * other[e321], 0.0]),
        );
    }
}
impl GeometricAntiProduct<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       19       27        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other.group0()[2] * self.group0()[1]) - (other.group0()[1] * self.group0()[2]),
                (other.group0()[0] * self.group0()[2]) - (other.group0()[2] * self.group0()[0]),
                (other.group0()[1] * self.group0()[0]) - (other.group0()[0] * self.group0()[1]),
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[2] * self.group1()[1]) + (other.group1()[2] * self.group0()[1]) - (other.group0()[1] * self.group1()[2]) - (other.group1()[1] * self.group0()[2]),
                (other.group0()[0] * self.group1()[2]) + (other.group1()[0] * self.group0()[2]) - (other.group0()[2] * self.group1()[0]) - (other.group1()[2] * self.group0()[0]),
                (other.group0()[1] * self.group1()[0]) + (other.group1()[1] * self.group0()[0]) - (other.group0()[0] * self.group1()[1]) - (other.group1()[0] * self.group0()[1]),
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
impl GeometricAntiProduct<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       19       27        0
    //  no simd       28       36        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]) + (self.group0()[1] * other.group0()[2]),
                (self.group0()[1] * other.group0()[3]) + (self.group0()[2] * other.group0()[0]),
                (self.group0()[0] * other.group0()[1]) + (self.group0()[2] * other.group0()[3]),
                -(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]) - (crate::swizzle!(self.group0(), 2, 0, 1).extend_to_4(self.group0()[0]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self.group0()[0] * other.group1()[3]) + (self.group0()[1] * other.group1()[2]) + (self.group1()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2]),
                (self.group0()[1] * other.group1()[3]) + (self.group0()[2] * other.group1()[0]) + (self.group1()[1] * other.group0()[3]) + (self.group1()[2] * other.group0()[0]),
                (self.group0()[0] * other.group1()[1]) + (self.group0()[2] * other.group1()[3]) + (self.group1()[0] * other.group0()[1]) + (self.group1()[2] * other.group0()[3]),
                -(self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2]) - (self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2]),
            ]) - (crate::swizzle!(self.group0(), 2, 0, 1).extend_to_4(self.group0()[0]) * crate::swizzle!(other.group1(), 1, 2, 0, 0))
                - (crate::swizzle!(self.group1(), 2, 0, 1).extend_to_4(self.group1()[0]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       31        0
    //    simd2        3        3        0
    //    simd3        7        9        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       34       45        0
    //  no simd       57       72        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(self.group1()[0] * other.group2()[0]) - (self.group1()[1] * other.group2()[1]) - (self.group1()[2] * other.group2()[2]),
                0.0,
            ]) - (Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]))
                - (Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]))
                - (Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group4()[3]) + (self.group0()[1] * other.group1()[2]) + (self.group1()[0] * other.group1()[3]) + (self.group1()[1] * other.group4()[2])
                    - (self.group0()[2] * other.group1()[1]),
                (self.group0()[1] * other.group4()[3]) + (self.group0()[2] * other.group1()[0]) + (self.group1()[1] * other.group1()[3]) + (self.group1()[2] * other.group4()[0])
                    - (self.group0()[0] * other.group1()[2]),
                (self.group0()[0] * other.group1()[1]) + (self.group0()[2] * other.group4()[3]) + (self.group1()[0] * other.group4()[1]) + (self.group1()[2] * other.group1()[3])
                    - (self.group0()[1] * other.group1()[0]),
                -(self.group0()[1] * other.group4()[1]) - (self.group0()[2] * other.group4()[2]),
            ]) - (crate::swizzle!(self.group1(), 2, 0, 1).extend_to_4(self.group0()[0]) * crate::swizzle!(other.group4(), 1, 2, 0, 0)),
            // e41, e42, e43
            (Simd32x3::from(other.group0()[1]) * self.group0()) + (crate::swizzle!(self.group0(), 1, 2, 0) * crate::swizzle!(other.group2(), 2, 0, 1))
                - (crate::swizzle!(self.group0(), 2, 0, 1) * crate::swizzle!(other.group2(), 1, 2, 0)),
            // e23, e31, e12
            (Simd32x3::from(other.group0()[0]) * self.group0())
                + (Simd32x3::from(other.group0()[1]) * self.group1())
                + (crate::swizzle!(self.group0(), 1, 2, 0) * crate::swizzle!(other.group3(), 2, 0, 1))
                + (crate::swizzle!(self.group1(), 1, 2, 0) * crate::swizzle!(other.group2(), 2, 0, 1))
                - (crate::swizzle!(self.group0(), 2, 0, 1) * crate::swizzle!(other.group3(), 1, 2, 0))
                - (crate::swizzle!(self.group1(), 2, 0, 1) * crate::swizzle!(other.group2(), 1, 2, 0)),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self.group0()[0] * other.group1()[3]) + (self.group0()[1] * other.group4()[2]),
                (self.group0()[1] * other.group1()[3]) + (self.group0()[2] * other.group4()[0]),
                (self.group0()[0] * other.group4()[1]) + (self.group0()[2] * other.group1()[3]),
                -(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group4()[1])
                    - (self.group1()[2] * other.group4()[2]),
            ]) - (crate::swizzle!(self.group0(), 2, 0, 1).extend_to_4(self.group1()[0]) * crate::swizzle!(other.group4(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<Origin> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0] * other[e4], self.group1()[1] * other[e4], self.group1()[2] * other[e4], 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([self.group0()[0] * other[e4], self.group0()[1] * other[e4], self.group0()[2] * other[e4], 0.0]),
        );
    }
}
impl GeometricAntiProduct<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       15        0
    //  no simd       13       21        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2]),
                (self.group0()[1] * other.group0()[3]) + (self.group1()[2] * other.group0()[0]),
                (self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[1]),
                -(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]) - (crate::swizzle!(self.group1(), 2, 0, 1).extend_to_4(self.group0()[0]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
            // e423, e431, e412, e321
            Simd32x4::from([
                self.group0()[1] * other.group0()[2],
                self.group0()[2] * other.group0()[0],
                self.group0()[0] * other.group0()[1],
                -(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2]),
            ]) - (crate::swizzle!(self.group0(), 2, 0, 1).extend_to_4(self.group1()[0]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       15        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] * other.group0()[2]) + (self.group1()[0] * other.group0()[3]) - (self.group0()[2] * other.group0()[1]),
                (self.group0()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3]) - (self.group0()[0] * other.group0()[2]),
                (self.group0()[0] * other.group0()[1]) + (self.group1()[2] * other.group0()[3]) - (self.group0()[1] * other.group0()[0]),
                0.0,
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                self.group0()[0] * other.group0()[3],
                self.group0()[1] * other.group0()[3],
                self.group0()[2] * other.group0()[3],
                -(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(other[scalar]) * self.group0());
    }
}
impl std::ops::Div<geometric_anti_product> for Motor {
    type Output = geometric_anti_product_partial<Motor>;
    fn div(self, _rhs: geometric_anti_product) -> Self::Output {
        geometric_anti_product_partial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e1234]) * self.group1(),
        );
    }
}
impl GeometricAntiProduct<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        3        0
    // no simd        4       12        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other.group0()[1]) * self.group0(),
            // e23, e31, e12, scalar
            (Simd32x4::from(other.group0()[0]) * self.group0()) + (Simd32x4::from(other.group0()[1]) * self.group1()),
        );
    }
}
impl GeometricAntiProduct<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       31        0
    //  no simd       40       49        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[2] * self.group0()[1])
                    + (other.group0()[3] * self.group1()[0])
                    + (other.group1()[0] * self.group1()[3])
                    + (other.group1()[2] * self.group1()[1])
                    + (other.group1()[3] * self.group0()[0]),
                (other.group0()[1] * self.group0()[3])
                    + (other.group0()[3] * self.group1()[1])
                    + (other.group1()[0] * self.group1()[2])
                    + (other.group1()[1] * self.group1()[3])
                    + (other.group1()[3] * self.group0()[1]),
                (other.group0()[2] * self.group0()[3])
                    + (other.group0()[3] * self.group1()[2])
                    + (other.group1()[1] * self.group1()[0])
                    + (other.group1()[2] * self.group1()[3])
                    + (other.group1()[3] * self.group0()[2]),
                (other.group1()[2] * self.group0()[2]) * -1.0,
            ]) + (crate::swizzle!(other.group0(), 0, 0, 1, 3) * crate::swizzle!(self.group0(), 3, 2, 0, 3))
                - (Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group1()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[1]]) * crate::swizzle!(other.group1(), 1, 2, 0, 1)),
            // e423, e431, e412, e321
            Simd32x4::from([
                other.group1()[2] * self.group0()[1],
                other.group1()[1] * self.group0()[3],
                other.group1()[2] * self.group0()[3],
                -(other.group0()[1] * self.group0()[1])
                    - (other.group0()[2] * self.group0()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2]),
            ]) + (Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                + (crate::swizzle!(other.group1(), 0, 0, 1, 3) * crate::swizzle!(self.group0(), 3, 2, 0, 3))
                - (Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[0], other.group0()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
        );
    }
}
impl GeometricAntiProduct<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0] * other[e321], self.group0()[1] * other[e321], self.group0()[2] * other[e321], 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3] * other[e321]]),
        );
    }
}
impl GeometricAntiProduct<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       19       27        0
    //  no simd       28       36        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other.group0()[0] * self.group0()[3]) + (other.group0()[2] * self.group0()[1]),
                (other.group0()[0] * self.group0()[2]) + (other.group0()[1] * self.group0()[3]),
                (other.group0()[1] * self.group0()[0]) + (other.group0()[2] * self.group0()[3]),
                -(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]) - (crate::swizzle!(other.group0(), 1, 2, 0).extend_to_4(other.group0()[0]) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[0] * self.group1()[3]) + (other.group0()[2] * self.group1()[1]) + (other.group1()[0] * self.group0()[3]) + (other.group1()[2] * self.group0()[1]),
                (other.group0()[0] * self.group1()[2]) + (other.group0()[1] * self.group1()[3]) + (other.group1()[0] * self.group0()[2]) + (other.group1()[1] * self.group0()[3]),
                (other.group0()[1] * self.group1()[0]) + (other.group0()[2] * self.group1()[3]) + (other.group1()[1] * self.group0()[0]) + (other.group1()[2] * self.group0()[3]),
                -(other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]) - (other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) - (crate::swizzle!(other.group0(), 1, 2, 0).extend_to_4(other.group0()[0]) * crate::swizzle!(self.group1(), 2, 0, 1, 0))
                - (crate::swizzle!(other.group1(), 1, 2, 0).extend_to_4(other.group1()[0]) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
        );
    }
}
impl GeometricAntiProduct<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       30        0
    //  no simd       40       48        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other.group0()[2] * self.group0()[1]) + (other.group0()[3] * self.group0()[0]),
                (other.group0()[1] * self.group0()[3]) + (other.group0()[3] * self.group0()[1]),
                (other.group0()[2] * self.group0()[3]) + (other.group0()[3] * self.group0()[2]),
                -(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]) + (crate::swizzle!(other.group0(), 0, 0, 1, 3) * crate::swizzle!(self.group0(), 3, 2, 0, 3))
                - (crate::swizzle!(other.group0(), 1, 2, 0, 0) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[2] * self.group1()[1]) + (other.group0()[3] * self.group1()[0]) + (other.group1()[2] * self.group0()[1]) + (other.group1()[3] * self.group0()[0]),
                (other.group0()[1] * self.group1()[3]) + (other.group0()[3] * self.group1()[1]) + (other.group1()[1] * self.group0()[3]) + (other.group1()[3] * self.group0()[1]),
                (other.group0()[2] * self.group1()[3]) + (other.group0()[3] * self.group1()[2]) + (other.group1()[2] * self.group0()[3]) + (other.group1()[3] * self.group0()[2]),
                -(other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]) - (other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]),
            ]) + (crate::swizzle!(other.group0(), 0, 0, 1, 3) * crate::swizzle!(self.group1(), 3, 2, 0, 3))
                + (crate::swizzle!(other.group1(), 0, 0, 1, 3) * crate::swizzle!(self.group0(), 3, 2, 0, 3))
                - (crate::swizzle!(other.group0(), 1, 2, 0, 0) * crate::swizzle!(self.group1(), 2, 0, 1, 0))
                - (crate::swizzle!(other.group1(), 1, 2, 0, 0) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       33        0
    //    simd2        4        4        0
    //    simd3       10       12        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       42       54        0
    //  no simd       81       97        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other.group0()[1] * self.group1()[3]) - (other.group3()[0] * self.group0()[0]) - (other.group3()[1] * self.group0()[1]) - (other.group3()[2] * self.group0()[2]),
                0.0,
            ]) + (Simd32x2::from(self.group0()[3]) * other.group0())
                - (Simd32x2::from(other.group2()[0]) * Simd32x2::from([self.group1()[0], self.group0()[0]]))
                - (Simd32x2::from(other.group2()[1]) * Simd32x2::from([self.group1()[1], self.group0()[1]]))
                - (Simd32x2::from(other.group2()[2]) * Simd32x2::from([self.group1()[2], self.group0()[2]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] * other.group1()[2])
                    + (self.group0()[3] * other.group1()[0])
                    + (self.group1()[0] * other.group1()[3])
                    + (self.group1()[1] * other.group4()[2])
                    + (self.group1()[3] * other.group4()[0]),
                (self.group0()[2] * other.group1()[0])
                    + (self.group0()[3] * other.group1()[1])
                    + (self.group1()[1] * other.group1()[3])
                    + (self.group1()[2] * other.group4()[0])
                    + (self.group1()[3] * other.group4()[1]),
                (self.group0()[2] * other.group4()[3])
                    + (self.group0()[3] * other.group1()[2])
                    + (self.group1()[0] * other.group4()[1])
                    + (self.group1()[2] * other.group1()[3])
                    + (self.group1()[3] * other.group4()[2]),
                (self.group0()[2] * other.group4()[2]) * -1.0,
            ]) + (Simd32x4::from([other.group4()[3], other.group4()[3], other.group1()[1], other.group1()[3]]) * crate::swizzle!(self.group0(), 0, 1, 0, 3))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[1]]) * crate::swizzle!(other.group4(), 1, 2, 0, 1))
                - (Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[0], other.group4()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
            // e41, e42, e43
            (Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from([self.group0()[1], self.group0()[3], self.group0()[3]]) * crate::swizzle!(other.group2(), 2, 1, 2))
                + (Simd32x3::from([self.group0()[3], self.group0()[2], self.group0()[0]]) * crate::swizzle!(other.group2(), 0, 0, 1))
                - (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * crate::swizzle!(other.group2(), 1, 2, 0)),
            // e23, e31, e12
            (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([self.group0()[1], self.group0()[3], self.group0()[3]]) * crate::swizzle!(other.group3(), 2, 1, 2))
                + (Simd32x3::from([self.group0()[3], self.group0()[2], self.group0()[0]]) * crate::swizzle!(other.group3(), 0, 0, 1))
                + (Simd32x3::from([self.group1()[1], self.group1()[3], self.group1()[3]]) * crate::swizzle!(other.group2(), 2, 1, 2))
                + (Simd32x3::from([self.group1()[3], self.group1()[2], self.group1()[0]]) * crate::swizzle!(other.group2(), 0, 0, 1))
                - (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * crate::swizzle!(other.group3(), 1, 2, 0))
                - (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]) * crate::swizzle!(other.group2(), 1, 2, 0)),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self.group0()[1] * other.group4()[2]) + (self.group0()[3] * other.group4()[0]),
                (self.group0()[2] * other.group4()[0]) + (self.group0()[3] * other.group4()[1]),
                (self.group0()[2] * other.group1()[3]) + (self.group0()[3] * other.group4()[2]),
                (self.group1()[3] * other.group1()[3])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group4()[0])
                    - (self.group1()[1] * other.group4()[1])
                    - (self.group1()[2] * other.group4()[2]),
            ]) + (Simd32x4::from([other.group1()[3], other.group1()[3], other.group4()[1], other.group4()[3]]) * crate::swizzle!(self.group0(), 0, 1, 0, 3))
                - (Simd32x4::from([other.group4()[1], other.group4()[2], other.group4()[0], other.group1()[0]]) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
        );
    }
}
impl GeometricAntiProduct<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e4]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]),
            // e423, e431, e412, e321
            Simd32x4::from(other[e4]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]),
        );
    }
}
impl GeometricAntiProduct<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       20       28        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2]) + (self.group1()[3] * other.group0()[0]),
                (self.group0()[1] * other.group0()[3]) + (self.group1()[2] * other.group0()[0]) + (self.group1()[3] * other.group0()[1]),
                (self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[1]) + (self.group1()[3] * other.group0()[2]),
                -(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]) - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
            // e423, e431, e412, e321
            Simd32x4::from([
                self.group0()[3] * other.group0()[0],
                self.group0()[3] * other.group0()[1],
                self.group0()[3] * other.group0()[2],
                -(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2]),
            ]) + (crate::swizzle!(self.group0(), 1, 2, 0, 3) * crate::swizzle!(other.group0(), 2, 0, 1, 3))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       20        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] * other.group0()[2]) + (self.group0()[3] * other.group0()[0]) + (self.group1()[0] * other.group0()[3]) - (self.group0()[2] * other.group0()[1]),
                (self.group0()[2] * other.group0()[0]) + (self.group0()[3] * other.group0()[1]) + (self.group1()[1] * other.group0()[3]) - (self.group0()[0] * other.group0()[2]),
                (self.group0()[0] * other.group0()[1]) + (self.group0()[3] * other.group0()[2]) + (self.group1()[2] * other.group0()[3]) - (self.group0()[1] * other.group0()[0]),
                self.group0()[3] * other.group0()[3],
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                self.group0()[0] * other.group0()[3],
                self.group0()[1] * other.group0()[3],
                self.group0()[2] * other.group0()[3],
                (self.group1()[3] * other.group0()[3]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group0(),
        );
    }
}
impl std::ops::Div<geometric_anti_product> for MultiVector {
    type Output = geometric_anti_product_partial<MultiVector>;
    fn div(self, _rhs: geometric_anti_product) -> Self::Output {
        geometric_anti_product_partial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[e1234]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group4(),
        );
    }
}
impl GeometricAntiProduct<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        6       18        0
    //  no simd        8       24        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(other.group0()[0] * self.group0()[1]) + (other.group0()[1] * self.group0()[0]), other.group0()[1] * self.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[1] * self.group1()[0]) - (other.group0()[0] * self.group4()[0]),
                (other.group0()[1] * self.group1()[1]) - (other.group0()[0] * self.group4()[1]),
                (other.group0()[1] * self.group1()[2]) - (other.group0()[0] * self.group4()[2]),
                other.group0()[1] * self.group1()[3],
            ]),
            // e41, e42, e43
            Simd32x3::from(other.group0()[1]) * self.group2(),
            // e23, e31, e12
            (Simd32x3::from(other.group0()[0]) * self.group2()) + (Simd32x3::from(other.group0()[1]) * self.group3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                other.group0()[1] * self.group4()[0],
                other.group0()[1] * self.group4()[1],
                other.group0()[1] * self.group4()[2],
                (other.group0()[1] * self.group4()[3]) - (other.group0()[0] * self.group1()[3]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       50        0
    //    simd2        4        4        0
    //    simd3        6        6        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       50       65        0
    //  no simd       81       96        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other.group1()[3] * self.group1()[3]) - (other.group0()[0] * self.group4()[0]) - (other.group0()[1] * self.group4()[1]) - (other.group0()[2] * self.group4()[2]),
                0.0,
            ]) + (Simd32x2::from(other.group1()[0]) * Simd32x2::from([self.group1()[0], self.group4()[0]]))
                + (Simd32x2::from(other.group1()[1]) * Simd32x2::from([self.group1()[1], self.group4()[1]]))
                + (Simd32x2::from(other.group1()[2]) * Simd32x2::from([self.group1()[2], self.group4()[2]]))
                - (Simd32x2::from(other.group0()[3]) * Simd32x2::from([self.group4()[3], self.group1()[3]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group1()[0])
                    + (self.group2()[0] * other.group1()[3])
                    + (self.group2()[1] * other.group0()[2])
                    + (self.group3()[0] * other.group0()[3])
                    + (self.group3()[1] * other.group1()[2])
                    - (self.group2()[2] * other.group0()[1]),
                (self.group0()[0] * other.group1()[1])
                    + (self.group2()[1] * other.group1()[3])
                    + (self.group2()[2] * other.group0()[0])
                    + (self.group3()[1] * other.group0()[3])
                    + (self.group3()[2] * other.group1()[0])
                    - (self.group2()[0] * other.group0()[2]),
                (self.group0()[0] * other.group1()[2])
                    + (self.group2()[0] * other.group0()[1])
                    + (self.group2()[2] * other.group1()[3])
                    + (self.group3()[0] * other.group1()[1])
                    + (self.group3()[2] * other.group0()[3])
                    - (self.group2()[1] * other.group0()[0]),
                -(self.group2()[1] * other.group1()[1]) - (self.group2()[2] * other.group1()[2]),
            ]) + (Simd32x4::from(self.group0()[1]) * other.group0())
                - (crate::swizzle!(self.group3(), 2, 0, 1).extend_to_4(self.group2()[0]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
            // e41, e42, e43
            Simd32x3::from([
                (other.group1()[1] * self.group4()[2]) - (other.group1()[2] * self.group4()[1]),
                (other.group1()[2] * self.group4()[0]) - (other.group1()[0] * self.group4()[2]),
                (other.group1()[0] * self.group4()[1]) - (other.group1()[1] * self.group4()[0]),
            ]) - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            Simd32x3::from([
                (other.group0()[2] * self.group4()[1]) + (other.group1()[1] * self.group1()[2]) - (other.group0()[1] * self.group4()[2]) - (other.group1()[2] * self.group1()[1]),
                (other.group0()[0] * self.group4()[2]) + (other.group1()[2] * self.group1()[0]) - (other.group0()[2] * self.group4()[0]) - (other.group1()[0] * self.group1()[2]),
                (other.group0()[1] * self.group4()[0]) + (other.group1()[0] * self.group1()[1]) - (other.group0()[0] * self.group4()[1]) - (other.group1()[1] * self.group1()[0]),
            ]) + (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                self.group2()[1] * other.group1()[2],
                self.group2()[2] * other.group1()[0],
                self.group2()[0] * other.group1()[1],
                -(self.group2()[0] * other.group0()[0])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[2] * other.group0()[2])
                    - (self.group3()[1] * other.group1()[1])
                    - (self.group3()[2] * other.group1()[2]),
            ]) + (Simd32x4::from(self.group0()[1]) * other.group1())
                + (Simd32x4::from(other.group0()[3]) * crate::swizzle!(self.group2(), 0, 1, 2).extend_to_4(self.group0()[0]))
                - (crate::swizzle!(self.group2(), 2, 0, 1).extend_to_4(self.group3()[0]) * crate::swizzle!(other.group1(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self.group1()[3] * other[e321], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group2()[0] * other[e321], self.group2()[1] * other[e321], self.group2()[2] * other[e321], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[1] * other[e321]]),
        );
    }
}
impl GeometricAntiProduct<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       31        0
    //    simd2        3        3        0
    //    simd3        7        9        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       34       45        0
    //  no simd       57       72        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(other.group1()[0] * self.group2()[0]) - (other.group1()[1] * self.group2()[1]) - (other.group1()[2] * self.group2()[2]),
                0.0,
            ]) - (Simd32x2::from(other.group0()[0]) * Simd32x2::from([self.group3()[0], self.group2()[0]]))
                - (Simd32x2::from(other.group0()[1]) * Simd32x2::from([self.group3()[1], self.group2()[1]]))
                - (Simd32x2::from(other.group0()[2]) * Simd32x2::from([self.group3()[2], self.group2()[2]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[0] * self.group4()[3]) + (other.group0()[2] * self.group1()[1]) + (other.group1()[1] * self.group4()[2])
                    - (other.group0()[1] * self.group1()[2])
                    - (other.group1()[0] * self.group1()[3]),
                (other.group0()[0] * self.group1()[2]) + (other.group0()[1] * self.group4()[3]) + (other.group1()[2] * self.group4()[0])
                    - (other.group0()[2] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[3]),
                (other.group0()[1] * self.group1()[0]) + (other.group0()[2] * self.group4()[3]) + (other.group1()[0] * self.group4()[1])
                    - (other.group0()[0] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[3]),
                -(other.group0()[1] * self.group4()[1]) - (other.group0()[2] * self.group4()[2]),
            ]) - (crate::swizzle!(other.group1(), 2, 0, 1).extend_to_4(other.group0()[0]) * crate::swizzle!(self.group4(), 1, 2, 0, 0)),
            // e41, e42, e43
            (Simd32x3::from(self.group0()[1]) * other.group0()) + (crate::swizzle!(other.group0(), 2, 0, 1) * crate::swizzle!(self.group2(), 1, 2, 0))
                - (crate::swizzle!(other.group0(), 1, 2, 0) * crate::swizzle!(self.group2(), 2, 0, 1)),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[0]) * other.group0())
                + (Simd32x3::from(self.group0()[1]) * other.group1())
                + (crate::swizzle!(other.group0(), 2, 0, 1) * crate::swizzle!(self.group3(), 1, 2, 0))
                + (crate::swizzle!(other.group1(), 2, 0, 1) * crate::swizzle!(self.group2(), 1, 2, 0))
                - (crate::swizzle!(other.group0(), 1, 2, 0) * crate::swizzle!(self.group3(), 2, 0, 1))
                - (crate::swizzle!(other.group1(), 1, 2, 0) * crate::swizzle!(self.group2(), 2, 0, 1)),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group0()[0] * self.group1()[3]) - (other.group0()[1] * self.group4()[2]),
                (other.group0()[1] * self.group1()[3]) - (other.group0()[2] * self.group4()[0]),
                (other.group0()[2] * self.group1()[3]) - (other.group0()[0] * self.group4()[1]),
                (other.group1()[1] * self.group4()[1]) + (other.group1()[2] * self.group4()[2])
                    - (other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2]),
            ]) + (crate::swizzle!(other.group0(), 2, 0, 1).extend_to_4(other.group1()[0]) * crate::swizzle!(self.group4(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd2        4        4        0
    //    simd3       10       12        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       37       47        0
    //  no simd       82       96        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self.group0()[1] * other.group1()[3]) - (self.group3()[0] * other.group0()[0]) - (self.group3()[1] * other.group0()[1]) - (self.group3()[2] * other.group0()[2]),
                0.0,
            ]) + (Simd32x2::from(other.group0()[3]) * self.group0())
                - (Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]))
                - (Simd32x2::from(self.group2()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]))
                - (Simd32x2::from(self.group2()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[2] * self.group1()[1]) + (other.group0()[3] * self.group1()[0]) + (other.group1()[1] * self.group4()[2]) - (other.group1()[0] * self.group1()[3]),
                (other.group0()[1] * self.group4()[3]) + (other.group0()[3] * self.group1()[1]) + (other.group1()[2] * self.group4()[0]) - (other.group1()[1] * self.group1()[3]),
                (other.group0()[2] * self.group4()[3]) + (other.group0()[3] * self.group1()[2]) + (other.group1()[0] * self.group4()[1]) - (other.group1()[2] * self.group1()[3]),
                0.0,
            ]) + (Simd32x4::from([self.group4()[3], self.group1()[2], self.group1()[0], self.group1()[3]]) * crate::swizzle!(other.group0(), 0, 0, 1, 3))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[1]]) * crate::swizzle!(self.group4(), 1, 2, 0, 1))
                - (Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[3], other.group0()[2]]) * crate::swizzle!(self.group4(), 0, 1, 2, 2))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group4()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
            // e41, e42, e43
            (Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[3]]) * crate::swizzle!(self.group2(), 1, 2, 2))
                + (Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * crate::swizzle!(self.group2(), 0, 1, 0))
                - (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * crate::swizzle!(self.group2(), 2, 0, 1)),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + (Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[3]]) * crate::swizzle!(self.group3(), 1, 2, 2))
                + (Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * crate::swizzle!(self.group3(), 0, 1, 0))
                + (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[3]]) * crate::swizzle!(self.group2(), 1, 2, 2))
                + (Simd32x3::from([other.group1()[3], other.group1()[3], other.group1()[1]]) * crate::swizzle!(self.group2(), 0, 1, 0))
                - (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * crate::swizzle!(self.group3(), 2, 0, 1))
                - (Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]) * crate::swizzle!(self.group2(), 2, 0, 1)),
            // e423, e431, e412, e321
            Simd32x4::from([
                other.group0()[3] * self.group4()[0],
                other.group0()[1] * self.group1()[3],
                other.group0()[2] * self.group1()[3],
                (other.group1()[1] * self.group4()[1]) + (other.group1()[2] * self.group4()[2])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[3] * self.group1()[3]),
            ]) + (Simd32x4::from([other.group0()[2], other.group0()[3], other.group0()[3], other.group1()[0]]) * crate::swizzle!(self.group4(), 1, 1, 2, 0))
                + (Simd32x4::from([self.group1()[3], self.group4()[2], self.group4()[0], self.group4()[3]]) * crate::swizzle!(other.group0(), 0, 0, 1, 3))
                - (Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group1()[0]]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       67       82        0
    //    simd2        8        8        0
    //    simd3       18       18        0
    //    simd4       10       10        0
    // Totals...
    // yes simd      103      118        0
    //  no simd      177      192        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other.group0()[1] * self.group0()[0]) + (other.group4()[3] * self.group1()[3])
                    - (other.group3()[0] * self.group2()[0])
                    - (other.group3()[1] * self.group2()[1])
                    - (other.group3()[2] * self.group2()[2])
                    - (other.group1()[0] * self.group4()[0])
                    - (other.group1()[1] * self.group4()[1])
                    - (other.group1()[2] * self.group4()[2]),
                0.0,
            ]) + (Simd32x2::from(self.group0()[1]) * other.group0())
                + (Simd32x2::from(other.group4()[0]) * Simd32x2::from([self.group1()[0], self.group4()[0]]))
                + (Simd32x2::from(other.group4()[1]) * Simd32x2::from([self.group1()[1], self.group4()[1]]))
                + (Simd32x2::from(other.group4()[2]) * Simd32x2::from([self.group1()[2], self.group4()[2]]))
                - (Simd32x2::from(other.group2()[0]) * Simd32x2::from([self.group3()[0], self.group2()[0]]))
                - (Simd32x2::from(other.group2()[1]) * Simd32x2::from([self.group3()[1], self.group2()[1]]))
                - (Simd32x2::from(other.group2()[2]) * Simd32x2::from([self.group3()[2], self.group2()[2]]))
                - (Simd32x2::from(other.group1()[3]) * Simd32x2::from([self.group4()[3], self.group1()[3]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group4()[0])
                    + (other.group2()[0] * self.group4()[3])
                    + (other.group2()[2] * self.group1()[1])
                    + (other.group3()[1] * self.group4()[2])
                    + (self.group2()[0] * other.group4()[3])
                    + (self.group2()[1] * other.group1()[2])
                    + (self.group3()[0] * other.group1()[3])
                    + (self.group3()[1] * other.group4()[2])
                    - (other.group2()[1] * self.group1()[2])
                    - (other.group3()[0] * self.group1()[3])
                    - (self.group2()[2] * other.group1()[1]),
                (self.group0()[0] * other.group4()[1])
                    + (other.group2()[0] * self.group1()[2])
                    + (other.group2()[1] * self.group4()[3])
                    + (other.group3()[2] * self.group4()[0])
                    + (self.group2()[1] * other.group4()[3])
                    + (self.group2()[2] * other.group1()[0])
                    + (self.group3()[1] * other.group1()[3])
                    + (self.group3()[2] * other.group4()[0])
                    - (other.group2()[2] * self.group1()[0])
                    - (other.group3()[1] * self.group1()[3])
                    - (self.group2()[0] * other.group1()[2]),
                (self.group0()[0] * other.group4()[2])
                    + (other.group2()[1] * self.group1()[0])
                    + (other.group2()[2] * self.group4()[3])
                    + (other.group3()[0] * self.group4()[1])
                    + (self.group2()[0] * other.group1()[1])
                    + (self.group2()[2] * other.group4()[3])
                    + (self.group3()[0] * other.group4()[1])
                    + (self.group3()[2] * other.group1()[3])
                    - (other.group2()[0] * self.group1()[1])
                    - (other.group3()[2] * self.group1()[3])
                    - (self.group2()[1] * other.group1()[0]),
                -(other.group2()[2] * self.group4()[2]) - (self.group2()[1] * other.group4()[1]) - (self.group2()[2] * other.group4()[2]),
            ]) + (Simd32x4::from(other.group0()[1]) * self.group1())
                + (Simd32x4::from(self.group0()[1]) * other.group1())
                - (crate::swizzle!(other.group0(), 0, 0).extend_to_4(other.group0()[0], other.group2()[0]) * crate::swizzle!(self.group4(), 0, 1, 2, 0))
                - (crate::swizzle!(other.group3(), 2, 0, 1).extend_to_4(other.group2()[1]) * crate::swizzle!(self.group4(), 1, 2, 0, 1))
                - (crate::swizzle!(self.group3(), 2, 0, 1).extend_to_4(self.group2()[0]) * crate::swizzle!(other.group4(), 1, 2, 0, 0)),
            // e41, e42, e43
            Simd32x3::from([
                (other.group4()[1] * self.group4()[2]) - (other.group4()[2] * self.group4()[1]),
                (other.group4()[2] * self.group4()[0]) - (other.group4()[0] * self.group4()[2]),
                (other.group4()[0] * self.group4()[1]) - (other.group4()[1] * self.group4()[0]),
            ]) + (Simd32x3::from(other.group0()[1]) * self.group2())
                + (Simd32x3::from(self.group0()[1]) * other.group2())
                + (crate::swizzle!(other.group2(), 2, 0, 1) * crate::swizzle!(self.group2(), 1, 2, 0))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                - (crate::swizzle!(other.group2(), 1, 2, 0) * crate::swizzle!(self.group2(), 2, 0, 1)),
            // e23, e31, e12
            Simd32x3::from([
                (other.group1()[2] * self.group4()[1]) + (other.group4()[1] * self.group1()[2]) - (other.group1()[1] * self.group4()[2]) - (other.group4()[2] * self.group1()[1]),
                (other.group1()[0] * self.group4()[2]) + (other.group4()[2] * self.group1()[0]) - (other.group1()[2] * self.group4()[0]) - (other.group4()[0] * self.group1()[2]),
                (other.group1()[1] * self.group4()[0]) + (other.group4()[0] * self.group1()[1]) - (other.group1()[0] * self.group4()[1]) - (other.group4()[1] * self.group1()[0]),
            ]) + (Simd32x3::from(other.group0()[0]) * self.group2())
                + (Simd32x3::from(other.group0()[1]) * self.group3())
                + (Simd32x3::from(self.group0()[0]) * other.group2())
                + (Simd32x3::from(self.group0()[1]) * other.group3())
                + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (crate::swizzle!(other.group2(), 2, 0, 1) * crate::swizzle!(self.group3(), 1, 2, 0))
                + (crate::swizzle!(other.group3(), 2, 0, 1) * crate::swizzle!(self.group2(), 1, 2, 0))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                - (crate::swizzle!(other.group2(), 1, 2, 0) * crate::swizzle!(self.group3(), 2, 0, 1))
                - (crate::swizzle!(other.group3(), 1, 2, 0) * crate::swizzle!(self.group2(), 2, 0, 1)),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group2()[0] * self.group1()[3]) + (self.group2()[1] * other.group4()[2]) - (other.group2()[1] * self.group4()[2]),
                (other.group2()[1] * self.group1()[3]) + (self.group2()[2] * other.group4()[0]) - (other.group2()[2] * self.group4()[0]),
                (other.group2()[2] * self.group1()[3]) + (self.group2()[0] * other.group4()[1]) - (other.group2()[0] * self.group4()[1]),
                (other.group3()[1] * self.group4()[1]) + (other.group3()[2] * self.group4()[2])
                    - (other.group0()[0] * self.group1()[3])
                    - (other.group2()[0] * self.group1()[0])
                    - (other.group2()[1] * self.group1()[1])
                    - (other.group2()[2] * self.group1()[2])
                    - (self.group2()[0] * other.group1()[0])
                    - (self.group2()[1] * other.group1()[1])
                    - (self.group2()[2] * other.group1()[2])
                    - (self.group3()[1] * other.group4()[1])
                    - (self.group3()[2] * other.group4()[2]),
            ]) + (Simd32x4::from(other.group0()[1]) * self.group4())
                + (Simd32x4::from(self.group0()[1]) * other.group4())
                + (Simd32x4::from(other.group1()[3]) * crate::swizzle!(self.group2(), 0, 1, 2).extend_to_4(self.group0()[0]))
                + (crate::swizzle!(other.group2(), 2, 0, 1).extend_to_4(other.group3()[0]) * crate::swizzle!(self.group4(), 1, 2, 0, 0))
                - (crate::swizzle!(self.group2(), 2, 0, 1).extend_to_4(self.group3()[0]) * crate::swizzle!(other.group4(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       24        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[e4]) * Simd32x2::from([self.group4()[3], self.group1()[3]]) * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from(other[e4]) * crate::swizzle!(self.group3(), 0, 1, 2).extend_to_4(self.group0()[1]),
            // e41, e42, e43
            Simd32x3::from(other[e4]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(other[e4]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(other[e4]) * crate::swizzle!(self.group2(), 0, 1, 2).extend_to_4(self.group0()[0]),
        );
    }
}
impl GeometricAntiProduct<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       29        0
    //    simd2        3        3        0
    //    simd3        3        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       23       38        0
    //  no simd       41       56        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self.group1()[3] * other.group0()[3], 0.0])
                + (Simd32x2::from(other.group0()[0]) * Simd32x2::from([self.group1()[0], self.group4()[0]]))
                + (Simd32x2::from(other.group0()[1]) * Simd32x2::from([self.group1()[1], self.group4()[1]]))
                + (Simd32x2::from(other.group0()[2]) * Simd32x2::from([self.group1()[2], self.group4()[2]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group0()[0]) + (self.group2()[0] * other.group0()[3]) + (self.group3()[1] * other.group0()[2]),
                (self.group0()[0] * other.group0()[1]) + (self.group2()[1] * other.group0()[3]) + (self.group3()[2] * other.group0()[0]),
                (self.group0()[0] * other.group0()[2]) + (self.group2()[2] * other.group0()[3]) + (self.group3()[0] * other.group0()[1]),
                -(self.group2()[1] * other.group0()[1]) - (self.group2()[2] * other.group0()[2]),
            ]) - (crate::swizzle!(self.group3(), 2, 0, 1).extend_to_4(self.group2()[0]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
            // e41, e42, e43
            Simd32x3::from([
                (self.group4()[2] * other.group0()[1]) - (self.group4()[1] * other.group0()[2]),
                (self.group4()[0] * other.group0()[2]) - (self.group4()[2] * other.group0()[0]),
                (self.group4()[1] * other.group0()[0]) - (self.group4()[0] * other.group0()[1]),
            ]) - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e23, e31, e12
            Simd32x3::from([
                (self.group1()[2] * other.group0()[1]) - (self.group1()[1] * other.group0()[2]),
                (self.group1()[0] * other.group0()[2]) - (self.group1()[2] * other.group0()[0]),
                (self.group1()[1] * other.group0()[0]) - (self.group1()[0] * other.group0()[1]),
            ]) + (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                self.group2()[1] * other.group0()[2],
                self.group2()[2] * other.group0()[0],
                self.group2()[0] * other.group0()[1],
                -(self.group3()[1] * other.group0()[1]) - (self.group3()[2] * other.group0()[2]),
            ]) + (Simd32x4::from(self.group0()[1]) * other.group0())
                - (crate::swizzle!(self.group2(), 2, 0, 1).extend_to_4(self.group3()[0]) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       32        0
    //    simd3        2        4        0
    // Totals...
    // yes simd       20       36        0
    //  no simd       24       44        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(self.group4()[0] * other.group0()[0]) - (self.group4()[1] * other.group0()[1]) - (self.group4()[2] * other.group0()[2]) - (self.group4()[3] * other.group0()[3]),
                self.group1()[3] * other.group0()[3] * -1.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[1] * other.group0()[0]) + (self.group2()[1] * other.group0()[2]) + (self.group3()[0] * other.group0()[3]) - (self.group2()[2] * other.group0()[1]),
                (self.group0()[1] * other.group0()[1]) + (self.group2()[2] * other.group0()[0]) + (self.group3()[1] * other.group0()[3]) - (self.group2()[0] * other.group0()[2]),
                (self.group0()[1] * other.group0()[2]) + (self.group2()[0] * other.group0()[1]) + (self.group3()[2] * other.group0()[3]) - (self.group2()[1] * other.group0()[0]),
                self.group0()[1] * other.group0()[3],
            ]),
            // e41, e42, e43
            Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([
                (self.group4()[1] * other.group0()[2]) - (self.group4()[2] * other.group0()[1]),
                (self.group4()[2] * other.group0()[0]) - (self.group4()[0] * other.group0()[2]),
                (self.group4()[0] * other.group0()[1]) - (self.group4()[1] * other.group0()[0]),
            ]) + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                self.group2()[0] * other.group0()[3],
                self.group2()[1] * other.group0()[3],
                self.group2()[2] * other.group0()[3],
                (self.group0()[0] * other.group0()[3]) - (self.group2()[0] * other.group0()[0]) - (self.group2()[1] * other.group0()[1]) - (self.group2()[2] * other.group0()[2]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       12        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self.group0()[1] * other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([
                self.group4()[0] * other[scalar] * -1.0,
                self.group4()[1] * other[scalar] * -1.0,
                self.group4()[2] * other[scalar] * -1.0,
                0.0,
            ]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group2(),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, self.group1()[3] * other[scalar] * -1.0]),
        );
    }
}
impl std::ops::Div<geometric_anti_product> for Origin {
    type Output = geometric_anti_product_partial<Origin>;
    fn div(self, _rhs: geometric_anti_product) -> Self::Output {
        geometric_anti_product_partial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e1234] * self[e4]);
    }
}
impl GeometricAntiProduct<DualNum> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1] * self[e4]]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self[e4] * -1.0]),
        );
    }
}
impl GeometricAntiProduct<Flector> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e4]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
        );
    }
}
impl GeometricAntiProduct<Horizon> for Origin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * self[e4]);
    }
}
impl GeometricAntiProduct<Line> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0] * self[e4] * -1.0, other.group1()[1] * self[e4] * -1.0, other.group1()[2] * self[e4] * -1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([other.group0()[0] * self[e4], other.group0()[1] * self[e4], other.group0()[2] * self[e4], 0.0]),
        );
    }
}
impl GeometricAntiProduct<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e4]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        2        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       29        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e4]) * Simd32x2::from([other.group4()[3], other.group1()[3]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e4]) * crate::swizzle!(other.group3(), 0, 1, 2).extend_to_4(other.group0()[1]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(self[e4]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4]) * crate::swizzle!(other.group2(), 0, 1, 2).extend_to_4(other.group0()[0]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl GeometricAntiProduct<Origin> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e4] * self[e4] * -1.0);
    }
}
impl GeometricAntiProduct<Plane> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([other.group0()[0] * self[e4] * -1.0, other.group0()[1] * self[e4] * -1.0, other.group0()[2] * self[e4] * -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3] * self[e4]]),
        );
    }
}
impl GeometricAntiProduct<Point> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3] * self[e4] * -1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([other.group0()[0] * self[e4], other.group0()[1] * self[e4], other.group0()[2] * self[e4], 0.0]),
        );
    }
}
impl GeometricAntiProduct<Scalar> for Origin {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e4] * other[scalar] * -1.0);
    }
}
impl std::ops::Div<geometric_anti_product> for Plane {
    type Output = geometric_anti_product_partial<Plane>;
    fn div(self, _rhs: geometric_anti_product) -> Self::Output {
        geometric_anti_product_partial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e1234]) * self.group0());
    }
}
impl GeometricAntiProduct<DualNum> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       10        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                other.group0()[0] * self.group0()[0] * -1.0,
                other.group0()[0] * self.group0()[1] * -1.0,
                other.group0()[0] * self.group0()[2] * -1.0,
                0.0,
            ]),
            // e423, e431, e412, e321
            Simd32x4::from(other.group0()[1]) * self.group0(),
        );
    }
}
impl GeometricAntiProduct<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       20       28        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(other.group0()[3] * self.group0()[0]) - (other.group1()[2] * self.group0()[1]),
                -(other.group0()[3] * self.group0()[1]) - (other.group1()[0] * self.group0()[2]),
                -(other.group0()[3] * self.group0()[2]) - (other.group1()[1] * self.group0()[0]),
                (other.group1()[1] * self.group0()[1]) + (other.group1()[2] * self.group0()[2]),
            ]) + (crate::swizzle!(other.group1(), 1, 2, 0, 0) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[2] * self.group0()[1]) + (other.group1()[3] * self.group0()[0]),
                (other.group0()[0] * self.group0()[2]) + (other.group1()[3] * self.group0()[1]),
                (other.group0()[1] * self.group0()[0]) + (other.group1()[3] * self.group0()[2]),
                -(other.group0()[2] * self.group0()[2]) - (other.group0()[3] * self.group0()[3]),
            ]) - (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[1]]) * crate::swizzle!(self.group0(), 3, 3, 3, 1))
                - (crate::swizzle!(other.group0(), 1, 2, 0, 0) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
        );
    }
}
impl GeometricAntiProduct<Horizon> for Plane {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
        );
    }
}
impl GeometricAntiProduct<Line> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       18        0
    //  no simd       13       24        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[0] * self.group0()[3]) + (other.group1()[1] * self.group0()[2]),
                (other.group0()[1] * self.group0()[3]) + (other.group1()[2] * self.group0()[0]),
                (other.group0()[2] * self.group0()[3]) + (other.group1()[0] * self.group0()[1]),
                -(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]) - (crate::swizzle!(other.group1(), 2, 0, 1).extend_to_4(other.group0()[0]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group0()[1] * self.group0()[2]) * -1.0,
                (other.group0()[2] * self.group0()[0]) * -1.0,
                (other.group0()[0] * self.group0()[1]) * -1.0,
                (other.group1()[1] * self.group0()[1]) + (other.group1()[2] * self.group0()[2]),
            ]) + (crate::swizzle!(other.group0(), 2, 0, 1).extend_to_4(other.group1()[0]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        8       20        0
    //  no simd       20       32        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[0] * self.group0()[3]) + (other.group1()[1] * self.group0()[2]),
                (other.group0()[1] * self.group0()[3]) + (other.group1()[2] * self.group0()[0]),
                (other.group0()[2] * self.group0()[3]) + (other.group1()[0] * self.group0()[1]),
                (other.group0()[2] * self.group0()[2]) * -1.0,
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * crate::swizzle!(self.group0(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[3], other.group0()[1]]) * crate::swizzle!(self.group0(), 0, 1, 2, 1)),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group0()[1] * self.group0()[2]) * -1.0,
                (other.group0()[2] * self.group0()[0]) * -1.0,
                (other.group0()[0] * self.group0()[1]) * -1.0,
                (other.group1()[1] * self.group0()[1]) + (other.group1()[2] * self.group0()[2]),
            ]) + (Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[0]]) * crate::swizzle!(self.group0(), 0, 1, 2, 0))
                + (crate::swizzle!(other.group0(), 2, 0, 1, 3) * crate::swizzle!(self.group0(), 1, 2, 0, 3)),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       35        0
    //    simd3        3        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       22       42        0
    //  no simd       40       60        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(other.group1()[0] * self.group0()[0]) - (other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2]) - (other.group1()[3] * self.group0()[3]),
                (other.group4()[0] * self.group0()[0]) + (other.group4()[1] * self.group0()[1]) + (other.group4()[2] * self.group0()[2]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group2()[0] * self.group0()[3]) + (other.group3()[1] * self.group0()[2]),
                (other.group2()[1] * self.group0()[3]) + (other.group3()[2] * self.group0()[0]),
                (other.group2()[2] * self.group0()[3]) + (other.group3()[0] * self.group0()[1]),
                (other.group2()[2] * self.group0()[2]) * -1.0,
            ]) - (crate::swizzle!(other.group0(), 0, 0).extend_to_4(other.group0()[0], other.group2()[0]) * crate::swizzle!(self.group0(), 0, 1, 2, 0))
                - (crate::swizzle!(other.group3(), 2, 0, 1).extend_to_4(other.group2()[1]) * crate::swizzle!(self.group0(), 1, 2, 0, 1)),
            // e41, e42, e43
            Simd32x3::from([
                (other.group4()[1] * self.group0()[2]) - (other.group4()[2] * self.group0()[1]),
                (other.group4()[2] * self.group0()[0]) - (other.group4()[0] * self.group0()[2]),
                (other.group4()[0] * self.group0()[1]) - (other.group4()[1] * self.group0()[0]),
            ]) - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e23, e31, e12
            Simd32x3::from([
                (other.group1()[2] * self.group0()[1]) - (other.group1()[1] * self.group0()[2]),
                (other.group1()[0] * self.group0()[2]) - (other.group1()[2] * self.group0()[0]),
                (other.group1()[1] * self.group0()[0]) - (other.group1()[0] * self.group0()[1]),
            ]) + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group2()[1] * self.group0()[2]) * -1.0,
                (other.group2()[2] * self.group0()[0]) * -1.0,
                (other.group2()[0] * self.group0()[1]) * -1.0,
                (other.group3()[1] * self.group0()[1]) + (other.group3()[2] * self.group0()[2]),
            ]) + (Simd32x4::from(other.group0()[1]) * self.group0())
                + (crate::swizzle!(other.group2(), 2, 0, 1).extend_to_4(other.group3()[0]) * crate::swizzle!(self.group0(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<Origin> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self.group0()[0] * other[e4] * -1.0, self.group0()[1] * other[e4] * -1.0, self.group0()[2] * other[e4] * -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3] * other[e4] * -1.0]),
        );
    }
}
impl GeometricAntiProduct<Plane> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       14        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5       15        0
    //  no simd        8       18        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other.group0()[2] * self.group0()[1]) * -1.0,
                (other.group0()[0] * self.group0()[2]) * -1.0,
                (other.group0()[1] * self.group0()[0]) * -1.0,
                (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]),
            ]) + (crate::swizzle!(other.group0(), 1, 2, 0, 0) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[3] * self.group0()[0]) - (other.group0()[0] * self.group0()[3]),
                (other.group0()[3] * self.group0()[1]) - (other.group0()[1] * self.group0()[3]),
                (other.group0()[3] * self.group0()[2]) - (other.group0()[2] * self.group0()[3]),
                0.0,
            ]),
        );
    }
}
impl GeometricAntiProduct<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       12        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       13        0
    //  no simd        6       16        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                self.group0()[0] * other.group0()[3] * -1.0,
                self.group0()[1] * other.group0()[3] * -1.0,
                self.group0()[2] * other.group0()[3] * -1.0,
                0.0,
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                self.group0()[1] * other.group0()[2],
                self.group0()[2] * other.group0()[0],
                self.group0()[0] * other.group0()[1],
                -(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[3] * other.group0()[3]),
            ]) - (crate::swizzle!(self.group0(), 2, 0, 1, 0) * crate::swizzle!(other.group0(), 1, 2, 0, 0)),
        );
    }
}
impl GeometricAntiProduct<Scalar> for Plane {
    type Output = Point;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([
            self.group0()[0] * other[scalar] * -1.0,
            self.group0()[1] * other[scalar] * -1.0,
            self.group0()[2] * other[scalar] * -1.0,
            0.0,
        ]));
    }
}
impl std::ops::Div<geometric_anti_product> for Point {
    type Output = geometric_anti_product_partial<Point>;
    fn div(self, _rhs: geometric_anti_product) -> Self::Output {
        geometric_anti_product_partial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e1234]) * self.group0());
    }
}
impl GeometricAntiProduct<DualNum> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other.group0()[1]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0] * self.group0()[3] * -1.0]),
        );
    }
}
impl GeometricAntiProduct<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(other.group0()[3] * self.group0()[0]) - (other.group1()[2] * self.group0()[1]),
                -(other.group0()[3] * self.group0()[1]) - (other.group1()[0] * self.group0()[2]),
                -(other.group0()[3] * self.group0()[2]) - (other.group1()[1] * self.group0()[0]),
                (other.group1()[2] * self.group0()[2]) + (other.group1()[3] * self.group0()[3]),
            ]) + (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]) * crate::swizzle!(self.group0(), 3, 3, 3, 0))
                + (crate::swizzle!(other.group1(), 1, 2, 0, 1) * crate::swizzle!(self.group0(), 2, 0, 1, 1)),
        );
    }
}
impl GeometricAntiProduct<Horizon> for Point {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ self.group0()[3] * other[e321]);
    }
}
impl GeometricAntiProduct<Line> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       15        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[2] * self.group0()[1]) - (other.group0()[1] * self.group0()[2]) - (other.group1()[0] * self.group0()[3]),
                (other.group0()[0] * self.group0()[2]) - (other.group0()[2] * self.group0()[0]) - (other.group1()[1] * self.group0()[3]),
                (other.group0()[1] * self.group0()[0]) - (other.group0()[0] * self.group0()[1]) - (other.group1()[2] * self.group0()[3]),
                0.0,
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                other.group0()[0] * self.group0()[3],
                other.group0()[1] * self.group0()[3],
                other.group0()[2] * self.group0()[3],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       20        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[2] * self.group0()[1]) + (other.group0()[3] * self.group0()[0]) - (other.group0()[1] * self.group0()[2]) - (other.group1()[0] * self.group0()[3]),
                (other.group0()[0] * self.group0()[2]) + (other.group0()[3] * self.group0()[1]) - (other.group0()[2] * self.group0()[0]) - (other.group1()[1] * self.group0()[3]),
                (other.group0()[1] * self.group0()[0]) + (other.group0()[3] * self.group0()[2]) - (other.group0()[0] * self.group0()[1]) - (other.group1()[2] * self.group0()[3]),
                other.group0()[3] * self.group0()[3],
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                other.group0()[0] * self.group0()[3],
                other.group0()[1] * self.group0()[3],
                other.group0()[2] * self.group0()[3],
                -(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2]) - (other.group1()[3] * self.group0()[3]),
            ]),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       32        0
    //    simd3        2        4        0
    // Totals...
    // yes simd       20       36        0
    //  no simd       24       44        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other.group4()[0] * self.group0()[0]) + (other.group4()[1] * self.group0()[1]) + (other.group4()[2] * self.group0()[2]) + (other.group4()[3] * self.group0()[3]),
                other.group1()[3] * self.group0()[3] * -1.0,
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[1] * self.group0()[0]) + (other.group2()[2] * self.group0()[1]) - (other.group2()[1] * self.group0()[2]) - (other.group3()[0] * self.group0()[3]),
                (other.group0()[1] * self.group0()[1]) + (other.group2()[0] * self.group0()[2]) - (other.group2()[2] * self.group0()[0]) - (other.group3()[1] * self.group0()[3]),
                (other.group0()[1] * self.group0()[2]) + (other.group2()[1] * self.group0()[0]) - (other.group2()[0] * self.group0()[1]) - (other.group3()[2] * self.group0()[3]),
                other.group0()[1] * self.group0()[3],
            ]),
            // e41, e42, e43
            Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([
                (other.group4()[1] * self.group0()[2]) - (other.group4()[2] * self.group0()[1]),
                (other.group4()[2] * self.group0()[0]) - (other.group4()[0] * self.group0()[2]),
                (other.group4()[0] * self.group0()[1]) - (other.group4()[1] * self.group0()[0]),
            ]) + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                other.group2()[0] * self.group0()[3],
                other.group2()[1] * self.group0()[3],
                other.group2()[2] * self.group0()[3],
                -(other.group0()[0] * self.group0()[3]) - (other.group2()[0] * self.group0()[0]) - (other.group2()[1] * self.group0()[1]) - (other.group2()[2] * self.group0()[2]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Origin> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, 0.0, self.group0()[3] * other[e4] * -1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0] * other[e4] * -1.0, self.group0()[1] * other[e4] * -1.0, self.group0()[2] * other[e4] * -1.0, 0.0]),
        );
    }
}
impl GeometricAntiProduct<Plane> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       16        0
    //  no simd        6       19        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                other.group0()[0] * self.group0()[3] * -1.0,
                other.group0()[1] * self.group0()[3] * -1.0,
                other.group0()[2] * self.group0()[3] * -1.0,
                0.0,
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[2] * self.group0()[1]) * -1.0,
                (other.group0()[0] * self.group0()[2]) * -1.0,
                (other.group0()[1] * self.group0()[0]) * -1.0,
                (other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]) + (other.group0()[3] * self.group0()[3]),
            ]) + (crate::swizzle!(other.group0(), 1, 2, 0, 0) * crate::swizzle!(self.group0(), 2, 0, 1, 0)),
        );
    }
}
impl GeometricAntiProduct<Point> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        8        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3] * self.group0()[3] * -1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[0] * self.group0()[3]) - (other.group0()[3] * self.group0()[0]),
                (other.group0()[1] * self.group0()[3]) - (other.group0()[3] * self.group0()[1]),
                (other.group0()[2] * self.group0()[3]) - (other.group0()[3] * self.group0()[2]),
                0.0,
            ]),
        );
    }
}
impl GeometricAntiProduct<Scalar> for Point {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self.group0()[3] * other[scalar] * -1.0);
    }
}
impl std::ops::Div<geometric_anti_product> for Scalar {
    type Output = geometric_anti_product_partial<Scalar>;
    fn div(self, _rhs: geometric_anti_product) -> Self::Output {
        geometric_anti_product_partial(self)
    }
}
impl GeometricAntiProduct<AntiScalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e1234] * self[scalar]);
    }
}
impl GeometricAntiProduct<DualNum> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other.group0()[1] * self[scalar]);
    }
}
impl GeometricAntiProduct<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0] * self[scalar], other.group1()[1] * self[scalar], other.group1()[2] * self[scalar], 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3] * self[scalar]]),
        );
    }
}
impl GeometricAntiProduct<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(self[scalar]) * other.group0());
    }
}
impl GeometricAntiProduct<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group0(),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other.group0()[1] * self[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group4()[0] * self[scalar], other.group4()[1] * self[scalar], other.group4()[2] * self[scalar], 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group2(),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3] * self[scalar]]),
        );
    }
}
impl GeometricAntiProduct<Origin> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[e4] * self[scalar]);
    }
}
impl GeometricAntiProduct<Plane> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([
            other.group0()[0] * self[scalar],
            other.group0()[1] * self[scalar],
            other.group0()[2] * self[scalar],
            0.0,
        ]));
    }
}
impl GeometricAntiProduct<Point> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other.group0()[3] * self[scalar]);
    }
}
