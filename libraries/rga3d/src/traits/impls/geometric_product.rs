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
//   Median:         0       3       0
//  Average:         5       9       0
//  Maximum:        81      92       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       1       0
//   Median:         0      10       0
//  Average:        11      19       0
//  Maximum:       181     192       0
impl std::ops::Div<GeometricProductInfix> for AntiScalar {
    type Output = GeometricProductInfixPartial<AntiScalar>;
    fn div(self, _rhs: GeometricProductInfix) -> Self::Output {
        GeometricProductInfixPartial(self)
    }
}
impl GeometricProduct<DualNum> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar]);
    }
}
impl GeometricProduct<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([self[e1234], self[e1234], self[e1234], 0.0]) * other.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Horizon> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e1234] * other[e321] * -1.0);
    }
}
impl GeometricProduct<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(self[e1234]) * other.group1(), /* e23, e31, e12 */ Simd32x3::from(0.0));
    }
}
impl GeometricProduct<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group1(),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl GeometricProduct<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       16        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, self[e1234] * other[scalar]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * other[e321] * -1.0),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([self[e1234], self[e1234], self[e1234], 0.0]) * other.group1().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Plane> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e1234] * other[e321] * -1.0);
    }
}
impl GeometricProduct<Point> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([self[e1234], self[e1234], self[e1234], 0.0]) * other.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Scalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e1234] * other[scalar]);
    }
}
impl std::ops::Div<GeometricProductInfix> for DualNum {
    type Output = GeometricProductInfixPartial<DualNum>;
    fn div(self, _rhs: GeometricProductInfix) -> Self::Output {
        GeometricProductInfixPartial(self)
    }
}
impl GeometricProduct<AntiScalar> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e1234] * self[scalar]);
    }
}
impl GeometricProduct<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])]),
        );
    }
}
impl GeometricProduct<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        4       13        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * self.group0().xx().with_zw(self[scalar], (self[scalar] * other[e4]) - (self[e1234] * other[e321])),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * other.group1().xyz()) - (Simd32x3::from(self[e1234]) * other.group0().xyz())).with_w(self[scalar] * other[e321]),
        );
    }
}
impl GeometricProduct<Horizon> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(self[scalar] * other[e321]),
        );
    }
}
impl GeometricProduct<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        3        0
    // no simd        3        9        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group0()) + (Simd32x3::from(self[e1234]) * other.group1()),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group1(),
        );
    }
}
impl GeometricProduct<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        3        0
    // no simd        4       12        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[scalar]) * other.group0()) + (Simd32x4::from(self[e1234]) * other.group1()),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group1(),
        );
    }
}
impl GeometricProduct<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        8       25        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar] * other[scalar], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 1.0]) * self.group0().xx().with_zw(self[scalar], (self[scalar] * other[e4]) - (self[e1234] * other[e321])),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group2()) + (Simd32x3::from(self[e1234]) * other.group3()),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group3(),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * other.group4().xyz()) - (Simd32x3::from(self[e1234]) * other.group1().xyz())).with_w(self[scalar] * other[e321]),
        );
    }
}
impl GeometricProduct<Origin> for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[scalar] * other[e4]);
    }
}
impl GeometricProduct<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[e1234] * other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * other.group0(),
        );
    }
}
impl GeometricProduct<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
            // e423, e431, e412, e321
            self.group0().yy().with_zw(self[e1234], 0.0) * Simd32x3::from(1.0).with_w(0.0) * other.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar]) * self.group0());
    }
}
impl std::ops::Div<GeometricProductInfix> for Flector {
    type Output = GeometricProductInfixPartial<Flector>;
    fn div(self, _rhs: GeometricProductInfix) -> Self::Output {
        GeometricProductInfixPartial(self)
    }
}
impl GeometricProduct<AntiScalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e1234] * self[e321]),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1234], other[e1234], other[e1234], 0.0]) * self.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricProduct<DualNum> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        4       13        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], 1.0]) * other.group0().xx().with_zw(other[scalar], (other[scalar] * self[e4]) + (other[e1234] * self[e321])),
            // e423, e431, e412, e321
            ((Simd32x3::from(other[scalar]) * self.group1().xyz()) + (Simd32x3::from(other[e1234]) * self.group0().xyz())).with_w(other[scalar] * self[e321]),
        );
    }
}
impl GeometricProduct<Flector> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        9       10        0
    // Totals...
    // yes simd       13       18        0
    //  no simd       40       48        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]))
                + (other.group1().zxyz() * self.group0().yzxz())
                + (self.group0().ww().with_zw(self[e431], self[e1]) * other.group0().xyx().with_w(other[e423]))
                + (self.group1().zx().with_zw(self[e4], self[e2]) * other.group0().yzz().with_w(other[e431]))
                - (other.group0().zxyx() * self.group1().yzxx())
                - (other.group0().wwwy() * self.group0().xyz().with_w(self[e431]))
                - (self.group0().zx().with_zw(self[e321], self[e321]) * other.group1().yzz().with_w(other[e4]))
                - (self.group1().ww().with_zw(self[e2], self[e412]) * other.group1().xyx().with_w(other[e3])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(other[e2] * self[e3]) - (other[e321] * self[e1]),
                -(other[e3] * self[e1]) - (other[e321] * self[e2]),
                -(other[e3] * self[e321]) - (other[e321] * self[e3]),
                (other[e2] * self[e2]) + (other[e3] * self[e3]),
            ]) + (other.group0().zxyx() * self.group0().yzxx())
                - (self.group1().ww().with_zw(self[e2], self[e321]) * other.group0().xyx().with_w(other[e321])),
        );
    }
}
impl GeometricProduct<Horizon> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        3        0
    // no simd        0       12        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e4]),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl GeometricProduct<Line> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        1        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       19       29        0
    //  no simd       28       37        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e3] * other[e31]) + (self[e321] * other[e23]),
                (self[e1] * other[e12]) + (self[e321] * other[e31]),
                (self[e2] * other[e23]) + (self[e321] * other[e12]),
                -(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]),
            ]) - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e3] * other[e42]) + (self[e4] * other[e23]) + (self[e412] * other[e31]) + (self[e321] * other[e41]),
                (self[e1] * other[e43]) + (self[e4] * other[e31]) + (self[e423] * other[e12]) + (self[e321] * other[e42]),
                (self[e2] * other[e41]) + (self[e4] * other[e12]) + (self[e431] * other[e23]) + (self[e321] * other[e43]),
                self[e3] * other[e12] * -1.0,
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]))
                - (other.group1().zxy() * self.group1().yzx()).with_w(self[e2] * other[e31]),
        );
    }
}
impl GeometricProduct<Motor> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       27        0
    //    simd3        0        2        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       22       33        0
    //  no simd       40       49        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e321] * other[e23],
                self[e321] * other[e31],
                self[e321] * other[e12],
                -(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]),
            ]) + (self.group0().xxyw() * other.group1().wzxw())
                + (self.group0().zyz() * other.group1().yww()).with_w(self[e321] * other[e1234])
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e3] * other[e42]) + (self[e4] * other[e23]) + (self[e423] * other[scalar]) + (self[e412] * other[e31]) + (self[e321] * other[e41]),
                (self[e2] * other[e1234]) + (self[e4] * other[e31]) + (self[e423] * other[e12]) + (self[e431] * other[scalar]) + (self[e321] * other[e42]),
                (self[e3] * other[e1234]) + (self[e4] * other[e12]) + (self[e431] * other[e23]) + (self[e412] * other[scalar]) + (self[e321] * other[e43]),
                self[e3] * other[e12] * -1.0,
            ]) + (self.group0().xxy() * other.group0().wzx()).with_w(self[e321] * other[scalar])
                - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23]))
                - (other.group1().zxyy() * self.group1().yzx().with_w(self[e2])),
        );
    }
}
impl GeometricProduct<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       30        0
    //    simd2        4        4        0
    //    simd3       10       13        0
    //    simd4        6        5        0
    // Totals...
    // yes simd       39       52        0
    //  no simd       81       97        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self[e4] * other[e321]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4])])
                + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]]))
                - (Simd32x2::from([other[e321], other[e1]]) * self.group1().wx()),
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e3] * other[e31],
                self[e1] * other[e12],
                self[e321] * other[e12],
                -(self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]),
            ]) + (Simd32x4::from(other[scalar]) * self.group0())
                + (self.group1().ww().with_zw(self[e2], self[e321]) * other.group3().xyx().with_w(other[e1234]))
                - (self.group0().yzxx() * other.group3().zxy().with_w(other[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group1().xyz())
                + (Simd32x3::from([other[e2], other[e321], other[e321]]) * self.group1().zyz())
                + (Simd32x3::from([other[e321], other[e3], other[e1]]) * self.group1().xxy())
                + (self.group0().yzx() * other.group4().zxy())
                - (Simd32x3::from(self[e321]) * other.group4().xyz())
                - (Simd32x3::from([other[e4], other[e412], other[e423]]) * self.group0().xxy())
                - (Simd32x3::from([other[e431], other[e4], other[e4]]) * self.group0().zyz())
                - (self.group1().yzx() * other.group1().zxy()),
            // e23, e31, e12
            (self.group0().yzx() * other.group1().zxy())
                - (Simd32x3::from(self[e321]) * other.group1().xyz())
                - (Simd32x3::from([other[e2], other[e321], other[e321]]) * self.group0().zyz())
                - (Simd32x3::from([other[e321], other[e3], other[e1]]) * self.group0().xxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e1] * other[e1234]) + (self[e3] * other[e42]) + (self[e4] * other[e23]) + (self[e412] * other[e31]) + (self[e321] * other[e41]),
                (self[e1] * other[e43]) + (self[e2] * other[e1234]) + (self[e4] * other[e31]) + (self[e423] * other[e12]) + (self[e321] * other[e42]),
                (self[e2] * other[e41]) + (self[e3] * other[e1234]) + (self[e4] * other[e12]) + (self[e431] * other[e23]) + (self[e321] * other[e43]),
                self[e3] * other[e12] * -1.0,
            ]) + (Simd32x4::from(other[scalar]) * self.group1())
                - (self.group0().yzxx() * other.group2().zxy().with_w(other[e23]))
                - (other.group3().zxy() * self.group1().yzx()).with_w(self[e2] * other[e31]),
        );
    }
}
impl GeometricProduct<Origin> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e4]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl GeometricProduct<Plane> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(self[e3] * other[e431]) - (self[e321] * other[e423]),
                -(self[e1] * other[e412]) - (self[e321] * other[e431]),
                -(self[e2] * other[e423]) - (self[e321] * other[e412]),
                (self[e3] * other[e412]) + (self[e4] * other[e321]),
            ]) + (self.group0().yzxx() * other.group0().zxyx())
                + (other.group0().wwwy() * self.group1().xyz().with_w(self[e2])),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * self.group0().xyz().with_w(self[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl GeometricProduct<Point> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       20       28        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e4] * other[e1]) + (self[e412] * other[e2]),
                (self[e4] * other[e2]) + (self[e423] * other[e3]),
                (self[e4] * other[e3]) + (self[e431] * other[e1]),
                -(self[e412] * other[e3]) - (self[e321] * other[e4]),
            ]) - (self.group1().yzxy() * other.group0().zxyy())
                - (other.group0().wwwx() * self.group0().xyz().with_w(self[e423])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(self[e3] * other[e2]) - (self[e321] * other[e1]),
                -(self[e1] * other[e3]) - (self[e321] * other[e2]),
                -(self[e2] * other[e1]) - (self[e321] * other[e3]),
                (self[e2] * other[e2]) + (self[e3] * other[e3]),
            ]) + (self.group0().yzxx() * other.group0().zxyx()),
        );
    }
}
impl GeometricProduct<Scalar> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * self.group1(),
        );
    }
}
impl std::ops::Div<GeometricProductInfix> for Horizon {
    type Output = GeometricProductInfixPartial<Horizon>;
    fn div(self, _rhs: GeometricProductInfix) -> Self::Output {
        GeometricProductInfixPartial(self)
    }
}
impl GeometricProduct<AntiScalar> for Horizon {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e1234] * self[e321]);
    }
}
impl GeometricProduct<DualNum> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e1234] * self[e321]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).with_w(other[scalar] * self[e321]),
        );
    }
}
impl GeometricProduct<Flector> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e321]) * other.group1().xyz().with_w(other[e4]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * other.group0().xyz().with_w(other[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl GeometricProduct<Horizon> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[e321] * self[e321] * -1.0);
    }
}
impl GeometricProduct<Line> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * other.group1().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * other.group0().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Motor> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e321]) * other.group1().xyz().with_w(other[e1234]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e321]) * other.group0().xyz().with_w(other[scalar]),
        );
    }
}
impl GeometricProduct<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       24        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e321]) * Simd32x2::from([other[e321], other[e4]]) * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from(self[e321]) * other.group3().with_w(other[e1234]),
            // e41, e42, e43
            Simd32x3::from(self[e321]) * other.group4().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * other.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(self[e321]) * other.group2().with_w(other[scalar]),
        );
    }
}
impl GeometricProduct<Origin> for Horizon {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e321] * other[e4] * -1.0);
    }
}
impl GeometricProduct<Plane> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * other.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(self[e321] * other[e321] * -1.0),
        );
    }
}
impl GeometricProduct<Point> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(self[e321] * other[e4] * -1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([self[e321], self[e321], self[e321], 0.0]) * other.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Scalar> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ self[e321] * other[scalar]);
    }
}
impl std::ops::Div<GeometricProductInfix> for Line {
    type Output = GeometricProductInfixPartial<Line>;
    fn div(self, _rhs: GeometricProductInfix) -> Self::Output {
        GeometricProductInfixPartial(self)
    }
}
impl GeometricProduct<AntiScalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(other[e1234]) * self.group1(), /* e23, e31, e12 */ Simd32x3::from(0.0));
    }
}
impl GeometricProduct<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        3        0
    // no simd        3        9        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group1()),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group1(),
        );
    }
}
impl GeometricProduct<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       14        0
    //    simd3        0        2        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       14       20        0
    //  no simd       32       36        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group1().zyz().with_w(self[e42]))
                + (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group1().xxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((other[e3] * self[e43]) - (other[e431] * self[e31]) - (other[e412] * self[e12]))
                - (self.group1().yzx() * other.group0().zxy()).with_w(other[e423] * self[e23]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e431] * self[e12]),
                (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e412] * self[e23]),
                (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e423] * self[e31]),
                0.0,
            ]) - (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group0().zyz().with_w(self[e31]))
                - (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group0().xxy().with_w(self[e23]))
                - (self.group1().yzx() * other.group1().zxy()).with_w(other[e3] * self[e12]),
        );
    }
}
impl GeometricProduct<Horizon> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e321], other[e321], other[e321], 0.0]) * self.group1().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([other[e321], other[e321], other[e321], 0.0]) * self.group0().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        0        3        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       19       27        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e42] * self[e12]) + (other[e31] * self[e43]),
                (other[e43] * self[e23]) + (other[e12] * self[e41]),
                (other[e41] * self[e31]) + (other[e23] * self[e42]),
                -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]) - (other.group0().zxy() * self.group1().yzx()).with_w(other[e41] * self[e23])
                - (other.group1().zxy() * self.group0().yzx()).with_w(other[e42] * self[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[e31] * self[e12],
                other[e12] * self[e23],
                other[e23] * self[e31],
                -(other[e31] * self[e31]) - (other[e12] * self[e12]),
            ]) - (other.group1().zxy() * self.group1().yzx()).with_w(other[e23] * self[e23]),
        );
    }
}
impl GeometricProduct<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd3        0        1        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       28       36        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e41] * other[scalar]) + (self[e43] * other[e31]) + (self[e23] * other[e1234]) + (self[e12] * other[e42]),
                (self[e41] * other[e12]) + (self[e42] * other[scalar]) + (self[e23] * other[e43]) + (self[e31] * other[e1234]),
                (self[e42] * other[e23]) + (self[e43] * other[scalar]) + (self[e31] * other[e41]) + (self[e12] * other[e1234]),
                -(self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
            ]) - (other.group1().zxyx() * self.group0().yzx().with_w(self[e41]))
                - (self.group1().yzx() * other.group0().zxy()).with_w(self[e42] * other[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e23] * other[scalar]) + (self[e12] * other[e31]),
                (self[e23] * other[e12]) + (self[e31] * other[scalar]),
                (self[e31] * other[e23]) + (self[e12] * other[scalar]),
                -(self[e31] * other[e31]) - (self[e12] * other[e12]),
            ]) - (other.group1().zxyx() * self.group1().yzx().with_w(self[e23])),
        );
    }
}
impl GeometricProduct<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       17        0
    //    simd2        3        3        0
    //    simd3        7       11        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       26       35        0
    //  no simd       61       72        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, -(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])])
                - (Simd32x2::from(other[e23]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e31]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e12]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group1().zyz().with_w(self[e42]))
                + (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group1().xxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((self[e43] * other[e3]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
                - (self.group1().yzx() * other.group1().zxy()).with_w(self[e23] * other[e423]),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group0())
                + (Simd32x3::from(other[e1234]) * self.group1())
                + (self.group0().zxy() * other.group3().yzx())
                + (self.group1().zxy() * other.group2().yzx())
                - (self.group0().yzx() * other.group3().zxy())
                - (self.group1().yzx() * other.group2().zxy()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group1()) + (self.group1().zxy() * other.group3().yzx()) - (self.group1().yzx() * other.group3().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]) + (self[e12] * other[e431]),
                (self[e43] * other[e1]) + (self[e23] * other[e412]) + (self[e31] * other[e4]),
                (self[e41] * other[e2]) + (self[e31] * other[e423]) + (self[e12] * other[e4]),
                0.0,
            ]) - (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group0().zyz().with_w(self[e31]))
                - (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group0().xxy().with_w(self[e23]))
                - (self.group1().yzx() * other.group4().zxy()).with_w(self[e12] * other[e3]),
        );
    }
}
impl GeometricProduct<Origin> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e4], other[e4], other[e4], 0.0]) * self.group1().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd       10       16        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e321], other[e321], other[e321], 1.0]) * self.group1().with_w(-(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])),
            // e423, e431, e412, e321
            (self.group1().zxy() * other.group0().yzx()).with_w(0.0)
                - (Simd32x3::from(other[e321]) * self.group0()).with_w(0.0)
                - (self.group1().yzx() * other.group0().zxy()).with_w(0.0),
        );
    }
}
impl GeometricProduct<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       18        0
    //  no simd       13       24        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e31] * other[e3] * -1.0,
                self[e12] * other[e1] * -1.0,
                self[e23] * other[e2] * -1.0,
                (self[e42] * other[e2]) + (self[e43] * other[e3]),
            ]) + (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]),
                (self[e43] * other[e1]) + (self[e31] * other[e4]),
                (self[e41] * other[e2]) + (self[e12] * other[e4]),
                -(self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl GeometricProduct<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group1(),
        );
    }
}
impl std::ops::Div<GeometricProductInfix> for Motor {
    type Output = GeometricProductInfixPartial<Motor>;
    fn div(self, _rhs: GeometricProductInfix) -> Self::Output {
        GeometricProductInfixPartial(self)
    }
}
impl GeometricProduct<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234]) * self.group1(),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl GeometricProduct<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        3        0
    // no simd        4       12        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[scalar]) * self.group0()) + (Simd32x4::from(other[e1234]) * self.group1()),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group1(),
        );
    }
}
impl GeometricProduct<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       18        0
    //    simd3        0        2        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       20       26        0
    //  no simd       44       48        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            (other.group0().xyxx() * self.group1().wwy().with_w(self[e41]))
                + (other.group0().yzzy() * self.group1().zxw().with_w(self[e42]))
                + Simd32x3::from(0.0).with_w((other[e4] * self[scalar]) - (other[e431] * self[e31]) - (other[e412] * self[e12]) - (other[e321] * self[e1234]))
                + (other.group1().www() * self.group1().xyz()).with_w(other[e3] * self[e43])
                - (self.group1().yzxx() * other.group0().zxy().with_w(other[e423])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e4] * self[e23]) + (other[e423] * self[scalar]) + (other[e431] * self[e12]) - (other[e321] * self[e41]),
                (other[e4] * self[e31]) + (other[e431] * self[scalar]) + (other[e412] * self[e23]) - (other[e321] * self[e42]),
                (other[e4] * self[e12]) + (other[e423] * self[e31]) + (other[e412] * self[scalar]) - (other[e321] * self[e43]),
                0.0,
            ]) + (other.group0().zxy() * self.group0().yzx()).with_w(other[e321] * self[scalar])
                - (other.group0().xyxx() * self.group0().wwy().with_w(self[e23]))
                - (other.group0().yzzy() * self.group0().zxw().with_w(self[e31]))
                - (self.group1().yzxz() * other.group1().zxy().with_w(other[e3])),
        );
    }
}
impl GeometricProduct<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e321]) * self.group1().xyz().with_w(self[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(other[e321]) * self.group0().xyz().with_w(self[scalar]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl GeometricProduct<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd3        0        1        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       28       36        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e41] * self[scalar]) + (other[e42] * self[e12]) + (other[e23] * self[e1234]) + (other[e31] * self[e43]),
                (other[e42] * self[scalar]) + (other[e43] * self[e23]) + (other[e31] * self[e1234]) + (other[e12] * self[e41]),
                (other[e41] * self[e31]) + (other[e43] * self[scalar]) + (other[e23] * self[e42]) + (other[e12] * self[e1234]),
                -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]) - (self.group1().yzxx() * other.group0().zxy().with_w(other[e41]))
                - (other.group1().zxy() * self.group0().yzx()).with_w(other[e42] * self[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e23] * self[scalar]) + (other[e31] * self[e12]),
                (other[e31] * self[scalar]) + (other[e12] * self[e23]),
                (other[e23] * self[e31]) + (other[e12] * self[scalar]),
                -(other[e31] * self[e31]) - (other[e12] * self[e12]),
            ]) - (self.group1().yzxx() * other.group1().zxy().with_w(other[e23])),
        );
    }
}
impl GeometricProduct<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       26        0
    //    simd3        0        2        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       22       32        0
    //  no simd       40       48        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e1234] * self[e23]) + (other[e23] * self[e1234]) + (other[e31] * self[e43]) + (other[scalar] * self[e41]),
                (other[e1234] * self[e31]) + (other[e31] * self[e1234]) + (other[e12] * self[e41]) + (other[scalar] * self[e42]),
                (other[e1234] * self[e12]) + (other[e23] * self[e42]) + (other[e12] * self[e1234]) + (other[scalar] * self[e43]),
                -(other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]) + (other.group0().xyxw() * self.group1().wwyw())
                + (other.group0().yzz() * self.group1().zxw()).with_w(other[scalar] * self[e1234])
                - (other.group0().zxyx() * self.group1().yzxx())
                - (other.group1().zxy() * self.group0().yzx()).with_w(other[e42] * self[e31]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e31] * self[e12]) + (other[scalar] * self[e23]),
                (other[e12] * self[e23]) + (other[scalar] * self[e31]),
                (other[e12] * self[scalar]) + (other[scalar] * self[e12]),
                -(other[e31] * self[e31]) - (other[e12] * self[e12]),
            ]) + (other.group1().xyxw() * self.group1().wwyw())
                - (other.group1().zxyx() * self.group1().yzxx()),
        );
    }
}
impl GeometricProduct<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       22        0
    //    simd2        4        4        0
    //    simd3       10       14        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       37       46        0
    //  no simd       85       96        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self[scalar] * other[e1234]) - (self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12])])
                + (Simd32x2::from(other[scalar]) * Simd32x2::from([self[scalar], self[e1234]]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([other[e12], other[e43]])),
            // e1, e2, e3, e4
            (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group1().zyz().with_w(self[e42]))
                + (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group1().xxy().with_w(self[e41]))
                + (other.group1().xyzz() * self.group1().www().with_w(self[e43]))
                + Simd32x3::from(0.0).with_w((self[scalar] * other[e4]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]))
                - (self.group1().yzx() * other.group1().zxy()).with_w(self[e1234] * other[e321]),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group0().xyz())
                + (Simd32x3::from(other[e1234]) * self.group1().xyz())
                + (other.group2().xyx() * self.group1().wwy())
                + (other.group2().yzz() * self.group1().zxw())
                + (other.group3().xyx() * self.group0().wwy())
                + (other.group3().yzz() * self.group0().zxw())
                - (other.group2().zxy() * self.group1().yzx())
                - (other.group3().zxy() * self.group0().yzx()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group1().xyz()) + (other.group3().xyx() * self.group1().wwy()) + (other.group3().yzz() * self.group1().zxw())
                - (other.group3().zxy() * self.group1().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e23] * other[e4]) + (self[e12] * other[e431]) + (self[scalar] * other[e423]) - (self[e31] * other[e412]),
                (self[e23] * other[e412]) + (self[e31] * other[e4]) + (self[scalar] * other[e431]) - (self[e12] * other[e423]),
                (self[e31] * other[e423]) + (self[e12] * other[e4]) + (self[scalar] * other[e412]) - (self[e23] * other[e431]),
                0.0,
            ]) + (self.group0().yzx() * other.group1().zxy()).with_w(self[scalar] * other[e321])
                - (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group0().zyz().with_w(self[e31]))
                - (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group0().xxy().with_w(self[e23]))
                - (other.group1().xyzz() * self.group0().www().with_w(self[e12])),
        );
    }
}
impl GeometricProduct<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[scalar] * other[e4]),
            // e423, e431, e412, e321
            Simd32x4::from([other[e4], other[e4], other[e4], 0.0]) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        0
    //  no simd       12       21        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e321], other[e321], other[e321], 1.0])
                * self
                    .group1()
                    .xyz()
                    .with_w(-(self[e1234] * other[e321]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * other.group0().xyz()) + (self.group1().zxy() * other.group0().yzx())
                - (Simd32x3::from(other[e321]) * self.group0().xyz())
                - (self.group1().yzx() * other.group0().zxy()))
            .with_w(self[scalar] * other[e321]),
        );
    }
}
impl GeometricProduct<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       16        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        8       20        0
    //  no simd       20       32        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e31] * other[e3] * -1.0,
                self[e12] * other[e1] * -1.0,
                self[e23] * other[e2] * -1.0,
                (self[e43] * other[e3]) + (self[scalar] * other[e4]),
            ]) + (other.group0().xyzy() * self.group1().www().with_w(self[e42]))
                + (other.group0().yzxx() * self.group1().zxy().with_w(self[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]),
                (self[e43] * other[e1]) + (self[e31] * other[e4]),
                (self[e41] * other[e2]) + (self[e12] * other[e4]),
                self[e12] * other[e3] * -1.0,
            ]) - (other.group0().xyzy() * self.group0().www().with_w(self[e31]))
                - (other.group0().yzxx() * self.group0().zxy().with_w(self[e23])),
        );
    }
}
impl GeometricProduct<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group1(),
        );
    }
}
impl std::ops::Div<GeometricProductInfix> for MultiVector {
    type Output = GeometricProductInfixPartial<MultiVector>;
    fn div(self, _rhs: GeometricProductInfix) -> Self::Output {
        GeometricProductInfixPartial(self)
    }
}
impl GeometricProduct<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       15        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, other[e1234] * self[scalar]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e1234] * self[e321]),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1234], other[e1234], other[e1234], 0.0]) * self.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricProduct<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        2        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        8       25        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e1], self[e2], self[e3], 1.0]) * other.group0().xx().with_zw(other[scalar], (other[scalar] * self[e4]) + (other[e1234] * self[e321])),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(other[e1234]) * self.group3()),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group3(),
            // e423, e431, e412, e321
            ((Simd32x3::from(other[scalar]) * self.group4().xyz()) + (Simd32x3::from(other[e1234]) * self.group1().xyz())).with_w(other[scalar] * self[e321]),
        );
    }
}
impl GeometricProduct<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       21        0
    //    simd2        4        4        0
    //    simd3       10       13        0
    //    simd4        8        7        0
    // Totals...
    // yes simd       37       45        0
    //  no simd       85       96        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (other[e321] * self[e4]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321])])
                + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]]))
                - (Simd32x2::from([other[e321], other[e1]]) * self.group4().wx()),
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]) * other.group0())
                + (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group3().zyz().with_w(self[e42]))
                + (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group3().xxy().with_w(self[e41]))
                + Simd32x3::from(0.0).with_w((other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]))
                - (self.group3().yzx() * other.group0().zxy()).with_w(other[e321] * self[e1234]),
            // e41, e42, e43
            (Simd32x3::from(other[e321]) * self.group4().xyz())
                + (Simd32x3::from([self[e4], self[e4], self[e431]]) * other.group0().xyx())
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * other.group0().yzz())
                + (other.group1().zxy() * self.group1().yzx())
                - (Simd32x3::from(other[e4]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * other.group1().yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * other.group1().xyx())
                - (other.group0().zxy() * self.group4().yzx()),
            // e23, e31, e12
            (other.group0().zxy() * self.group1().yzx())
                - (Simd32x3::from(other[e321]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * other.group0().yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * other.group0().xyx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e431] * self[e12]) - (other[e412] * self[e31]),
                (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e412] * self[e23]) - (other[e423] * self[e12]),
                (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e423] * self[e31]) - (other[e431] * self[e23]),
                0.0,
            ]) + (Simd32x4::from(self[scalar]) * other.group1())
                - (Simd32x4::from([other[e2], other[e321], other[e321], other[e3]]) * self.group2().zyz().with_w(self[e12]))
                - (Simd32x4::from([other[e321], other[e3], other[e1], other[e2]]) * self.group2().xxy().with_w(self[e31]))
                - (other.group0().xyzx() * self.group0().yy().with_zw(self[e1234], self[e23])),
        );
    }
}
impl GeometricProduct<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        2        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       29        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[e321]) * Simd32x2::from([self[e321], self[e4]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e321]) * self.group3().with_w(self[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e41, e42, e43
            Simd32x3::from(other[e321]) * self.group4().xyz(),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(other[e321]) * self.group2().with_w(self[scalar]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
    }
}
impl GeometricProduct<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       29        0
    //    simd2        3        3        0
    //    simd3        7       10        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       31       44        0
    //  no simd       57       73        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, -(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])])
                - (Simd32x2::from(self[e23]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([other[e12], other[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e23] * self[e321]) + (other[e31] * self[e3]),
                (other[e31] * self[e321]) + (other[e12] * self[e1]),
                (other[e23] * self[e2]) + (other[e12] * self[e321]),
                -(other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]),
            ]) - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group0())
                + (Simd32x3::from(self[e1234]) * other.group1())
                + (other.group0().yzx() * self.group3().zxy())
                + (other.group1().yzx() * self.group2().zxy())
                - (other.group0().zxy() * self.group3().yzx())
                - (other.group1().zxy() * self.group2().yzx()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group1()) + (other.group1().yzx() * self.group3().zxy()) - (other.group1().zxy() * self.group3().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e42] * self[e3]) + (other[e23] * self[e4]) + (other[e31] * self[e412]),
                (other[e42] * self[e321]) + (other[e43] * self[e1]) + (other[e31] * self[e4]) + (other[e12] * self[e423]),
                (other[e41] * self[e2]) + (other[e43] * self[e321]) + (other[e23] * self[e431]) + (other[e12] * self[e4]),
                other[e12] * self[e3] * -1.0,
            ]) - (self.group1().yzxx() * other.group0().zxy().with_w(other[e23]))
                - (other.group1().zxy() * self.group4().yzx()).with_w(other[e31] * self[e2]),
        );
    }
}
impl GeometricProduct<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       29        0
    //    simd2        4        4        0
    //    simd3       10       12        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       39       51        0
    //  no simd       81       97        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (other[scalar] * self[e1234]) - (other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12])])
                + (Simd32x2::from(self[scalar]) * Simd32x2::from([other[scalar], other[e1234]]))
                - (Simd32x2::from(other[e23]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e31]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e12]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                other[scalar] * self[e1],
                other[scalar] * self[e2],
                other[scalar] * self[e3],
                -(other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]),
            ]) + (other.group1().yzzw() * self.group1().zx().with_zw(self[e321], self[e4]))
                + (self.group4().ww().with_zw(self[e2], self[e321]) * other.group1().xyx().with_w(other[e1234]))
                - (self.group1().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group0().xyz())
                + (Simd32x3::from(self[e1234]) * other.group1().xyz())
                + (self.group2().xxy() * other.group1().wzx())
                + (self.group2().zyz() * other.group1().yww())
                + (self.group3().xxy() * other.group0().wzx())
                + (self.group3().zyz() * other.group0().yww())
                - (self.group2().yzx() * other.group1().zxy())
                - (self.group3().yzx() * other.group0().zxy()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group1().xyz()) + (self.group3().xxy() * other.group1().wzx()) + (self.group3().zyz() * other.group1().yww())
                - (self.group3().yzx() * other.group1().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e42] * self[e3]) + (other[e1234] * self[e1]) + (other[e23] * self[e4]) + (other[e31] * self[e412]) + (other[scalar] * self[e423]),
                (other[e43] * self[e1]) + (other[e1234] * self[e2]) + (other[e31] * self[e4]) + (other[e12] * self[e423]) + (other[scalar] * self[e431]),
                (other[e43] * self[e321]) + (other[e1234] * self[e3]) + (other[e23] * self[e431]) + (other[e12] * self[e4]) + (other[scalar] * self[e412]),
                other[e12] * self[e3] * -1.0,
            ]) + (self.group4().ww().with_zw(self[e2], self[e321]) * other.group0().xyx().with_w(other[scalar]))
                - (other.group1().zxyy() * self.group4().yzx().with_w(self[e2]))
                - (self.group1().yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl GeometricProduct<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       44        0
    //    simd2        8        8        0
    //    simd3       22       28        0
    //    simd4       16       12        0
    // Totals...
    // yes simd       81       92        0
    //  no simd      181      192        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (other[e1234] * self[scalar]) + (other[e321] * self[e4])
                    - (other[e2] * self[e431])
                    - (other[e3] * self[e412])
                    - (other[e4] * self[e321])
                    - (other[e23] * self[e41])
                    - (other[e31] * self[e42])
                    - (other[e12] * self[e43]),
            ]) + (Simd32x2::from(other[scalar]) * self.group0())
                + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([other[e12], other[e43]]))
                - (Simd32x2::from([other[e321], other[e1]]) * self.group4().wx()),
            // e1, e2, e3, e4
            (Simd32x4::from(other[scalar]) * self.group1())
                + (Simd32x4::from([other[e2], other[e321], other[e321], other[e3]]) * self.group3().zyz().with_w(self[e43]))
                + (Simd32x4::from([other[e321], other[e3], other[e1], other[e2]]) * self.group3().xxy().with_w(self[e42]))
                + (self.group0().xx().with_zw(self[scalar], other[e1234]) * other.group1().xyz().with_w(self[e321]))
                + (self.group1().zx().with_zw(self[e321], other[e1]) * other.group3().yzz().with_w(self[e41]))
                + (self.group4().ww().with_zw(self[e2], other[e4]) * other.group3().xyx().with_w(self[scalar]))
                + Simd32x3::from(0.0).with_w(
                    -(other[e42] * self[e2])
                        - (other[e43] * self[e3])
                        - (other[e23] * self[e423])
                        - (other[e31] * self[e431])
                        - (other[e12] * self[e412])
                        - (other[e423] * self[e23])
                        - (other[e431] * self[e31])
                        - (other[e412] * self[e12]),
                )
                - (other.group3().zxy() * self.group1().yzx()).with_w(other[e321] * self[e1234])
                - (self.group3().yzx() * other.group1().zxy()).with_w(other[e41] * self[e1]),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group2())
                + (Simd32x3::from(other[e1234]) * self.group3())
                + (Simd32x3::from(other[e321]) * self.group4().xyz())
                + (Simd32x3::from(self[scalar]) * other.group2())
                + (Simd32x3::from(self[e1234]) * other.group3())
                + (Simd32x3::from([self[e4], self[e4], self[e431]]) * other.group1().xyx())
                + (Simd32x3::from([self[e412], self[e423], self[e4]]) * other.group1().yzz())
                + (other.group2().yzx() * self.group3().zxy())
                + (other.group3().yzx() * self.group2().zxy())
                + (other.group4().zxy() * self.group1().yzx())
                - (Simd32x3::from(other[e4]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * other.group4().yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * other.group4().xyx())
                - (other.group2().zxy() * self.group3().yzx())
                - (other.group3().zxy() * self.group2().yzx())
                - (other.group1().zxy() * self.group4().yzx()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group3())
                + (Simd32x3::from(self[scalar]) * other.group3())
                + (other.group3().yzx() * self.group3().zxy())
                + (other.group1().zxy() * self.group1().yzx())
                - (Simd32x3::from(other[e321]) * self.group1().xyz())
                - (Simd32x3::from([self[e3], self[e1], self[e321]]) * other.group1().yzz())
                - (Simd32x3::from([self[e321], self[e321], self[e2]]) * other.group1().xyx())
                - (other.group3().zxy() * self.group3().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e3] * self[e42])
                    + (other[e4] * self[e23])
                    + (other[e41] * self[e321])
                    + (other[e42] * self[e3])
                    + (other[e23] * self[e4])
                    + (other[e31] * self[e412])
                    + (other[e423] * self[scalar])
                    + (other[e431] * self[e12]),
                (other[e1] * self[e43])
                    + (other[e4] * self[e31])
                    + (other[e42] * self[e321])
                    + (other[e43] * self[e1])
                    + (other[e31] * self[e4])
                    + (other[e12] * self[e423])
                    + (other[e431] * self[scalar])
                    + (other[e412] * self[e23]),
                (other[e2] * self[e41])
                    + (other[e4] * self[e12])
                    + (other[e41] * self[e2])
                    + (other[e43] * self[e321])
                    + (other[e23] * self[e431])
                    + (other[e12] * self[e4])
                    + (other[e423] * self[e31])
                    + (other[e412] * self[scalar]),
                0.0,
            ]) + (Simd32x4::from(other[scalar]) * self.group4())
                + (other.group0().yy().with_zw(other[e1234], self[scalar]) * self.group1().xyz().with_w(other[e321]))
                - (Simd32x4::from([other[e2], other[e321], other[e321], other[e2]]) * self.group2().zyz().with_w(self[e31]))
                - (Simd32x4::from([other[e321], other[e3], other[e1], other[e1]]) * self.group2().xxy().with_w(self[e23]))
                - (self.group1().yzxy() * other.group2().zxy().with_w(other[e31]))
                - (self.group0().yy().with_zw(self[e1234], other[e23]) * other.group1().xyz().with_w(self[e1]))
                - (other.group3().zxy() * self.group4().yzx()).with_w(other[e12] * self[e3])
                - (self.group3().yzx() * other.group4().zxy()).with_w(other[e3] * self[e12]),
        );
    }
}
impl GeometricProduct<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       18        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, self[e321] * other[e4]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(self[scalar] * other[e4]),
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e4], other[e4], other[e4], 0.0]) * self.group3().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd2        0        1        0
    //    simd3        6       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       24       46        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                self[e321] * other[e321],
                (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]),
            ]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e321], other[e321], other[e321], 1.0])
                * self
                    .group3()
                    .with_w(-(self[e1234] * other[e321]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412])),
            // e41, e42, e43
            (Simd32x3::from(other[e321]) * self.group4().xyz()) + (self.group1().yzx() * other.group0().zxy())
                - (Simd32x3::from(self[e321]) * other.group0().xyz())
                - (self.group1().zxy() * other.group0().yzx()),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * self.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * other.group0().xyz()) + (self.group3().zxy() * other.group0().yzx())
                - (Simd32x3::from(other[e321]) * self.group2())
                - (self.group3().yzx() * other.group0().zxy()))
            .with_w(self[scalar] * other[e321]),
        );
    }
}
impl GeometricProduct<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       23        0
    //    simd3        5        7        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       18       34        0
    //  no simd       40       60        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]),
                -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e31] * other[e3] * -1.0,
                self[e12] * other[e1] * -1.0,
                self[e23] * other[e2] * -1.0,
                (self[e42] * other[e2]) + (self[e43] * other[e3]),
            ]) + (Simd32x4::from(self[scalar]) * other.group0())
                + (other.group0().yzxx() * self.group3().zxy().with_w(self[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group0().xyz()) + (self.group4().zxy() * other.group0().yzx())
                - (Simd32x3::from(other[e4]) * self.group1().xyz())
                - (self.group4().yzx() * other.group0().zxy()),
            // e23, e31, e12
            (self.group1().yzx() * other.group0().zxy()) - (Simd32x3::from(self[e321]) * other.group0().xyz()) - (self.group1().zxy() * other.group0().yzx()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]),
                (self[e43] * other[e1]) + (self[e31] * other[e4]),
                (self[e41] * other[e2]) + (self[e12] * other[e4]),
                self[e12] * other[e3] * -1.0,
            ]) - (other.group0().xyzx() * self.group0().yy().with_zw(self[e1234], self[e23]))
                - (other.group0().yzxy() * self.group2().zxy().with_w(self[e31])),
        );
    }
}
impl GeometricProduct<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[scalar]) * self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group1(),
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group2(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * self.group4(),
        );
    }
}
impl std::ops::Div<GeometricProductInfix> for Origin {
    type Output = GeometricProductInfixPartial<Origin>;
    fn div(self, _rhs: GeometricProductInfix) -> Self::Output {
        GeometricProductInfixPartial(self)
    }
}
impl GeometricProduct<DualNum> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[scalar] * self[e4]);
    }
}
impl GeometricProduct<Flector> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * other.group0().xyz().with_w(other[e321]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl GeometricProduct<Horizon> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e321] * self[e4]);
    }
}
impl GeometricProduct<Line> for Origin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 0.0]) * other.group1().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Motor> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[scalar] * self[e4]),
            // e423, e431, e412, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 0.0]) * other.group1().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricProduct<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       15        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, other[e321] * self[e4]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[scalar] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group1().xyz(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([self[e4], self[e4], self[e4], 0.0]) * other.group3().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Plane> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ self[e4] * other[e321]);
    }
}
impl GeometricProduct<Point> for Origin {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(self[e4]) * other.group0().xyz(), /* e23, e31, e12 */ Simd32x3::from(0.0));
    }
}
impl GeometricProduct<Scalar> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ self[e4] * other[scalar]);
    }
}
impl std::ops::Div<GeometricProductInfix> for Plane {
    type Output = GeometricProductInfixPartial<Plane>;
    fn div(self, _rhs: GeometricProductInfix) -> Self::Output {
        GeometricProductInfixPartial(self)
    }
}
impl GeometricProduct<AntiScalar> for Plane {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e1234] * self[e321]);
    }
}
impl GeometricProduct<DualNum> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e1234] * self[e321]),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * self.group0(),
        );
    }
}
impl GeometricProduct<Flector> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        2        4        0
    // Totals...
    // yes simd        6       12        0
    //  no simd       12       24        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e2] * self[e412]) + (other[e321] * self[e423]),
                (other[e3] * self[e423]) + (other[e321] * self[e431]),
                (other[e1] * self[e431]) + (other[e321] * self[e412]),
                -(other[e3] * self[e412]) - (other[e4] * self[e321]),
            ]) - (other.group0().zxyx() * self.group0().yzxx())
                - (self.group0().wwwy() * other.group1().xyz().with_w(other[e2])),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * other.group0().xyz().with_w(other[e321]) * Simd32x4::from(-1.0),
        );
    }
}
impl GeometricProduct<Horizon> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       10        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([other[e321], other[e321], other[e321], 0.0]) * self.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(other[e321] * self[e321] * -1.0),
        );
    }
}
impl GeometricProduct<Line> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd       10       16        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 1.0]) * other.group1().with_w(-(other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412])),
            // e423, e431, e412, e321
            (Simd32x3::from(self[e321]) * other.group0()).with_w(0.0) + (other.group1().yzx() * self.group0().zxy()).with_w(0.0)
                - (other.group1().zxy() * self.group0().yzx()).with_w(0.0),
        );
    }
}
impl GeometricProduct<Motor> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        5        0
    //    simd3        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        6       10        0
    //  no simd       12       21        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
                * other
                    .group1()
                    .xyz()
                    .with_w((other[e1234] * self[e321]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412])),
            // e423, e431, e412, e321
            ((Simd32x3::from(other[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[e321]) * other.group0().xyz()) + (other.group1().yzx() * self.group0().zxy())
                - (other.group1().zxy() * self.group0().yzx()))
            .with_w(other[scalar] * self[e321]),
        );
    }
}
impl GeometricProduct<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       10        0
    //    simd2        0        1        0
    //    simd3        6       10        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       24       46        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                other[e321] * self[e321],
                -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
            ]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self[e321], self[e321], self[e321], 1.0])
                * other
                    .group3()
                    .with_w((other[e1234] * self[e321]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412])),
            // e41, e42, e43
            (Simd32x3::from(other[e321]) * self.group0().xyz()) + (other.group1().yzx() * self.group0().zxy())
                - (Simd32x3::from(self[e321]) * other.group4().xyz())
                - (other.group1().zxy() * self.group0().yzx()),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * other.group1().xyz() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            ((Simd32x3::from(other[scalar]) * self.group0().xyz()) + (Simd32x3::from(self[e321]) * other.group2()) + (other.group3().yzx() * self.group0().zxy())
                - (other.group3().zxy() * self.group0().yzx()))
            .with_w(other[scalar] * self[e321]),
        );
    }
}
impl GeometricProduct<Origin> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e4] * self[e321] * -1.0);
    }
}
impl GeometricProduct<Plane> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3        8        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(other[e321]) * self.group0().xyz()) - (Simd32x3::from(self[e321]) * other.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).with_w(other[e321] * self[e321] * -1.0),
        );
    }
}
impl GeometricProduct<Point> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        6       22        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                self[e412] * other[e2],
                self[e423] * other[e3],
                self[e431] * other[e1],
                -(self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
            ]) - (self.group0().yzxx() * other.group0().zxyx()),
            // e23, e31, e12, scalar
            Simd32x3::from(1.0).with_w(0.0) * self.group0().www().with_w(0.0) * other.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Scalar> for Plane {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar]) * self.group0());
    }
}
impl std::ops::Div<GeometricProductInfix> for Point {
    type Output = GeometricProductInfixPartial<Point>;
    fn div(self, _rhs: GeometricProductInfix) -> Self::Output {
        GeometricProductInfixPartial(self)
    }
}
impl GeometricProduct<AntiScalar> for Point {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e1234], other[e1234], other[e1234], 0.0]) * self.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricProduct<DualNum> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            other.group0().yy().with_zw(other[e1234], 0.0) * Simd32x3::from(1.0).with_w(0.0) * self.group0().xyz().with_w(0.0) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Flector> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       20       28        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                -(other[e4] * self[e1]) - (other[e431] * self[e3]),
                -(other[e4] * self[e2]) - (other[e412] * self[e1]),
                -(other[e4] * self[e3]) - (other[e423] * self[e2]),
                (other[e412] * self[e3]) + (other[e321] * self[e4]),
            ]) + (other.group1().zxyy() * self.group0().yzxy())
                + (self.group0().wwwx() * other.group0().xyz().with_w(other[e423])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                -(other[e2] * self[e3]) - (other[e321] * self[e1]),
                -(other[e3] * self[e1]) - (other[e321] * self[e2]),
                -(other[e1] * self[e2]) - (other[e321] * self[e3]),
                (other[e2] * self[e2]) + (other[e3] * self[e3]),
            ]) + (other.group0().zxyx() * self.group0().yzxx()),
        );
    }
}
impl GeometricProduct<Horizon> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).with_w(other[e321] * self[e4]),
            // e23, e31, e12, scalar
            Simd32x4::from([other[e321], other[e321], other[e321], 0.0]) * self.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Line> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       15        0
    //  no simd       13       21        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e31] * self[e3], other[e12] * self[e1], other[e23] * self[e2], -(other[e42] * self[e2]) - (other[e43] * self[e3])])
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e42] * self[e3]) + (other[e23] * self[e4]),
                (other[e43] * self[e1]) + (other[e31] * self[e4]),
                (other[e41] * self[e2]) + (other[e12] * self[e4]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl GeometricProduct<Motor> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       11       19        0
    //  no simd       20       28        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                other[scalar] * self[e1],
                other[scalar] * self[e2],
                other[scalar] * self[e3],
                -(other[e42] * self[e2]) - (other[e43] * self[e3]),
            ]) + (other.group1().yzxw() * self.group0().zxyw())
                - (self.group0().yzxx() * other.group1().zxy().with_w(other[e41])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e42] * self[e3]) + (other[e1234] * self[e1]) + (other[e23] * self[e4]),
                (other[e43] * self[e1]) + (other[e1234] * self[e2]) + (other[e31] * self[e4]),
                (other[e41] * self[e2]) + (other[e1234] * self[e3]) + (other[e12] * self[e4]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (self.group0().yzxx() * other.group0().zxy().with_w(other[e23])),
        );
    }
}
impl GeometricProduct<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       17        0
    //    simd2        3        3        0
    //    simd3        5        7        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       19       30        0
    //  no simd       41       56        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[e321] * self[e4]])
                + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]])),
            // e1, e2, e3, e4
            Simd32x4::from([other[e31] * self[e3], other[e12] * self[e1], other[e23] * self[e2], -(other[e42] * self[e2]) - (other[e43] * self[e3])])
                + (Simd32x4::from(other[scalar]) * self.group0())
                - (self.group0().yzxx() * other.group3().zxy().with_w(other[e41])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * other.group1().xyz()) + (other.group4().zxy() * self.group0().yzx())
                - (Simd32x3::from(other[e4]) * self.group0().xyz())
                - (other.group4().yzx() * self.group0().zxy()),
            // e23, e31, e12
            (other.group1().zxy() * self.group0().yzx()) - (Simd32x3::from(other[e321]) * self.group0().xyz()) - (other.group1().yzx() * self.group0().zxy()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e1234] * self[e1]) + (other[e42] * self[e3]) + (other[e23] * self[e4]),
                (other[e1234] * self[e2]) + (other[e43] * self[e1]) + (other[e31] * self[e4]),
                (other[e1234] * self[e3]) + (other[e41] * self[e2]) + (other[e12] * self[e4]),
                -(other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) - (self.group0().yzxx() * other.group2().zxy().with_w(other[e23])),
        );
    }
}
impl GeometricProduct<Origin> for Point {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
    }
}
impl GeometricProduct<Plane> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        3       13        0
    //  no simd        6       25        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                other[e431] * self[e3] * -1.0,
                other[e412] * self[e1] * -1.0,
                other[e423] * self[e2] * -1.0,
                (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
            ]) + (other.group0().zxyx() * self.group0().yzxx()),
            // e23, e31, e12, scalar
            Simd32x3::from(1.0).with_w(0.0) * other.group0().www().with_w(0.0) * self.group0().xyz().with_w(0.0) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
    }
}
impl GeometricProduct<Point> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        8       18        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x3::from(self[e4]) * other.group0().xyz()) - (Simd32x3::from(other[e4]) * self.group0().xyz())).with_w(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                other[e2] * self[e3] * -1.0,
                other[e3] * self[e1] * -1.0,
                other[e1] * self[e2] * -1.0,
                (other[e2] * self[e2]) + (other[e3] * self[e3]),
            ]) + (other.group0().zxyx() * self.group0().yzxx()),
        );
    }
}
impl GeometricProduct<Scalar> for Point {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0());
    }
}
impl std::ops::Div<GeometricProductInfix> for Scalar {
    type Output = GeometricProductInfixPartial<Scalar>;
    fn div(self, _rhs: GeometricProductInfix) -> Self::Output {
        GeometricProductInfixPartial(self)
    }
}
impl GeometricProduct<AntiScalar> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return AntiScalar::from_groups(/* e1234 */ other[e1234] * self[scalar]);
    }
}
impl GeometricProduct<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn geometric_product(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[scalar]) * other.group0());
    }
}
impl GeometricProduct<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * other.group1(),
        );
    }
}
impl GeometricProduct<Horizon> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e321 */ other[e321] * self[scalar]);
    }
}
impl GeometricProduct<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn geometric_product(self, other: Line) -> Self::Output {
        use crate::elements::*;
        return Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group1(),
        );
    }
}
impl GeometricProduct<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn geometric_product(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group1(),
        );
    }
}
impl GeometricProduct<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn geometric_product(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[scalar]) * other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group1(),
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group2(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group3(),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * other.group4(),
        );
    }
}
impl GeometricProduct<Origin> for Scalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return Origin::from_groups(/* e4 */ other[e4] * self[scalar]);
    }
}
impl GeometricProduct<Plane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_product(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[scalar]) * other.group0());
    }
}
impl GeometricProduct<Point> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn geometric_product(self, other: Point) -> Self::Output {
        use crate::elements::*;
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[scalar]) * other.group0());
    }
}
impl GeometricProduct<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn geometric_product(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]);
    }
}
