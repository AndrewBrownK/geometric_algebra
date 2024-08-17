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
//   Median:        12      28       0
//  Average:        25      40       0
//  Maximum:       206     239       0
//
//  No SIMD:   add/sub    mul    div
//  Minimum:         0       2       0
//   Median:        19      39       0
//  Average:        44      64       0
//  Maximum:       354     394       0
impl InfixSandwich for AntiScalar {}
impl Sandwich<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self[e1234] * other.group1()[3] * -1.0)]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e1234] * other.group0()[0] * -1.0),
                (self[e1234] * other.group0()[1] * -1.0),
                (self[e1234] * other.group0()[2] * -1.0),
                0.0,
            ]),
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
        let geometric_product = Line::from_groups(/* e41, e42, e43 */ (Simd32x3::from(self[e1234]) * other.group1()), /* e23, e31, e12 */ Simd32x3::from(0.0));
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
            (Simd32x4::from(self[e1234]) * other.group1()),
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
    //      f32        0       14        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0       16        0
    //  no simd        0       20        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self[e1234] * other.group0()[0])]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self[e1234] * other.group4()[3] * -1.0)]),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group3()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self[e1234] * other.group1()[0] * -1.0),
                (self[e1234] * other.group1()[1] * -1.0),
                (self[e1234] * other.group1()[2] * -1.0),
                0.0,
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([
            (self[e1234] * other.group0()[0] * -1.0),
            (self[e1234] * other.group0()[1] * -1.0),
            (self[e1234] * other.group0()[2] * -1.0),
            0.0,
        ]));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl InfixSandwich for DualNum {}
