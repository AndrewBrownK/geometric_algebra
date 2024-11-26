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
//   Median:         0       5       0
//  Average:         8      14       0
//  Maximum:       130     145       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         0      12       0
//  Average:        10      19       0
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
        use crate::elements::*;
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234]]),
        );
    }
}
impl GeometricAntiProduct<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       12        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[scalar] * other[e423]) + (self[e1234] * other[e1]),
                (self[scalar] * other[e431]) + (self[e1234] * other[e2]),
                (self[scalar] * other[e412]) + (self[e1234] * other[e3]),
                self[e1234] * other[e4],
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                self[e1234] * other[e423],
                self[e1234] * other[e431],
                self[e1234] * other[e412],
                (self[scalar] * other[e4]) + (self[e1234] * other[e321]),
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
        return Horizon::from_groups(/* e321 */ self[e1234] * other[e321]);
    }
}
impl GeometricAntiProduct<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        3        0
    // no simd        3        9        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group0()) + (Simd32x3::from(self[e1234]) * other.group1()),
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
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[scalar]) * other.group0()) + (Simd32x4::from(self[e1234]) * other.group1()),
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self[scalar] * other[e1234]) + (self[e1234] * other[scalar]), self[e1234] * other[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[scalar] * other[e423]) + (self[e1234] * other[e1]),
                (self[scalar] * other[e431]) + (self[e1234] * other[e2]),
                (self[scalar] * other[e412]) + (self[e1234] * other[e3]),
                self[e1234] * other[e4],
            ]),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group2(),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group2()) + (Simd32x3::from(self[e1234]) * other.group3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                self[e1234] * other[e423],
                self[e1234] * other[e431],
                self[e1234] * other[e412],
                (self[scalar] * other[e4]) + (self[e1234] * other[e321]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Origin> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, self[e1234] * other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, self[scalar] * other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
    }
}
impl GeometricAntiProduct<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       11        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[scalar] * other[e423], self[scalar] * other[e431], self[scalar] * other[e412], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group0(),
        );
    }
}
impl GeometricAntiProduct<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, self[scalar] * other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
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
        return Scalar::from_groups(/* scalar */ self[e1234] * other[scalar]);
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
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e1234] * self[e1]) - (other[scalar] * self[e423]),
                (other[e1234] * self[e2]) - (other[scalar] * self[e431]),
                (other[e1234] * self[e3]) - (other[scalar] * self[e412]),
                other[e1234] * self[e4],
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                other[e1234] * self[e423],
                other[e1234] * self[e431],
                other[e1234] * self[e412],
                (other[e1234] * self[e321]) - (other[scalar] * self[e4]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       31       39        0
    //  no simd       40       48        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e431] * self[e412]) - (other[e423] * self[e4]) - (other[e412] * self[e431]),
                (other[e412] * self[e423]) - (other[e423] * self[e412]) - (other[e431] * self[e4]),
                (other[e423] * self[e431]) - (other[e431] * self[e423]) - (other[e412] * self[e4]),
                (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]),
            ]) - (Simd32x4::from(other[e4]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e3] * self[e431]) + (other[e431] * self[e3]) + (other[e321] * self[e423]) - (other[e2] * self[e412]) - (other[e423] * self[e321]) - (other[e412] * self[e2]),
                (other[e1] * self[e412]) + (other[e412] * self[e1]) + (other[e321] * self[e431]) - (other[e3] * self[e423]) - (other[e423] * self[e3]) - (other[e431] * self[e321]),
                (other[e2] * self[e423]) + (other[e423] * self[e2]) + (other[e321] * self[e412]) - (other[e1] * self[e431]) - (other[e431] * self[e1]) - (other[e412] * self[e321]),
                (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
            ]) + (Simd32x4::from(self[e4]) * Simd32x4::from([other[e1], other[e2], other[e3], other[e321]]))
                - (Simd32x4::from(other[e4]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e321]])),
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
            Simd32x4::from(other[e321]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]),
        );
    }
}
impl GeometricAntiProduct<Line> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       36        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e2] * other[e43]) + (self[e412] * other[e31]) + (self[e321] * other[e41]) - (self[e3] * other[e42]) - (self[e4] * other[e23]) - (self[e431] * other[e12]),
                (self[e3] * other[e41]) + (self[e423] * other[e12]) + (self[e321] * other[e42]) - (self[e1] * other[e43]) - (self[e4] * other[e31]) - (self[e412] * other[e23]),
                (self[e1] * other[e42]) + (self[e431] * other[e23]) + (self[e321] * other[e43]) - (self[e2] * other[e41]) - (self[e4] * other[e12]) - (self[e423] * other[e31]),
                -(self[e423] * other[e41]) - (self[e431] * other[e42]) - (self[e412] * other[e43]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e4] * other[e41]) + (self[e431] * other[e43]) - (self[e412] * other[e42]),
                (self[e4] * other[e42]) + (self[e412] * other[e41]) - (self[e423] * other[e43]),
                (self[e4] * other[e43]) + (self[e423] * other[e42]) - (self[e431] * other[e41]),
                (self[e423] * other[e23]) + (self[e431] * other[e31]) + (self[e412] * other[e12]) - (self[e1] * other[e41]) - (self[e2] * other[e42]) - (self[e3] * other[e43]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       40        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       34       42        0
    //  no simd       40       48        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e2] * other[e43]) + (self[e412] * other[e31]) + (self[e321] * other[e41])
                    - (self[e3] * other[e42])
                    - (self[e4] * other[e23])
                    - (self[e423] * other[scalar])
                    - (self[e431] * other[e12]),
                (self[e3] * other[e41]) + (self[e423] * other[e12]) + (self[e321] * other[e42])
                    - (self[e1] * other[e43])
                    - (self[e4] * other[e31])
                    - (self[e431] * other[scalar])
                    - (self[e412] * other[e23]),
                (self[e1] * other[e42]) + (self[e431] * other[e23]) + (self[e321] * other[e43])
                    - (self[e2] * other[e41])
                    - (self[e4] * other[e12])
                    - (self[e423] * other[e31])
                    - (self[e412] * other[scalar]),
                -(self[e423] * other[e41]) - (self[e431] * other[e42]) - (self[e412] * other[e43]),
            ]) + (Simd32x4::from(other[e1234]) * self.group0()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e4] * other[e41]) + (self[e431] * other[e43]) - (self[e412] * other[e42]),
                (self[e4] * other[e42]) + (self[e412] * other[e41]) - (self[e423] * other[e43]),
                (self[e4] * other[e43]) + (self[e423] * other[e42]) - (self[e431] * other[e41]),
                (self[e423] * other[e23]) + (self[e431] * other[e31]) + (self[e412] * other[e12])
                    - (self[e1] * other[e41])
                    - (self[e2] * other[e42])
                    - (self[e3] * other[e43])
                    - (self[e4] * other[scalar]),
            ]) + (Simd32x4::from(other[e1234]) * self.group1()),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       47       62        0
    //    simd2        4        4        0
    //    simd3        6        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       59       74        0
    //  no simd       81       96        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self[e4] * other[e321]) - (self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]), 0.0])
                + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
                - (Simd32x2::from(other[e4]) * Simd32x2::from([self[e321], self[e4]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e2] * other[e43]) + (self[e412] * other[e31]) + (self[e321] * other[e41])
                    - (self[e3] * other[e42])
                    - (self[e4] * other[e23])
                    - (self[e423] * other[scalar])
                    - (self[e431] * other[e12]),
                (self[e3] * other[e41]) + (self[e423] * other[e12]) + (self[e321] * other[e42])
                    - (self[e1] * other[e43])
                    - (self[e4] * other[e31])
                    - (self[e431] * other[scalar])
                    - (self[e412] * other[e23]),
                (self[e1] * other[e42]) + (self[e431] * other[e23]) + (self[e321] * other[e43])
                    - (self[e2] * other[e41])
                    - (self[e4] * other[e12])
                    - (self[e423] * other[e31])
                    - (self[e412] * other[scalar]),
                -(self[e423] * other[e41]) - (self[e431] * other[e42]) - (self[e412] * other[e43]),
            ]) + (Simd32x4::from(other[e1234]) * self.group0()),
            // e41, e42, e43
            Simd32x3::from([
                (self[e412] * other[e431]) - (self[e431] * other[e412]),
                (self[e423] * other[e412]) - (self[e412] * other[e423]),
                (self[e431] * other[e423]) - (self[e423] * other[e431]),
            ]) - (Simd32x3::from(self[e4]) * Simd32x3::from([other[e423], other[e431], other[e412]]))
                - (Simd32x3::from(other[e4]) * Simd32x3::from([self[e423], self[e431], self[e412]])),
            // e23, e31, e12
            Simd32x3::from([
                (self[e3] * other[e431]) + (self[e431] * other[e3]) - (self[e2] * other[e412]) - (self[e412] * other[e2]),
                (self[e1] * other[e412]) + (self[e412] * other[e1]) - (self[e3] * other[e423]) - (self[e423] * other[e3]),
                (self[e2] * other[e423]) + (self[e423] * other[e2]) - (self[e1] * other[e431]) - (self[e431] * other[e1]),
            ]) + (Simd32x3::from(self[e4]) * Simd32x3::from([other[e1], other[e2], other[e3]]))
                + (Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                - (Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]]))
                - (Simd32x3::from(other[e4]) * Simd32x3::from([self[e1], self[e2], self[e3]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e4] * other[e41]) + (self[e431] * other[e43]) - (self[e412] * other[e42]),
                (self[e4] * other[e42]) + (self[e412] * other[e41]) - (self[e423] * other[e43]),
                (self[e4] * other[e43]) + (self[e423] * other[e42]) - (self[e431] * other[e41]),
                (self[e423] * other[e23]) + (self[e431] * other[e31]) + (self[e412] * other[e12])
                    - (self[e1] * other[e41])
                    - (self[e2] * other[e42])
                    - (self[e3] * other[e43])
                    - (self[e4] * other[scalar]),
            ]) + (Simd32x4::from(other[e1234]) * self.group1()),
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
            Simd32x4::from(other[e4]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e4]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e321]]) * Simd32x4::from(-1.0),
        );
    }
}
impl GeometricAntiProduct<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       17       25        0
    //  no simd       20       28        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e412] * other[e431]) - (self[e4] * other[e423]) - (self[e431] * other[e412]),
                (self[e423] * other[e412]) - (self[e4] * other[e431]) - (self[e412] * other[e423]),
                (self[e431] * other[e423]) - (self[e4] * other[e412]) - (self[e423] * other[e431]),
                (self[e423] * other[e423]) + (self[e431] * other[e431]) + (self[e412] * other[e412]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e3] * other[e431]) - (self[e2] * other[e412]) - (self[e321] * other[e423]),
                (self[e1] * other[e412]) - (self[e3] * other[e423]) - (self[e321] * other[e431]),
                (self[e2] * other[e423]) - (self[e1] * other[e431]) - (self[e321] * other[e412]),
                (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]),
            ]) + (Simd32x4::from(other[e321]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]])),
        );
    }
}
impl GeometricAntiProduct<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       12       24        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e4]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e4] * other[e1]) + (self[e431] * other[e3]) - (self[e412] * other[e2]),
                (self[e4] * other[e2]) + (self[e412] * other[e1]) - (self[e423] * other[e3]),
                (self[e4] * other[e3]) + (self[e423] * other[e2]) - (self[e431] * other[e1]),
                -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]),
            ]) - (Simd32x4::from(other[e4]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e321]])),
        );
    }
}
impl GeometricAntiProduct<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e423] * other[scalar], self[e431] * other[scalar], self[e412] * other[scalar], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, self[e4] * other[scalar]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
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
        return Horizon::from_groups(/* e321 */ other[e1234] * self[e321]);
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
            Simd32x4::from(self[e321]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]]) * Simd32x4::from(-1.0),
        );
    }
}
impl GeometricAntiProduct<Line> for Horizon {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321] * other[e41], self[e321] * other[e42], self[e321] * other[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricAntiProduct<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321] * other[e41], self[e321] * other[e42], self[e321] * other[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, self[e321] * other[e1234]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       21        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e321] * other[e4], 1.0]) * Simd32x2::from([-1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e321] * other[e41], self[e321] * other[e42], self[e321] * other[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, self[e321] * other[e1234]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
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
            Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
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
        return Scalar::from_groups(/* scalar */ self[e321] * other[e4] * -1.0);
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
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group1()),
        );
    }
}
impl GeometricAntiProduct<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       36        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e412] * self[e31]) + (other[e321] * self[e41]) - (other[e2] * self[e43]) - (other[e431] * self[e12]),
                (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e423] * self[e12]) + (other[e321] * self[e42]) - (other[e3] * self[e41]) - (other[e412] * self[e23]),
                (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e431] * self[e23]) + (other[e321] * self[e43]) - (other[e1] * self[e42]) - (other[e423] * self[e31]),
                -(other[e423] * self[e41]) - (other[e431] * self[e42]) - (other[e412] * self[e43]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e4] * self[e41]) + (other[e412] * self[e42]) - (other[e431] * self[e43]),
                (other[e4] * self[e42]) + (other[e423] * self[e43]) - (other[e412] * self[e41]),
                (other[e4] * self[e43]) + (other[e431] * self[e41]) - (other[e423] * self[e42]),
                -(other[e1] * self[e41]) - (other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Horizon> for Line {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e321] * self[e41], other[e321] * self[e42], other[e321] * self[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricAntiProduct<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       19       27        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e43] * self[e42]) - (other[e42] * self[e43]),
                (other[e41] * self[e43]) - (other[e43] * self[e41]),
                (other[e42] * self[e41]) - (other[e41] * self[e42]),
                -(other[e41] * self[e41]) - (other[e42] * self[e42]) - (other[e43] * self[e43]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e43] * self[e31]) + (other[e12] * self[e42]) - (other[e42] * self[e12]) - (other[e31] * self[e43]),
                (other[e41] * self[e12]) + (other[e23] * self[e43]) - (other[e43] * self[e23]) - (other[e12] * self[e41]),
                (other[e42] * self[e23]) + (other[e31] * self[e41]) - (other[e41] * self[e31]) - (other[e23] * self[e42]),
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       36        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e41] * other[e1234]) + (self[e42] * other[e43]) - (self[e43] * other[e42]),
                (self[e42] * other[e1234]) + (self[e43] * other[e41]) - (self[e41] * other[e43]),
                (self[e41] * other[e42]) + (self[e43] * other[e1234]) - (self[e42] * other[e41]),
                -(self[e41] * other[e41]) - (self[e42] * other[e42]) - (self[e43] * other[e43]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e41] * other[scalar]) + (self[e42] * other[e12]) + (self[e23] * other[e1234]) + (self[e31] * other[e43])
                    - (self[e43] * other[e31])
                    - (self[e12] * other[e42]),
                (self[e42] * other[scalar]) + (self[e43] * other[e23]) + (self[e31] * other[e1234]) + (self[e12] * other[e41])
                    - (self[e41] * other[e12])
                    - (self[e23] * other[e43]),
                (self[e41] * other[e31]) + (self[e43] * other[scalar]) + (self[e23] * other[e42]) + (self[e12] * other[e1234])
                    - (self[e42] * other[e23])
                    - (self[e31] * other[e41]),
                -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
            ]),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       57        0
    //    simd2        3        3        0
    //    simd3        3        3        0
    // Totals...
    // yes simd       48       63        0
    //  no simd       57       72        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]), 0.0])
                - (Simd32x2::from(self[e41]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e42]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e43]) * Simd32x2::from([other[e12], other[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e321]) + (self[e42] * other[e3]) + (self[e23] * other[e4]) + (self[e31] * other[e412]) - (self[e43] * other[e2]) - (self[e12] * other[e431]),
                (self[e42] * other[e321]) + (self[e43] * other[e1]) + (self[e31] * other[e4]) + (self[e12] * other[e423]) - (self[e41] * other[e3]) - (self[e23] * other[e412]),
                (self[e41] * other[e2]) + (self[e43] * other[e321]) + (self[e23] * other[e431]) + (self[e12] * other[e4]) - (self[e42] * other[e1]) - (self[e31] * other[e423]),
                -(self[e41] * other[e423]) - (self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]),
            // e41, e42, e43
            Simd32x3::from([
                (self[e42] * other[e43]) - (self[e43] * other[e42]),
                (self[e43] * other[e41]) - (self[e41] * other[e43]),
                (self[e41] * other[e42]) - (self[e42] * other[e41]),
            ]) + (Simd32x3::from(other[e1234]) * self.group0()),
            // e23, e31, e12
            Simd32x3::from([
                (self[e42] * other[e12]) + (self[e31] * other[e43]) - (self[e43] * other[e31]) - (self[e12] * other[e42]),
                (self[e43] * other[e23]) + (self[e12] * other[e41]) - (self[e41] * other[e12]) - (self[e23] * other[e43]),
                (self[e41] * other[e31]) + (self[e23] * other[e42]) - (self[e42] * other[e23]) - (self[e31] * other[e41]),
            ]) + (Simd32x3::from(other[scalar]) * self.group0())
                + (Simd32x3::from(other[e1234]) * self.group1()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e41] * other[e4]) + (self[e42] * other[e412]) - (self[e43] * other[e431]),
                (self[e42] * other[e4]) + (self[e43] * other[e423]) - (self[e41] * other[e412]),
                (self[e41] * other[e431]) + (self[e43] * other[e4]) - (self[e42] * other[e423]),
                -(self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Origin> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       14        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e23] * other[e4], self[e31] * other[e4], self[e12] * other[e4], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([self[e41] * other[e4], self[e42] * other[e4], self[e43] * other[e4], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricAntiProduct<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       21        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e321]) + (self[e31] * other[e412]) - (self[e12] * other[e431]),
                (self[e42] * other[e321]) + (self[e12] * other[e423]) - (self[e23] * other[e412]),
                (self[e43] * other[e321]) + (self[e23] * other[e431]) - (self[e31] * other[e423]),
                -(self[e41] * other[e423]) - (self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e412]) - (self[e43] * other[e431]),
                (self[e43] * other[e423]) - (self[e41] * other[e412]),
                (self[e41] * other[e431]) - (self[e42] * other[e423]),
                -(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       15        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]) - (self[e43] * other[e2]),
                (self[e43] * other[e1]) + (self[e31] * other[e4]) - (self[e41] * other[e3]),
                (self[e41] * other[e2]) + (self[e12] * other[e4]) - (self[e42] * other[e1]),
                0.0,
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                self[e41] * other[e4],
                self[e42] * other[e4],
                self[e43] * other[e4],
                -(self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]),
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
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[scalar]) * self.group0()) + (Simd32x4::from(other[e1234]) * self.group1()),
        );
    }
}
impl GeometricAntiProduct<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       31       39        0
    //  no simd       40       48        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e423] * self[scalar]) + (other[e412] * self[e31]) + (other[e321] * self[e41])
                    - (other[e2] * self[e43])
                    - (other[e431] * self[e12]),
                (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e423] * self[e12]) + (other[e431] * self[scalar]) + (other[e321] * self[e42])
                    - (other[e3] * self[e41])
                    - (other[e412] * self[e23]),
                (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e431] * self[e23]) + (other[e412] * self[scalar]) + (other[e321] * self[e43])
                    - (other[e1] * self[e42])
                    - (other[e423] * self[e31]),
                -(other[e423] * self[e41]) - (other[e431] * self[e42]) - (other[e412] * self[e43]),
            ]) + (Simd32x4::from(self[e1234]) * other.group0()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e412] * self[e42]) - (other[e431] * self[e43]),
                (other[e423] * self[e43]) - (other[e412] * self[e41]),
                (other[e431] * self[e41]) - (other[e423] * self[e42]),
                -(other[e1] * self[e41]) - (other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]),
            ]) + (Simd32x4::from(other[e4]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]))
                + (Simd32x4::from(self[e1234]) * other.group1()),
        );
    }
}
impl GeometricAntiProduct<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e321] * self[e41], other[e321] * self[e42], other[e321] * self[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321] * self[e1234]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
    }
}
impl GeometricAntiProduct<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       36        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e41] * self[e1234]) + (other[e43] * self[e42]) - (other[e42] * self[e43]),
                (other[e41] * self[e43]) + (other[e42] * self[e1234]) - (other[e43] * self[e41]),
                (other[e42] * self[e41]) + (other[e43] * self[e1234]) - (other[e41] * self[e42]),
                -(other[e41] * self[e41]) - (other[e42] * self[e42]) - (other[e43] * self[e43]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e41] * self[scalar]) + (other[e43] * self[e31]) + (other[e23] * self[e1234]) + (other[e12] * self[e42])
                    - (other[e42] * self[e12])
                    - (other[e31] * self[e43]),
                (other[e41] * self[e12]) + (other[e42] * self[scalar]) + (other[e23] * self[e43]) + (other[e31] * self[e1234])
                    - (other[e43] * self[e23])
                    - (other[e12] * self[e41]),
                (other[e42] * self[e23]) + (other[e43] * self[scalar]) + (other[e31] * self[e41]) + (other[e12] * self[e1234])
                    - (other[e41] * self[e31])
                    - (other[e23] * self[e42]),
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       31       39        0
    //  no simd       40       48        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e43] * self[e42]) + (other[e1234] * self[e41]) - (other[e42] * self[e43]),
                (other[e41] * self[e43]) + (other[e1234] * self[e42]) - (other[e43] * self[e41]),
                (other[e42] * self[e41]) + (other[e1234] * self[e43]) - (other[e41] * self[e42]),
                -(other[e41] * self[e41]) - (other[e42] * self[e42]) - (other[e43] * self[e43]),
            ]) + (Simd32x4::from(self[e1234]) * other.group0()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e43] * self[e31]) + (other[e1234] * self[e23]) + (other[e12] * self[e42]) + (other[scalar] * self[e41])
                    - (other[e42] * self[e12])
                    - (other[e31] * self[e43]),
                (other[e41] * self[e12]) + (other[e1234] * self[e31]) + (other[e23] * self[e43]) + (other[scalar] * self[e42])
                    - (other[e43] * self[e23])
                    - (other[e12] * self[e41]),
                (other[e42] * self[e23]) + (other[e1234] * self[e12]) + (other[e31] * self[e41]) + (other[scalar] * self[e43])
                    - (other[e41] * self[e31])
                    - (other[e23] * self[e42]),
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]) + (Simd32x4::from(self[e1234]) * other.group1())
                + (Simd32x4::from(self[scalar]) * other.group0()),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       43       58        0
    //    simd2        4        4        0
    //    simd3        6        6        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       56       71        0
    //  no simd       81       96        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self[scalar] * other[e1234]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]), 0.0])
                + (Simd32x2::from(self[e1234]) * other.group0())
                - (Simd32x2::from(self[e41]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e42]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e43]) * Simd32x2::from([other[e12], other[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e321]) + (self[e42] * other[e3]) + (self[e23] * other[e4]) + (self[e31] * other[e412]) + (self[scalar] * other[e423])
                    - (self[e43] * other[e2])
                    - (self[e12] * other[e431]),
                (self[e42] * other[e321]) + (self[e43] * other[e1]) + (self[e31] * other[e4]) + (self[e12] * other[e423]) + (self[scalar] * other[e431])
                    - (self[e41] * other[e3])
                    - (self[e23] * other[e412]),
                (self[e41] * other[e2]) + (self[e43] * other[e321]) + (self[e23] * other[e431]) + (self[e12] * other[e4]) + (self[scalar] * other[e412])
                    - (self[e42] * other[e1])
                    - (self[e31] * other[e423]),
                -(self[e41] * other[e423]) - (self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]) + (Simd32x4::from(self[e1234]) * other.group1()),
            // e41, e42, e43
            Simd32x3::from([
                (self[e42] * other[e43]) - (self[e43] * other[e42]),
                (self[e43] * other[e41]) - (self[e41] * other[e43]),
                (self[e41] * other[e42]) - (self[e42] * other[e41]),
            ]) + (Simd32x3::from(self[e1234]) * other.group2())
                + (Simd32x3::from(other[e1234]) * Simd32x3::from([self[e41], self[e42], self[e43]])),
            // e23, e31, e12
            Simd32x3::from([
                (self[e42] * other[e12]) + (self[e31] * other[e43]) - (self[e43] * other[e31]) - (self[e12] * other[e42]),
                (self[e43] * other[e23]) + (self[e12] * other[e41]) - (self[e41] * other[e12]) - (self[e23] * other[e43]),
                (self[e41] * other[e31]) + (self[e23] * other[e42]) - (self[e42] * other[e23]) - (self[e31] * other[e41]),
            ]) + (Simd32x3::from(self[e1234]) * other.group3())
                + (Simd32x3::from(self[scalar]) * other.group2())
                + (Simd32x3::from(other[scalar]) * Simd32x3::from([self[e41], self[e42], self[e43]]))
                + (Simd32x3::from(other[e1234]) * Simd32x3::from([self[e23], self[e31], self[e12]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e412]) - (self[e43] * other[e431]),
                (self[e43] * other[e423]) - (self[e41] * other[e412]),
                (self[e41] * other[e431]) - (self[e42] * other[e423]),
                -(self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
            ]) + (Simd32x4::from(self[e1234]) * other.group4())
                + (Simd32x4::from(other[e4]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]])),
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
            Simd32x4::from(other[e4]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e1234]]),
            // e423, e431, e412, e321
            Simd32x4::from(other[e4]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
        );
    }
}
impl GeometricAntiProduct<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       17       25        0
    //  no simd       20       28        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e41] * other[e321]) + (self[e31] * other[e412]) + (self[scalar] * other[e423]) - (self[e12] * other[e431]),
                (self[e42] * other[e321]) + (self[e12] * other[e423]) + (self[scalar] * other[e431]) - (self[e23] * other[e412]),
                (self[e43] * other[e321]) + (self[e23] * other[e431]) + (self[scalar] * other[e412]) - (self[e31] * other[e423]),
                -(self[e41] * other[e423]) - (self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e412]) - (self[e43] * other[e431]),
                (self[e43] * other[e423]) - (self[e41] * other[e412]),
                (self[e41] * other[e431]) - (self[e42] * other[e423]),
                -(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
            ]) + (Simd32x4::from(self[e1234]) * other.group0()),
        );
    }
}
impl GeometricAntiProduct<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       20        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e1234] * other[e1]) + (self[e23] * other[e4]) - (self[e43] * other[e2]),
                (self[e43] * other[e1]) + (self[e1234] * other[e2]) + (self[e31] * other[e4]) - (self[e41] * other[e3]),
                (self[e41] * other[e2]) + (self[e1234] * other[e3]) + (self[e12] * other[e4]) - (self[e42] * other[e1]),
                self[e1234] * other[e4],
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                self[e41] * other[e4],
                self[e42] * other[e4],
                self[e43] * other[e4],
                (self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]),
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
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e1234] * self[e1]) - (other[scalar] * self[e423]),
                (other[e1234] * self[e2]) - (other[scalar] * self[e431]),
                (other[e1234] * self[e3]) - (other[scalar] * self[e412]),
                other[e1234] * self[e4],
            ]),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group2(),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(other[e1234]) * self.group3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                other[e1234] * self[e423],
                other[e1234] * self[e431],
                other[e1234] * self[e412],
                (other[e1234] * self[e321]) - (other[scalar] * self[e4]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       43       58        0
    //    simd2        4        4        0
    //    simd3        6        6        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       56       71        0
    //  no simd       81       96        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]), 0.0])
                + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
                - (Simd32x2::from(other[e4]) * Simd32x2::from([self[e321], self[e4]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e423] * self[scalar]) + (other[e412] * self[e31]) + (other[e321] * self[e41])
                    - (other[e2] * self[e43])
                    - (other[e431] * self[e12]),
                (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e423] * self[e12]) + (other[e431] * self[scalar]) + (other[e321] * self[e42])
                    - (other[e3] * self[e41])
                    - (other[e412] * self[e23]),
                (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e431] * self[e23]) + (other[e412] * self[scalar]) + (other[e321] * self[e43])
                    - (other[e1] * self[e42])
                    - (other[e423] * self[e31]),
                -(other[e423] * self[e41]) - (other[e431] * self[e42]) - (other[e412] * self[e43]),
            ]) + (Simd32x4::from(self[e1234]) * other.group0()),
            // e41, e42, e43
            Simd32x3::from([
                (other[e431] * self[e412]) - (other[e412] * self[e431]),
                (other[e412] * self[e423]) - (other[e423] * self[e412]),
                (other[e423] * self[e431]) - (other[e431] * self[e423]),
            ]) - (Simd32x3::from(other[e4]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                - (Simd32x3::from(self[e4]) * Simd32x3::from([other[e423], other[e431], other[e412]])),
            // e23, e31, e12
            Simd32x3::from([
                (other[e3] * self[e431]) + (other[e431] * self[e3]) - (other[e2] * self[e412]) - (other[e412] * self[e2]),
                (other[e1] * self[e412]) + (other[e412] * self[e1]) - (other[e3] * self[e423]) - (other[e423] * self[e3]),
                (other[e2] * self[e423]) + (other[e423] * self[e2]) - (other[e1] * self[e431]) - (other[e431] * self[e1]),
            ]) + (Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                + (Simd32x3::from(self[e4]) * Simd32x3::from([other[e1], other[e2], other[e3]]))
                - (Simd32x3::from(other[e4]) * Simd32x3::from([self[e1], self[e2], self[e3]]))
                - (Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e412] * self[e42]) - (other[e431] * self[e43]),
                (other[e423] * self[e43]) - (other[e412] * self[e41]),
                (other[e431] * self[e41]) - (other[e423] * self[e42]),
                -(other[e1] * self[e41]) - (other[e2] * self[e42]) - (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]),
            ]) + (Simd32x4::from(other[e4]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]))
                + (Simd32x4::from(self[e1234]) * other.group1()),
        );
    }
}
impl GeometricAntiProduct<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       18        0
    fn geometric_anti_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[e321] * self[e4], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e321] * self[e41], other[e321] * self[e42], other[e321] * self[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321] * self[e1234]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
    }
}
impl GeometricAntiProduct<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       57        0
    //    simd2        3        3        0
    //    simd3        3        3        0
    // Totals...
    // yes simd       48       63        0
    //  no simd       57       72        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([-(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), 0.0])
                - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e43] * self[e2]) + (other[e31] * self[e412]) - (other[e42] * self[e3]) - (other[e23] * self[e4]) - (other[e12] * self[e431]),
                (other[e41] * self[e3]) + (other[e42] * self[e321]) + (other[e12] * self[e423]) - (other[e43] * self[e1]) - (other[e23] * self[e412]) - (other[e31] * self[e4]),
                (other[e42] * self[e1]) + (other[e43] * self[e321]) + (other[e23] * self[e431]) - (other[e41] * self[e2]) - (other[e31] * self[e423]) - (other[e12] * self[e4]),
                -(other[e41] * self[e423]) - (other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]),
            // e41, e42, e43
            Simd32x3::from([
                (other[e43] * self[e42]) - (other[e42] * self[e43]),
                (other[e41] * self[e43]) - (other[e43] * self[e41]),
                (other[e42] * self[e41]) - (other[e41] * self[e42]),
            ]) + (Simd32x3::from(self[e1234]) * other.group0()),
            // e23, e31, e12
            Simd32x3::from([
                (other[e43] * self[e31]) + (other[e12] * self[e42]) - (other[e42] * self[e12]) - (other[e31] * self[e43]),
                (other[e41] * self[e12]) + (other[e23] * self[e43]) - (other[e43] * self[e23]) - (other[e12] * self[e41]),
                (other[e42] * self[e23]) + (other[e31] * self[e41]) - (other[e41] * self[e31]) - (other[e23] * self[e42]),
            ]) + (Simd32x3::from(self[scalar]) * other.group0())
                + (Simd32x3::from(self[e1234]) * other.group1()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e41] * self[e4]) + (other[e43] * self[e431]) - (other[e42] * self[e412]),
                (other[e41] * self[e412]) + (other[e42] * self[e4]) - (other[e43] * self[e423]),
                (other[e42] * self[e423]) + (other[e43] * self[e4]) - (other[e41] * self[e431]),
                (other[e23] * self[e423]) + (other[e31] * self[e431]) + (other[e12] * self[e412]) - (other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       47       62        0
    //    simd2        4        4        0
    //    simd3        6        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       59       74        0
    //  no simd       81       96        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[scalar] * self[e1234]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]), 0.0])
                + (Simd32x2::from(other[e1234]) * self.group0())
                - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e43] * self[e2]) + (other[e31] * self[e412])
                    - (other[e42] * self[e3])
                    - (other[e23] * self[e4])
                    - (other[e12] * self[e431])
                    - (other[scalar] * self[e423]),
                (other[e41] * self[e3]) + (other[e42] * self[e321]) + (other[e12] * self[e423])
                    - (other[e43] * self[e1])
                    - (other[e23] * self[e412])
                    - (other[e31] * self[e4])
                    - (other[scalar] * self[e431]),
                (other[e42] * self[e1]) + (other[e43] * self[e321]) + (other[e23] * self[e431])
                    - (other[e41] * self[e2])
                    - (other[e31] * self[e423])
                    - (other[e12] * self[e4])
                    - (other[scalar] * self[e412]),
                -(other[e41] * self[e423]) - (other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]) + (Simd32x4::from(other[e1234]) * self.group1()),
            // e41, e42, e43
            Simd32x3::from([
                (other[e43] * self[e42]) - (other[e42] * self[e43]),
                (other[e41] * self[e43]) - (other[e43] * self[e41]),
                (other[e42] * self[e41]) - (other[e41] * self[e42]),
            ]) + (Simd32x3::from(other[e1234]) * self.group2())
                + (Simd32x3::from(self[e1234]) * Simd32x3::from([other[e41], other[e42], other[e43]])),
            // e23, e31, e12
            Simd32x3::from([
                (other[e43] * self[e31]) + (other[e12] * self[e42]) - (other[e42] * self[e12]) - (other[e31] * self[e43]),
                (other[e41] * self[e12]) + (other[e23] * self[e43]) - (other[e43] * self[e23]) - (other[e12] * self[e41]),
                (other[e42] * self[e23]) + (other[e31] * self[e41]) - (other[e41] * self[e31]) - (other[e23] * self[e42]),
            ]) + (Simd32x3::from(other[e1234]) * self.group3())
                + (Simd32x3::from(other[scalar]) * self.group2())
                + (Simd32x3::from(self[scalar]) * Simd32x3::from([other[e41], other[e42], other[e43]]))
                + (Simd32x3::from(self[e1234]) * Simd32x3::from([other[e23], other[e31], other[e12]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e41] * self[e4]) + (other[e43] * self[e431]) - (other[e42] * self[e412]),
                (other[e41] * self[e412]) + (other[e42] * self[e4]) - (other[e43] * self[e423]),
                (other[e42] * self[e423]) + (other[e43] * self[e4]) - (other[e41] * self[e431]),
                (other[e23] * self[e423]) + (other[e31] * self[e431]) + (other[e12] * self[e412])
                    - (other[e41] * self[e1])
                    - (other[e42] * self[e2])
                    - (other[e43] * self[e3])
                    - (other[scalar] * self[e4]),
            ]) + (Simd32x4::from(other[e1234]) * self.group4()),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      105      120        0
    //    simd2        8        8        0
    //    simd3       12       12        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      130      145        0
    //  no simd      177      192        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[e1234] * self[scalar]) + (other[e321] * self[e4])
                    - (other[e1] * self[e423])
                    - (other[e2] * self[e431])
                    - (other[e3] * self[e412])
                    - (other[e23] * self[e41])
                    - (other[e31] * self[e42])
                    - (other[e12] * self[e43]),
                0.0,
            ]) + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]]))
                + (Simd32x2::from(self[e1234]) * other.group0())
                - (Simd32x2::from(other[e4]) * Simd32x2::from([self[e321], self[e4]]))
                - (Simd32x2::from(other[e41]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e42]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e43]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e3] * self[e42])
                    + (other[e4] * self[e23])
                    + (other[e41] * self[e321])
                    + (other[e43] * self[e2])
                    + (other[e31] * self[e412])
                    + (other[e423] * self[scalar])
                    + (other[e412] * self[e31])
                    + (other[e321] * self[e41])
                    - (other[scalar] * self[e423])
                    - (other[e2] * self[e43])
                    - (other[e42] * self[e3])
                    - (other[e23] * self[e4])
                    - (other[e12] * self[e431])
                    - (other[e431] * self[e12]),
                (other[e1] * self[e43])
                    + (other[e4] * self[e31])
                    + (other[e41] * self[e3])
                    + (other[e42] * self[e321])
                    + (other[e12] * self[e423])
                    + (other[e423] * self[e12])
                    + (other[e431] * self[scalar])
                    + (other[e321] * self[e42])
                    - (other[scalar] * self[e431])
                    - (other[e3] * self[e41])
                    - (other[e43] * self[e1])
                    - (other[e23] * self[e412])
                    - (other[e31] * self[e4])
                    - (other[e412] * self[e23]),
                (other[e2] * self[e41])
                    + (other[e4] * self[e12])
                    + (other[e42] * self[e1])
                    + (other[e43] * self[e321])
                    + (other[e23] * self[e431])
                    + (other[e431] * self[e23])
                    + (other[e412] * self[scalar])
                    + (other[e321] * self[e43])
                    - (other[scalar] * self[e412])
                    - (other[e1] * self[e42])
                    - (other[e41] * self[e2])
                    - (other[e31] * self[e423])
                    - (other[e12] * self[e4])
                    - (other[e423] * self[e31]),
                -(other[e41] * self[e423])
                    - (other[e42] * self[e431])
                    - (other[e43] * self[e412])
                    - (other[e423] * self[e41])
                    - (other[e431] * self[e42])
                    - (other[e412] * self[e43]),
            ]) + (Simd32x4::from(other[e1234]) * self.group1())
                + (Simd32x4::from(self[e1234]) * other.group1()),
            // e41, e42, e43
            Simd32x3::from([
                (other[e43] * self[e42]) + (other[e431] * self[e412]) - (other[e42] * self[e43]) - (other[e412] * self[e431]),
                (other[e41] * self[e43]) + (other[e412] * self[e423]) - (other[e43] * self[e41]) - (other[e423] * self[e412]),
                (other[e42] * self[e41]) + (other[e423] * self[e431]) - (other[e41] * self[e42]) - (other[e431] * self[e423]),
            ]) + (Simd32x3::from(other[e1234]) * self.group2())
                + (Simd32x3::from(self[e1234]) * other.group2())
                - (Simd32x3::from(other[e4]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                - (Simd32x3::from(self[e4]) * Simd32x3::from([other[e423], other[e431], other[e412]])),
            // e23, e31, e12
            Simd32x3::from([
                (other[e3] * self[e431]) + (other[e43] * self[e31]) + (other[e12] * self[e42]) + (other[e431] * self[e3])
                    - (other[e2] * self[e412])
                    - (other[e42] * self[e12])
                    - (other[e31] * self[e43])
                    - (other[e412] * self[e2]),
                (other[e1] * self[e412]) + (other[e41] * self[e12]) + (other[e23] * self[e43]) + (other[e412] * self[e1])
                    - (other[e3] * self[e423])
                    - (other[e43] * self[e23])
                    - (other[e12] * self[e41])
                    - (other[e423] * self[e3]),
                (other[e2] * self[e423]) + (other[e42] * self[e23]) + (other[e31] * self[e41]) + (other[e423] * self[e2])
                    - (other[e1] * self[e431])
                    - (other[e41] * self[e31])
                    - (other[e23] * self[e42])
                    - (other[e431] * self[e1]),
            ]) + (Simd32x3::from(other[scalar]) * self.group2())
                + (Simd32x3::from(other[e1234]) * self.group3())
                + (Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                + (Simd32x3::from(self[scalar]) * other.group2())
                + (Simd32x3::from(self[e1234]) * other.group3())
                + (Simd32x3::from(self[e4]) * Simd32x3::from([other[e1], other[e2], other[e3]]))
                - (Simd32x3::from(other[e4]) * Simd32x3::from([self[e1], self[e2], self[e3]]))
                - (Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e41] * self[e4]) + (other[e43] * self[e431]) + (other[e412] * self[e42]) - (other[e42] * self[e412]) - (other[e431] * self[e43]),
                (other[e41] * self[e412]) + (other[e42] * self[e4]) + (other[e423] * self[e43]) - (other[e43] * self[e423]) - (other[e412] * self[e41]),
                (other[e42] * self[e423]) + (other[e43] * self[e4]) + (other[e431] * self[e41]) - (other[e41] * self[e431]) - (other[e423] * self[e42]),
                (other[e23] * self[e423]) + (other[e31] * self[e431]) + (other[e12] * self[e412])
                    - (other[scalar] * self[e4])
                    - (other[e1] * self[e41])
                    - (other[e2] * self[e42])
                    - (other[e3] * self[e43])
                    - (other[e41] * self[e1])
                    - (other[e42] * self[e2])
                    - (other[e43] * self[e3])
                    - (other[e423] * self[e23])
                    - (other[e431] * self[e31])
                    - (other[e412] * self[e12]),
            ]) + (Simd32x4::from(other[e1234]) * self.group4())
                + (Simd32x4::from(other[e4]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]))
                + (Simd32x4::from(self[e1234]) * other.group4()),
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
            Simd32x2::from(other[e4]) * Simd32x2::from([self[e321], self[e4]]) * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from(other[e4]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e1234]]),
            // e41, e42, e43
            Simd32x3::from(other[e4]) * Simd32x3::from([self[e423], self[e431], self[e412]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(other[e4]) * Simd32x3::from([self[e1], self[e2], self[e3]]) * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(other[e4]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
        );
    }
}
impl GeometricAntiProduct<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd2        3        3        0
    //    simd3        3        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       44        0
    //  no simd       41       56        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e4] * other[e321], 0.0])
                + (Simd32x2::from(other[e423]) * Simd32x2::from([self[e1], self[e423]]))
                + (Simd32x2::from(other[e431]) * Simd32x2::from([self[e2], self[e431]]))
                + (Simd32x2::from(other[e412]) * Simd32x2::from([self[e3], self[e412]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[scalar] * other[e423]) + (self[e41] * other[e321]) + (self[e31] * other[e412]) - (self[e12] * other[e431]),
                (self[scalar] * other[e431]) + (self[e42] * other[e321]) + (self[e12] * other[e423]) - (self[e23] * other[e412]),
                (self[scalar] * other[e412]) + (self[e43] * other[e321]) + (self[e23] * other[e431]) - (self[e31] * other[e423]),
                -(self[e41] * other[e423]) - (self[e42] * other[e431]) - (self[e43] * other[e412]),
            ]),
            // e41, e42, e43
            Simd32x3::from([
                (self[e412] * other[e431]) - (self[e431] * other[e412]),
                (self[e423] * other[e412]) - (self[e412] * other[e423]),
                (self[e431] * other[e423]) - (self[e423] * other[e431]),
            ]) - (Simd32x3::from(self[e4]) * Simd32x3::from([other[e423], other[e431], other[e412]])),
            // e23, e31, e12
            Simd32x3::from([
                (self[e3] * other[e431]) - (self[e2] * other[e412]),
                (self[e1] * other[e412]) - (self[e3] * other[e423]),
                (self[e2] * other[e423]) - (self[e1] * other[e431]),
            ]) + (Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                - (Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e412]) - (self[e43] * other[e431]),
                (self[e43] * other[e423]) - (self[e41] * other[e412]),
                (self[e41] * other[e431]) - (self[e42] * other[e423]),
                -(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
            ]) + (Simd32x4::from(self[e1234]) * other.group0()),
        );
    }
}
impl GeometricAntiProduct<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       31        0
    //    simd2        0        1        0
    //    simd3        2        4        0
    // Totals...
    // yes simd       20       36        0
    //  no simd       24       45        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
                self[e4] * other[e4],
            ]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e1234] * other[e1]) + (self[e42] * other[e3]) + (self[e23] * other[e4]) - (self[e43] * other[e2]),
                (self[e1234] * other[e2]) + (self[e43] * other[e1]) + (self[e31] * other[e4]) - (self[e41] * other[e3]),
                (self[e1234] * other[e3]) + (self[e41] * other[e2]) + (self[e12] * other[e4]) - (self[e42] * other[e1]),
                self[e1234] * other[e4],
            ]),
            // e41, e42, e43
            Simd32x3::from(other[e4]) * Simd32x3::from([self[e423], self[e431], self[e412]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([
                (self[e431] * other[e3]) - (self[e412] * other[e2]),
                (self[e412] * other[e1]) - (self[e423] * other[e3]),
                (self[e423] * other[e2]) - (self[e431] * other[e1]),
            ]) + (Simd32x3::from(self[e4]) * Simd32x3::from([other[e1], other[e2], other[e3]]))
                - (Simd32x3::from(other[e4]) * Simd32x3::from([self[e1], self[e2], self[e3]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                self[e41] * other[e4],
                self[e42] * other[e4],
                self[e43] * other[e4],
                (self[scalar] * other[e4]) - (self[e41] * other[e1]) - (self[e42] * other[e2]) - (self[e43] * other[e3]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       18        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[e1234] * other[scalar], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e423] * other[scalar], self[e431] * other[scalar], self[e412] * other[scalar], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group2(),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, self[e4] * other[scalar]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
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
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e1234] * self[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[scalar] * self[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
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
            Simd32x4::from(self[e4]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e4]) * Simd32x4::from([other[e1], other[e2], other[e3], other[e321]]),
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
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       14        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e23] * self[e4], other[e31] * self[e4], other[e12] * self[e4], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([other[e41] * self[e4], other[e42] * self[e4], other[e43] * self[e4], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
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
            Simd32x4::from(self[e4]) * Simd32x4::from([other[e23], other[e31], other[e12], other[e1234]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4]) * Simd32x4::from([other[e41], other[e42], other[e43], other[scalar]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
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
            Simd32x2::from(self[e4]) * Simd32x2::from([other[e321], other[e4]]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(self[e4]) * Simd32x4::from([other[e23], other[e31], other[e12], other[e1234]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(self[e4]) * Simd32x3::from([other[e1], other[e2], other[e3]]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4]) * Simd32x4::from([other[e41], other[e42], other[e43], other[scalar]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
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
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self[e4] * other[e423], self[e4] * other[e431], self[e4] * other[e412], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([1.0, 1.0, 1.0, self[e4] * other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
    }
}
impl GeometricAntiProduct<Point> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([1.0, 1.0, 1.0, self[e4] * other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([self[e4] * other[e1], self[e4] * other[e2], self[e4] * other[e3], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
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
    //      f32        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       11        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[scalar] * self[e423], other[scalar] * self[e431], other[scalar] * self[e412], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group0(),
        );
    }
}
impl GeometricAntiProduct<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       17       25        0
    //  no simd       20       28        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e431] * self[e412]) - (other[e4] * self[e423]) - (other[e412] * self[e431]),
                (other[e412] * self[e423]) - (other[e4] * self[e431]) - (other[e423] * self[e412]),
                (other[e423] * self[e431]) - (other[e4] * self[e412]) - (other[e431] * self[e423]),
                (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e3] * self[e431]) + (other[e321] * self[e423]) - (other[e2] * self[e412]),
                (other[e1] * self[e412]) + (other[e321] * self[e431]) - (other[e3] * self[e423]),
                (other[e2] * self[e423]) + (other[e321] * self[e412]) - (other[e1] * self[e431]),
                -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
            ]) - (Simd32x4::from(self[e321]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]])),
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
            Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
        );
    }
}
impl GeometricAntiProduct<Line> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       21        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]) - (other[e12] * self[e431]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]) - (other[e23] * self[e412]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]) - (other[e31] * self[e423]),
                -(other[e41] * self[e423]) - (other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e43] * self[e431]) - (other[e42] * self[e412]),
                (other[e41] * self[e412]) - (other[e43] * self[e423]),
                (other[e42] * self[e423]) - (other[e41] * self[e431]),
                (other[e23] * self[e423]) + (other[e31] * self[e431]) + (other[e12] * self[e412]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       17       25        0
    //  no simd       20       28        0
    fn geometric_anti_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]) - (other[e12] * self[e431]) - (other[scalar] * self[e423]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]) - (other[e23] * self[e412]) - (other[scalar] * self[e431]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]) - (other[e31] * self[e423]) - (other[scalar] * self[e412]),
                -(other[e41] * self[e423]) - (other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e43] * self[e431]) - (other[e42] * self[e412]),
                (other[e41] * self[e412]) - (other[e43] * self[e423]),
                (other[e42] * self[e423]) - (other[e41] * self[e431]),
                (other[e23] * self[e423]) + (other[e31] * self[e431]) + (other[e12] * self[e412]),
            ]) + (Simd32x4::from(other[e1234]) * self.group0()),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       27       43        0
    //    simd3        3        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       31       47        0
    //  no simd       40       56        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
                (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e31] * self[e412]) - (other[scalar] * self[e423]) - (other[e12] * self[e431]),
                (other[e42] * self[e321]) + (other[e12] * self[e423]) - (other[scalar] * self[e431]) - (other[e23] * self[e412]),
                (other[e43] * self[e321]) + (other[e23] * self[e431]) - (other[scalar] * self[e412]) - (other[e31] * self[e423]),
                -(other[e41] * self[e423]) - (other[e42] * self[e431]) - (other[e43] * self[e412]),
            ]),
            // e41, e42, e43
            Simd32x3::from([
                (other[e431] * self[e412]) - (other[e412] * self[e431]),
                (other[e412] * self[e423]) - (other[e423] * self[e412]),
                (other[e423] * self[e431]) - (other[e431] * self[e423]),
            ]) - (Simd32x3::from(other[e4]) * Simd32x3::from([self[e423], self[e431], self[e412]])),
            // e23, e31, e12
            Simd32x3::from([
                (other[e3] * self[e431]) - (other[e2] * self[e412]),
                (other[e1] * self[e412]) - (other[e3] * self[e423]),
                (other[e2] * self[e423]) - (other[e1] * self[e431]),
            ]) + (Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]))
                - (Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e43] * self[e431]) - (other[e42] * self[e412]),
                (other[e41] * self[e412]) - (other[e43] * self[e423]),
                (other[e42] * self[e423]) - (other[e41] * self[e431]),
                (other[e23] * self[e423]) + (other[e31] * self[e431]) + (other[e12] * self[e412]),
            ]) + (Simd32x4::from(other[e1234]) * self.group0()),
        );
    }
}
impl GeometricAntiProduct<Origin> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([other[e4] * self[e423], other[e4] * self[e431], other[e4] * self[e412], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([1.0, 1.0, 1.0, other[e4] * self[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl GeometricAntiProduct<Plane> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       15        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e431] * self[e412]) - (other[e412] * self[e431]),
                (other[e412] * self[e423]) - (other[e423] * self[e412]),
                (other[e423] * self[e431]) - (other[e431] * self[e423]),
                (other[e423] * self[e423]) + (other[e431] * self[e431]) + (other[e412] * self[e412]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e321] * self[e423]) - (other[e423] * self[e321]),
                (other[e321] * self[e431]) - (other[e431] * self[e321]),
                (other[e321] * self[e412]) - (other[e412] * self[e321]),
                0.0,
            ]),
        );
    }
}
impl GeometricAntiProduct<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       17        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self[e423] * other[e4], self[e431] * other[e4], self[e412] * other[e4], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e431] * other[e3]) - (self[e412] * other[e2]),
                (self[e412] * other[e1]) - (self[e423] * other[e3]),
                (self[e423] * other[e2]) - (self[e431] * other[e1]),
                -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Scalar> for Plane {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn geometric_anti_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e423] * other[scalar], self[e431] * other[scalar], self[e412] * other[scalar], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
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
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn geometric_anti_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[scalar] * self[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
    }
}
impl GeometricAntiProduct<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        9       15        0
    //  no simd       12       24        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e431] * self[e3]) - (other[e4] * self[e1]) - (other[e412] * self[e2]),
                (other[e412] * self[e1]) - (other[e4] * self[e2]) - (other[e423] * self[e3]),
                (other[e423] * self[e2]) - (other[e4] * self[e3]) - (other[e431] * self[e1]),
                (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]),
            ]) + (Simd32x4::from(self[e4]) * Simd32x4::from([other[e1], other[e2], other[e3], other[e321]])),
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
        return Scalar::from_groups(/* scalar */ other[e321] * self[e4]);
    }
}
impl GeometricAntiProduct<Line> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       15        0
    fn geometric_anti_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e43] * self[e2]) - (other[e42] * self[e3]) - (other[e23] * self[e4]),
                (other[e41] * self[e3]) - (other[e43] * self[e1]) - (other[e31] * self[e4]),
                (other[e42] * self[e1]) - (other[e41] * self[e2]) - (other[e12] * self[e4]),
                0.0,
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                other[e41] * self[e4],
                other[e42] * self[e4],
                other[e43] * self[e4],
                -(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]),
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
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e43] * self[e2]) + (other[e1234] * self[e1]) - (other[e42] * self[e3]) - (other[e23] * self[e4]),
                (other[e41] * self[e3]) + (other[e1234] * self[e2]) - (other[e43] * self[e1]) - (other[e31] * self[e4]),
                (other[e42] * self[e1]) + (other[e1234] * self[e3]) - (other[e41] * self[e2]) - (other[e12] * self[e4]),
                other[e1234] * self[e4],
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                other[e41] * self[e4],
                other[e42] * self[e4],
                other[e43] * self[e4],
                -(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[scalar] * self[e4]),
            ]),
        );
    }
}
impl GeometricAntiProduct<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       31        0
    //    simd2        0        1        0
    //    simd3        2        4        0
    // Totals...
    // yes simd       20       36        0
    //  no simd       24       45        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
                other[e4] * self[e4],
            ]) * Simd32x2::from([1.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e1234] * self[e1]) + (other[e43] * self[e2]) - (other[e42] * self[e3]) - (other[e23] * self[e4]),
                (other[e1234] * self[e2]) + (other[e41] * self[e3]) - (other[e43] * self[e1]) - (other[e31] * self[e4]),
                (other[e1234] * self[e3]) + (other[e42] * self[e1]) - (other[e41] * self[e2]) - (other[e12] * self[e4]),
                other[e1234] * self[e4],
            ]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from([
                (other[e431] * self[e3]) - (other[e412] * self[e2]),
                (other[e412] * self[e1]) - (other[e423] * self[e3]),
                (other[e423] * self[e2]) - (other[e431] * self[e1]),
            ]) + (Simd32x3::from(self[e4]) * Simd32x3::from([other[e1], other[e2], other[e3]]))
                - (Simd32x3::from(other[e4]) * Simd32x3::from([self[e1], self[e2], self[e3]])),
            // e423, e431, e412, e321
            Simd32x4::from([
                other[e41] * self[e4],
                other[e42] * self[e4],
                other[e43] * self[e4],
                -(other[scalar] * self[e4]) - (other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Origin> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn geometric_anti_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([1.0, 1.0, 1.0, other[e4] * self[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([other[e4] * self[e1], other[e4] * self[e2], other[e4] * self[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricAntiProduct<Plane> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       13        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       14        0
    //  no simd        6       17        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([other[e423] * self[e4], other[e431] * self[e4], other[e412] * self[e4], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e431] * self[e3]) - (other[e412] * self[e2]),
                (other[e412] * self[e1]) - (other[e423] * self[e3]),
                (other[e423] * self[e2]) - (other[e431] * self[e1]),
                (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
            ]),
        );
    }
}
impl GeometricAntiProduct<Point> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        3       11        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([1.0, 1.0, 1.0, other[e4] * self[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e1] * self[e4]) - (other[e4] * self[e1]),
                (other[e2] * self[e4]) - (other[e4] * self[e2]),
                (other[e3] * self[e4]) - (other[e4] * self[e3]),
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
        return Horizon::from_groups(/* e321 */ self[e4] * other[scalar] * -1.0);
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
        return Scalar::from_groups(/* scalar */ other[e1234] * self[scalar]);
    }
}
impl GeometricAntiProduct<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn geometric_anti_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e423] * self[scalar], other[e431] * self[scalar], other[e412] * self[scalar], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e4] * self[scalar]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
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
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       18        0
    fn geometric_anti_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[e1234] * self[scalar], 1.0]) * Simd32x2::from([1.0, 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e423] * self[scalar], other[e431] * self[scalar], other[e412] * self[scalar], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group2(),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e4] * self[scalar]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
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
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn geometric_anti_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e423] * self[scalar], other[e431] * self[scalar], other[e412] * self[scalar], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricAntiProduct<Point> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_anti_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[e4] * self[scalar]);
    }
}
