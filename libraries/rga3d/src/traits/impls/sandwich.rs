use crate::traits::GeometricProduct;
use crate::traits::Reverse;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 109
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        15      30       0
//  Average:        34      47       0
//  Maximum:       260     293       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        19      39       0
//  Average:        44      64       0
//  Maximum:       354     394       0
impl std::ops::Div<sandwich> for AntiScalar {
    type Output = sandwich_partial<AntiScalar>;
    fn div(self, _rhs: sandwich) -> Self::Output {
        sandwich_partial(self)
    }
}
impl Sandwich<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       17        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(self[e1234] * other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from([self[e1234] * other[e1], self[e1234] * other[e2], self[e1234] * other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(self[e1234]) * other.group1(), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group1(),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       11        0
    //    simd2        0        2        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       17        0
    //  no simd        0       29        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, self[e1234] * other[scalar]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(self[e1234] * other[e321] * -1.0),
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([self[e1234] * other[e1], self[e1234] * other[e2], self[e1234] * other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([self[e1234] * other[e1], self[e1234] * other[e2], self[e1234] * other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl std::ops::Div<sandwich> for DualNum {
    type Output = sandwich_partial<DualNum>;
    fn div(self, _rhs: sandwich) -> Self::Output {
        sandwich_partial(self)
    }
}
impl Sandwich<AntiScalar> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = AntiScalar::from_groups(/* e1234 */ other[e1234] * self[scalar]);
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       12        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4       16        0
    //  no simd        8       24        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                self[scalar] * other[e1],
                self[scalar] * other[e2],
                self[scalar] * other[e3],
                (self[scalar] * other[e4]) - (self[e1234] * other[e321]),
            ]),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * other.group1().truncate_to_3()) - (Simd32x3::from(self[e1234]) * other.group0().truncate_to_3()))
                .extend_to_4(self[scalar] * other[e321]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        9        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        2       11        0
    //  no simd        4       15        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(self[e1234] * other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).extend_to_4(self[scalar] * other[e321]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        6        0
    // no simd        6       18        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group0()) + (Simd32x3::from(self[e1234]) * other.group1()),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group1(),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        6        0
    // no simd        8       24        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[scalar]) * other.group0()) + (Simd32x4::from(self[e1234]) * other.group1()),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group1(),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       18        0
    //    simd3        4       10        0
    // Totals...
    // yes simd        8       28        0
    //  no simd       16       48        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self[scalar] * other[scalar], (self[scalar] * other[e1234]) + (self[e1234] * other[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from([
                self[scalar] * other[e1],
                self[scalar] * other[e2],
                self[scalar] * other[e3],
                (self[scalar] * other[e4]) - (self[e1234] * other[e321]),
            ]),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group2()) + (Simd32x3::from(self[e1234]) * other.group3()),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group3(),
            // e423, e431, e412, e321
            ((Simd32x3::from(self[scalar]) * other.group4().truncate_to_3()) - (Simd32x3::from(self[e1234]) * other.group1().truncate_to_3()))
                .extend_to_4(self[scalar] * other[e321]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Origin::from_groups(/* e4 */ self[scalar] * other[e4]);
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        8        0
    //    simd3        1        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2       11        0
    //  no simd        4       18        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(self[e1234] * other[e321] * -1.0),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * other.group0(),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        9        0
    //    simd3        1        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2       13        0
    //  no simd        4       23        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([self[e1234] * other[e1], self[e1234] * other[e2], self[e1234] * other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        5        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[scalar]) * self.group0());
        return geometric_product.geometric_product(self.reverse());
    }
}
impl std::ops::Div<sandwich> for Flector {
    type Output = sandwich_partial<Flector>;
    fn div(self, _rhs: sandwich) -> Self::Output {
        sandwich_partial(self)
    }
}
impl Sandwich<AntiScalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       31       45        0
    //  no simd       40       60        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e1234] * self[e321]),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1234] * self[e1], other[e1234] * self[e2], other[e1234] * self[e3], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       42        0
    //    simd3        1        2        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       33       48        0
    //  no simd       44       64        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                other[scalar] * self[e1],
                other[scalar] * self[e2],
                other[scalar] * self[e3],
                (other[scalar] * self[e4]) + (other[e1234] * self[e321]),
            ]),
            // e423, e431, e412, e321
            ((Simd32x3::from(other[scalar]) * self.group1().truncate_to_3()) + (Simd32x3::from(other[e1234]) * self.group0().truncate_to_3()))
                .extend_to_4(other[scalar] * self[e321]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       65       82        0
    //  no simd       80      100        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e2] * self[e412]) + (other[e412] * self[e2]) + (other[e321] * self[e423]) - (other[e3] * self[e431]) - (other[e423] * self[e321]) - (other[e431] * self[e3]),
                (other[e3] * self[e423]) + (other[e423] * self[e3]) + (other[e321] * self[e431]) - (other[e1] * self[e412]) - (other[e431] * self[e321]) - (other[e412] * self[e1]),
                (other[e1] * self[e431]) + (other[e431] * self[e1]) + (other[e321] * self[e412]) - (other[e2] * self[e423]) - (other[e423] * self[e2]) - (other[e412] * self[e321]),
                (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
            ]) + (Simd32x4::from(self[e4]) * Simd32x4::from([other[e1], other[e2], other[e3], other[e321]]))
                - (Simd32x4::from(other[e4]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e321]])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e3] * self[e2]) - (other[e2] * self[e3]) - (other[e321] * self[e1]),
                (other[e1] * self[e3]) - (other[e3] * self[e1]) - (other[e321] * self[e2]),
                (other[e2] * self[e1]) - (other[e1] * self[e2]) - (other[e321] * self[e3]),
                (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]),
            ]) - (Simd32x4::from(self[e321]) * Simd32x4::from([other[e1], other[e2], other[e3], other[e321]])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       40        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       34       46        0
    //  no simd       40       64        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e321]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e321]]) * Simd32x4::from(-1.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       59       76        0
    //  no simd       68       88        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e3] * other[e31]) + (self[e321] * other[e23]) - (self[e2] * other[e12]),
                (self[e1] * other[e12]) + (self[e321] * other[e31]) - (self[e3] * other[e23]),
                (self[e2] * other[e23]) + (self[e321] * other[e12]) - (self[e1] * other[e31]),
                -(self[e1] * other[e41]) - (self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e3] * other[e42]) + (self[e4] * other[e23]) + (self[e412] * other[e31]) + (self[e321] * other[e41]) - (self[e2] * other[e43]) - (self[e431] * other[e12]),
                (self[e1] * other[e43]) + (self[e4] * other[e31]) + (self[e423] * other[e12]) + (self[e321] * other[e42]) - (self[e3] * other[e41]) - (self[e412] * other[e23]),
                (self[e2] * other[e41]) + (self[e4] * other[e12]) + (self[e431] * other[e23]) + (self[e321] * other[e43]) - (self[e1] * other[e42]) - (self[e423] * other[e31]),
                -(self[e1] * other[e23]) - (self[e2] * other[e31]) - (self[e3] * other[e12]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       62       79        0
    //  no simd       80      100        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e3] * other[e31]) - (self[e2] * other[e12]),
                (self[e1] * other[e12]) - (self[e3] * other[e23]),
                (self[e2] * other[e23]) - (self[e1] * other[e31]),
                -(self[e1] * other[e41]) - (self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]),
            ]) + (Simd32x4::from(self[e321]) * Simd32x4::from([other[e23], other[e31], other[e12], other[e1234]]))
                + (Simd32x4::from(other[scalar]) * self.group0()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e1] * other[e1234]) + (self[e3] * other[e42]) + (self[e4] * other[e23]) + (self[e412] * other[e31]) + (self[e321] * other[e41])
                    - (self[e2] * other[e43])
                    - (self[e431] * other[e12]),
                (self[e1] * other[e43]) + (self[e2] * other[e1234]) + (self[e4] * other[e31]) + (self[e423] * other[e12]) + (self[e321] * other[e42])
                    - (self[e3] * other[e41])
                    - (self[e412] * other[e23]),
                (self[e2] * other[e41]) + (self[e3] * other[e1234]) + (self[e4] * other[e12]) + (self[e431] * other[e23]) + (self[e321] * other[e43])
                    - (self[e1] * other[e42])
                    - (self[e423] * other[e31]),
                -(self[e1] * other[e23]) - (self[e2] * other[e31]) - (self[e3] * other[e12]),
            ]) + (Simd32x4::from(other[scalar]) * self.group1()),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       90      120        0
    //    simd2        8        8        0
    //    simd3       12       12        0
    //    simd4        5        6        0
    // Totals...
    // yes simd      115      146        0
    //  no simd      162      196        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self[e4] * other[e321]) - (self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3])])
                + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]]))
                - (Simd32x2::from(self[e321]) * Simd32x2::from([other[e321], other[e4]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e3] * other[e31]) - (self[e2] * other[e12]),
                (self[e1] * other[e12]) - (self[e3] * other[e23]),
                (self[e2] * other[e23]) - (self[e1] * other[e31]),
                -(self[e1] * other[e41]) - (self[e2] * other[e42]) - (self[e3] * other[e43]) - (self[e423] * other[e23]) - (self[e431] * other[e31]) - (self[e412] * other[e12]),
            ]) + (Simd32x4::from(self[e321]) * Simd32x4::from([other[e23], other[e31], other[e12], other[e1234]]))
                + (Simd32x4::from(other[scalar]) * self.group0()),
            // e41, e42, e43
            Simd32x3::from([
                (self[e2] * other[e412]) + (self[e412] * other[e2]) - (self[e3] * other[e431]) - (self[e431] * other[e3]),
                (self[e3] * other[e423]) + (self[e423] * other[e3]) - (self[e1] * other[e412]) - (self[e412] * other[e1]),
                (self[e1] * other[e431]) + (self[e431] * other[e1]) - (self[e2] * other[e423]) - (self[e423] * other[e2]),
            ]) + (Simd32x3::from(self[e4]) * other.group1().truncate_to_3())
                + (Simd32x3::from(other[e321]) * self.group1().truncate_to_3())
                - (Simd32x3::from(self[e321]) * other.group4().truncate_to_3())
                - (Simd32x3::from(other[e4]) * self.group0().truncate_to_3()),
            // e23, e31, e12
            Simd32x3::from([
                (self[e2] * other[e3]) - (self[e3] * other[e2]),
                (self[e3] * other[e1]) - (self[e1] * other[e3]),
                (self[e1] * other[e2]) - (self[e2] * other[e1]),
            ]) - (Simd32x3::from(self[e321]) * other.group1().truncate_to_3())
                - (Simd32x3::from(other[e321]) * self.group0().truncate_to_3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e1] * other[e1234]) + (self[e3] * other[e42]) + (self[e4] * other[e23]) + (self[e412] * other[e31]) + (self[e321] * other[e41])
                    - (self[e2] * other[e43])
                    - (self[e431] * other[e12]),
                (self[e1] * other[e43]) + (self[e2] * other[e1234]) + (self[e4] * other[e31]) + (self[e423] * other[e12]) + (self[e321] * other[e42])
                    - (self[e3] * other[e41])
                    - (self[e412] * other[e23]),
                (self[e2] * other[e41]) + (self[e3] * other[e1234]) + (self[e4] * other[e12]) + (self[e431] * other[e23]) + (self[e321] * other[e43])
                    - (self[e1] * other[e42])
                    - (self[e423] * other[e31]),
                -(self[e1] * other[e23]) - (self[e2] * other[e31]) - (self[e3] * other[e12]),
            ]) + (Simd32x4::from(other[scalar]) * self.group1()),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       40        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       34       45        0
    //  no simd       40       60        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e4]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e321]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       43       58        0
    //  no simd       52       76        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e2] * other[e412]) - (self[e3] * other[e431]) - (self[e321] * other[e423]),
                (self[e3] * other[e423]) - (self[e1] * other[e412]) - (self[e321] * other[e431]),
                (self[e1] * other[e431]) - (self[e2] * other[e423]) - (self[e321] * other[e412]),
                (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]),
            ]) + (Simd32x4::from(other[e321]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]])),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e321]]) * Simd32x4::from(-1.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       51       68        0
    //  no simd       60       80        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e4] * other[e1]) + (self[e412] * other[e2]) - (self[e431] * other[e3]),
                (self[e4] * other[e2]) + (self[e423] * other[e3]) - (self[e412] * other[e1]),
                (self[e4] * other[e3]) + (self[e431] * other[e1]) - (self[e423] * other[e2]),
                -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]),
            ]) - (Simd32x4::from(other[e4]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e321]])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e2] * other[e3]) - (self[e3] * other[e2]) - (self[e321] * other[e1]),
                (self[e3] * other[e1]) - (self[e1] * other[e3]) - (self[e321] * other[e2]),
                (self[e1] * other[e2]) - (self[e2] * other[e1]) - (self[e321] * other[e3]),
                (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       31       42        0
    //  no simd       40       60        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * self.group1(),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl std::ops::Div<sandwich> for Horizon {
    type Output = sandwich_partial<Horizon>;
    fn div(self, _rhs: sandwich) -> Self::Output {
        sandwich_partial(self)
    }
}
impl Sandwich<AntiScalar> for Horizon {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Origin::from_groups(/* e4 */ other[e1234] * self[e321]);
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       15        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e1234] * self[e321]),
            // e423, e431, e412, e321
            Simd32x3::from(0.0).extend_to_4(other[scalar] * self[e321]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       33        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e321]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * Simd32x4::from([other[e1], other[e2], other[e3], other[e321]]) * Simd32x4::from(-1.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Horizon {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ other[e321] * self[e321] * -1.0);
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       27        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321] * other[e23], self[e321] * other[e31], self[e321] * other[e12], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([self[e321] * other[e41], self[e321] * other[e42], self[e321] * other[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       21        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e321]) * Simd32x4::from([other[e23], other[e31], other[e12], other[e1234]]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e321]) * Simd32x4::from([other[e41], other[e42], other[e43], other[scalar]]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        4        0
    //    simd3        0        7        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0       18        0
    //  no simd        0       54        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(self[e321]) * Simd32x2::from([other[e321], other[e4]]) * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            Simd32x4::from(self[e321]) * Simd32x4::from([other[e23], other[e31], other[e12], other[e1234]]),
            // e41, e42, e43
            Simd32x3::from(self[e321]) * other.group4().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(self[e321]) * Simd32x4::from([other[e41], other[e42], other[e43], other[scalar]]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Horizon {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = AntiScalar::from_groups(/* e1234 */ self[e321] * other[e4] * -1.0);
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       26        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self[e321] * other[e423], self[e321] * other[e431], self[e321] * other[e412], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).extend_to_4(self[e321] * other[e321] * -1.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0       11        0
    //  no simd        0       26        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).extend_to_4(self[e321] * other[e4] * -1.0),
            // e23, e31, e12, scalar
            Simd32x4::from([self[e321] * other[e1], self[e321] * other[e2], self[e321] * other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Horizon::from_groups(/* e321 */ self[e321] * other[scalar]);
        return geometric_product.geometric_product(self.reverse());
    }
}
impl std::ops::Div<sandwich> for Line {
    type Output = sandwich_partial<Line>;
    fn div(self, _rhs: sandwich) -> Self::Output {
        sandwich_partial(self)
    }
}
impl Sandwich<AntiScalar> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       19       30        0
    //  no simd       19       36        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(other[e1234]) * self.group1(), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        0
    //    simd3        1        5        0
    // Totals...
    // yes simd       20       32        0
    //  no simd       22       42        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group1()),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group1(),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       56       74        0
    //  no simd       56       78        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e2] * self[e12]) + (other[e321] * self[e23]) - (other[e3] * self[e31]),
                (other[e3] * self[e23]) + (other[e321] * self[e31]) - (other[e1] * self[e12]),
                (other[e1] * self[e31]) + (other[e321] * self[e12]) - (other[e2] * self[e23]),
                (other[e1] * self[e41]) + (other[e2] * self[e42]) + (other[e3] * self[e43]) - (other[e423] * self[e23]) - (other[e431] * self[e31]) - (other[e412] * self[e12]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e431] * self[e12]) - (other[e2] * self[e43]) - (other[e412] * self[e31]) - (other[e321] * self[e41]),
                (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e412] * self[e23]) - (other[e3] * self[e41]) - (other[e423] * self[e12]) - (other[e321] * self[e42]),
                (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e423] * self[e31]) - (other[e1] * self[e42]) - (other[e431] * self[e23]) - (other[e321] * self[e43]),
                -(other[e1] * self[e23]) - (other[e2] * self[e31]) - (other[e3] * self[e12]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       42        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       28       46        0
    //  no simd       28       56        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e321] * self[e23], other[e321] * self[e31], other[e321] * self[e12], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([other[e321] * self[e41], other[e321] * self[e42], other[e321] * self[e43], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       47       63        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       47       65        0
    //  no simd       47       69        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e42] * self[e12]) + (other[e31] * self[e43]) - (other[e43] * self[e31]) - (other[e12] * self[e42]),
                (other[e43] * self[e23]) + (other[e12] * self[e41]) - (other[e41] * self[e12]) - (other[e23] * self[e43]),
                (other[e41] * self[e31]) + (other[e23] * self[e42]) - (other[e42] * self[e23]) - (other[e31] * self[e41]),
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e31] * self[e12]) - (other[e12] * self[e31]),
                (other[e12] * self[e23]) - (other[e23] * self[e12]),
                (other[e23] * self[e31]) - (other[e31] * self[e23]),
                -(other[e23] * self[e23]) - (other[e31] * self[e31]) - (other[e12] * self[e12]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       56       74        0
    //  no simd       56       78        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e41] * other[scalar]) + (self[e43] * other[e31]) + (self[e23] * other[e1234]) + (self[e12] * other[e42])
                    - (self[e42] * other[e12])
                    - (self[e31] * other[e43]),
                (self[e41] * other[e12]) + (self[e42] * other[scalar]) + (self[e23] * other[e43]) + (self[e31] * other[e1234])
                    - (self[e43] * other[e23])
                    - (self[e12] * other[e41]),
                (self[e42] * other[e23]) + (self[e43] * other[scalar]) + (self[e31] * other[e41]) + (self[e12] * other[e1234])
                    - (self[e41] * other[e31])
                    - (self[e23] * other[e42]),
                -(self[e41] * other[e23]) - (self[e42] * other[e31]) - (self[e43] * other[e12]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e23] * other[scalar]) + (self[e12] * other[e31]) - (self[e31] * other[e12]),
                (self[e23] * other[e12]) + (self[e31] * other[scalar]) - (self[e12] * other[e23]),
                (self[e31] * other[e23]) + (self[e12] * other[scalar]) - (self[e23] * other[e31]),
                -(self[e23] * other[e23]) - (self[e31] * other[e31]) - (self[e12] * other[e12]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       84      114        0
    //    simd2        6        6        0
    //    simd3        6        8        0
    // Totals...
    // yes simd       96      128        0
    //  no simd      114      150        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, -(self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])])
                - (Simd32x2::from(other[e23]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e31]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e12]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e23] * other[e321]) + (self[e12] * other[e2]) - (self[e31] * other[e3]),
                (self[e23] * other[e3]) + (self[e31] * other[e321]) - (self[e12] * other[e1]),
                (self[e31] * other[e1]) + (self[e12] * other[e321]) - (self[e23] * other[e2]),
                (self[e41] * other[e1]) + (self[e42] * other[e2]) + (self[e43] * other[e3]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
            ]),
            // e41, e42, e43
            Simd32x3::from([
                (self[e43] * other[e31]) + (self[e12] * other[e42]) - (self[e42] * other[e12]) - (self[e31] * other[e43]),
                (self[e41] * other[e12]) + (self[e23] * other[e43]) - (self[e43] * other[e23]) - (self[e12] * other[e41]),
                (self[e42] * other[e23]) + (self[e31] * other[e41]) - (self[e41] * other[e31]) - (self[e23] * other[e42]),
            ]) + (Simd32x3::from(other[scalar]) * self.group0())
                + (Simd32x3::from(other[e1234]) * self.group1()),
            // e23, e31, e12
            Simd32x3::from([
                (self[e12] * other[e31]) - (self[e31] * other[e12]),
                (self[e23] * other[e12]) - (self[e12] * other[e23]),
                (self[e31] * other[e23]) - (self[e23] * other[e31]),
            ]) + (Simd32x3::from(other[scalar]) * self.group1()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]) + (self[e12] * other[e431]) - (self[e41] * other[e321]) - (self[e43] * other[e2]) - (self[e31] * other[e412]),
                (self[e43] * other[e1]) + (self[e23] * other[e412]) + (self[e31] * other[e4]) - (self[e41] * other[e3]) - (self[e42] * other[e321]) - (self[e12] * other[e423]),
                (self[e41] * other[e2]) + (self[e31] * other[e423]) + (self[e12] * other[e4]) - (self[e42] * other[e1]) - (self[e43] * other[e321]) - (self[e23] * other[e431]),
                -(self[e23] * other[e1]) - (self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd3        0        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       19        0
    //  no simd        9       28        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([self[e23] * other[e4], self[e31] * other[e4], self[e12] * other[e4], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       48        0
    //    simd3        1        3        0
    // Totals...
    // yes simd       34       51        0
    //  no simd       36       57        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e23] * other[e321],
                self[e31] * other[e321],
                self[e12] * other[e321],
                -(self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
            ]),
            // e423, e431, e412, e321
            (Simd32x3::from([
                (self[e12] * other[e431]) - (self[e31] * other[e412]),
                (self[e23] * other[e412]) - (self[e12] * other[e423]),
                (self[e31] * other[e423]) - (self[e23] * other[e431]),
            ]) - (Simd32x3::from(other[e321]) * self.group0()))
            .extend_to_4(0.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       57        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       41       59        0
    //  no simd       41       63        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e12] * other[e2]) - (self[e31] * other[e3]),
                (self[e23] * other[e3]) - (self[e12] * other[e1]),
                (self[e31] * other[e1]) - (self[e23] * other[e2]),
                (self[e41] * other[e1]) + (self[e42] * other[e2]) + (self[e43] * other[e3]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]) - (self[e43] * other[e2]),
                (self[e43] * other[e1]) + (self[e31] * other[e4]) - (self[e41] * other[e3]),
                (self[e41] * other[e2]) + (self[e12] * other[e4]) - (self[e42] * other[e1]),
                -(self[e23] * other[e1]) - (self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        0
    //    simd3        0        4        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       19       39        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[scalar]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group1(),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl std::ops::Div<sandwich> for Motor {
    type Output = sandwich_partial<Motor>;
    fn div(self, _rhs: sandwich) -> Self::Output {
        sandwich_partial(self)
    }
}
impl Sandwich<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       31       42        0
    //  no simd       40       60        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234]) * self.group1(),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        4        8        0
    // Totals...
    // yes simd       32       44        0
    //  no simd       44       68        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[scalar]) * self.group0()) + (Simd32x4::from(other[e1234]) * self.group1()),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group1(),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       65       83        0
    //  no simd       80      104        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e2] * self[e12]) + (other[e321] * self[e23]) - (other[e3] * self[e31]),
                (other[e3] * self[e23]) + (other[e321] * self[e31]) - (other[e1] * self[e12]),
                (other[e1] * self[e31]) + (other[e321] * self[e12]) - (other[e2] * self[e23]),
                (other[e1] * self[e41]) + (other[e2] * self[e42]) + (other[e3] * self[e43])
                    - (other[e423] * self[e23])
                    - (other[e431] * self[e31])
                    - (other[e412] * self[e12])
                    - (other[e321] * self[e1234]),
            ]) + (Simd32x4::from(self[scalar]) * other.group0()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e431] * self[e12])
                    - (other[e1] * self[e1234])
                    - (other[e2] * self[e43])
                    - (other[e412] * self[e31])
                    - (other[e321] * self[e41]),
                (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e412] * self[e23])
                    - (other[e2] * self[e1234])
                    - (other[e3] * self[e41])
                    - (other[e423] * self[e12])
                    - (other[e321] * self[e42]),
                (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e423] * self[e31])
                    - (other[e1] * self[e42])
                    - (other[e3] * self[e1234])
                    - (other[e431] * self[e23])
                    - (other[e321] * self[e43]),
                -(other[e1] * self[e23]) - (other[e2] * self[e31]) - (other[e3] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * other.group1()),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        9        0
    // Totals...
    // yes simd       31       45        0
    //  no simd       40       72        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e321]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(other[e321]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       59       77        0
    //  no simd       68       92        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e41] * self[scalar]) + (other[e42] * self[e12]) + (other[e23] * self[e1234]) + (other[e31] * self[e43])
                    - (other[e43] * self[e31])
                    - (other[e12] * self[e42]),
                (other[e42] * self[scalar]) + (other[e43] * self[e23]) + (other[e31] * self[e1234]) + (other[e12] * self[e41])
                    - (other[e41] * self[e12])
                    - (other[e23] * self[e43]),
                (other[e41] * self[e31]) + (other[e43] * self[scalar]) + (other[e23] * self[e42]) + (other[e12] * self[e1234])
                    - (other[e42] * self[e23])
                    - (other[e31] * self[e41]),
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e23] * self[scalar]) + (other[e31] * self[e12]) - (other[e12] * self[e31]),
                (other[e31] * self[scalar]) + (other[e12] * self[e23]) - (other[e23] * self[e12]),
                (other[e23] * self[e31]) + (other[e12] * self[scalar]) - (other[e31] * self[e23]),
                -(other[e23] * self[e23]) - (other[e31] * self[e31]) - (other[e12] * self[e12]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       62       80        0
    //  no simd       80      104        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e42] * self[e12]) + (other[e1234] * self[e23]) + (other[e31] * self[e43]) + (other[scalar] * self[e41])
                    - (other[e43] * self[e31])
                    - (other[e12] * self[e42]),
                (other[e43] * self[e23]) + (other[e1234] * self[e31]) + (other[e12] * self[e41]) + (other[scalar] * self[e42])
                    - (other[e41] * self[e12])
                    - (other[e23] * self[e43]),
                (other[e41] * self[e31]) + (other[e1234] * self[e12]) + (other[e23] * self[e42]) + (other[scalar] * self[e43])
                    - (other[e42] * self[e23])
                    - (other[e31] * self[e41]),
                -(other[e41] * self[e23]) - (other[e42] * self[e31]) - (other[e43] * self[e12]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43]),
            ]) + (Simd32x4::from(self[e1234]) * other.group1())
                + (Simd32x4::from(self[scalar]) * other.group0()),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e31] * self[e12]) + (other[scalar] * self[e23]) - (other[e12] * self[e31]),
                (other[e12] * self[e23]) + (other[scalar] * self[e31]) - (other[e23] * self[e12]),
                (other[e23] * self[e31]) + (other[scalar] * self[e12]) - (other[e31] * self[e23]),
                -(other[e23] * self[e23]) - (other[e31] * self[e31]) - (other[e12] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * other.group1()),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       90      120        0
    //    simd2        8        8        0
    //    simd3       12       12        0
    //    simd4        5        7        0
    // Totals...
    // yes simd      115      147        0
    //  no simd      162      200        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self[scalar] * other[e1234]) - (self[e23] * other[e41]) - (self[e31] * other[e42]) - (self[e12] * other[e43])])
                + (Simd32x2::from(other[scalar]) * Simd32x2::from([self[scalar], self[e1234]]))
                - (Simd32x2::from(other[e23]) * Simd32x2::from([self[e23], self[e41]]))
                - (Simd32x2::from(other[e31]) * Simd32x2::from([self[e31], self[e42]]))
                - (Simd32x2::from(other[e12]) * Simd32x2::from([self[e12], self[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e23] * other[e321]) + (self[e12] * other[e2]) - (self[e31] * other[e3]),
                (self[e23] * other[e3]) + (self[e31] * other[e321]) - (self[e12] * other[e1]),
                (self[e31] * other[e1]) + (self[e12] * other[e321]) - (self[e23] * other[e2]),
                (self[e41] * other[e1]) + (self[e42] * other[e2]) + (self[e43] * other[e3])
                    - (self[e1234] * other[e321])
                    - (self[e23] * other[e423])
                    - (self[e31] * other[e431])
                    - (self[e12] * other[e412]),
            ]) + (Simd32x4::from(self[scalar]) * other.group1()),
            // e41, e42, e43
            Simd32x3::from([
                (self[e43] * other[e31]) + (self[e12] * other[e42]) - (self[e42] * other[e12]) - (self[e31] * other[e43]),
                (self[e41] * other[e12]) + (self[e23] * other[e43]) - (self[e43] * other[e23]) - (self[e12] * other[e41]),
                (self[e42] * other[e23]) + (self[e31] * other[e41]) - (self[e41] * other[e31]) - (self[e23] * other[e42]),
            ]) + (Simd32x3::from(self[e1234]) * other.group3())
                + (Simd32x3::from(self[scalar]) * other.group2())
                + (Simd32x3::from(other[scalar]) * self.group0().truncate_to_3())
                + (Simd32x3::from(other[e1234]) * self.group1().truncate_to_3()),
            // e23, e31, e12
            Simd32x3::from([
                (self[e12] * other[e31]) - (self[e31] * other[e12]),
                (self[e23] * other[e12]) - (self[e12] * other[e23]),
                (self[e31] * other[e23]) - (self[e23] * other[e31]),
            ]) + (Simd32x3::from(self[scalar]) * other.group3())
                + (Simd32x3::from(other[scalar]) * self.group1().truncate_to_3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]) + (self[e12] * other[e431])
                    - (self[e41] * other[e321])
                    - (self[e43] * other[e2])
                    - (self[e1234] * other[e1])
                    - (self[e31] * other[e412]),
                (self[e43] * other[e1]) + (self[e23] * other[e412]) + (self[e31] * other[e4])
                    - (self[e41] * other[e3])
                    - (self[e42] * other[e321])
                    - (self[e1234] * other[e2])
                    - (self[e12] * other[e423]),
                (self[e41] * other[e2]) + (self[e31] * other[e423]) + (self[e12] * other[e4])
                    - (self[e42] * other[e1])
                    - (self[e43] * other[e321])
                    - (self[e1234] * other[e3])
                    - (self[e23] * other[e431]),
                -(self[e23] * other[e1]) - (self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]) + (Simd32x4::from(self[scalar]) * other.group4()),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       31       46        0
    //  no simd       40       64        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(self[scalar] * other[e4]),
            // e423, e431, e412, e321
            Simd32x4::from([self[e23] * other[e4], self[e31] * other[e4], self[e12] * other[e4], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       34       50        0
    //    simd3        2        2        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       39       57        0
    //  no simd       52       76        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e23] * other[e321],
                self[e31] * other[e321],
                self[e12] * other[e321],
                -(self[e1234] * other[e321]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
            ]),
            // e423, e431, e412, e321
            (Simd32x3::from([
                (self[e12] * other[e431]) - (self[e31] * other[e412]),
                (self[e23] * other[e412]) - (self[e12] * other[e423]),
                (self[e31] * other[e423]) - (self[e23] * other[e431]),
            ]) + (Simd32x3::from(self[scalar]) * other.group0().truncate_to_3())
                - (Simd32x3::from(other[e321]) * self.group0().truncate_to_3()))
            .extend_to_4(self[scalar] * other[e321]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4        4        6        0
    // Totals...
    // yes simd       48       66        0
    //  no simd       60       84        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e12] * other[e2]) - (self[e31] * other[e3]),
                (self[e23] * other[e3]) - (self[e12] * other[e1]),
                (self[e31] * other[e1]) - (self[e23] * other[e2]),
                (self[e41] * other[e1]) + (self[e42] * other[e2]) + (self[e43] * other[e3]),
            ]) + (Simd32x4::from(self[scalar]) * other.group0()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]) - (self[e43] * other[e2]) - (self[e1234] * other[e1]),
                (self[e43] * other[e1]) + (self[e31] * other[e4]) - (self[e41] * other[e3]) - (self[e1234] * other[e2]),
                (self[e41] * other[e2]) + (self[e12] * other[e4]) - (self[e42] * other[e1]) - (self[e1234] * other[e3]),
                -(self[e23] * other[e1]) - (self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       31       43        0
    //  no simd       40       64        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[scalar]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group1(),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl std::ops::Div<sandwich> for MultiVector {
    type Output = sandwich_partial<MultiVector>;
    fn div(self, _rhs: sandwich) -> Self::Output {
        sandwich_partial(self)
    }
}
impl Sandwich<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      105      125        0
    //    simd2        8        9        0
    //    simd3       12       15        0
    //    simd4        5        7        0
    // Totals...
    // yes simd      130      156        0
    //  no simd      177      216        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, other[e1234] * self[scalar]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e1234] * self[e321]),
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1234] * self[e1], other[e1234] * self[e2], other[e1234] * self[e3], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      107      129        0
    //    simd2        8        8        0
    //    simd3       14       19        0
    //    simd4        5        6        0
    // Totals...
    // yes simd      134      162        0
    //  no simd      185      226        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other[scalar] * self[scalar], (other[scalar] * self[e1234]) + (other[e1234] * self[scalar])]),
            // e1, e2, e3, e4
            Simd32x4::from([
                other[scalar] * self[e1],
                other[scalar] * self[e2],
                other[scalar] * self[e3],
                (other[scalar] * self[e4]) + (other[e1234] * self[e321]),
            ]),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group2()) + (Simd32x3::from(other[e1234]) * self.group3()),
            // e23, e31, e12
            Simd32x3::from(other[scalar]) * self.group3(),
            // e423, e431, e412, e321
            ((Simd32x3::from(other[scalar]) * self.group4().truncate_to_3()) + (Simd32x3::from(other[e1234]) * self.group1().truncate_to_3()))
                .extend_to_4(other[scalar] * self[e321]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      152      182        0
    //    simd2       12       12        0
    //    simd3       18       20        0
    //    simd4        7        8        0
    // Totals...
    // yes simd      189      222        0
    //  no simd      258      298        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (other[e321] * self[e4]) - (other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412])])
                + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]]))
                - (Simd32x2::from(self[e321]) * Simd32x2::from([other[e321], other[e4]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e2] * self[e12]) + (other[e321] * self[e23]) - (other[e3] * self[e31]),
                (other[e3] * self[e23]) + (other[e321] * self[e31]) - (other[e1] * self[e12]),
                (other[e1] * self[e31]) + (other[e321] * self[e12]) - (other[e2] * self[e23]),
                (other[e1] * self[e41]) + (other[e2] * self[e42]) + (other[e3] * self[e43])
                    - (other[e423] * self[e23])
                    - (other[e431] * self[e31])
                    - (other[e412] * self[e12])
                    - (other[e321] * self[e1234]),
            ]) + (Simd32x4::from(self[scalar]) * other.group0()),
            // e41, e42, e43
            Simd32x3::from([
                (other[e2] * self[e412]) + (other[e412] * self[e2]) - (other[e3] * self[e431]) - (other[e431] * self[e3]),
                (other[e3] * self[e423]) + (other[e423] * self[e3]) - (other[e1] * self[e412]) - (other[e412] * self[e1]),
                (other[e1] * self[e431]) + (other[e431] * self[e1]) - (other[e2] * self[e423]) - (other[e423] * self[e2]),
            ]) + (Simd32x3::from(other[e321]) * self.group4().truncate_to_3())
                + (Simd32x3::from(self[e4]) * other.group0().truncate_to_3())
                - (Simd32x3::from(other[e4]) * self.group1().truncate_to_3())
                - (Simd32x3::from(self[e321]) * other.group1().truncate_to_3()),
            // e23, e31, e12
            Simd32x3::from([
                (other[e3] * self[e2]) - (other[e2] * self[e3]),
                (other[e1] * self[e3]) - (other[e3] * self[e1]),
                (other[e2] * self[e1]) - (other[e1] * self[e2]),
            ]) - (Simd32x3::from(other[e321]) * self.group1().truncate_to_3())
                - (Simd32x3::from(self[e321]) * other.group0().truncate_to_3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e3] * self[e42]) + (other[e4] * self[e23]) + (other[e431] * self[e12])
                    - (other[e1] * self[e1234])
                    - (other[e2] * self[e43])
                    - (other[e412] * self[e31])
                    - (other[e321] * self[e41]),
                (other[e1] * self[e43]) + (other[e4] * self[e31]) + (other[e412] * self[e23])
                    - (other[e2] * self[e1234])
                    - (other[e3] * self[e41])
                    - (other[e423] * self[e12])
                    - (other[e321] * self[e42]),
                (other[e2] * self[e41]) + (other[e4] * self[e12]) + (other[e423] * self[e31])
                    - (other[e1] * self[e42])
                    - (other[e3] * self[e1234])
                    - (other[e431] * self[e23])
                    - (other[e321] * self[e43]),
                -(other[e1] * self[e23]) - (other[e2] * self[e31]) - (other[e3] * self[e12]),
            ]) + (Simd32x4::from(self[scalar]) * other.group1()),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      105      120        0
    //    simd2        8       10        0
    //    simd3       12       17        0
    //    simd4        5       10        0
    // Totals...
    // yes simd      130      157        0
    //  no simd      177      231        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(other[e321]) * Simd32x2::from([self[e321], self[e4]]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(other[e321]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e1234]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e41, e42, e43
            Simd32x3::from(other[e321]) * self.group4().truncate_to_3(),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * self.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            Simd32x4::from(other[e321]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      147      177        0
    //    simd2       11       11        0
    //    simd3       15       17        0
    //    simd4        5        6        0
    // Totals...
    // yes simd      178      211        0
    //  no simd      234      274        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, -(other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])])
                - (Simd32x2::from(self[e23]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([other[e12], other[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e23] * self[e321]) + (other[e31] * self[e3]) - (other[e12] * self[e2]),
                (other[e31] * self[e321]) + (other[e12] * self[e1]) - (other[e23] * self[e3]),
                (other[e23] * self[e2]) + (other[e12] * self[e321]) - (other[e31] * self[e1]),
                -(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]),
            ]),
            // e41, e42, e43
            Simd32x3::from([
                (other[e42] * self[e12]) + (other[e31] * self[e43]) - (other[e43] * self[e31]) - (other[e12] * self[e42]),
                (other[e43] * self[e23]) + (other[e12] * self[e41]) - (other[e41] * self[e12]) - (other[e23] * self[e43]),
                (other[e41] * self[e31]) + (other[e23] * self[e42]) - (other[e42] * self[e23]) - (other[e31] * self[e41]),
            ]) + (Simd32x3::from(self[scalar]) * other.group0())
                + (Simd32x3::from(self[e1234]) * other.group1()),
            // e23, e31, e12
            Simd32x3::from([
                (other[e31] * self[e12]) - (other[e12] * self[e31]),
                (other[e12] * self[e23]) - (other[e23] * self[e12]),
                (other[e23] * self[e31]) - (other[e31] * self[e23]),
            ]) + (Simd32x3::from(self[scalar]) * other.group1()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e41] * self[e321]) + (other[e42] * self[e3]) + (other[e23] * self[e4]) + (other[e31] * self[e412]) - (other[e43] * self[e2]) - (other[e12] * self[e431]),
                (other[e42] * self[e321]) + (other[e43] * self[e1]) + (other[e31] * self[e4]) + (other[e12] * self[e423]) - (other[e41] * self[e3]) - (other[e23] * self[e412]),
                (other[e41] * self[e2]) + (other[e43] * self[e321]) + (other[e23] * self[e431]) + (other[e12] * self[e4]) - (other[e42] * self[e1]) - (other[e31] * self[e423]),
                -(other[e23] * self[e1]) - (other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      148      178        0
    //    simd2       12       12        0
    //    simd3       18       20        0
    //    simd4        8        9        0
    // Totals...
    // yes simd      186      219        0
    //  no simd      258      298        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (other[scalar] * self[e1234]) - (other[e23] * self[e41]) - (other[e31] * self[e42]) - (other[e12] * self[e43])])
                + (Simd32x2::from(self[scalar]) * Simd32x2::from([other[scalar], other[e1234]]))
                - (Simd32x2::from(self[e23]) * Simd32x2::from([other[e23], other[e41]]))
                - (Simd32x2::from(self[e31]) * Simd32x2::from([other[e31], other[e42]]))
                - (Simd32x2::from(self[e12]) * Simd32x2::from([other[e12], other[e43]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e31] * self[e3]) - (other[e12] * self[e2]),
                (other[e12] * self[e1]) - (other[e23] * self[e3]),
                (other[e23] * self[e2]) - (other[e31] * self[e1]),
                -(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]),
            ]) + (Simd32x4::from(other[scalar]) * self.group1())
                + (Simd32x4::from(self[e321]) * Simd32x4::from([other[e23], other[e31], other[e12], other[e1234]])),
            // e41, e42, e43
            Simd32x3::from([
                (other[e42] * self[e12]) + (other[e31] * self[e43]) - (other[e43] * self[e31]) - (other[e12] * self[e42]),
                (other[e43] * self[e23]) + (other[e12] * self[e41]) - (other[e41] * self[e12]) - (other[e23] * self[e43]),
                (other[e41] * self[e31]) + (other[e23] * self[e42]) - (other[e42] * self[e23]) - (other[e31] * self[e41]),
            ]) + (Simd32x3::from(other[e1234]) * self.group3())
                + (Simd32x3::from(other[scalar]) * self.group2())
                + (Simd32x3::from(self[scalar]) * other.group0().truncate_to_3())
                + (Simd32x3::from(self[e1234]) * other.group1().truncate_to_3()),
            // e23, e31, e12
            Simd32x3::from([
                (other[e31] * self[e12]) - (other[e12] * self[e31]),
                (other[e12] * self[e23]) - (other[e23] * self[e12]),
                (other[e23] * self[e31]) - (other[e31] * self[e23]),
            ]) + (Simd32x3::from(other[scalar]) * self.group3())
                + (Simd32x3::from(self[scalar]) * other.group1().truncate_to_3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e42] * self[e3]) + (other[e1234] * self[e1]) + (other[e23] * self[e4]) + (other[e31] * self[e412]) + (other[scalar] * self[e423])
                    - (other[e43] * self[e2])
                    - (other[e12] * self[e431]),
                (other[e43] * self[e1]) + (other[e1234] * self[e2]) + (other[e31] * self[e4]) + (other[e12] * self[e423]) + (other[scalar] * self[e431])
                    - (other[e41] * self[e3])
                    - (other[e23] * self[e412]),
                (other[e41] * self[e2]) + (other[e1234] * self[e3]) + (other[e23] * self[e431]) + (other[e12] * self[e4]) + (other[scalar] * self[e412])
                    - (other[e42] * self[e1])
                    - (other[e31] * self[e423]),
                -(other[e23] * self[e1]) - (other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) + (Simd32x4::from(self[e321]) * Simd32x4::from([other[e41], other[e42], other[e43], other[scalar]])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      210      240        0
    //    simd2       16       16        0
    //    simd3       24       26        0
    //    simd4       10       11        0
    // Totals...
    // yes simd      260      293        0
    //  no simd      354      394        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                0.0,
                (other[e1234] * self[scalar]) + (other[e321] * self[e4])
                    - (other[e1] * self[e423])
                    - (other[e2] * self[e431])
                    - (other[e3] * self[e412])
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
                - (Simd32x2::from(self[e321]) * Simd32x2::from([other[e321], other[e4]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e2] * self[e12]) + (other[e31] * self[e3]) + (other[e321] * self[e23]) - (other[e3] * self[e31]) - (other[e12] * self[e2]),
                (other[e3] * self[e23]) + (other[e12] * self[e1]) + (other[e321] * self[e31]) - (other[e1] * self[e12]) - (other[e23] * self[e3]),
                (other[e1] * self[e31]) + (other[e23] * self[e2]) + (other[e321] * self[e12]) - (other[e2] * self[e23]) - (other[e31] * self[e1]),
                (other[e1] * self[e41]) + (other[e2] * self[e42]) + (other[e3] * self[e43])
                    - (other[e41] * self[e1])
                    - (other[e42] * self[e2])
                    - (other[e43] * self[e3])
                    - (other[e23] * self[e423])
                    - (other[e31] * self[e431])
                    - (other[e12] * self[e412])
                    - (other[e423] * self[e23])
                    - (other[e431] * self[e31])
                    - (other[e412] * self[e12])
                    - (other[e321] * self[e1234]),
            ]) + (Simd32x4::from(other[scalar]) * self.group1())
                + (Simd32x4::from(self[scalar]) * other.group1())
                + (Simd32x4::from(self[e321]) * Simd32x4::from([other[e23], other[e31], other[e12], other[e1234]])),
            // e41, e42, e43
            Simd32x3::from([
                (other[e2] * self[e412]) + (other[e42] * self[e12]) + (other[e31] * self[e43]) + (other[e412] * self[e2])
                    - (other[e3] * self[e431])
                    - (other[e43] * self[e31])
                    - (other[e12] * self[e42])
                    - (other[e431] * self[e3]),
                (other[e3] * self[e423]) + (other[e43] * self[e23]) + (other[e12] * self[e41]) + (other[e423] * self[e3])
                    - (other[e1] * self[e412])
                    - (other[e41] * self[e12])
                    - (other[e23] * self[e43])
                    - (other[e412] * self[e1]),
                (other[e1] * self[e431]) + (other[e41] * self[e31]) + (other[e23] * self[e42]) + (other[e431] * self[e1])
                    - (other[e2] * self[e423])
                    - (other[e42] * self[e23])
                    - (other[e31] * self[e41])
                    - (other[e423] * self[e2]),
            ]) + (Simd32x3::from(other[scalar]) * self.group2())
                + (Simd32x3::from(other[e1234]) * self.group3())
                + (Simd32x3::from(other[e321]) * self.group4().truncate_to_3())
                + (Simd32x3::from(self[scalar]) * other.group2())
                + (Simd32x3::from(self[e1234]) * other.group3())
                + (Simd32x3::from(self[e4]) * other.group1().truncate_to_3())
                - (Simd32x3::from(other[e4]) * self.group1().truncate_to_3())
                - (Simd32x3::from(self[e321]) * other.group4().truncate_to_3()),
            // e23, e31, e12
            Simd32x3::from([
                (other[e3] * self[e2]) + (other[e31] * self[e12]) - (other[e2] * self[e3]) - (other[e12] * self[e31]),
                (other[e1] * self[e3]) + (other[e12] * self[e23]) - (other[e3] * self[e1]) - (other[e23] * self[e12]),
                (other[e2] * self[e1]) + (other[e23] * self[e31]) - (other[e1] * self[e2]) - (other[e31] * self[e23]),
            ]) + (Simd32x3::from(other[scalar]) * self.group3())
                + (Simd32x3::from(self[scalar]) * other.group3())
                - (Simd32x3::from(other[e321]) * self.group1().truncate_to_3())
                - (Simd32x3::from(self[e321]) * other.group1().truncate_to_3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e1234] * self[e1])
                    + (other[e3] * self[e42])
                    + (other[e4] * self[e23])
                    + (other[e41] * self[e321])
                    + (other[e42] * self[e3])
                    + (other[e23] * self[e4])
                    + (other[e31] * self[e412])
                    + (other[e431] * self[e12])
                    - (other[e1] * self[e1234])
                    - (other[e2] * self[e43])
                    - (other[e43] * self[e2])
                    - (other[e12] * self[e431])
                    - (other[e412] * self[e31])
                    - (other[e321] * self[e41]),
                (other[e1234] * self[e2])
                    + (other[e1] * self[e43])
                    + (other[e4] * self[e31])
                    + (other[e42] * self[e321])
                    + (other[e43] * self[e1])
                    + (other[e31] * self[e4])
                    + (other[e12] * self[e423])
                    + (other[e412] * self[e23])
                    - (other[e2] * self[e1234])
                    - (other[e3] * self[e41])
                    - (other[e41] * self[e3])
                    - (other[e23] * self[e412])
                    - (other[e423] * self[e12])
                    - (other[e321] * self[e42]),
                (other[e1234] * self[e3])
                    + (other[e2] * self[e41])
                    + (other[e4] * self[e12])
                    + (other[e41] * self[e2])
                    + (other[e43] * self[e321])
                    + (other[e23] * self[e431])
                    + (other[e12] * self[e4])
                    + (other[e423] * self[e31])
                    - (other[e1] * self[e42])
                    - (other[e3] * self[e1234])
                    - (other[e42] * self[e1])
                    - (other[e31] * self[e423])
                    - (other[e431] * self[e23])
                    - (other[e321] * self[e43]),
                -(other[e1] * self[e23]) - (other[e2] * self[e31]) - (other[e3] * self[e12]) - (other[e23] * self[e1]) - (other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]) + (Simd32x4::from(other[scalar]) * self.group4())
                + (Simd32x4::from(self[scalar]) * other.group4()),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      105      125        0
    //    simd2        8        9        0
    //    simd3       12       16        0
    //    simd4        5        7        0
    // Totals...
    // yes simd      130      157        0
    //  no simd      177      219        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, self[e321] * other[e4]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(self[scalar] * other[e4]),
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([self[e23] * other[e4], self[e31] * other[e4], self[e12] * other[e4], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      117      145        0
    //    simd2        8        9        0
    //    simd3       16       20        0
    //    simd4        5        6        0
    // Totals...
    // yes simd      146      180        0
    //  no simd      201      247        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                self[e321] * other[e321],
                (self[e1] * other[e423]) + (self[e2] * other[e431]) + (self[e3] * other[e412]) + (self[e4] * other[e321]),
            ]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([
                self[e23] * other[e321],
                self[e31] * other[e321],
                self[e12] * other[e321],
                -(self[e1234] * other[e321]) - (self[e23] * other[e423]) - (self[e31] * other[e431]) - (self[e12] * other[e412]),
            ]),
            // e41, e42, e43
            Simd32x3::from([
                (self[e2] * other[e412]) - (self[e3] * other[e431]),
                (self[e3] * other[e423]) - (self[e1] * other[e412]),
                (self[e1] * other[e431]) - (self[e2] * other[e423]),
            ]) + (Simd32x3::from(other[e321]) * self.group4().truncate_to_3())
                - (Simd32x3::from(self[e321]) * other.group0().truncate_to_3()),
            // e23, e31, e12
            Simd32x3::from(other[e321]) * self.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            (Simd32x3::from([
                (self[e12] * other[e431]) - (self[e31] * other[e412]),
                (self[e23] * other[e412]) - (self[e12] * other[e423]),
                (self[e31] * other[e423]) - (self[e23] * other[e431]),
            ]) + (Simd32x3::from(self[scalar]) * other.group0().truncate_to_3())
                - (Simd32x3::from(other[e321]) * self.group2()))
            .extend_to_4(self[scalar] * other[e321]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      132      163        0
    //    simd2        8        8        0
    //    simd3       15       17        0
    //    simd4        6        7        0
    // Totals...
    // yes simd      161      195        0
    //  no simd      217      258        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self[e1] * other[e1]) + (self[e2] * other[e2]) + (self[e3] * other[e3]),
                -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self[e12] * other[e2]) - (self[e31] * other[e3]),
                (self[e23] * other[e3]) - (self[e12] * other[e1]),
                (self[e31] * other[e1]) - (self[e23] * other[e2]),
                (self[e41] * other[e1]) + (self[e42] * other[e2]) + (self[e43] * other[e3]),
            ]) + (Simd32x4::from(self[scalar]) * other.group0()),
            // e41, e42, e43
            Simd32x3::from([
                (self[e412] * other[e2]) - (self[e431] * other[e3]),
                (self[e423] * other[e3]) - (self[e412] * other[e1]),
                (self[e431] * other[e1]) - (self[e423] * other[e2]),
            ]) + (Simd32x3::from(self[e4]) * other.group0().truncate_to_3())
                - (Simd32x3::from(other[e4]) * self.group1().truncate_to_3()),
            // e23, e31, e12
            Simd32x3::from([
                (self[e2] * other[e3]) - (self[e3] * other[e2]),
                (self[e3] * other[e1]) - (self[e1] * other[e3]),
                (self[e1] * other[e2]) - (self[e2] * other[e1]),
            ]) - (Simd32x3::from(self[e321]) * other.group0().truncate_to_3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e42] * other[e3]) + (self[e23] * other[e4]) - (self[e1234] * other[e1]) - (self[e43] * other[e2]),
                (self[e43] * other[e1]) + (self[e31] * other[e4]) - (self[e1234] * other[e2]) - (self[e41] * other[e3]),
                (self[e41] * other[e2]) + (self[e12] * other[e4]) - (self[e1234] * other[e3]) - (self[e42] * other[e1]),
                -(self[e23] * other[e1]) - (self[e31] * other[e2]) - (self[e12] * other[e3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      105      120        0
    //    simd2        8        9        0
    //    simd3       12       16        0
    //    simd4        5        8        0
    // Totals...
    // yes simd      130      153        0
    //  no simd      177      218        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
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
        return geometric_product.geometric_product(self.reverse());
    }
}
impl std::ops::Div<sandwich> for Origin {
    type Output = sandwich_partial<Origin>;
    fn div(self, _rhs: sandwich) -> Self::Output {
        sandwich_partial(self)
    }
}
impl Sandwich<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * Simd32x4::from([other[e1], other[e2], other[e3], other[e321]]),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        9        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e23] * self[e4], other[e31] * self[e4], other[e12] * self[e4], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       16        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[scalar] * self[e4]),
            // e423, e431, e412, e321
            Simd32x4::from([other[e23] * self[e4], other[e31] * self[e4], other[e12] * self[e4], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd2        0        2        0
    //    simd3        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       17        0
    //  no simd        0       31        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([1.0, other[e321] * self[e4]]) * Simd32x2::from([0.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[scalar] * self[e4]),
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group1().truncate_to_3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([other[e23] * self[e4], other[e31] * self[e4], other[e12] * self[e4], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Origin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       10        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e4]) * other.group0().truncate_to_3(),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl std::ops::Div<sandwich> for Plane {
    type Output = sandwich_partial<Plane>;
    fn div(self, _rhs: sandwich) -> Self::Output {
        sandwich_partial(self)
    }
}
impl Sandwich<AntiScalar> for Plane {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Origin::from_groups(/* e4 */ other[e1234] * self[e321]);
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd4        1        5        0
    // Totals...
    // yes simd        9       18        0
    //  no simd       12       33        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x3::from(0.0).extend_to_4(other[e1234] * self[e321]),
            // e423, e431, e412, e321
            Simd32x4::from(other[scalar]) * self.group0(),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       26        0
    //    simd3        2        2        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       17       32        0
    //  no simd       24       48        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e2] * self[e412]) + (other[e321] * self[e423]) - (other[e3] * self[e431]),
                (other[e3] * self[e423]) + (other[e321] * self[e431]) - (other[e1] * self[e412]),
                (other[e1] * self[e431]) + (other[e321] * self[e412]) - (other[e2] * self[e423]),
                -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]),
            ]) - (Simd32x4::from(self[e321]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]])),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * Simd32x4::from([other[e1], other[e2], other[e3], other[e321]]) * Simd32x4::from(-1.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       19        0
    //    simd3        2        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        8       23        0
    //  no simd       12       33        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([other[e321] * self[e423], other[e321] * self[e431], other[e321] * self[e412], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).extend_to_4(other[e321] * self[e321] * -1.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd3        0        1        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       15       29        0
    //  no simd       21       43        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                other[e23] * self[e321],
                other[e31] * self[e321],
                other[e12] * self[e321],
                -(other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e31] * self[e412]) - (other[e12] * self[e431]),
                (other[e12] * self[e423]) - (other[e23] * self[e412]),
                (other[e23] * self[e431]) - (other[e31] * self[e423]),
                0.0,
            ]) + (Simd32x3::from(self[e321]) * other.group0()).extend_to_4(0.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       26        0
    //    simd3        2        2        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       17       32        0
    //  no simd       24       48        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                other[e23] * self[e321],
                other[e31] * self[e321],
                other[e12] * self[e321],
                (other[e1234] * self[e321]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]),
            ]),
            // e423, e431, e412, e321
            (Simd32x3::from([
                (other[e31] * self[e412]) - (other[e12] * self[e431]),
                (other[e12] * self[e423]) - (other[e23] * self[e412]),
                (other[e23] * self[e431]) - (other[e31] * self[e423]),
            ]) + (Simd32x3::from(other[scalar]) * self.group0().truncate_to_3())
                + (Simd32x3::from(self[e321]) * other.group0().truncate_to_3()))
            .extend_to_4(other[scalar] * self[e321]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       50        0
    //    simd2        0        2        0
    //    simd3        8       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       32       65        0
    //  no simd       48       94        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                other[e321] * self[e321],
                -(other[e1] * self[e423]) - (other[e2] * self[e431]) - (other[e3] * self[e412]) - (other[e4] * self[e321]),
            ]) * Simd32x2::from([-1.0, 1.0]),
            // e1, e2, e3, e4
            Simd32x4::from([
                other[e23] * self[e321],
                other[e31] * self[e321],
                other[e12] * self[e321],
                (other[e1234] * self[e321]) - (other[e23] * self[e423]) - (other[e31] * self[e431]) - (other[e12] * self[e412]),
            ]),
            // e41, e42, e43
            Simd32x3::from([
                (other[e2] * self[e412]) - (other[e3] * self[e431]),
                (other[e3] * self[e423]) - (other[e1] * self[e412]),
                (other[e1] * self[e431]) - (other[e2] * self[e423]),
            ]) + (Simd32x3::from(other[e321]) * self.group0().truncate_to_3())
                - (Simd32x3::from(self[e321]) * other.group4().truncate_to_3()),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * other.group1().truncate_to_3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            (Simd32x3::from([
                (other[e31] * self[e412]) - (other[e12] * self[e431]),
                (other[e12] * self[e423]) - (other[e23] * self[e412]),
                (other[e23] * self[e431]) - (other[e31] * self[e423]),
            ]) + (Simd32x3::from(other[scalar]) * self.group0().truncate_to_3())
                + (Simd32x3::from(self[e321]) * other.group2()))
            .extend_to_4(other[scalar] * self[e321]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Plane {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = AntiScalar::from_groups(/* e1234 */ other[e4] * self[e321] * -1.0);
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       22        0
    //    simd3        2        3        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        9       26        0
    //  no simd       16       35        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([(other[e423] * self[e321]) * -1.0, (other[e431] * self[e321]) * -1.0, (other[e412] * self[e321]) * -1.0, 0.0])
                + (Simd32x3::from(other[e321]) * self.group0().truncate_to_3()).extend_to_4(0.0),
            // e23, e31, e12, scalar
            Simd32x3::from(0.0).extend_to_4(other[e321] * self[e321] * -1.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd3        2        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       14       31        0
    //  no simd       18       41        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e412] * other[e2]) - (self[e431] * other[e3]),
                (self[e423] * other[e3]) - (self[e412] * other[e1]),
                (self[e431] * other[e1]) - (self[e423] * other[e2]),
                -(self[e423] * other[e1]) - (self[e431] * other[e2]) - (self[e412] * other[e3]) - (self[e321] * other[e4]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([self[e321] * other[e1], self[e321] * other[e2], self[e321] * other[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd3        0        1        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        1       11        0
    //  no simd        4       19        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[scalar]) * self.group0());
        return geometric_product.geometric_product(self.reverse());
    }
}
impl std::ops::Div<sandwich> for Point {
    type Output = sandwich_partial<Point>;
    fn div(self, _rhs: sandwich) -> Self::Output {
        sandwich_partial(self)
    }
}
impl Sandwich<AntiScalar> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       16        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       18        0
    //  no simd        6       24        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other[e1234] * self[e1], other[e1234] * self[e2], other[e1234] * self[e3], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       27        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       17       30        0
    //  no simd       20       39        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[scalar]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([other[e1234] * self[e1], other[e1234] * self[e2], other[e1234] * self[e3], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       34       50        0
    //  no simd       40       56        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e412] * self[e2]) - (other[e4] * self[e1]) - (other[e431] * self[e3]),
                (other[e423] * self[e3]) - (other[e4] * self[e2]) - (other[e412] * self[e1]),
                (other[e431] * self[e1]) - (other[e4] * self[e3]) - (other[e423] * self[e2]),
                (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]),
            ]) + (Simd32x4::from(self[e4]) * Simd32x4::from([other[e1], other[e2], other[e3], other[e321]])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e3] * self[e2]) - (other[e2] * self[e3]) - (other[e321] * self[e1]),
                (other[e1] * self[e3]) - (other[e3] * self[e1]) - (other[e321] * self[e2]),
                (other[e2] * self[e1]) - (other[e1] * self[e2]) - (other[e321] * self[e3]),
                (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       17       30        0
    //  no simd       20       36        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x3::from(0.0).extend_to_4(other[e321] * self[e4]),
            // e23, e31, e12, scalar
            Simd32x4::from([other[e321] * self[e1], other[e321] * self[e2], other[e321] * self[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       45        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       30       46        0
    //  no simd       33       49        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e31] * self[e3]) - (other[e12] * self[e2]),
                (other[e12] * self[e1]) - (other[e23] * self[e3]),
                (other[e23] * self[e2]) - (other[e31] * self[e1]),
                -(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e42] * self[e3]) + (other[e23] * self[e4]) - (other[e43] * self[e2]),
                (other[e43] * self[e1]) + (other[e31] * self[e4]) - (other[e41] * self[e3]),
                (other[e41] * self[e2]) + (other[e12] * self[e4]) - (other[e42] * self[e1]),
                -(other[e23] * self[e1]) - (other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       34       50        0
    //  no simd       40       56        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e31] * self[e3]) - (other[e12] * self[e2]),
                (other[e12] * self[e1]) - (other[e23] * self[e3]),
                (other[e23] * self[e2]) - (other[e31] * self[e1]),
                -(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]),
            ]) + (Simd32x4::from(other[scalar]) * self.group0()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e42] * self[e3]) + (other[e1234] * self[e1]) + (other[e23] * self[e4]) - (other[e43] * self[e2]),
                (other[e43] * self[e1]) + (other[e1234] * self[e2]) + (other[e31] * self[e4]) - (other[e41] * self[e3]),
                (other[e41] * self[e2]) + (other[e1234] * self[e3]) + (other[e12] * self[e4]) - (other[e42] * self[e1]),
                -(other[e23] * self[e1]) - (other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       49       80        0
    //    simd2        3        3        0
    //    simd3        6        6        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       60       91        0
    //  no simd       81      112        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, other[e321] * self[e4]])
                + (Simd32x2::from(self[e1]) * Simd32x2::from([other[e1], other[e423]]))
                + (Simd32x2::from(self[e2]) * Simd32x2::from([other[e2], other[e431]]))
                + (Simd32x2::from(self[e3]) * Simd32x2::from([other[e3], other[e412]])),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other[e31] * self[e3]) - (other[e12] * self[e2]),
                (other[e12] * self[e1]) - (other[e23] * self[e3]),
                (other[e23] * self[e2]) - (other[e31] * self[e1]),
                -(other[e41] * self[e1]) - (other[e42] * self[e2]) - (other[e43] * self[e3]),
            ]) + (Simd32x4::from(other[scalar]) * self.group0()),
            // e41, e42, e43
            Simd32x3::from([
                (other[e412] * self[e2]) - (other[e431] * self[e3]),
                (other[e423] * self[e3]) - (other[e412] * self[e1]),
                (other[e431] * self[e1]) - (other[e423] * self[e2]),
            ]) + (Simd32x3::from(self[e4]) * other.group1().truncate_to_3())
                - (Simd32x3::from(other[e4]) * self.group0().truncate_to_3()),
            // e23, e31, e12
            Simd32x3::from([
                (other[e3] * self[e2]) - (other[e2] * self[e3]),
                (other[e1] * self[e3]) - (other[e3] * self[e1]),
                (other[e2] * self[e1]) - (other[e1] * self[e2]),
            ]) - (Simd32x3::from(other[e321]) * self.group0().truncate_to_3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other[e1234] * self[e1]) + (other[e42] * self[e3]) + (other[e23] * self[e4]) - (other[e43] * self[e2]),
                (other[e1234] * self[e2]) + (other[e43] * self[e1]) + (other[e31] * self[e4]) - (other[e41] * self[e3]),
                (other[e1234] * self[e3]) + (other[e41] * self[e2]) + (other[e12] * self[e4]) - (other[e42] * self[e1]),
                -(other[e23] * self[e1]) - (other[e31] * self[e2]) - (other[e12] * self[e3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       13       23        0
    //  no simd       13       27        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e4]) * self.group0().truncate_to_3() * Simd32x3::from(-1.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       26       45        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other[e412] * self[e2]) - (other[e431] * self[e3]),
                (other[e423] * self[e3]) - (other[e412] * self[e1]),
                (other[e431] * self[e1]) - (other[e423] * self[e2]),
                (other[e423] * self[e1]) + (other[e431] * self[e2]) + (other[e412] * self[e3]) + (other[e321] * self[e4]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([other[e321] * self[e1], other[e321] * self[e2], other[e321] * self[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       39        0
    //    simd3        0        1        0
    //    simd4        2        1        0
    // Totals...
    // yes simd       23       41        0
    //  no simd       29       46        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([(other[e4] * self[e1]) * -1.0, (other[e4] * self[e2]) * -1.0, (other[e4] * self[e3]) * -1.0, 0.0])
                + (Simd32x3::from(self[e4]) * other.group0().truncate_to_3()).extend_to_4(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other[e3] * self[e2]) - (other[e2] * self[e3]),
                (other[e1] * self[e3]) - (other[e3] * self[e1]),
                (other[e2] * self[e1]) - (other[e1] * self[e2]),
                (other[e1] * self[e1]) + (other[e2] * self[e2]) + (other[e3] * self[e3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd3        0        1        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        6       17        0
    //  no simd        9       22        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[scalar]) * self.group0());
        return geometric_product.geometric_product(self.reverse());
    }
}
impl std::ops::Div<sandwich> for Scalar {
    type Output = sandwich_partial<Scalar>;
    fn div(self, _rhs: sandwich) -> Self::Output {
        sandwich_partial(self)
    }
}
impl Sandwich<AntiScalar> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = AntiScalar::from_groups(/* e1234 */ other[e1234] * self[scalar]);
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[scalar]) * other.group0());
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[scalar]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[scalar]) * other.group1(),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Horizon::from_groups(/* e321 */ other[e321] * self[scalar]);
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        4        0
    // no simd        0       12        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[scalar]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[scalar]) * other.group1(),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[scalar]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group1(),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       32        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
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
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Scalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Origin::from_groups(/* e4 */ other[e4] * self[scalar]);
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[scalar]) * other.group0());
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[scalar]) * other.group0());
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Scalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Scalar::from_groups(/* scalar */ other[scalar] * self[scalar]);
        return geometric_product.geometric_product(self.reverse());
    }
}