impl Sandwich<AntiScalar> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = AntiScalar::from_groups(/* e1234 */ (self.group0()[0] * other[e1234]));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            (self.group0()[0] * other.group0()[0]),
            ((self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0])),
        ]));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       24        0
    fn sandwich(self, other: Flector) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group0()[0]),
                (self.group0()[0] * other.group0()[1]),
                (self.group0()[0] * other.group0()[2]),
                ((self.group0()[0] * other.group0()[3]) - (self.group0()[1] * other.group1()[3])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group0()[0])),
                ((self.group0()[0] * other.group1()[1]) - (self.group0()[1] * other.group0()[1])),
                ((self.group0()[0] * other.group1()[2]) - (self.group0()[1] * other.group0()[2])),
                (self.group0()[0] * other.group1()[3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       15        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[1] * other[e321] * -1.0)]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[0] * other[e321])]),
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
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            ((Simd32x3::from(self.group0()[0]) * other.group0()) + (Simd32x3::from(self.group0()[1]) * other.group1())),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[0]) * other.group1()),
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
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x4::from(self.group0()[0]) * other.group0()) + (Simd32x4::from(self.group0()[1]) * other.group1())),
            // e23, e31, e12, scalar
            (Simd32x4::from(self.group0()[0]) * other.group1()),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       30        0
    //    simd3        2        6        0
    // Totals...
    // yes simd       12       36        0
    //  no simd       16       48        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self.group0()[0] * other.group0()[0]),
                ((self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group1()[0]),
                (self.group0()[0] * other.group1()[1]),
                (self.group0()[0] * other.group1()[2]),
                ((self.group0()[0] * other.group1()[3]) - (self.group0()[1] * other.group4()[3])),
            ]),
            // e41, e42, e43
            ((Simd32x3::from(self.group0()[0]) * other.group2()) + (Simd32x3::from(self.group0()[1]) * other.group3())),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[0]) * other.group3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((self.group0()[0] * other.group4()[0]) - (self.group0()[1] * other.group1()[0])),
                ((self.group0()[0] * other.group4()[1]) - (self.group0()[1] * other.group1()[1])),
                ((self.group0()[0] * other.group4()[2]) - (self.group0()[1] * other.group1()[2])),
                (self.group0()[0] * other.group4()[3]),
            ]),
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
        let geometric_product = Origin::from_groups(/* e4 */ (self.group0()[0] * other[e4]));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       14        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       15        0
    //  no simd        4       18        0
    fn sandwich(self, other: Plane) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[1] * other.group0()[3] * -1.0)]),
            // e423, e431, e412, e321
            (Simd32x4::from(self.group0()[0]) * other.group0()),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       18        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       19        0
    //  no simd        4       22        0
    fn sandwich(self, other: Point) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[0]) * other.group0()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self.group0()[1] * other.group0()[0] * -1.0),
                (self.group0()[1] * other.group0()[1] * -1.0),
                (self.group0()[1] * other.group0()[2] * -1.0),
                0.0,
            ]),
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
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ (self.group0() * Simd32x2::from(other[scalar])));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl InfixSandwich for Flector {}
impl Sandwich<AntiScalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       20        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       16       29        0
    //  no simd       40       56        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group1()[3] * other[e1234])]),
            // e423, e431, e412, e321
            Simd32x4::from([(self.group0()[0] * other[e1234]), (self.group0()[1] * other[e1234]), (self.group0()[2] * other[e1234]), 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       28        0
    //    simd4        8        9        0
    // Totals...
    // yes simd       20       37        0
    //  no simd       44       64        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group0()[0]),
                (self.group0()[1] * other.group0()[0]),
                (self.group0()[2] * other.group0()[0]),
                ((self.group0()[3] * other.group0()[0]) + (self.group1()[3] * other.group0()[1])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((self.group0()[0] * other.group0()[1]) + (self.group1()[0] * other.group0()[0])),
                ((self.group0()[1] * other.group0()[1]) + (self.group1()[1] * other.group0()[0])),
                ((self.group0()[2] * other.group0()[1]) + (self.group1()[2] * other.group0()[0])),
                (self.group1()[3] * other.group0()[0]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       36        0
    //    simd4       15       16        0
    // Totals...
    // yes simd       36       52        0
    //  no simd       81      100        0
    fn sandwich(self, other: Flector) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (-(Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * swizzle!(other.group0(), 2, 0, 1, 2))
                + (Simd32x4::from(other.group1()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]))
                + (swizzle!(self.group0(), 3, 3, 3, 2) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[2]]))
                - (swizzle!(other.group0(), 3, 3, 3, 1) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]))
                + (swizzle!(self.group0(), 1, 2, 0, 0) * swizzle!(other.group1(), 2, 0, 1, 0))
                + Simd32x4::from([
                    ((self.group1()[2] * other.group0()[1]) - (self.group0()[2] * other.group1()[1])),
                    ((self.group1()[0] * other.group0()[2]) - (self.group0()[0] * other.group1()[2])),
                    ((self.group1()[1] * other.group0()[0]) - (self.group0()[1] * other.group1()[0])),
                    (-(self.group1()[0] * other.group0()[0]) + (self.group0()[1] * other.group1()[1])),
                ])),
            // e23, e31, e12, scalar
            (-(Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                + (swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(other.group0(), 2, 0, 1, 2))
                + Simd32x4::from([
                    (-(self.group0()[2] * other.group0()[1]) - (self.group0()[0] * other.group1()[3])),
                    (-(self.group0()[0] * other.group0()[2]) - (self.group0()[1] * other.group1()[3])),
                    (-(self.group0()[2] * other.group1()[3]) - (self.group0()[1] * other.group0()[0])),
                    ((self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       20        0
    //    simd4        7       11        0
    // Totals...
    // yes simd       20       31        0
    //  no simd       41       64        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[e321]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]])),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e321]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from(-1.0)),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       38       55        0
    //  no simd       68       88        0
    fn sandwich(self, other: Line) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group1()[0]) + (self.group0()[2] * other.group1()[1])),
                    ((self.group1()[3] * other.group1()[1]) + (self.group0()[0] * other.group1()[2])),
                    ((self.group1()[3] * other.group1()[2]) + (self.group0()[1] * other.group1()[0])),
                    (-(self.group1()[2] * other.group1()[2])
                        - (self.group1()[1] * other.group1()[1])
                        - (self.group1()[0] * other.group1()[0])
                        - (self.group0()[0] * other.group0()[0])
                        - (self.group0()[1] * other.group0()[1])),
                ])),
            // e423, e431, e412, e321
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group0()[0]) + (self.group1()[2] * other.group1()[1]) - (self.group1()[1] * other.group1()[2])
                        + (self.group0()[3] * other.group1()[0])
                        + (self.group0()[2] * other.group0()[1])),
                    ((self.group1()[3] * other.group0()[1]) - (self.group1()[2] * other.group1()[0])
                        + (self.group1()[0] * other.group1()[2])
                        + (self.group0()[3] * other.group1()[1])
                        + (self.group0()[0] * other.group0()[2])),
                    ((self.group1()[3] * other.group0()[2]) + (self.group1()[1] * other.group1()[0]) - (self.group1()[0] * other.group1()[1])
                        + (self.group0()[3] * other.group1()[2])
                        + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       41        0
    //    simd4       14       15        0
    // Totals...
    // yes simd       38       56        0
    //  no simd       80      101        0
    fn sandwich(self, other: Motor) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]))
                + (swizzle!(self.group0(), 2, 0, 2, 3) * swizzle!(other.group1(), 1, 2, 3, 3))
                - (swizzle!(other.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group1()[2]]))
                + Simd32x4::from([
                    (self.group0()[0] * other.group1()[3]),
                    (self.group0()[1] * other.group1()[3]),
                    (self.group0()[1] * other.group1()[0]),
                    (-(self.group1()[1] * other.group1()[1])
                        - (self.group1()[0] * other.group1()[0])
                        - (self.group0()[2] * other.group0()[2])
                        - (self.group0()[0] * other.group0()[0])
                        - (self.group0()[1] * other.group0()[1])),
                ])),
            // e423, e431, e412, e321
            ((Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (swizzle!(other.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group0()[2]]))
                - (swizzle!(self.group0(), 1, 2, 0, 0) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]))
                + Simd32x4::from([
                    ((self.group1()[2] * other.group1()[1])
                        + (self.group1()[0] * other.group1()[3])
                        + (self.group0()[3] * other.group1()[0])
                        + (self.group0()[2] * other.group0()[1])
                        + (self.group0()[0] * other.group0()[3])),
                    ((self.group1()[1] * other.group1()[3])
                        + (self.group1()[0] * other.group1()[2])
                        + (self.group0()[3] * other.group1()[1])
                        + (self.group0()[0] * other.group0()[2])
                        + (self.group0()[1] * other.group0()[3])),
                    ((self.group1()[2] * other.group1()[3])
                        + (self.group1()[1] * other.group1()[0])
                        + (self.group0()[3] * other.group1()[2])
                        + (self.group0()[2] * other.group0()[3])
                        + (self.group0()[1] * other.group0()[0])),
                    ((self.group0()[1] * other.group1()[1]) * -1.0),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       70      101        0
    //    simd2        8        8        0
    //    simd3       12       12        0
    //    simd4       10       11        0
    // Totals...
    // yes simd      100      132        0
    //  no simd      162      197        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (-(Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group4()[3], other.group1()[3]]))
                + (Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]))
                + (Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]))
                + (Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]))
                + Simd32x2::from([
                    0.0,
                    (-(self.group1()[2] * other.group1()[2]) - (self.group1()[1] * other.group1()[1]) - (self.group1()[0] * other.group1()[0])
                        + (self.group0()[3] * other.group4()[3])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]))
                + (swizzle!(self.group0(), 2, 0, 2, 3) * Simd32x4::from([other.group3()[1], other.group3()[2], other.group0()[0], other.group0()[0]]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[2]]))
                + Simd32x4::from([
                    (self.group0()[0] * other.group0()[0]),
                    (self.group0()[1] * other.group0()[0]),
                    (self.group0()[1] * other.group3()[0]),
                    (-(self.group1()[2] * other.group3()[2])
                        - (self.group1()[1] * other.group3()[1])
                        - (self.group1()[0] * other.group3()[0])
                        - (self.group0()[0] * other.group2()[0])
                        - (self.group0()[1] * other.group2()[1])),
                ])),
            // e41, e42, e43
            (-(Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + Simd32x3::from([
                    ((self.group1()[2] * other.group1()[1]) - (self.group1()[1] * other.group1()[2]) - (self.group0()[2] * other.group4()[1])
                        + (self.group0()[1] * other.group4()[2])),
                    (-(self.group1()[2] * other.group1()[0]) + (self.group1()[0] * other.group1()[2]) + (self.group0()[2] * other.group4()[0])
                        - (self.group0()[0] * other.group4()[2])),
                    ((self.group1()[1] * other.group1()[0]) - (self.group1()[0] * other.group1()[1]) + (self.group0()[0] * other.group4()[1])
                        - (self.group0()[1] * other.group4()[0])),
                ])),
            // e23, e31, e12
            (-(Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + Simd32x3::from([
                    (-(self.group0()[2] * other.group1()[1]) + (self.group0()[1] * other.group1()[2])),
                    ((self.group0()[2] * other.group1()[0]) - (self.group0()[0] * other.group1()[2])),
                    ((self.group0()[0] * other.group1()[1]) - (self.group0()[1] * other.group1()[0])),
                ])),
            // e423, e431, e412, e321
            ((Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[2]]))
                + Simd32x4::from([
                    ((self.group1()[2] * other.group3()[1]) - (self.group1()[1] * other.group3()[2])
                        + (self.group1()[0] * other.group0()[0])
                        + (self.group0()[3] * other.group3()[0])
                        + (self.group0()[2] * other.group2()[1])
                        + (self.group0()[0] * other.group0()[1])),
                    (-(self.group1()[2] * other.group3()[0])
                        + (self.group1()[1] * other.group0()[0])
                        + (self.group1()[0] * other.group3()[2])
                        + (self.group0()[3] * other.group3()[1])
                        + (self.group0()[0] * other.group2()[2])
                        + (self.group0()[1] * other.group0()[1])),
                    ((self.group1()[2] * other.group0()[0]) + (self.group1()[1] * other.group3()[0]) - (self.group1()[0] * other.group3()[1])
                        + (self.group0()[3] * other.group3()[2])
                        + (self.group0()[2] * other.group0()[1])
                        + (self.group0()[1] * other.group2()[0])),
                    (-(self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       20        0
    //    simd4        7       10        0
    // Totals...
    // yes simd       20       30        0
    //  no simd       41       60        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[e4]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from(-1.0)),
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
    //      f32       17       28        0
    //    simd4        9       12        0
    // Totals...
    // yes simd       26       40        0
    //  no simd       53       76        0
    fn sandwich(self, other: Plane) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]))
                + (swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(other.group0(), 2, 0, 1, 2))
                + Simd32x4::from([
                    (-(self.group1()[3] * other.group0()[0]) - (self.group0()[2] * other.group0()[1])),
                    (-(self.group1()[3] * other.group0()[1]) - (self.group0()[0] * other.group0()[2])),
                    (-(self.group1()[3] * other.group0()[2]) - (self.group0()[1] * other.group0()[0])),
                    ((self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
            // e23, e31, e12, scalar
            (Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from(-1.0)),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       36        0
    //    simd4       10       11        0
    // Totals...
    // yes simd       31       47        0
    //  no simd       61       80        0
    fn sandwich(self, other: Point) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (-(swizzle!(self.group1(), 1, 2, 0, 3) * swizzle!(other.group0(), 2, 0, 1, 3))
                - (swizzle!(other.group0(), 3, 3, 3, 2) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[2] * other.group0()[1]) + (self.group0()[3] * other.group0()[0])),
                    ((self.group1()[0] * other.group0()[2]) + (self.group0()[3] * other.group0()[1])),
                    ((self.group1()[1] * other.group0()[0]) + (self.group0()[3] * other.group0()[2])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
            // e23, e31, e12, scalar
            ((swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(other.group0(), 2, 0, 1, 2))
                + Simd32x4::from([
                    (-(self.group1()[3] * other.group0()[0]) - (self.group0()[2] * other.group0()[1])),
                    (-(self.group1()[3] * other.group0()[1]) - (self.group0()[0] * other.group0()[2])),
                    (-(self.group1()[3] * other.group0()[2]) - (self.group0()[1] * other.group0()[0])),
                    ((self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        8       11        0
    // Totals...
    // yes simd       16       27        0
    //  no simd       40       60        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other[scalar])),
            // e423, e431, e412, e321
            (self.group1() * Simd32x4::from(other[scalar])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl InfixSandwich for Horizon {}
impl Sandwich<AntiScalar> for Horizon {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Origin::from_groups(/* e4 */ (self[e321] * other[e1234]));
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
            Simd32x4::from([0.0, 0.0, 0.0, (self[e321] * other.group0()[1])]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (self[e321] * other.group0()[0])]),
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
            (Simd32x4::from(self[e321]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) * Simd32x4::from(-1.0)),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e321]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
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
        let geometric_product = Scalar::from_groups(/* scalar */ (self[e321] * other[e321] * -1.0));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       19        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([(self[e321] * other.group1()[0]), (self[e321] * other.group1()[1]), (self[e321] * other.group1()[2]), 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([(self[e321] * other.group0()[0]), (self[e321] * other.group0()[1]), (self[e321] * other.group0()[2]), 0.0]),
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
            (Simd32x4::from(self[e321]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]])),
            // e423, e431, e412, e321
            (Simd32x4::from(self[e321]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
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
            (Simd32x2::from(self[e321]) * Simd32x2::from([other.group4()[3], other.group1()[3]]) * Simd32x2::from(-1.0)),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e321]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]])),
            // e41, e42, e43
            (Simd32x3::from(self[e321]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from(self[e321]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            (Simd32x4::from(self[e321]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]])),
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
        let geometric_product = AntiScalar::from_groups(/* e1234 */ (self[e321] * other[e4] * -1.0));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       13        0
    //  no simd        0       25        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self[e321] * other.group0()[0] * -1.0),
                (self[e321] * other.group0()[1] * -1.0),
                (self[e321] * other.group0()[2] * -1.0),
                0.0,
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self[e321] * other.group0()[3] * -1.0)]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       13        0
    //  no simd        0       25        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (self[e321] * other.group0()[3] * -1.0)]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self[e321] * other.group0()[0] * -1.0),
                (self[e321] * other.group0()[1] * -1.0),
                (self[e321] * other.group0()[2] * -1.0),
                0.0,
            ]),
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
        let geometric_product = Horizon::from_groups(/* e321 */ (self[e321] * other[scalar]));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl InfixSandwich for Line {}
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
        let geometric_product = Line::from_groups(/* e41, e42, e43 */ (self.group1() * Simd32x3::from(other[e1234])), /* e23, e31, e12 */ Simd32x3::from(0.0));
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
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            ((self.group0() * Simd32x3::from(other.group0()[0])) + (self.group1() * Simd32x3::from(other.group0()[1]))),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(other.group0()[0])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       56        0
    //    simd3        0        2        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       44       62        0
    //  no simd       56       78        0
    fn sandwich(self, other: Flector) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            ((swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group1()[3]) - (self.group1()[1] * other.group0()[2])),
                    (-(self.group1()[2] * other.group0()[0]) + (self.group1()[1] * other.group1()[3])),
                    ((self.group1()[2] * other.group1()[3]) - (self.group1()[0] * other.group0()[1])),
                    (-(self.group1()[2] * other.group1()[2]) - (self.group1()[1] * other.group1()[1]) - (self.group1()[0] * other.group1()[0])
                        + (self.group0()[0] * other.group0()[0])
                        + (self.group0()[1] * other.group0()[1])),
                ])),
            // e423, e431, e412, e321
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[2] * other.group1()[1]) - (self.group1()[1] * other.group1()[2]) + (self.group1()[0] * other.group0()[3])
                        - (self.group0()[0] * other.group1()[3])
                        + (self.group0()[1] * other.group0()[2])),
                    (-(self.group1()[2] * other.group1()[0])
                        + (self.group1()[1] * other.group0()[3])
                        + (self.group1()[0] * other.group1()[2])
                        + (self.group0()[2] * other.group0()[0])
                        - (self.group0()[1] * other.group1()[3])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group1()[1] * other.group1()[0])
                        - (self.group1()[0] * other.group1()[1])
                        - (self.group0()[2] * other.group1()[3])
                        + (self.group0()[0] * other.group0()[1])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       37        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       22       41        0
    //  no simd       28       51        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([(self.group1()[0] * other[e321]), (self.group1()[1] * other[e321]), (self.group1()[2] * other[e321]), 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self.group0()[0] * other[e321] * -1.0),
                (self.group0()[1] * other[e321] * -1.0),
                (self.group0()[2] * other[e321] * -1.0),
                0.0,
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       51        0
    //    simd3        0        2        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       38       56        0
    //  no simd       47       69        0
    fn sandwich(self, other: Line) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                ((self.group1()[2] * other.group0()[1]) - (self.group1()[1] * other.group0()[2]) - (self.group0()[1] * other.group1()[2]) + (self.group0()[2] * other.group1()[1])),
                (-(self.group1()[2] * other.group0()[0]) + (self.group1()[0] * other.group0()[2]) + (self.group0()[0] * other.group1()[2])
                    - (self.group0()[2] * other.group1()[0])),
                ((self.group1()[1] * other.group0()[0]) - (self.group1()[0] * other.group0()[1]) - (self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (-(self.group1()[1] * other.group1()[2]) + (self.group1()[2] * other.group1()[1])),
                ((self.group1()[0] * other.group1()[2]) - (self.group1()[2] * other.group1()[0])),
                (-(self.group1()[0] * other.group1()[1]) + (self.group1()[1] * other.group1()[0])),
                (-(self.group1()[2] * other.group1()[2]) - (self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd3        0        2        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       38       56        0
    //  no simd       56       78        0
    fn sandwich(self, other: Motor) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (-(swizzle!(other.group0(), 2, 0, 1, 2) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group1()[2]]))
                - (swizzle!(other.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group1()[2] * other.group0()[1])
                        + (self.group1()[0] * other.group0()[3])
                        + (self.group0()[2] * other.group1()[1])
                        + (self.group0()[0] * other.group1()[3])),
                    ((self.group1()[1] * other.group0()[3])
                        + (self.group1()[0] * other.group0()[2])
                        + (self.group0()[0] * other.group1()[2])
                        + (self.group0()[1] * other.group1()[3])),
                    ((self.group1()[2] * other.group0()[3])
                        + (self.group1()[1] * other.group0()[0])
                        + (self.group0()[2] * other.group1()[3])
                        + (self.group0()[1] * other.group1()[0])),
                    (-(self.group1()[1] * other.group0()[1])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group0()[0] * other.group1()[0])
                        - (self.group0()[1] * other.group1()[1])),
                ])),
            // e23, e31, e12, scalar
            (-(swizzle!(other.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[2] * other.group1()[1]) + (self.group1()[0] * other.group1()[3])),
                    ((self.group1()[0] * other.group1()[2]) + (self.group1()[1] * other.group1()[3])),
                    ((self.group1()[2] * other.group1()[3]) + (self.group1()[1] * other.group1()[0])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       62        0
    //    simd2        6        6        0
    //    simd3       14       20        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       68       92        0
    //  no simd      114      150        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (-(Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]))
                - (Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]))
                - (Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]))
                + Simd32x2::from([
                    0.0,
                    (-(self.group0()[2] * other.group3()[2]) - (self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
                ])),
            // e1, e2, e3, e4
            ((swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group4()[3]) - (self.group1()[1] * other.group1()[2])),
                    (-(self.group1()[2] * other.group1()[0]) + (self.group1()[1] * other.group4()[3])),
                    ((self.group1()[2] * other.group4()[3]) - (self.group1()[0] * other.group1()[1])),
                    (-(self.group1()[2] * other.group4()[2]) - (self.group1()[1] * other.group4()[1]) - (self.group1()[0] * other.group4()[0])
                        + (self.group0()[0] * other.group1()[0])
                        + (self.group0()[1] * other.group1()[1])),
                ])),
            // e41, e42, e43
            ((swizzle!(self.group1(), 2, 1, 2) * Simd32x3::from([other.group2()[1], other.group0()[1], other.group0()[1]]))
                - (swizzle!(self.group1(), 1, 2, 0) * swizzle!(other.group2(), 2, 0, 1))
                + (swizzle!(self.group1(), 0, 0, 1) * Simd32x3::from([other.group0()[1], other.group2()[2], other.group2()[0]]))
                + (swizzle!(self.group0(), 2, 0, 2) * Simd32x3::from([other.group3()[1], other.group3()[2], other.group0()[0]]))
                + (swizzle!(self.group0(), 0, 1, 1) * Simd32x3::from([other.group0()[0], other.group0()[0], other.group3()[0]]))
                - (swizzle!(self.group0(), 1, 2, 0) * swizzle!(other.group3(), 2, 0, 1))),
            // e23, e31, e12
            ((swizzle!(self.group1(), 2, 0, 2) * Simd32x3::from([other.group3()[1], other.group3()[2], other.group0()[0]]))
                + (swizzle!(self.group1(), 0, 1, 1) * Simd32x3::from([other.group0()[0], other.group0()[0], other.group3()[0]]))
                - (swizzle!(self.group1(), 1, 2, 0) * swizzle!(other.group3(), 2, 0, 1))),
            // e423, e431, e412, e321
            (-(swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[2] * other.group4()[1]) - (self.group1()[1] * other.group4()[2]) + (self.group1()[0] * other.group1()[3])
                        - (self.group0()[0] * other.group4()[3])
                        + (self.group0()[1] * other.group1()[2])),
                    (-(self.group1()[2] * other.group4()[0])
                        + (self.group1()[1] * other.group1()[3])
                        + (self.group1()[0] * other.group4()[2])
                        + (self.group0()[2] * other.group1()[0])
                        - (self.group0()[1] * other.group4()[3])),
                    ((self.group1()[2] * other.group1()[3]) + (self.group1()[1] * other.group4()[0])
                        - (self.group1()[0] * other.group4()[1])
                        - (self.group0()[2] * other.group4()[3])
                        + (self.group0()[0] * other.group1()[1])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       18        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        8       20        0
    //  no simd        8       24        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(self.group1()[0] * other[e4]), (self.group1()[1] * other[e4]), (self.group1()[2] * other[e4]), 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       43        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       30       47        0
    //  no simd       36       57        0
    fn sandwich(self, other: Plane) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] * other.group0()[3]),
                (self.group1()[1] * other.group0()[3]),
                (self.group1()[2] * other.group0()[3]),
                (-(self.group1()[2] * other.group0()[2]) - (self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((self.group1()[2] * other.group0()[1]) - (self.group0()[0] * other.group0()[3]) - (self.group1()[1] * other.group0()[2])),
                (-(self.group1()[2] * other.group0()[0]) - (self.group0()[1] * other.group0()[3]) + (self.group1()[0] * other.group0()[2])),
                ((self.group1()[1] * other.group0()[0]) - (self.group0()[2] * other.group0()[3]) - (self.group1()[0] * other.group0()[1])),
                0.0,
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       44        0
    //    simd3        0        2        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       29       50        0
    //  no simd       41       66        0
    fn sandwich(self, other: Point) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            ((swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group1()[1] * other.group0()[2]) * -1.0),
                    ((self.group1()[2] * other.group0()[0]) * -1.0),
                    ((self.group1()[0] * other.group0()[1]) * -1.0),
                    ((self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
            // e423, e431, e412, e321
            (-(swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group0()[3]) + (self.group0()[1] * other.group0()[2])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group0()[2] * other.group0()[0])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group0()[0] * other.group0()[1])),
                    (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1])),
                ])),
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
            (self.group0() * Simd32x3::from(other[scalar])),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(other[scalar])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl InfixSandwich for Motor {}
impl Sandwich<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       30        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       22       37        0
    //  no simd       40       58        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (self.group1() * Simd32x4::from(other[e1234])),
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
    //      f32       16       30        0
    //    simd4        7        9        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       44       66        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            ((self.group0() * Simd32x4::from(other.group0()[0])) + (self.group1() * Simd32x4::from(other.group0()[1]))),
            // e23, e31, e12, scalar
            (self.group1() * Simd32x4::from(other.group0()[0])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       51        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       42       64        0
    //  no simd       81      103        0
    fn sandwich(self, other: Flector) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group1()[3]) * other.group0())
                + (swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    (self.group1()[0] * other.group1()[3]),
                    (self.group1()[1] * other.group1()[3]),
                    (self.group1()[2] * other.group1()[3]),
                    (-(self.group1()[1] * other.group1()[1]) - (self.group1()[0] * other.group1()[0]) - (self.group0()[3] * other.group1()[3])
                        + (self.group0()[0] * other.group0()[0])
                        + (self.group0()[1] * other.group0()[1])),
                ])),
            // e423, e431, e412, e321
            ((Simd32x4::from(self.group1()[3]) * other.group1())
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                - (swizzle!(other.group0(), 0, 1, 2, 0) * Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[0]]))
                - (swizzle!(other.group0(), 1, 2, 0, 1) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[1]]))
                + Simd32x4::from([
                    ((self.group1()[2] * other.group1()[1]) + (self.group1()[0] * other.group0()[3]) - (self.group0()[0] * other.group1()[3])
                        + (self.group0()[1] * other.group0()[2])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group1()[0] * other.group1()[2]) + (self.group0()[2] * other.group0()[0])
                        - (self.group0()[1] * other.group1()[3])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group1()[1] * other.group1()[0]) - (self.group0()[2] * other.group1()[3])
                        + (self.group0()[0] * other.group0()[1])),
                    0.0,
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       31        0
    //    simd4        6       10        0
    // Totals...
    // yes simd       22       41        0
    //  no simd       40       71        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other[e321]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e321]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       54        0
    //    simd4        9        9        0
    // Totals...
    // yes simd       41       63        0
    //  no simd       68       90        0
    fn sandwich(self, other: Line) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (-(swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group0()[0])
                        + (self.group1()[2] * other.group0()[1])
                        + (self.group0()[3] * other.group1()[0])
                        + (self.group0()[2] * other.group1()[1])),
                    ((self.group1()[3] * other.group0()[1])
                        + (self.group1()[0] * other.group0()[2])
                        + (self.group0()[3] * other.group1()[1])
                        + (self.group0()[0] * other.group1()[2])),
                    ((self.group1()[3] * other.group0()[2])
                        + (self.group1()[1] * other.group0()[0])
                        + (self.group0()[3] * other.group1()[2])
                        + (self.group0()[1] * other.group1()[0])),
                    (-(self.group1()[1] * other.group0()[1])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group0()[0] * other.group1()[0])
                        - (self.group0()[1] * other.group1()[1])),
                ])),
            // e23, e31, e12, scalar
            (-(swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group1()[3] * other.group1()[0]) + (self.group1()[2] * other.group1()[1])),
                    ((self.group1()[3] * other.group1()[1]) + (self.group1()[0] * other.group1()[2])),
                    ((self.group1()[3] * other.group1()[2]) + (self.group1()[1] * other.group1()[0])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       54        0
    //    simd4       12       12        0
    // Totals...
    // yes simd       44       66        0
    //  no simd       80      102        0
    fn sandwich(self, other: Motor) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x4::from(self.group1()[3]) * other.group0()) - (swizzle!(self.group1(), 1, 2, 0, 2) * swizzle!(other.group0(), 2, 0, 1, 2))
                + (Simd32x4::from(self.group0()[3]) * other.group1())
                - (swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(other.group1(), 2, 0, 1, 2))
                + Simd32x4::from([
                    ((self.group1()[2] * other.group0()[1])
                        + (self.group1()[0] * other.group0()[3])
                        + (self.group0()[2] * other.group1()[1])
                        + (self.group0()[0] * other.group1()[3])),
                    ((self.group1()[1] * other.group0()[3])
                        + (self.group1()[0] * other.group0()[2])
                        + (self.group0()[0] * other.group1()[2])
                        + (self.group0()[1] * other.group1()[3])),
                    ((self.group1()[2] * other.group0()[3])
                        + (self.group1()[1] * other.group0()[0])
                        + (self.group0()[2] * other.group1()[3])
                        + (self.group0()[1] * other.group1()[0])),
                    (-(self.group1()[1] * other.group0()[1])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group0()[0] * other.group1()[0])
                        - (self.group0()[1] * other.group1()[1])),
                ])),
            // e23, e31, e12, scalar
            ((Simd32x4::from(self.group1()[3]) * other.group1()) - (swizzle!(self.group1(), 1, 2, 0, 2) * swizzle!(other.group1(), 2, 0, 1, 2))
                + Simd32x4::from([
                    ((self.group1()[2] * other.group1()[1]) + (self.group1()[0] * other.group1()[3])),
                    ((self.group1()[0] * other.group1()[2]) + (self.group1()[1] * other.group1()[3])),
                    ((self.group1()[2] * other.group1()[3]) + (self.group1()[1] * other.group1()[0])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       35       59        0
    //    simd2        8        8        0
    //    simd3       20       24        0
    //    simd4       13       13        0
    // Totals...
    // yes simd       76      104        0
    //  no simd      163      199        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            ((Simd32x2::from(self.group1()[3]) * other.group0())
                - (Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]))
                - (Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]))
                - (Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]))
                + Simd32x2::from([
                    0.0,
                    ((self.group0()[3] * other.group0()[0])
                        - (self.group0()[2] * other.group3()[2])
                        - (self.group0()[0] * other.group3()[0])
                        - (self.group0()[1] * other.group3()[1])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group1()[3]) * other.group1())
                + (swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group4()[2]]))
                + Simd32x4::from([
                    (self.group1()[0] * other.group4()[3]),
                    (self.group1()[1] * other.group4()[3]),
                    (self.group1()[2] * other.group4()[3]),
                    (-(self.group1()[1] * other.group4()[1]) - (self.group1()[0] * other.group4()[0]) - (self.group0()[3] * other.group4()[3])
                        + (self.group0()[0] * other.group1()[0])
                        + (self.group0()[1] * other.group1()[1])),
                ])),
            // e41, e42, e43
            ((Simd32x3::from(self.group1()[3]) * other.group2()) + (swizzle!(other.group2(), 1, 2, 0) * Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]))
                - (swizzle!(other.group2(), 2, 0, 1) * Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]))
                + (Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[3]) * other.group3())
                + (swizzle!(other.group3(), 1, 2, 0) * Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]))
                + (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (swizzle!(other.group3(), 2, 0, 1) * Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]))),
            // e23, e31, e12
            ((Simd32x3::from(self.group1()[3]) * other.group3())
                + (swizzle!(other.group3(), 1, 2, 0) * Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]))
                + (Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (swizzle!(other.group3(), 2, 0, 1) * Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]))),
            // e423, e431, e412, e321
            ((Simd32x4::from(self.group1()[3]) * other.group4())
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group1()[2]]))
                - (swizzle!(other.group1(), 0, 1, 2, 0) * Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[0]]))
                - (swizzle!(other.group1(), 1, 2, 0, 1) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[1]]))
                + Simd32x4::from([
                    ((self.group1()[2] * other.group4()[1]) + (self.group1()[0] * other.group1()[3]) - (self.group0()[0] * other.group4()[3])
                        + (self.group0()[1] * other.group1()[2])),
                    ((self.group1()[1] * other.group1()[3]) + (self.group1()[0] * other.group4()[2]) + (self.group0()[2] * other.group1()[0])
                        - (self.group0()[1] * other.group4()[3])),
                    ((self.group1()[2] * other.group1()[3]) + (self.group1()[1] * other.group4()[0]) - (self.group0()[2] * other.group4()[3])
                        + (self.group0()[0] * other.group1()[1])),
                    0.0,
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       35        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       41        0
    //  no simd       40       59        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group1()[3] * other[e4])]),
            // e423, e431, e412, e321
            Simd32x4::from([(self.group1()[0] * other[e4]), (self.group1()[1] * other[e4]), (self.group1()[2] * other[e4]), 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       51        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       34       57        0
    //  no simd       52       75        0
    fn sandwich(self, other: Plane) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] * other.group0()[3]),
                (self.group1()[1] * other.group0()[3]),
                (self.group1()[2] * other.group0()[3]),
                (-(self.group1()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group0()[3] * other.group0()[3])
                    - (self.group1()[0] * other.group0()[0])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((self.group1()[3] * other.group0()[0]) + (self.group1()[2] * other.group0()[1]) - (self.group0()[0] * other.group0()[3]) - (self.group1()[1] * other.group0()[2])),
                ((self.group1()[3] * other.group0()[1]) - (self.group1()[2] * other.group0()[0]) - (self.group0()[1] * other.group0()[3]) + (self.group1()[0] * other.group0()[2])),
                ((self.group1()[3] * other.group0()[2]) + (self.group1()[1] * other.group0()[0]) - (self.group0()[2] * other.group0()[3]) - (self.group1()[0] * other.group0()[1])),
                (self.group1()[3] * other.group0()[3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       47        0
    //    simd4       10       10        0
    // Totals...
    // yes simd       30       57        0
    //  no simd       60       87        0
    fn sandwich(self, other: Point) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group1()[3]) * other.group0())
                + (swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[2]]))
                + Simd32x4::from([
                    ((self.group1()[1] * other.group0()[2]) * -1.0),
                    ((self.group1()[2] * other.group0()[0]) * -1.0),
                    ((self.group1()[0] * other.group0()[1]) * -1.0),
                    ((self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
            // e423, e431, e412, e321
            (-(swizzle!(other.group0(), 0, 1, 2, 2) * Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[2]]))
                - (swizzle!(other.group0(), 1, 2, 0, 0) * Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]))
                + Simd32x4::from([
                    ((self.group1()[0] * other.group0()[3]) + (self.group0()[1] * other.group0()[2])),
                    ((self.group1()[1] * other.group0()[3]) + (self.group0()[2] * other.group0()[0])),
                    ((self.group1()[2] * other.group0()[3]) + (self.group0()[0] * other.group0()[1])),
                    ((self.group1()[1] * other.group0()[1]) * -1.0),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       30        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       22       38        0
    //  no simd       40       62        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (self.group0() * Simd32x4::from(other[scalar])),
            // e23, e31, e12, scalar
            (self.group1() * Simd32x4::from(other[scalar])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl InfixSandwich for MultiVector {}
impl Sandwich<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       67       87        0
    //    simd2        8        8        0
    //    simd3       18       21        0
    //    simd4       10       11        0
    // Totals...
    // yes simd      103      127        0
    //  no simd      177      210        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self.group0()[0] * other[e1234])]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group4()[3] * other[e1234])]),
            // e41, e42, e43
            (self.group3() * Simd32x3::from(other[e1234])),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([(self.group1()[0] * other[e1234]), (self.group1()[1] * other[e1234]), (self.group1()[2] * other[e1234]), 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       72       97        0
    //    simd2        8        8        0
    //    simd3       19       23        0
    //    simd4       10       11        0
    // Totals...
    // yes simd      109      139        0
    //  no simd      185      226        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self.group0()[0] * other.group0()[0]),
                ((self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] * other.group0()[0]),
                (self.group1()[1] * other.group0()[0]),
                (self.group1()[2] * other.group0()[0]),
                ((self.group1()[3] * other.group0()[0]) + (self.group4()[3] * other.group0()[1])),
            ]),
            // e41, e42, e43
            ((self.group2() * Simd32x3::from(other.group0()[0])) + (self.group3() * Simd32x3::from(other.group0()[1]))),
            // e23, e31, e12
            (self.group3() * Simd32x3::from(other.group0()[0])),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((self.group1()[0] * other.group0()[1]) + (self.group4()[0] * other.group0()[0])),
                ((self.group1()[1] * other.group0()[1]) + (self.group4()[1] * other.group0()[0])),
                ((self.group1()[2] * other.group0()[1]) + (self.group4()[2] * other.group0()[0])),
                (self.group4()[3] * other.group0()[0]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      102      133        0
    //    simd2       12       12        0
    //    simd3       24       26        0
    //    simd4       15       16        0
    // Totals...
    // yes simd      153      187        0
    //  no simd      258      299        0
    fn sandwich(self, other: Flector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (-(Simd32x2::from(self.group4()[3]) * Simd32x2::from([other.group1()[3], other.group0()[3]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], other.group1()[2]]))
                + (Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], other.group1()[0]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], other.group1()[1]]))
                + Simd32x2::from([
                    0.0,
                    (-(self.group4()[2] * other.group0()[2]) - (self.group4()[1] * other.group0()[1]) - (self.group4()[0] * other.group0()[0])
                        + (self.group1()[3] * other.group1()[3])),
                ])),
            // e1, e2, e3, e4
            ((swizzle!(other.group0(), 1, 1, 0, 2) * Simd32x4::from([self.group3()[2], self.group0()[0], self.group3()[1], self.group2()[2]]))
                + (swizzle!(other.group0(), 0, 2, 2, 1) * Simd32x4::from([self.group0()[0], self.group3()[0], self.group0()[0], self.group2()[1]]))
                + Simd32x4::from([
                    (-(self.group3()[1] * other.group0()[2]) + (self.group3()[0] * other.group1()[3])),
                    (-(self.group3()[2] * other.group0()[0]) + (self.group3()[1] * other.group1()[3])),
                    ((self.group3()[2] * other.group1()[3]) - (self.group3()[0] * other.group0()[1])),
                    (-(self.group3()[2] * other.group1()[2]) - (self.group3()[1] * other.group1()[1]) - (self.group3()[0] * other.group1()[0])
                        + (self.group2()[0] * other.group0()[0])
                        + (self.group0()[0] * other.group0()[3])
                        - (self.group0()[1] * other.group1()[3])),
                ])),
            // e41, e42, e43
            (-(Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + Simd32x3::from([
                    ((self.group4()[2] * other.group0()[1]) - (self.group4()[1] * other.group0()[2]) - (self.group1()[2] * other.group1()[1])
                        + (self.group1()[1] * other.group1()[2])),
                    (-(self.group4()[2] * other.group0()[0]) + (self.group4()[0] * other.group0()[2]) + (self.group1()[2] * other.group1()[0])
                        - (self.group1()[0] * other.group1()[2])),
                    ((self.group4()[1] * other.group0()[0]) - (self.group4()[0] * other.group0()[1]) + (self.group1()[0] * other.group1()[1])
                        - (self.group1()[1] * other.group1()[0])),
                ])),
            // e23, e31, e12
            (-(Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + Simd32x3::from([
                    (-(self.group1()[2] * other.group0()[1]) + (self.group1()[1] * other.group0()[2])),
                    ((self.group1()[2] * other.group0()[0]) - (self.group1()[0] * other.group0()[2])),
                    ((self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0])),
                ])),
            // e423, e431, e412, e321
            ((swizzle!(other.group1(), 1, 2, 0, 3) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group0()[0]]))
                - (swizzle!(other.group0(), 1, 2, 0, 2) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[2]]))
                - (swizzle!(other.group0(), 0, 1, 2, 1) * Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group3()[1]]))
                + Simd32x4::from([
                    (-(self.group3()[1] * other.group1()[2]) + (self.group3()[0] * other.group0()[3]) + (self.group2()[1] * other.group0()[2])
                        - (self.group2()[0] * other.group1()[3])
                        + (self.group0()[0] * other.group1()[0])),
                    (-(self.group3()[2] * other.group1()[0]) + (self.group3()[1] * other.group0()[3]) + (self.group2()[2] * other.group0()[0])
                        - (self.group2()[1] * other.group1()[3])
                        + (self.group0()[0] * other.group1()[1])),
                    ((self.group3()[2] * other.group0()[3]) - (self.group3()[0] * other.group1()[1]) - (self.group2()[2] * other.group1()[3])
                        + (self.group2()[0] * other.group0()[1])
                        + (self.group0()[0] * other.group1()[2])),
                    ((self.group3()[0] * other.group0()[0]) * -1.0),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       67       82        0
    //    simd2        8       10        0
    //    simd3       18       23        0
    //    simd4       10       15        0
    // Totals...
    // yes simd      103      130        0
    //  no simd      177      231        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from(other[e321]) * Simd32x2::from([self.group4()[3], self.group1()[3]]) * Simd32x2::from([-1.0, 1.0])),
            // e1, e2, e3, e4
            (Simd32x4::from(other[e321]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group0()[1]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])),
            // e41, e42, e43
            (Simd32x3::from(other[e321]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]])),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e321]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[0]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       89      113        0
    //    simd2       11       11        0
    //    simd3       25       29        0
    //    simd4       12       13        0
    // Totals...
    // yes simd      137      166        0
    //  no simd      234      274        0
    fn sandwich(self, other: Line) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (-(Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]))
                - (Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]))
                - (Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]))
                + Simd32x2::from([
                    0.0,
                    (-(self.group2()[2] * other.group1()[2]) - (self.group2()[0] * other.group1()[0]) - (self.group2()[1] * other.group1()[1])),
                ])),
            // e1, e2, e3, e4
            (-(swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    ((self.group4()[3] * other.group1()[0]) + (self.group1()[2] * other.group1()[1])),
                    ((self.group4()[3] * other.group1()[1]) + (self.group1()[0] * other.group1()[2])),
                    ((self.group4()[3] * other.group1()[2]) + (self.group1()[1] * other.group1()[0])),
                    (-(self.group4()[2] * other.group1()[2])
                        - (self.group4()[1] * other.group1()[1])
                        - (self.group4()[0] * other.group1()[0])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group1()[1] * other.group0()[1])),
                ])),
            // e41, e42, e43
            ((swizzle!(self.group3(), 2, 0, 1) * swizzle!(other.group0(), 1, 2, 0)) - (swizzle!(self.group3(), 1, 2, 0) * swizzle!(other.group0(), 2, 0, 1))
                + (swizzle!(self.group2(), 2, 0, 1) * swizzle!(other.group1(), 1, 2, 0))
                - (swizzle!(self.group2(), 1, 2, 0) * swizzle!(other.group1(), 2, 0, 1))
                + (Simd32x3::from(self.group0()[0]) * other.group0())
                + (Simd32x3::from(self.group0()[1]) * other.group1())),
            // e23, e31, e12
            ((swizzle!(other.group1(), 1, 1, 0) * Simd32x3::from([self.group3()[2], self.group0()[0], self.group3()[1]]))
                + (swizzle!(other.group1(), 0, 2, 2) * Simd32x3::from([self.group0()[0], self.group3()[0], self.group0()[0]]))
                - (swizzle!(self.group3(), 1, 2, 0) * swizzle!(other.group1(), 2, 0, 1))),
            // e423, e431, e412, e321
            (-(swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group4()[3] * other.group0()[0]) + (self.group4()[2] * other.group1()[1]) - (self.group4()[1] * other.group1()[2])
                        + (self.group1()[3] * other.group1()[0])
                        + (self.group1()[2] * other.group0()[1])),
                    ((self.group4()[3] * other.group0()[1]) - (self.group4()[2] * other.group1()[0])
                        + (self.group4()[0] * other.group1()[2])
                        + (self.group1()[3] * other.group1()[1])
                        + (self.group1()[0] * other.group0()[2])),
                    ((self.group4()[3] * other.group0()[2]) + (self.group4()[1] * other.group1()[0]) - (self.group4()[0] * other.group1()[1])
                        + (self.group1()[3] * other.group1()[2])
                        + (self.group1()[1] * other.group0()[0])),
                    (-(self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       86      111        0
    //    simd2       12       12        0
    //    simd3       28       32        0
    //    simd4       16       17        0
    // Totals...
    // yes simd      142      172        0
    //  no simd      258      299        0
    fn sandwich(self, other: Motor) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (-(Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]))
                - (Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]))
                + (Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[3], other.group0()[3]]))
                - (Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]))
                + Simd32x2::from([
                    0.0,
                    (-(self.group2()[2] * other.group1()[2]) - (self.group2()[1] * other.group1()[1]) - (self.group2()[0] * other.group1()[0])
                        + (self.group0()[1] * other.group1()[3])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]))
                + (swizzle!(self.group1(), 2, 0, 2, 3) * swizzle!(other.group1(), 1, 2, 3, 3))
                - (swizzle!(other.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group4()[2]]))
                + Simd32x4::from([
                    (self.group1()[0] * other.group1()[3]),
                    (self.group1()[1] * other.group1()[3]),
                    (self.group1()[1] * other.group1()[0]),
                    (-(self.group4()[1] * other.group1()[1])
                        - (self.group4()[0] * other.group1()[0])
                        - (self.group1()[2] * other.group0()[2])
                        - (self.group1()[0] * other.group0()[0])
                        - (self.group1()[1] * other.group0()[1])),
                ])),
            // e41, e42, e43
            ((swizzle!(self.group3(), 2, 1, 2) * Simd32x3::from([other.group0()[1], other.group0()[3], other.group0()[3]]))
                - (swizzle!(self.group3(), 1, 2, 0) * Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]))
                + (swizzle!(self.group3(), 0, 0, 1) * Simd32x3::from([other.group0()[3], other.group0()[2], other.group0()[0]]))
                + (swizzle!(self.group2(), 2, 1, 2) * Simd32x3::from([other.group1()[1], other.group1()[3], other.group1()[3]]))
                - (swizzle!(self.group2(), 1, 2, 0) * Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]))
                + (swizzle!(self.group2(), 0, 0, 1) * Simd32x3::from([other.group1()[3], other.group1()[2], other.group1()[0]]))
                + (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + (Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))),
            // e23, e31, e12
            ((swizzle!(self.group3(), 2, 1, 2) * Simd32x3::from([other.group1()[1], other.group1()[3], other.group1()[3]]))
                - (swizzle!(self.group3(), 1, 2, 0) * Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]))
                + (Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (swizzle!(self.group3(), 0, 0, 1) * Simd32x3::from([other.group1()[3], other.group1()[2], other.group1()[0]]))),
            // e423, e431, e412, e321
            ((Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (swizzle!(other.group1(), 2, 0, 1, 2) * Simd32x4::from([self.group4()[1], self.group4()[2], self.group4()[0], self.group1()[2]]))
                - (swizzle!(self.group1(), 1, 2, 0, 0) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]))
                + Simd32x4::from([
                    ((self.group4()[2] * other.group1()[1])
                        + (self.group4()[0] * other.group1()[3])
                        + (self.group1()[3] * other.group1()[0])
                        + (self.group1()[2] * other.group0()[1])
                        + (self.group1()[0] * other.group0()[3])),
                    ((self.group4()[1] * other.group1()[3])
                        + (self.group4()[0] * other.group1()[2])
                        + (self.group1()[3] * other.group1()[1])
                        + (self.group1()[0] * other.group0()[2])
                        + (self.group1()[1] * other.group0()[3])),
                    ((self.group4()[2] * other.group1()[3])
                        + (self.group4()[1] * other.group1()[0])
                        + (self.group1()[3] * other.group1()[2])
                        + (self.group1()[2] * other.group0()[3])
                        + (self.group1()[1] * other.group0()[0])),
                    ((self.group1()[1] * other.group1()[1]) * -1.0),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      134      164        0
    //    simd2       16       16        0
    //    simd3       36       38        0
    //    simd4       20       21        0
    // Totals...
    // yes simd      206      239        0
    //  no simd      354      394        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (-(Simd32x2::from(self.group4()[3]) * Simd32x2::from([other.group4()[3], other.group1()[3]]))
                - (Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]))
                - (Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]))
                - (Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]))
                + (Simd32x2::from(self.group0()[0]) * other.group0())
                + (Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]))
                + Simd32x2::from([
                    0.0,
                    (-(self.group4()[2] * other.group1()[2])
                        - (self.group4()[1] * other.group1()[1])
                        - (self.group4()[0] * other.group1()[0])
                        - (self.group2()[2] * other.group3()[2])
                        - (self.group2()[1] * other.group3()[1])
                        - (self.group2()[0] * other.group3()[0])
                        + (self.group1()[3] * other.group4()[3])
                        + (self.group0()[1] * other.group0()[0])),
                ])),
            // e1, e2, e3, e4
            ((Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]))
                + (swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[2]]))
                + (swizzle!(self.group1(), 2, 1, 2, 3) * Simd32x4::from([other.group3()[1], other.group0()[0], other.group0()[0], other.group0()[0]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[2]]))
                + (swizzle!(other.group1(), 0, 1, 2, 1) * Simd32x4::from([self.group0()[0], self.group0()[0], self.group0()[0], self.group2()[1]]))
                + Simd32x4::from([
                    (-(self.group3()[1] * other.group1()[2]) + (self.group3()[0] * other.group4()[3]) + (self.group1()[0] * other.group0()[0])),
                    (-(self.group3()[2] * other.group1()[0]) + (self.group3()[1] * other.group4()[3]) + (self.group1()[0] * other.group3()[2])),
                    ((self.group3()[2] * other.group4()[3]) - (self.group3()[0] * other.group1()[1]) + (self.group1()[1] * other.group3()[0])),
                    (-(self.group4()[2] * other.group3()[2])
                        - (self.group4()[1] * other.group3()[1])
                        - (self.group4()[0] * other.group3()[0])
                        - (self.group3()[2] * other.group4()[2])
                        - (self.group3()[1] * other.group4()[1])
                        - (self.group3()[0] * other.group4()[0])
                        + (self.group2()[0] * other.group1()[0])
                        - (self.group1()[1] * other.group2()[1])
                        - (self.group1()[0] * other.group2()[0])
                        + (self.group0()[0] * other.group1()[3])
                        - (self.group0()[1] * other.group4()[3])),
                ])),
            // e41, e42, e43
            (-(Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                + (swizzle!(self.group3(), 2, 1, 2) * Simd32x3::from([other.group2()[1], other.group0()[1], other.group0()[1]]))
                - (swizzle!(self.group3(), 1, 2, 0) * swizzle!(other.group2(), 2, 0, 1))
                + (swizzle!(self.group3(), 0, 0, 1) * Simd32x3::from([other.group0()[1], other.group2()[2], other.group2()[0]]))
                + (swizzle!(self.group2(), 2, 1, 2) * Simd32x3::from([other.group3()[1], other.group0()[0], other.group0()[0]]))
                - (swizzle!(self.group2(), 1, 2, 0) * swizzle!(other.group3(), 2, 0, 1))
                + (swizzle!(self.group2(), 0, 0, 1) * Simd32x3::from([other.group0()[0], other.group3()[2], other.group3()[0]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group0()[0]) * other.group2())
                + (Simd32x3::from(self.group0()[1]) * other.group3())
                + Simd32x3::from([
                    ((self.group4()[2] * other.group1()[1]) - (self.group4()[1] * other.group1()[2]) - (self.group1()[2] * other.group4()[1])
                        + (self.group1()[1] * other.group4()[2])),
                    (-(self.group4()[2] * other.group1()[0]) + (self.group4()[0] * other.group1()[2]) + (self.group1()[2] * other.group4()[0])
                        - (self.group1()[0] * other.group4()[2])),
                    ((self.group4()[1] * other.group1()[0]) - (self.group4()[0] * other.group1()[1]) - (self.group1()[1] * other.group4()[0])
                        + (self.group1()[0] * other.group4()[1])),
                ])),
            // e23, e31, e12
            (-(Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (swizzle!(self.group3(), 2, 1, 2) * Simd32x3::from([other.group3()[1], other.group0()[0], other.group0()[0]]))
                - (swizzle!(self.group3(), 1, 2, 0) * swizzle!(other.group3(), 2, 0, 1))
                + (swizzle!(self.group3(), 0, 0, 1) * Simd32x3::from([other.group0()[0], other.group3()[2], other.group3()[0]]))
                + (Simd32x3::from(self.group0()[0]) * other.group3())
                - (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + Simd32x3::from([
                    (-(self.group1()[2] * other.group1()[1]) + (self.group1()[1] * other.group1()[2])),
                    ((self.group1()[2] * other.group1()[0]) - (self.group1()[0] * other.group1()[2])),
                    (-(self.group1()[1] * other.group1()[0]) + (self.group1()[0] * other.group1()[1])),
                ])),
            // e423, e431, e412, e321
            ((Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]]))
                + (swizzle!(other.group4(), 1, 2, 0, 3) * Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group0()[0]]))
                - (swizzle!(other.group1(), 1, 2, 0, 2) * Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[2]]))
                - (swizzle!(self.group1(), 1, 2, 0, 2) * Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[2]]))
                - (swizzle!(other.group1(), 0, 1, 2, 1) * Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group3()[1]]))
                + Simd32x4::from([
                    ((self.group4()[2] * other.group3()[1]) - (self.group4()[1] * other.group3()[2]) + (self.group4()[0] * other.group0()[0])
                        - (self.group3()[1] * other.group4()[2])
                        + (self.group3()[0] * other.group1()[3])
                        + (self.group2()[1] * other.group1()[2])
                        - (self.group2()[0] * other.group4()[3])
                        + (self.group1()[3] * other.group3()[0])
                        + (self.group1()[2] * other.group2()[1])
                        + (self.group1()[0] * other.group0()[1])
                        + (self.group0()[0] * other.group4()[0])),
                    (-(self.group4()[2] * other.group3()[0]) + (self.group4()[1] * other.group0()[0]) + (self.group4()[0] * other.group3()[2])
                        - (self.group3()[2] * other.group4()[0])
                        + (self.group3()[1] * other.group1()[3])
                        + (self.group2()[2] * other.group1()[0])
                        - (self.group2()[1] * other.group4()[3])
                        + (self.group1()[3] * other.group3()[1])
                        + (self.group1()[1] * other.group0()[1])
                        + (self.group1()[0] * other.group2()[2])
                        + (self.group0()[0] * other.group4()[1])),
                    ((self.group4()[2] * other.group0()[0]) + (self.group4()[1] * other.group3()[0]) - (self.group4()[0] * other.group3()[1])
                        + (self.group3()[2] * other.group1()[3])
                        - (self.group3()[0] * other.group4()[1])
                        - (self.group2()[2] * other.group4()[3])
                        + (self.group2()[0] * other.group1()[1])
                        + (self.group1()[3] * other.group3()[2])
                        + (self.group1()[2] * other.group0()[1])
                        + (self.group1()[1] * other.group2()[0])
                        + (self.group0()[0] * other.group4()[2])),
                    (-(self.group3()[0] * other.group1()[0]) - (self.group1()[1] * other.group3()[1]) - (self.group1()[0] * other.group3()[0])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       67       88        0
    //    simd2        8        8        0
    //    simd3       18       22        0
    //    simd4       10       11        0
    // Totals...
    // yes simd      103      129        0
    //  no simd      177      214        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self.group4()[3] * other[e4] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[0] * other[e4])]),
            // e41, e42, e43
            (Simd32x3::from(other[e4]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([(self.group3()[0] * other[e4]), (self.group3()[1] * other[e4]), (self.group3()[2] * other[e4]), 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       85      114        0
    //    simd2        8        8        0
    //    simd3       20       24        0
    //    simd4       10       11        0
    // Totals...
    // yes simd      123      157        0
    //  no simd      201      246        0
    fn sandwich(self, other: Plane) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self.group4()[3] * other.group0()[3] * -1.0),
                ((self.group1()[3] * other.group0()[3]) + (self.group1()[2] * other.group0()[2]) + (self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group3()[0] * other.group0()[3]),
                (self.group3()[1] * other.group0()[3]),
                (self.group3()[2] * other.group0()[3]),
                (-(self.group3()[2] * other.group0()[2])
                    - (self.group3()[1] * other.group0()[1])
                    - (self.group0()[1] * other.group0()[3])
                    - (self.group3()[0] * other.group0()[0])),
            ]),
            // e41, e42, e43
            (-(Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                + Simd32x3::from([
                    ((self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1])),
                    (-(self.group1()[0] * other.group0()[2]) + (self.group1()[2] * other.group0()[0])),
                    ((self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0])),
                ])),
            // e23, e31, e12
            (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((self.group3()[2] * other.group0()[1]) - (self.group3()[1] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) - (self.group2()[0] * other.group0()[3])),
                (-(self.group3()[2] * other.group0()[0]) + (self.group3()[0] * other.group0()[2]) + (self.group0()[0] * other.group0()[1])
                    - (self.group2()[1] * other.group0()[3])),
                ((self.group3()[1] * other.group0()[0]) - (self.group3()[0] * other.group0()[1]) + (self.group0()[0] * other.group0()[2]) - (self.group2()[2] * other.group0()[3])),
                (self.group0()[0] * other.group0()[3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       82      117        0
    //    simd2        8        8        0
    //    simd3       21       23        0
    //    simd4       14       15        0
    // Totals...
    // yes simd      125      163        0
    //  no simd      217      262        0
    fn sandwich(self, other: Point) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                ((self.group1()[2] * other.group0()[2]) + (self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1])),
                (-(self.group4()[3] * other.group0()[3])
                    - (self.group4()[2] * other.group0()[2])
                    - (self.group4()[0] * other.group0()[0])
                    - (self.group4()[1] * other.group0()[1])),
            ]),
            // e1, e2, e3, e4
            ((swizzle!(other.group0(), 1, 1, 0, 2) * Simd32x4::from([self.group3()[2], self.group0()[0], self.group3()[1], self.group2()[2]]))
                + (swizzle!(other.group0(), 0, 2, 2, 1) * Simd32x4::from([self.group0()[0], self.group3()[0], self.group0()[0], self.group2()[1]]))
                + Simd32x4::from([
                    ((self.group3()[1] * other.group0()[2]) * -1.0),
                    ((self.group3()[2] * other.group0()[0]) * -1.0),
                    ((self.group3()[0] * other.group0()[1]) * -1.0),
                    ((self.group0()[0] * other.group0()[3]) + (self.group2()[0] * other.group0()[0])),
                ])),
            // e41, e42, e43
            (-(Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + Simd32x3::from([
                    ((self.group4()[2] * other.group0()[1]) - (self.group4()[1] * other.group0()[2])),
                    (-(self.group4()[2] * other.group0()[0]) + (self.group4()[0] * other.group0()[2])),
                    ((self.group4()[1] * other.group0()[0]) - (self.group4()[0] * other.group0()[1])),
                ])),
            // e23, e31, e12
            (-(Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + Simd32x3::from([
                    ((self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1])),
                    (-(self.group1()[0] * other.group0()[2]) + (self.group1()[2] * other.group0()[0])),
                    ((self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0])),
                ])),
            // e423, e431, e412, e321
            (-(swizzle!(other.group0(), 1, 1, 0, 2) * Simd32x4::from([self.group2()[2], self.group0()[1], self.group2()[1], self.group3()[2]]))
                - (swizzle!(other.group0(), 0, 2, 2, 0) * Simd32x4::from([self.group0()[1], self.group2()[0], self.group0()[1], self.group3()[0]]))
                + Simd32x4::from([
                    ((self.group3()[0] * other.group0()[3]) + (self.group2()[1] * other.group0()[2])),
                    ((self.group3()[1] * other.group0()[3]) + (self.group2()[2] * other.group0()[0])),
                    ((self.group3()[2] * other.group0()[3]) + (self.group2()[0] * other.group0()[1])),
                    ((self.group3()[1] * other.group0()[1]) * -1.0),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       67       82        0
    //    simd2        8        9        0
    //    simd3       18       22        0
    //    simd4       10       13        0
    // Totals...
    // yes simd      103      126        0
    //  no simd      177      218        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (self.group0() * Simd32x2::from(other[scalar])),
            // e1, e2, e3, e4
            (self.group1() * Simd32x4::from(other[scalar])),
            // e41, e42, e43
            (self.group2() * Simd32x3::from(other[scalar])),
            // e23, e31, e12
            (self.group3() * Simd32x3::from(other[scalar])),
            // e423, e431, e412, e321
            (self.group4() * Simd32x4::from(other[scalar])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl InfixSandwich for Origin {}
impl Sandwich<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e4]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
            // e23, e31, e12, scalar
            Simd32x4::from(0.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(self[e4] * other.group1()[0]), (self[e4] * other.group1()[1]), (self[e4] * other.group1()[2]), 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self[e4] * other.group1()[3])]),
            // e423, e431, e412, e321
            Simd32x4::from([(self[e4] * other.group1()[0]), (self[e4] * other.group1()[1]), (self[e4] * other.group1()[2]), 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       11        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0       14        0
    //  no simd        0       20        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self[e4] * other.group4()[3])]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self[e4] * other.group0()[0])]),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([(self[e4] * other.group3()[0]), (self[e4] * other.group3()[1]), (self[e4] * other.group3()[2]), 0.0]),
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
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]])),
            // e23, e31, e12
            Simd32x3::from(0.0),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl InfixSandwich for Plane {}
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
        let geometric_product = Origin::from_groups(/* e4 */ (self.group0()[3] * other[e1234]));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd4        2        6        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       12       33        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * other.group0()[1])]),
            // e423, e431, e412, e321
            (self.group0() * Simd32x4::from(other.group0()[0])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       18       33        0
    //  no simd       24       48        0
    fn sandwich(self, other: Flector) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (-(Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(other.group0(), 2, 0, 1, 2))
                + Simd32x4::from([
                    ((self.group0()[2] * other.group0()[1]) + (self.group0()[0] * other.group1()[3])),
                    ((self.group0()[0] * other.group0()[2]) + (self.group0()[1] * other.group1()[3])),
                    ((self.group0()[2] * other.group1()[3]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
            // e23, e31, e12, scalar
            (Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       25        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       26        0
    //  no simd       12       29        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([(self.group0()[0] * other[e321]), (self.group0()[1] * other[e321]), (self.group0()[2] * other[e321]), 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * other[e321] * -1.0)]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       23        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       14       28        0
    //  no simd       20       43        0
    fn sandwich(self, other: Line) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[3] * other.group1()[0]),
                (self.group0()[3] * other.group1()[1]),
                (self.group0()[3] * other.group1()[2]),
                (-(self.group0()[2] * other.group1()[2]) - (self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((self.group0()[3] * other.group0()[0]) - (self.group0()[1] * other.group1()[2]) + (self.group0()[2] * other.group1()[1])),
                ((self.group0()[3] * other.group0()[1]) + (self.group0()[0] * other.group1()[2]) - (self.group0()[2] * other.group1()[0])),
                ((self.group0()[3] * other.group0()[2]) - (self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
                0.0,
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        2        5        0
    // Totals...
    // yes simd       18       33        0
    //  no simd       24       48        0
    fn sandwich(self, other: Motor) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[3] * other.group1()[0]),
                (self.group0()[3] * other.group1()[1]),
                (self.group0()[3] * other.group1()[2]),
                ((self.group0()[3] * other.group0()[3]) - (self.group0()[2] * other.group1()[2]) - (self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((self.group0()[3] * other.group0()[0]) + (self.group0()[2] * other.group1()[1]) + (self.group0()[0] * other.group1()[3]) - (self.group0()[1] * other.group1()[2])),
                ((self.group0()[3] * other.group0()[1]) - (self.group0()[2] * other.group1()[0]) + (self.group0()[0] * other.group1()[2]) + (self.group0()[1] * other.group1()[3])),
                ((self.group0()[3] * other.group0()[2]) + (self.group0()[2] * other.group1()[3]) - (self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
                (self.group0()[3] * other.group1()[3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       64        0
    //    simd3        4        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       40       73        0
    //  no simd       48       92        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self.group0()[3] * other.group4()[3] * -1.0),
                (-(self.group0()[3] * other.group1()[3])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[3] * other.group3()[0]),
                (self.group0()[3] * other.group3()[1]),
                (self.group0()[3] * other.group3()[2]),
                ((self.group0()[3] * other.group0()[1]) - (self.group0()[2] * other.group3()[2]) - (self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
            ]),
            // e41, e42, e43
            (-(Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + Simd32x3::from([
                    ((self.group0()[2] * other.group1()[1]) - (self.group0()[1] * other.group1()[2])),
                    (-(self.group0()[2] * other.group1()[0]) + (self.group0()[0] * other.group1()[2])),
                    (-(self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group1()[0])),
                ])),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((self.group0()[3] * other.group2()[0]) + (self.group0()[2] * other.group3()[1]) + (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group3()[2])),
                ((self.group0()[3] * other.group2()[1]) - (self.group0()[2] * other.group3()[0]) + (self.group0()[0] * other.group3()[2]) + (self.group0()[1] * other.group0()[0])),
                ((self.group0()[3] * other.group2()[2]) + (self.group0()[2] * other.group0()[0]) - (self.group0()[0] * other.group3()[1]) + (self.group0()[1] * other.group3()[0])),
                (self.group0()[3] * other.group0()[0]),
            ]),
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
        let geometric_product = AntiScalar::from_groups(/* e1234 */ (self.group0()[3] * other[e4] * -1.0));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       28        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       15       29        0
    //  no simd       15       32        0
    fn sandwich(self, other: Plane) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                ((self.group0()[0] * other.group0()[3]) - (self.group0()[3] * other.group0()[0])),
                ((self.group0()[1] * other.group0()[3]) - (self.group0()[3] * other.group0()[1])),
                ((self.group0()[2] * other.group0()[3]) - (self.group0()[3] * other.group0()[2])),
                0.0,
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * other.group0()[3] * -1.0)]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       32        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       15       34        0
    //  no simd       18       40        0
    fn sandwich(self, other: Point) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (-(swizzle!(self.group0(), 1, 2, 0, 3) * swizzle!(other.group0(), 2, 0, 1, 3))
                + Simd32x4::from([
                    (self.group0()[2] * other.group0()[1]),
                    (self.group0()[0] * other.group0()[2]),
                    (self.group0()[1] * other.group0()[0]),
                    (-(self.group0()[2] * other.group0()[2]) - (self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self.group0()[3] * other.group0()[0] * -1.0),
                (self.group0()[3] * other.group0()[1] * -1.0),
                (self.group0()[3] * other.group0()[2] * -1.0),
                0.0,
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       16        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ (self.group0() * Simd32x4::from(other[scalar])));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl InfixSandwich for Point {}
impl Sandwich<AntiScalar> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        3       16        0
    //  no simd        6       19        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from([
            (self.group0()[0] * other[e1234]),
            (self.group0()[1] * other[e1234]),
            (self.group0()[2] * other[e1234]),
            0.0,
        ]));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       19        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       20       35        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (self.group0() * Simd32x4::from(other.group0()[0])),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self.group0()[0] * other.group0()[1]),
                (self.group0()[1] * other.group0()[1]),
                (self.group0()[2] * other.group0()[1]),
                0.0,
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       32        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       19       39        0
    //  no simd       40       60        0
    fn sandwich(self, other: Flector) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                + (swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(other.group1(), 2, 0, 1, 2))
                + Simd32x4::from([
                    (-(self.group0()[2] * other.group1()[1]) - (self.group0()[0] * other.group0()[3])),
                    (-(self.group0()[0] * other.group1()[2]) - (self.group0()[1] * other.group0()[3])),
                    (-(self.group0()[2] * other.group0()[3]) - (self.group0()[1] * other.group1()[0])),
                    ((self.group0()[0] * other.group1()[0]) + (self.group0()[1] * other.group1()[1])),
                ])),
            // e23, e31, e12, scalar
            ((swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(other.group0(), 2, 0, 1, 2))
                + Simd32x4::from([
                    (-(self.group0()[2] * other.group0()[1]) - (self.group0()[0] * other.group1()[3])),
                    (-(self.group0()[0] * other.group0()[2]) - (self.group0()[1] * other.group1()[3])),
                    (-(self.group0()[2] * other.group1()[3]) - (self.group0()[1] * other.group0()[0])),
                    ((self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       23        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        8       27        0
    //  no simd       20       39        0
    fn sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * other[e321])]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self.group0()[0] * other[e321] * -1.0),
                (self.group0()[1] * other[e321] * -1.0),
                (self.group0()[2] * other[e321] * -1.0),
                0.0,
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       29        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       18       34        0
    //  no simd       33       49        0
    fn sandwich(self, other: Line) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    (self.group0()[2] * other.group1()[1]),
                    (self.group0()[0] * other.group1()[2]),
                    (self.group0()[1] * other.group1()[0]),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
            // e423, e431, e412, e321
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group1()[0]) + (self.group0()[2] * other.group0()[1])),
                    ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group0()[2])),
                    ((self.group0()[3] * other.group1()[2]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       22       38        0
    //  no simd       40       56        0
    fn sandwich(self, other: Motor) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            ((swizzle!(self.group0(), 2, 0, 2, 3) * swizzle!(other.group1(), 1, 2, 3, 3))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]))
                + Simd32x4::from([
                    (self.group0()[0] * other.group1()[3]),
                    (self.group0()[1] * other.group1()[3]),
                    (self.group0()[1] * other.group1()[0]),
                    (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1])),
                ])),
            // e423, e431, e412, e321
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group1()[0]) + (self.group0()[2] * other.group0()[1]) + (self.group0()[0] * other.group0()[3])),
                    ((self.group0()[3] * other.group1()[1]) + (self.group0()[0] * other.group0()[2]) + (self.group0()[1] * other.group0()[3])),
                    ((self.group0()[3] * other.group1()[2]) + (self.group0()[2] * other.group0()[3]) + (self.group0()[1] * other.group0()[0])),
                    (-(self.group0()[0] * other.group1()[0]) - (self.group0()[1] * other.group1()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       64        0
    //    simd2        3        3        0
    //    simd3        6        6        0
    //    simd4        7        7        0
    // Totals...
    // yes simd       45       80        0
    //  no simd       81      116        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            ((Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]))
                + (Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]))
                + (Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]))
                + Simd32x2::from([0.0, (self.group0()[3] * other.group4()[3])])),
            // e1, e2, e3, e4
            ((swizzle!(self.group0(), 2, 0, 2, 3) * Simd32x4::from([other.group3()[1], other.group3()[2], other.group0()[0], other.group0()[0]]))
                - (swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[2]]))
                + Simd32x4::from([
                    (self.group0()[0] * other.group0()[0]),
                    (self.group0()[1] * other.group0()[0]),
                    (self.group0()[1] * other.group3()[0]),
                    (-(self.group0()[0] * other.group2()[0]) - (self.group0()[1] * other.group2()[1])),
                ])),
            // e41, e42, e43
            ((Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + Simd32x3::from([
                    (-(self.group0()[2] * other.group4()[1]) + (self.group0()[1] * other.group4()[2])),
                    ((self.group0()[2] * other.group4()[0]) - (self.group0()[0] * other.group4()[2])),
                    ((self.group0()[0] * other.group4()[1]) - (self.group0()[1] * other.group4()[0])),
                ])),
            // e23, e31, e12
            (-(Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + Simd32x3::from([
                    (-(self.group0()[2] * other.group1()[1]) + (self.group0()[1] * other.group1()[2])),
                    ((self.group0()[2] * other.group1()[0]) - (self.group0()[0] * other.group1()[2])),
                    ((self.group0()[0] * other.group1()[1]) - (self.group0()[1] * other.group1()[0])),
                ])),
            // e423, e431, e412, e321
            (-(swizzle!(self.group0(), 1, 2, 0, 2) * Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[2]]))
                + Simd32x4::from([
                    ((self.group0()[3] * other.group3()[0]) + (self.group0()[2] * other.group2()[1]) + (self.group0()[0] * other.group0()[1])),
                    ((self.group0()[3] * other.group3()[1]) + (self.group0()[0] * other.group2()[2]) + (self.group0()[1] * other.group0()[1])),
                    ((self.group0()[3] * other.group3()[2]) + (self.group0()[2] * other.group0()[1]) + (self.group0()[1] * other.group2()[0])),
                    (-(self.group0()[0] * other.group3()[0]) - (self.group0()[1] * other.group3()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd3        0        2        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       20        0
    //  no simd       13       30        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e4]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]) * Simd32x3::from(-1.0)),
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
    //      f32        6       31        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       11       36        0
    //  no simd       26       51        0
    fn sandwich(self, other: Plane) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            ((swizzle!(self.group0(), 1, 2, 0, 3) * swizzle!(other.group0(), 2, 0, 1, 3))
                + Simd32x4::from([
                    ((self.group0()[2] * other.group0()[1]) * -1.0),
                    ((self.group0()[0] * other.group0()[2]) * -1.0),
                    ((self.group0()[1] * other.group0()[0]) * -1.0),
                    ((self.group0()[2] * other.group0()[2]) + (self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3] * -1.0),
                (self.group0()[1] * other.group0()[3] * -1.0),
                (self.group0()[2] * other.group0()[3] * -1.0),
                0.0,
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       30        0
    //    simd4        5        5        0
    // Totals...
    // yes simd       13       35        0
    //  no simd       28       50        0
    fn sandwich(self, other: Point) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (-(self.group0()[0] * other.group0()[3]) + (self.group0()[3] * other.group0()[0])),
                (-(self.group0()[1] * other.group0()[3]) + (self.group0()[3] * other.group0()[1])),
                (-(self.group0()[2] * other.group0()[3]) + (self.group0()[3] * other.group0()[2])),
                0.0,
            ]),
            // e23, e31, e12, scalar
            ((swizzle!(self.group0(), 1, 2, 0, 2) * swizzle!(other.group0(), 2, 0, 1, 2))
                + Simd32x4::from([
                    ((self.group0()[2] * other.group0()[1]) * -1.0),
                    ((self.group0()[0] * other.group0()[2]) * -1.0),
                    ((self.group0()[1] * other.group0()[0]) * -1.0),
                    ((self.group0()[0] * other.group0()[0]) + (self.group0()[1] * other.group0()[1])),
                ])),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       14        0
    //    simd4        1        2        0
    // Totals...
    // yes simd        5       16        0
    //  no simd        8       22        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Point::from_groups(/* e1, e2, e3, e4 */ (self.group0() * Simd32x4::from(other[scalar])));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl InfixSandwich for Scalar {}
impl Sandwich<AntiScalar> for Scalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = AntiScalar::from_groups(/* e1234 */ (self[scalar] * other[e1234]));
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
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ (Simd32x2::from(self[scalar]) * other.group0()));
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
            (Simd32x4::from(self[scalar]) * other.group0()),
            // e423, e431, e412, e321
            (Simd32x4::from(self[scalar]) * other.group1()),
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
        let geometric_product = Horizon::from_groups(/* e321 */ (self[scalar] * other[e321]));
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
            (Simd32x3::from(self[scalar]) * other.group0()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group1()),
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
            (Simd32x4::from(self[scalar]) * other.group0()),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[scalar]) * other.group1()),
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
            (Simd32x2::from(self[scalar]) * other.group0()),
            // e1, e2, e3, e4
            (Simd32x4::from(self[scalar]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(self[scalar]) * other.group2()),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group3()),
            // e423, e431, e412, e321
            (Simd32x4::from(self[scalar]) * other.group4()),
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
        let geometric_product = Origin::from_groups(/* e4 */ (self[scalar] * other[e4]));
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
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x4::from(self[scalar]) * other.group0()));
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
        let geometric_product = Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x4::from(self[scalar]) * other.group0()));
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
        let geometric_product = Scalar::from_groups(/* scalar */ (self[scalar] * other[scalar]));
        return geometric_product.geometric_product(self.reverse());
    }
}
