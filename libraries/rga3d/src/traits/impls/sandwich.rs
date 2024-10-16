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
//   Median:        18      35       0
//  Average:        39      56       0
//  Maximum:       279     320       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        19      39       0
//  Average:        44      63       0
//  Maximum:       353     394       0
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
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * self[e1234] * -1.0)]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group0()[0] * self[e1234] * -1.0),
                (other.group0()[1] * self[e1234] * -1.0),
                (other.group0()[2] * self[e1234] * -1.0),
                0.0,
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Line::from_groups(/* e41, e42, e43 */ (Simd32x3::from(self[e1234]) * other.group1()), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
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
    //      f32        0       17        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0       18        0
    //  no simd        0       20        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (other.group0()[0] * self[e1234])]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group4()[3] * self[e1234] * -1.0)]),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group3()),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group1()[0] * self[e1234] * -1.0),
                (other.group1()[1] * self[e1234] * -1.0),
                (other.group1()[2] * self[e1234] * -1.0),
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
            (other.group0()[0] * self[e1234] * -1.0),
            (other.group0()[1] * self[e1234] * -1.0),
            (other.group0()[2] * self[e1234] * -1.0),
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
            (other.group0()[0] * self.group0()[0]),
            ((other.group0()[0] * self.group0()[1]) + (other.group0()[1] * self.group0()[0])),
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
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        6       18        0
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
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       15        0
    //  no simd        8       24        0
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
    //      f32       13       39        0
    //    simd3        1        3        0
    // Totals...
    // yes simd       14       42        0
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
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ (Simd32x2::from(other[scalar]) * self.group0()));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl InfixSandwich for Flector {}
impl Sandwich<AntiScalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       40       56        0
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
    //      add/sub      mul      div
    // f32       44       64        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[0] * self.group0()[0]),
                (other.group0()[0] * self.group0()[1]),
                (other.group0()[0] * self.group0()[2]),
                ((other.group0()[0] * self.group0()[3]) + (other.group0()[1] * self.group1()[3])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((other.group0()[0] * self.group1()[0]) + (other.group0()[1] * self.group0()[0])),
                ((other.group0()[0] * self.group1()[1]) + (other.group0()[1] * self.group0()[1])),
                ((other.group0()[0] * self.group1()[2]) + (other.group0()[1] * self.group0()[2])),
                (other.group0()[0] * self.group1()[3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       68        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       56       76        0
    //  no simd       80      100        0
    fn sandwich(self, other: Flector) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((other.group0()[1] * self.group1()[2]) - (other.group1()[1] * self.group0()[2])),
                ((other.group0()[2] * self.group1()[0]) - (other.group1()[2] * self.group0()[0])),
                ((other.group0()[0] * self.group1()[1]) - (other.group1()[0] * self.group0()[1])),
                (-(other.group0()[3] * self.group1()[3]) + (other.group1()[3] * self.group0()[3])),
            ]) + (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]) * swizzle!(self.group0(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[2]]) * swizzle!(self.group1(), 3, 3, 3, 2))
                - (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]) * swizzle!(other.group0(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[2]]) * swizzle!(other.group1(), 3, 3, 3, 2))
                - (swizzle!(other.group0(), 2, 0, 1, 0) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (swizzle!(other.group1(), 2, 0, 1, 1) * swizzle!(self.group0(), 1, 2, 0, 1))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                (-(other.group0()[1] * self.group0()[2]) - (other.group1()[3] * self.group0()[0])),
                (-(other.group0()[2] * self.group0()[0]) - (other.group1()[3] * self.group0()[1])),
                (-(other.group0()[0] * self.group0()[1]) - (other.group1()[3] * self.group0()[2])),
                ((other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2])),
            ]) - (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                + (swizzle!(other.group0(), 2, 0, 1, 0) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       40       55        0
    //  no simd       40       64        0
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
    //      f32       60       80        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       62       82        0
    //  no simd       68       88        0
    fn sandwich(self, other: Line) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group1()[0] * self.group1()[3]) + (other.group1()[1] * self.group0()[2])),
                ((other.group1()[1] * self.group1()[3]) + (other.group1()[2] * self.group0()[0])),
                ((other.group1()[0] * self.group0()[1]) + (other.group1()[2] * self.group1()[3])),
                (-(other.group0()[1] * self.group0()[1])
                    - (other.group0()[2] * self.group0()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[0] * self.group1()[3]) + (other.group0()[1] * self.group0()[2]) + (other.group1()[0] * self.group0()[3]) + (other.group1()[1] * self.group1()[2])
                    - (other.group1()[2] * self.group1()[1])),
                ((other.group0()[1] * self.group1()[3]) + (other.group0()[2] * self.group0()[0]) - (other.group1()[0] * self.group1()[2])
                    + (other.group1()[1] * self.group0()[3])
                    + (other.group1()[2] * self.group1()[0])),
                ((other.group0()[0] * self.group0()[1]) + (other.group0()[2] * self.group1()[3]) + (other.group1()[0] * self.group1()[1]) - (other.group1()[1] * self.group1()[0])
                    + (other.group1()[2] * self.group0()[3])),
                (-(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       77        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       62       83        0
    //  no simd       80      101        0
    fn sandwich(self, other: Motor) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                (self.group0()[2] * other.group1()[1]),
                (self.group0()[1] * other.group1()[3]),
                (self.group0()[2] * other.group1()[3]),
                (-(self.group0()[1] * other.group0()[1])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])),
            ]) + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))
                + (swizzle!(self.group0(), 0, 0, 1, 3) * swizzle!(other.group1(), 3, 2, 0, 3))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((self.group0()[0] * other.group0()[3])
                    + (self.group0()[2] * other.group0()[1])
                    + (self.group1()[0] * other.group1()[3])
                    + (self.group1()[2] * other.group1()[1])
                    + (self.group1()[3] * other.group0()[0])),
                ((self.group0()[0] * other.group0()[2])
                    + (self.group0()[1] * other.group0()[3])
                    + (self.group1()[0] * other.group1()[2])
                    + (self.group1()[1] * other.group1()[3])
                    + (self.group1()[3] * other.group0()[1])),
                ((self.group0()[1] * other.group0()[0])
                    + (self.group0()[2] * other.group0()[3])
                    + (self.group1()[1] * other.group1()[0])
                    + (self.group1()[2] * other.group1()[3])
                    + (self.group1()[3] * other.group0()[2])),
                ((self.group0()[2] * other.group1()[2]) * -1.0),
            ]) + (Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[3]]) * other.group1())
                - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group0()[1]]) * swizzle!(other.group1(), 2, 0, 1, 1))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      115      150        0
    //    simd2        4        4        0
    //    simd3        6        6        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      130      165        0
    //  no simd      161      196        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                0.0,
                ((self.group0()[3] * other.group4()[3]) - (self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2])),
            ]) + (Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]))
                + (Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]))
                + (Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]))
                - (Simd32x2::from(self.group1()[3]) * Simd32x2::from([other.group4()[3], other.group1()[3]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (other.group3()[1] * self.group0()[2]),
                (other.group3()[2] * self.group0()[0]),
                (other.group3()[0] * self.group0()[1]),
                (-(other.group2()[1] * self.group0()[1])
                    - (other.group2()[2] * self.group0()[2])
                    - (other.group3()[0] * self.group1()[0])
                    - (other.group3()[1] * self.group1()[1])
                    - (other.group3()[2] * self.group1()[2])),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group0())
                + (Simd32x4::from(self.group1()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]))
                - (Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e41, e42, e43
            (Simd32x3::from([
                ((self.group0()[1] * other.group4()[2]) - (self.group0()[2] * other.group4()[1]) - (self.group1()[1] * other.group1()[2]) + (self.group1()[2] * other.group1()[1])),
                (-(self.group0()[0] * other.group4()[2]) + (self.group0()[2] * other.group4()[0]) + (self.group1()[0] * other.group1()[2])
                    - (self.group1()[2] * other.group1()[0])),
                ((self.group0()[0] * other.group4()[1]) - (self.group0()[1] * other.group4()[0]) - (self.group1()[0] * other.group1()[1]) + (self.group1()[1] * other.group1()[0])),
            ]) + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))),
            // e23, e31, e12
            (Simd32x3::from([
                ((self.group0()[1] * other.group1()[2]) - (self.group0()[2] * other.group1()[1])),
                (-(self.group0()[0] * other.group1()[2]) + (self.group0()[2] * other.group1()[0])),
                ((self.group0()[0] * other.group1()[1]) - (self.group0()[1] * other.group1()[0])),
            ]) - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[0])
                    + (other.group2()[0] * self.group1()[3])
                    + (other.group2()[1] * self.group0()[2])
                    + (other.group3()[0] * self.group0()[3])
                    + (other.group3()[1] * self.group1()[2])
                    - (other.group3()[2] * self.group1()[1])),
                ((other.group0()[1] * self.group0()[1]) + (other.group2()[1] * self.group1()[3]) + (other.group2()[2] * self.group0()[0]) - (other.group3()[0] * self.group1()[2])
                    + (other.group3()[1] * self.group0()[3])
                    + (other.group3()[2] * self.group1()[0])),
                ((other.group0()[1] * self.group0()[2]) + (other.group2()[0] * self.group0()[1]) + (other.group2()[2] * self.group1()[3]) + (other.group3()[0] * self.group1()[1])
                    - (other.group3()[1] * self.group1()[0])
                    + (other.group3()[2] * self.group0()[3])),
                (-(other.group3()[1] * self.group0()[1]) - (other.group3()[2] * self.group0()[2])),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group1())
                - (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       40       54        0
    //  no simd       40       60        0
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
    //      f32       44       60        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       46       64        0
    //  no simd       52       76        0
    fn sandwich(self, other: Plane) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                (-(self.group0()[2] * other.group0()[1]) - (self.group1()[3] * other.group0()[0])),
                (-(self.group0()[0] * other.group0()[2]) - (self.group1()[3] * other.group0()[1])),
                (-(self.group0()[1] * other.group0()[0]) - (self.group1()[3] * other.group0()[2])),
                ((self.group0()[2] * other.group0()[2]) + (self.group0()[3] * other.group0()[3])),
            ]) + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[1]]) * swizzle!(other.group0(), 3, 3, 3, 1))
                + (swizzle!(self.group0(), 1, 2, 0, 0) * swizzle!(other.group0(), 2, 0, 1, 0))),
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
    //      f32       48       68        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       51       71        0
    //  no simd       60       80        0
    fn sandwich(self, other: Point) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((self.group0()[3] * other.group0()[0]) + (self.group1()[2] * other.group0()[1])),
                ((self.group0()[3] * other.group0()[1]) + (self.group1()[0] * other.group0()[2])),
                ((self.group0()[3] * other.group0()[2]) + (self.group1()[1] * other.group0()[0])),
                (-(self.group1()[2] * other.group0()[2]) - (self.group1()[3] * other.group0()[3])),
            ]) - (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[0]]) * swizzle!(other.group0(), 3, 3, 3, 0))
                - (swizzle!(self.group1(), 1, 2, 0, 1) * swizzle!(other.group0(), 2, 0, 1, 1))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                (-(self.group0()[2] * other.group0()[1]) - (self.group1()[3] * other.group0()[0])),
                (-(self.group0()[0] * other.group0()[2]) - (self.group1()[3] * other.group0()[1])),
                (-(self.group0()[1] * other.group0()[0]) - (self.group1()[3] * other.group0()[2])),
                ((self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2])),
            ]) + (swizzle!(self.group0(), 1, 2, 0, 0) * swizzle!(other.group0(), 2, 0, 1, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       40       54        0
    //  no simd       40       60        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other[scalar]) * self.group0()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[scalar]) * self.group1()),
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
        let geometric_product = Origin::from_groups(/* e4 */ (other[e1234] * self[e321]));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       15        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * self[e321])]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * self[e321])]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       13        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       17        0
    //  no simd        0       29        0
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
        let geometric_product = Scalar::from_groups(/* scalar */ (other[e321] * self[e321] * -1.0));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       19        0
    fn sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([(other.group1()[0] * self[e321]), (other.group1()[1] * self[e321]), (other.group1()[2] * self[e321]), 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([(other.group0()[0] * self[e321]), (other.group0()[1] * self[e321]), (other.group0()[2] * self[e321]), 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       13        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       15        0
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
    //      f32        0       25        0
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       33        0
    //  no simd        0       49        0
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
    //      add/sub      mul      div
    // f32        0       21        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other.group0()[0] * self[e321] * -1.0),
                (other.group0()[1] * self[e321] * -1.0),
                (other.group0()[2] * self[e321] * -1.0),
                0.0,
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * self[e321] * -1.0)]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       21        0
    fn sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * self[e321] * -1.0)]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[0] * self[e321] * -1.0),
                (other.group0()[1] * self[e321] * -1.0),
                (other.group0()[2] * self[e321] * -1.0),
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
    //      f32       19       33        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       19       34        0
    //  no simd       19       36        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Line::from_groups(/* e41, e42, e43 */ (Simd32x3::from(other[e1234]) * self.group1()), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       33        0
    //    simd3        1        3        0
    // Totals...
    // yes simd       20       36        0
    //  no simd       22       42        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            ((Simd32x3::from(other.group0()[0]) * self.group0()) + (Simd32x3::from(other.group0()[1]) * self.group1())),
            // e23, e31, e12
            (Simd32x3::from(other.group0()[0]) * self.group1()),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       70        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       50       72        0
    //  no simd       56       78        0
    fn sandwich(self, other: Flector) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group1()[0] * other.group1()[3]) - (self.group1()[1] * other.group0()[2])),
                ((self.group1()[1] * other.group1()[3]) - (self.group1()[2] * other.group0()[0])),
                (-(self.group1()[0] * other.group0()[1]) + (self.group1()[2] * other.group1()[3])),
                ((self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2])
                    - (self.group1()[0] * other.group1()[0])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])),
            ]) + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                (-(self.group0()[0] * other.group1()[3]) + (self.group0()[1] * other.group0()[2]) + (self.group1()[0] * other.group0()[3])
                    - (self.group1()[1] * other.group1()[2])
                    + (self.group1()[2] * other.group1()[1])),
                (-(self.group0()[1] * other.group1()[3])
                    + (self.group0()[2] * other.group0()[0])
                    + (self.group1()[0] * other.group1()[2])
                    + (self.group1()[1] * other.group0()[3])
                    - (self.group1()[2] * other.group1()[0])),
                ((self.group0()[0] * other.group0()[1]) - (self.group0()[2] * other.group1()[3]) - (self.group1()[0] * other.group1()[1])
                    + (self.group1()[1] * other.group1()[0])
                    + (self.group1()[2] * other.group0()[3])),
                (-(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2])),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       51        0
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
    //      add/sub      mul      div
    // f32       47       69        0
    fn sandwich(self, other: Line) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                ((other.group0()[1] * self.group1()[2]) - (other.group0()[2] * self.group1()[1]) + (other.group1()[1] * self.group0()[2]) - (other.group1()[2] * self.group0()[1])),
                (-(other.group0()[0] * self.group1()[2]) + (other.group0()[2] * self.group1()[0]) - (other.group1()[0] * self.group0()[2])
                    + (other.group1()[2] * self.group0()[0])),
                ((other.group0()[0] * self.group1()[1]) - (other.group0()[1] * self.group1()[0]) + (other.group1()[0] * self.group0()[1]) - (other.group1()[1] * self.group0()[0])),
                (-(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2])),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((other.group1()[1] * self.group1()[2]) - (other.group1()[2] * self.group1()[1])),
                (-(other.group1()[0] * self.group1()[2]) + (other.group1()[2] * self.group1()[0])),
                ((other.group1()[0] * self.group1()[1]) - (other.group1()[1] * self.group1()[0])),
                (-(other.group1()[0] * self.group1()[0]) - (other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2])),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       66        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       47       69        0
    //  no simd       56       78        0
    fn sandwich(self, other: Motor) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((self.group0()[0] * other.group1()[3]) + (self.group0()[2] * other.group1()[1]) + (self.group1()[0] * other.group0()[3]) + (self.group1()[2] * other.group0()[1])),
                ((self.group0()[0] * other.group1()[2]) + (self.group0()[1] * other.group1()[3]) + (self.group1()[0] * other.group0()[2]) + (self.group1()[1] * other.group0()[3])),
                ((self.group0()[1] * other.group1()[0]) + (self.group0()[2] * other.group1()[3]) + (self.group1()[1] * other.group0()[0]) + (self.group1()[2] * other.group0()[3])),
                (-(self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2])),
            ]) - (Simd32x4::from([self.group0()[1], self.group0()[2], self.group0()[0], self.group0()[0]]) * swizzle!(other.group1(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group1()[0]]) * swizzle!(other.group0(), 2, 0, 1, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((self.group1()[0] * other.group1()[3]) + (self.group1()[2] * other.group1()[1])),
                ((self.group1()[0] * other.group1()[2]) + (self.group1()[1] * other.group1()[3])),
                ((self.group1()[1] * other.group1()[0]) + (self.group1()[2] * other.group1()[3])),
                (-(self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2])),
            ]) - (Simd32x4::from([self.group1()[1], self.group1()[2], self.group1()[0], self.group1()[0]]) * swizzle!(other.group1(), 2, 0, 1, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       78      109        0
    //    simd2        3        3        0
    //    simd3        7        9        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       90      123        0
    //  no simd      113      150        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                0.0,
                (-(self.group1()[0] * other.group2()[0]) - (self.group1()[1] * other.group2()[1]) - (self.group1()[2] * other.group2()[2])),
            ]) - (Simd32x2::from(other.group3()[0]) * Simd32x2::from([self.group1()[0], self.group0()[0]]))
                - (Simd32x2::from(other.group3()[1]) * Simd32x2::from([self.group1()[1], self.group0()[1]]))
                - (Simd32x2::from(other.group3()[2]) * Simd32x2::from([self.group1()[2], self.group0()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group1()[0] * other.group4()[3]) - (self.group1()[1] * other.group1()[2])),
                ((self.group1()[1] * other.group4()[3]) - (self.group1()[2] * other.group1()[0])),
                (-(self.group1()[0] * other.group1()[1]) + (self.group1()[2] * other.group4()[3])),
                ((self.group0()[1] * other.group1()[1]) + (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group4()[0])
                    - (self.group1()[1] * other.group4()[1])
                    - (self.group1()[2] * other.group4()[2])),
            ]) + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(other.group1(), 1, 2, 0, 0))),
            // e41, e42, e43
            ((Simd32x3::from(other.group0()[0]) * self.group0()) + (Simd32x3::from(other.group0()[1]) * self.group1())
                - (swizzle!(self.group0(), 1, 2, 0) * swizzle!(other.group3(), 2, 0, 1))
                + (swizzle!(self.group0(), 2, 0, 1) * swizzle!(other.group3(), 1, 2, 0))
                - (swizzle!(self.group1(), 1, 2, 0) * swizzle!(other.group2(), 2, 0, 1))
                + (swizzle!(self.group1(), 2, 0, 1) * swizzle!(other.group2(), 1, 2, 0))),
            // e23, e31, e12
            ((Simd32x3::from(other.group0()[0]) * self.group1()) - (swizzle!(self.group1(), 1, 2, 0) * swizzle!(other.group3(), 2, 0, 1))
                + (swizzle!(self.group1(), 2, 0, 1) * swizzle!(other.group3(), 1, 2, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                (-(self.group0()[0] * other.group4()[3]) + (self.group0()[1] * other.group1()[2]) + (self.group1()[0] * other.group1()[3])
                    - (self.group1()[1] * other.group4()[2])
                    + (self.group1()[2] * other.group4()[1])),
                (-(self.group0()[1] * other.group4()[3])
                    + (self.group0()[2] * other.group1()[0])
                    + (self.group1()[0] * other.group4()[2])
                    + (self.group1()[1] * other.group1()[3])
                    - (self.group1()[2] * other.group4()[0])),
                ((self.group0()[0] * other.group1()[1]) - (self.group0()[2] * other.group4()[3]) - (self.group1()[0] * other.group4()[1])
                    + (self.group1()[1] * other.group4()[0])
                    + (self.group1()[2] * other.group1()[3])),
                (-(self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2])),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group1(), 1, 2, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       24        0
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
    //      add/sub      mul      div
    // f32       36       57        0
    fn sandwich(self, other: Plane) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] * other.group0()[3]),
                (self.group1()[1] * other.group0()[3]),
                (self.group1()[2] * other.group0()[3]),
                (-(self.group1()[0] * other.group0()[0]) - (self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (-(self.group0()[0] * other.group0()[3]) - (self.group1()[1] * other.group0()[2]) + (self.group1()[2] * other.group0()[1])),
                (-(self.group0()[1] * other.group0()[3]) + (self.group1()[0] * other.group0()[2]) - (self.group1()[2] * other.group0()[0])),
                (-(self.group0()[2] * other.group0()[3]) - (self.group1()[0] * other.group0()[1]) + (self.group1()[1] * other.group0()[0])),
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
    //      f32       33       58        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       35       60        0
    //  no simd       41       66        0
    fn sandwich(self, other: Point) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group1()[1] * other.group0()[2]) * -1.0),
                ((self.group1()[2] * other.group0()[0]) * -1.0),
                ((self.group1()[0] * other.group0()[1]) * -1.0),
                ((self.group0()[1] * other.group0()[1]) + (self.group0()[2] * other.group0()[2])),
            ]) + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((self.group0()[1] * other.group0()[2]) + (self.group1()[0] * other.group0()[3])),
                ((self.group0()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3])),
                ((self.group0()[0] * other.group0()[1]) + (self.group1()[2] * other.group0()[3])),
                (-(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2])),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       33        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       19       35        0
    //  no simd       19       39        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group0()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group1()),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl InfixSandwich for Motor {}
impl Sandwich<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       54        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       40       55        0
    //  no simd       40       58        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[e1234]) * self.group1()),
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
    //      f32       40       54        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       41       57        0
    //  no simd       44       66        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            ((Simd32x4::from(other.group0()[0]) * self.group0()) + (Simd32x4::from(other.group0()[1]) * self.group1())),
            // e23, e31, e12, scalar
            (Simd32x4::from(other.group0()[0]) * self.group1()),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       70        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       60       78        0
    //  no simd       84      102        0
    fn sandwich(self, other: Flector) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                0.0,
                0.0,
                0.0,
                ((other.group0()[2] * self.group0()[2]) - (other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2]) - (other.group1()[3] * self.group0()[3])),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[3], other.group0()[3]]) * self.group1())
                + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[3], self.group0()[1]]) * swizzle!(other.group0(), 1, 2, 2, 1))
                + (Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[1], self.group0()[0]]) * swizzle!(other.group0(), 0, 1, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[2] * self.group0()[1]) + (other.group1()[0] * self.group1()[3]) + (other.group1()[1] * self.group1()[2]) - (other.group1()[3] * self.group0()[0])),
                ((other.group0()[0] * self.group0()[2]) + (other.group1()[1] * self.group1()[3]) + (other.group1()[2] * self.group1()[0]) - (other.group1()[3] * self.group0()[1])),
                ((other.group0()[1] * self.group0()[0]) + (other.group1()[0] * self.group1()[1]) + (other.group1()[2] * self.group1()[3]) - (other.group1()[3] * self.group0()[2])),
                0.0,
            ]) + (Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[3]]) * self.group1())
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[2]]) * swizzle!(self.group1(), 1, 2, 0, 2))
                - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[3], self.group1()[1]]) * swizzle!(other.group0(), 1, 2, 2, 1))
                - (Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[1], self.group1()[0]]) * swizzle!(other.group0(), 0, 1, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       54        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       40       58        0
    //  no simd       40       70        0
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
    //      f32       56       78        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       59       81        0
    //  no simd       68       90        0
    fn sandwich(self, other: Line) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((other.group0()[0] * self.group1()[3]) + (other.group0()[1] * self.group1()[2]) + (other.group1()[0] * self.group0()[3]) + (other.group1()[1] * self.group0()[2])),
                ((other.group0()[1] * self.group1()[3]) + (other.group0()[2] * self.group1()[0]) + (other.group1()[1] * self.group0()[3]) + (other.group1()[2] * self.group0()[0])),
                ((other.group0()[0] * self.group1()[1]) + (other.group0()[2] * self.group1()[3]) + (other.group1()[0] * self.group0()[1]) + (other.group1()[2] * self.group0()[3])),
                (-(other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group0()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((other.group1()[0] * self.group1()[3]) + (other.group1()[1] * self.group1()[2])),
                ((other.group1()[1] * self.group1()[3]) + (other.group1()[2] * self.group1()[0])),
                ((other.group1()[0] * self.group1()[1]) + (other.group1()[2] * self.group1()[3])),
                (-(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group1()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       78        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       62       84        0
    //  no simd       80      102        0
    fn sandwich(self, other: Motor) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((other.group0()[1] * self.group1()[2]) + (other.group0()[3] * self.group1()[0]) + (other.group1()[1] * self.group0()[2]) + (other.group1()[3] * self.group0()[0])),
                ((other.group0()[2] * self.group1()[0]) + (other.group0()[3] * self.group1()[1]) + (other.group1()[2] * self.group0()[0]) + (other.group1()[3] * self.group0()[1])),
                ((other.group0()[2] * self.group1()[3]) + (other.group0()[3] * self.group1()[2]) + (other.group1()[2] * self.group0()[3]) + (other.group1()[3] * self.group0()[2])),
                (-(other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2])),
            ]) + (swizzle!(other.group0(), 0, 1, 0, 3) * swizzle!(self.group1(), 3, 3, 1, 3))
                - (swizzle!(other.group0(), 2, 0, 1, 0) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (swizzle!(other.group1(), 0, 1, 0, 3) * swizzle!(self.group0(), 3, 3, 1, 3))
                - (swizzle!(other.group1(), 2, 0, 1, 0) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((other.group1()[1] * self.group1()[2]) + (other.group1()[3] * self.group1()[0])),
                ((other.group1()[2] * self.group1()[0]) + (other.group1()[3] * self.group1()[1])),
                ((other.group1()[2] * self.group1()[3]) + (other.group1()[3] * self.group1()[2])),
                (-(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2])),
            ]) + (swizzle!(other.group1(), 0, 1, 0, 3) * swizzle!(self.group1(), 3, 3, 1, 3))
                - (swizzle!(other.group1(), 2, 0, 1, 0) * swizzle!(self.group1(), 1, 2, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       96      126        0
    //    simd2        4        4        0
    //    simd3       10       12        0
    //    simd4        7        7        0
    // Totals...
    // yes simd      117      149        0
    //  no simd      162      198        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                0.0,
                ((other.group0()[1] * self.group1()[3]) - (other.group3()[0] * self.group0()[0]) - (other.group3()[1] * self.group0()[1]) - (other.group3()[2] * self.group0()[2])),
            ]) + (Simd32x2::from(other.group0()[0]) * Simd32x2::from([self.group1()[3], self.group0()[3]]))
                - (Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]))
                - (Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]))
                - (Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (self.group1()[3] * other.group1()[0]),
                (self.group1()[1] * other.group4()[3]),
                (self.group1()[2] * other.group4()[3]),
                ((self.group0()[1] * other.group1()[1]) + (self.group0()[2] * other.group1()[2])
                    - (self.group0()[3] * other.group4()[3])
                    - (self.group1()[1] * other.group4()[1])
                    - (self.group1()[2] * other.group4()[2])),
            ]) + (Simd32x4::from([self.group1()[2], self.group1()[3], self.group1()[3], self.group0()[0]]) * swizzle!(other.group1(), 1, 1, 2, 0))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group4()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([other.group4()[3], other.group1()[2], other.group1()[0], other.group1()[3]]) * swizzle!(self.group1(), 0, 0, 1, 3))),
            // e41, e42, e43
            ((Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[0]]) * swizzle!(other.group3(), 2, 0, 1))
                + (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[3]]) * swizzle!(other.group3(), 1, 2, 2))
                + (Simd32x3::from([self.group0()[3], self.group0()[3], self.group0()[1]]) * swizzle!(other.group3(), 0, 1, 0))
                - (Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]) * swizzle!(other.group2(), 2, 0, 1))
                + (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[3]]) * swizzle!(other.group2(), 1, 2, 2))
                + (Simd32x3::from([self.group1()[3], self.group1()[3], self.group1()[1]]) * swizzle!(other.group2(), 0, 1, 0))),
            // e23, e31, e12
            ((Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (Simd32x3::from([self.group1()[1], self.group1()[2], self.group1()[0]]) * swizzle!(other.group3(), 2, 0, 1))
                + (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[3]]) * swizzle!(other.group3(), 1, 2, 2))
                + (Simd32x3::from([self.group1()[3], self.group1()[3], self.group1()[1]]) * swizzle!(other.group3(), 0, 1, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                (-(self.group0()[0] * other.group4()[3])
                    + (self.group0()[1] * other.group1()[2])
                    + (self.group1()[2] * other.group4()[1])
                    + (self.group1()[3] * other.group4()[0])),
                (-(self.group0()[1] * other.group4()[3])
                    + (self.group0()[2] * other.group1()[0])
                    + (self.group1()[1] * other.group1()[3])
                    + (self.group1()[3] * other.group4()[1])),
                ((self.group0()[0] * other.group1()[1]) - (self.group0()[2] * other.group4()[3]) + (self.group1()[2] * other.group1()[3]) + (self.group1()[3] * other.group4()[2])),
                0.0,
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[1]]) * swizzle!(other.group1(), 0, 1, 2, 1))
                + (Simd32x4::from([other.group1()[3], other.group4()[2], other.group4()[0], other.group4()[3]]) * swizzle!(self.group1(), 0, 0, 1, 3))
                - (Simd32x4::from([other.group4()[2], other.group4()[0], other.group4()[1], other.group1()[2]]) * swizzle!(self.group1(), 1, 2, 0, 2))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       40       58        0
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
    //      add/sub      mul      div
    // f32       52       74        0
    fn sandwich(self, other: Plane) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] * other.group0()[3]),
                (self.group1()[1] * other.group0()[3]),
                (self.group1()[2] * other.group0()[3]),
                (-(self.group0()[3] * other.group0()[3])
                    - (self.group1()[0] * other.group0()[0])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (-(self.group0()[0] * other.group0()[3]) - (self.group1()[1] * other.group0()[2])
                    + (self.group1()[2] * other.group0()[1])
                    + (self.group1()[3] * other.group0()[0])),
                (-(self.group0()[1] * other.group0()[3]) + (self.group1()[0] * other.group0()[2]) - (self.group1()[2] * other.group0()[0])
                    + (self.group1()[3] * other.group0()[1])),
                (-(self.group0()[2] * other.group0()[3]) - (self.group1()[0] * other.group0()[1])
                    + (self.group1()[1] * other.group0()[0])
                    + (self.group1()[3] * other.group0()[2])),
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
    //      f32       44       70        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       48       74        0
    //  no simd       60       86        0
    fn sandwich(self, other: Point) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group1()[1] * other.group0()[2]) * -1.0),
                ((self.group1()[2] * other.group0()[0]) * -1.0),
                ((self.group1()[0] * other.group0()[1]) * -1.0),
                ((self.group0()[2] * other.group0()[2]) + (self.group1()[3] * other.group0()[3])),
            ]) + (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group1()[3], self.group1()[3], self.group1()[3], self.group0()[1]]) * swizzle!(other.group0(), 0, 1, 2, 1))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((self.group0()[1] * other.group0()[2]) + (self.group1()[0] * other.group0()[3])),
                ((self.group0()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3])),
                ((self.group0()[0] * other.group0()[1]) + (self.group1()[2] * other.group0()[3])),
                ((self.group1()[2] * other.group0()[2]) * -1.0),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[1]]) * swizzle!(other.group0(), 0, 1, 2, 1))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       54        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       40       56        0
    //  no simd       40       62        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[scalar]) * self.group0()),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[scalar]) * self.group1()),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl InfixSandwich for MultiVector {}
impl Sandwich<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      176      207        0
    //    simd3        0        1        0
    // Totals...
    // yes simd      176      208        0
    //  no simd      176      210        0
    fn sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (self.group0()[0] * other[e1234])]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group4()[3] * other[e1234])]),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group3()),
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
    //      f32      181      217        0
    //    simd3        1        3        0
    // Totals...
    // yes simd      182      220        0
    //  no simd      184      226        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other.group0()[0] * self.group0()[0]),
                ((other.group0()[0] * self.group0()[1]) + (other.group0()[1] * self.group0()[0])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[0] * self.group1()[0]),
                (other.group0()[0] * self.group1()[1]),
                (other.group0()[0] * self.group1()[2]),
                ((other.group0()[0] * self.group1()[3]) + (other.group0()[1] * self.group4()[3])),
            ]),
            // e41, e42, e43
            ((Simd32x3::from(other.group0()[0]) * self.group2()) + (Simd32x3::from(other.group0()[1]) * self.group3())),
            // e23, e31, e12
            (Simd32x3::from(other.group0()[0]) * self.group3()),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((other.group0()[0] * self.group4()[0]) + (other.group0()[1] * self.group1()[0])),
                ((other.group0()[0] * self.group4()[1]) + (other.group0()[1] * self.group1()[1])),
                ((other.group0()[0] * self.group4()[2]) + (other.group0()[1] * self.group1()[2])),
                (other.group0()[0] * self.group4()[3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      211      253        0
    //    simd2        4        4        0
    //    simd3        6        6        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      226      268        0
    //  no simd      257      299        0
    fn sandwich(self, other: Flector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                0.0,
                (-(other.group0()[0] * self.group4()[0]) - (other.group0()[1] * self.group4()[1]) - (other.group0()[2] * self.group4()[2])
                    + (other.group1()[3] * self.group1()[3])),
            ]) + (Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group0()[0], other.group1()[0]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group0()[1], other.group1()[1]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group0()[2], other.group1()[2]]))
                - (Simd32x2::from(self.group4()[3]) * Simd32x2::from([other.group1()[3], other.group0()[3]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group3()[0] * other.group1()[3]) - (self.group3()[1] * other.group0()[2])),
                ((self.group3()[1] * other.group1()[3]) - (self.group3()[2] * other.group0()[0])),
                (-(self.group3()[0] * other.group0()[1]) + (self.group3()[2] * other.group1()[3])),
                (-(self.group0()[1] * other.group1()[3]) + (self.group2()[1] * other.group0()[1]) + (self.group2()[2] * other.group0()[2])
                    - (self.group3()[0] * other.group1()[0])
                    - (self.group3()[1] * other.group1()[1])
                    - (self.group3()[2] * other.group1()[2])),
            ]) + (Simd32x4::from(self.group0()[0]) * other.group0())
                + (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
            // e41, e42, e43
            (Simd32x3::from([
                ((other.group0()[1] * self.group4()[2]) - (other.group0()[2] * self.group4()[1]) - (other.group1()[1] * self.group1()[2]) + (other.group1()[2] * self.group1()[1])),
                (-(other.group0()[0] * self.group4()[2]) + (other.group0()[2] * self.group4()[0]) + (other.group1()[0] * self.group1()[2])
                    - (other.group1()[2] * self.group1()[0])),
                ((other.group0()[0] * self.group4()[1]) - (other.group0()[1] * self.group4()[0]) - (other.group1()[0] * self.group1()[1]) + (other.group1()[1] * self.group1()[0])),
            ]) - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))),
            // e23, e31, e12
            (Simd32x3::from([
                (-(other.group0()[1] * self.group1()[2]) + (other.group0()[2] * self.group1()[1])),
                ((other.group0()[0] * self.group1()[2]) - (other.group0()[2] * self.group1()[0])),
                (-(other.group0()[0] * self.group1()[1]) + (other.group0()[1] * self.group1()[0])),
            ]) - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                (-(self.group2()[0] * other.group1()[3]) + (self.group2()[1] * other.group0()[2]) + (self.group3()[0] * other.group0()[3])
                    - (self.group3()[1] * other.group1()[2])
                    + (self.group3()[2] * other.group1()[1])),
                (-(self.group2()[1] * other.group1()[3])
                    + (self.group2()[2] * other.group0()[0])
                    + (self.group3()[0] * other.group1()[2])
                    + (self.group3()[1] * other.group0()[3])
                    - (self.group3()[2] * other.group1()[0])),
                ((self.group2()[0] * other.group0()[1]) - (self.group2()[2] * other.group1()[3]) - (self.group3()[0] * other.group1()[1])
                    + (self.group3()[1] * other.group1()[0])
                    + (self.group3()[2] * other.group0()[3])),
                ((self.group3()[2] * other.group0()[2]) * -1.0),
            ]) + (Simd32x4::from(self.group0()[0]) * other.group1())
                - (Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group3()[0]]) * swizzle!(other.group0(), 0, 1, 2, 0))
                - (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[1]]) * swizzle!(other.group0(), 1, 2, 0, 1))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      176      202        0
    //    simd2        0        2        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd      176      211        0
    //  no simd      176      231        0
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
    //      f32      198      233        0
    //    simd2        3        3        0
    //    simd3        7        9        0
    //    simd4        2        2        0
    // Totals...
    // yes simd      210      247        0
    //  no simd      233      274        0
    fn sandwich(self, other: Line) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                0.0,
                (-(other.group1()[0] * self.group2()[0]) - (other.group1()[1] * self.group2()[1]) - (other.group1()[2] * self.group2()[2])),
            ]) - (Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]))
                - (Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]))
                - (Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group1()[0] * self.group4()[3]) + (other.group1()[1] * self.group1()[2])),
                ((other.group1()[1] * self.group4()[3]) + (other.group1()[2] * self.group1()[0])),
                ((other.group1()[0] * self.group1()[1]) + (other.group1()[2] * self.group4()[3])),
                (-(other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group4()[0])
                    - (other.group1()[1] * self.group4()[1])
                    - (other.group1()[2] * self.group4()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))),
            // e41, e42, e43
            ((Simd32x3::from(self.group0()[0]) * other.group0())
                + (Simd32x3::from(self.group0()[1]) * other.group1())
                + (swizzle!(other.group0(), 1, 2, 0) * swizzle!(self.group3(), 2, 0, 1))
                - (swizzle!(other.group0(), 2, 0, 1) * swizzle!(self.group3(), 1, 2, 0))
                + (swizzle!(other.group1(), 1, 2, 0) * swizzle!(self.group2(), 2, 0, 1))
                - (swizzle!(other.group1(), 2, 0, 1) * swizzle!(self.group2(), 1, 2, 0))),
            // e23, e31, e12
            ((Simd32x3::from(self.group0()[0]) * other.group1()) + (swizzle!(other.group1(), 1, 2, 0) * swizzle!(self.group3(), 2, 0, 1))
                - (swizzle!(other.group1(), 2, 0, 1) * swizzle!(self.group3(), 1, 2, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[0] * self.group4()[3]) + (other.group0()[1] * self.group1()[2]) + (other.group1()[0] * self.group1()[3]) + (other.group1()[1] * self.group4()[2])
                    - (other.group1()[2] * self.group4()[1])),
                ((other.group0()[1] * self.group4()[3]) + (other.group0()[2] * self.group1()[0]) - (other.group1()[0] * self.group4()[2])
                    + (other.group1()[1] * self.group1()[3])
                    + (other.group1()[2] * self.group4()[0])),
                ((other.group0()[0] * self.group1()[1]) + (other.group0()[2] * self.group4()[3]) + (other.group1()[0] * self.group4()[1]) - (other.group1()[1] * self.group4()[0])
                    + (other.group1()[2] * self.group1()[3])),
                (-(other.group1()[1] * self.group1()[1]) - (other.group1()[2] * self.group1()[2])),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      199      235        0
    //    simd2        4        4        0
    //    simd3       10       12        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      218      256        0
    //  no simd      257      299        0
    fn sandwich(self, other: Motor) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                0.0,
                ((self.group0()[1] * other.group1()[3]) - (self.group3()[0] * other.group0()[0]) - (self.group3()[1] * other.group0()[1]) - (self.group3()[2] * other.group0()[2])),
            ]) + (Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[3], other.group0()[3]]))
                - (Simd32x2::from(other.group1()[0]) * Simd32x2::from([self.group3()[0], self.group2()[0]]))
                - (Simd32x2::from(other.group1()[1]) * Simd32x2::from([self.group3()[1], self.group2()[1]]))
                - (Simd32x2::from(other.group1()[2]) * Simd32x2::from([self.group3()[2], self.group2()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group1()[1] * self.group1()[2]) + (other.group1()[3] * self.group1()[0])),
                ((other.group1()[2] * self.group1()[0]) + (other.group1()[3] * self.group1()[1])),
                ((other.group1()[2] * self.group4()[3]) + (other.group1()[3] * self.group1()[2])),
                (-(other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2]) + (other.group0()[3] * self.group4()[3])
                    - (other.group1()[0] * self.group4()[0])
                    - (other.group1()[1] * self.group4()[1])
                    - (other.group1()[2] * self.group4()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group4()[3], self.group4()[3], self.group1()[1], self.group1()[3]]) * swizzle!(other.group1(), 0, 1, 0, 3))),
            // e41, e42, e43
            ((Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + (Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([other.group0()[1], other.group0()[3], other.group0()[3]]) * swizzle!(self.group3(), 2, 1, 2))
                - (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[1]]) * swizzle!(self.group3(), 1, 2, 0))
                + (Simd32x3::from([other.group0()[3], other.group0()[2], other.group0()[0]]) * swizzle!(self.group3(), 0, 0, 1))
                + (Simd32x3::from([other.group1()[1], other.group1()[3], other.group1()[3]]) * swizzle!(self.group2(), 2, 1, 2))
                - (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]) * swizzle!(self.group2(), 1, 2, 0))
                + (Simd32x3::from([other.group1()[3], other.group1()[2], other.group1()[0]]) * swizzle!(self.group2(), 0, 0, 1))),
            // e23, e31, e12
            ((Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (Simd32x3::from([other.group1()[1], other.group1()[3], other.group1()[3]]) * swizzle!(self.group3(), 2, 1, 2))
                - (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[1]]) * swizzle!(self.group3(), 1, 2, 0))
                + (Simd32x3::from([other.group1()[3], other.group1()[2], other.group1()[0]]) * swizzle!(self.group3(), 0, 0, 1))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[1] * self.group1()[2])
                    + (other.group0()[3] * self.group1()[0])
                    + (other.group1()[0] * self.group1()[3])
                    + (other.group1()[1] * self.group4()[2])
                    + (other.group1()[3] * self.group4()[0])),
                ((other.group0()[2] * self.group1()[0])
                    + (other.group0()[3] * self.group1()[1])
                    + (other.group1()[1] * self.group1()[3])
                    + (other.group1()[2] * self.group4()[0])
                    + (other.group1()[3] * self.group4()[1])),
                ((other.group0()[0] * self.group1()[1])
                    + (other.group0()[3] * self.group1()[2])
                    + (other.group1()[0] * self.group4()[1])
                    + (other.group1()[2] * self.group1()[3])
                    + (other.group1()[3] * self.group4()[2])),
                ((other.group1()[2] * self.group1()[2]) * -1.0),
            ]) + (Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]))
                - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group4()[1], self.group4()[2], self.group4()[0], self.group1()[1]]) * swizzle!(other.group1(), 2, 0, 1, 1))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      243      284        0
    //    simd2        8        8        0
    //    simd3       18       18        0
    //    simd4       10       10        0
    // Totals...
    // yes simd      279      320        0
    //  no simd      353      394        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                0.0,
                ((other.group0()[1] * self.group0()[0])
                    - (other.group3()[0] * self.group2()[0])
                    - (other.group3()[1] * self.group2()[1])
                    - (other.group3()[2] * self.group2()[2])
                    - (other.group1()[0] * self.group4()[0])
                    - (other.group1()[1] * self.group4()[1])
                    - (other.group1()[2] * self.group4()[2])
                    + (other.group4()[3] * self.group1()[3])),
            ]) + (Simd32x2::from(other.group0()[0]) * self.group0())
                - (Simd32x2::from(self.group3()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]))
                - (Simd32x2::from(self.group3()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]))
                - (Simd32x2::from(self.group3()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]))
                + (Simd32x2::from(self.group1()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]))
                + (Simd32x2::from(self.group1()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]))
                + (Simd32x2::from(self.group1()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]))
                - (Simd32x2::from(self.group4()[3]) * Simd32x2::from([other.group4()[3], other.group1()[3]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group3()[1] * self.group1()[2]) + (self.group3()[0] * other.group4()[3]) - (self.group3()[1] * other.group1()[2])),
                ((other.group3()[2] * self.group1()[0]) + (self.group3()[1] * other.group4()[3]) - (self.group3()[2] * other.group1()[0])),
                ((other.group3()[0] * self.group1()[1]) - (self.group3()[0] * other.group1()[1]) + (self.group3()[2] * other.group4()[3])),
                (-(self.group0()[1] * other.group4()[3])
                    - (other.group2()[1] * self.group1()[1])
                    - (other.group2()[2] * self.group1()[2])
                    - (other.group3()[0] * self.group4()[0])
                    - (other.group3()[1] * self.group4()[1])
                    - (other.group3()[2] * self.group4()[2])
                    + (self.group2()[1] * other.group1()[1])
                    + (self.group2()[2] * other.group1()[2])
                    - (self.group3()[0] * other.group4()[0])
                    - (self.group3()[1] * other.group4()[1])
                    - (self.group3()[2] * other.group4()[2])),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group1())
                + (Simd32x4::from(self.group0()[0]) * other.group1())
                + (Simd32x4::from(self.group4()[3]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]))
                - (Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[0]]) * swizzle!(other.group1(), 1, 2, 0, 0))),
            // e41, e42, e43
            (Simd32x3::from([
                ((other.group1()[1] * self.group4()[2]) - (other.group1()[2] * self.group4()[1]) - (other.group4()[1] * self.group1()[2]) + (other.group4()[2] * self.group1()[1])),
                (-(other.group1()[0] * self.group4()[2]) + (other.group1()[2] * self.group4()[0]) + (other.group4()[0] * self.group1()[2])
                    - (other.group4()[2] * self.group1()[0])),
                ((other.group1()[0] * self.group4()[1]) - (other.group1()[1] * self.group4()[0]) - (other.group4()[0] * self.group1()[1]) + (other.group4()[1] * self.group1()[0])),
            ]) + (Simd32x3::from(other.group0()[0]) * self.group2())
                + (Simd32x3::from(other.group0()[1]) * self.group3())
                + (Simd32x3::from(self.group0()[0]) * other.group2())
                + (Simd32x3::from(self.group0()[1]) * other.group3())
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                + (swizzle!(other.group2(), 1, 2, 0) * swizzle!(self.group3(), 2, 0, 1))
                - (swizzle!(other.group2(), 2, 0, 1) * swizzle!(self.group3(), 1, 2, 0))
                + (swizzle!(other.group3(), 1, 2, 0) * swizzle!(self.group2(), 2, 0, 1))
                - (swizzle!(other.group3(), 2, 0, 1) * swizzle!(self.group2(), 1, 2, 0))),
            // e23, e31, e12
            (Simd32x3::from([
                (-(other.group1()[1] * self.group1()[2]) + (other.group1()[2] * self.group1()[1])),
                ((other.group1()[0] * self.group1()[2]) - (other.group1()[2] * self.group1()[0])),
                (-(other.group1()[0] * self.group1()[1]) + (other.group1()[1] * self.group1()[0])),
            ]) + (Simd32x3::from(other.group0()[0]) * self.group3())
                + (Simd32x3::from(self.group0()[0]) * other.group3())
                - (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                + (swizzle!(other.group3(), 1, 2, 0) * swizzle!(self.group3(), 2, 0, 1))
                - (swizzle!(other.group3(), 2, 0, 1) * swizzle!(self.group3(), 1, 2, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[1] * self.group1()[0])
                    + (other.group2()[0] * self.group4()[3])
                    + (other.group2()[1] * self.group1()[2])
                    + (other.group3()[0] * self.group1()[3])
                    + (other.group3()[1] * self.group4()[2])
                    - (other.group3()[2] * self.group4()[1])
                    - (self.group2()[0] * other.group4()[3])
                    + (self.group2()[1] * other.group1()[2])
                    + (self.group3()[0] * other.group1()[3])
                    - (self.group3()[1] * other.group4()[2])
                    + (self.group3()[2] * other.group4()[1])),
                ((other.group0()[1] * self.group1()[1]) + (other.group2()[1] * self.group4()[3]) + (other.group2()[2] * self.group1()[0]) - (other.group3()[0] * self.group4()[2])
                    + (other.group3()[1] * self.group1()[3])
                    + (other.group3()[2] * self.group4()[0])
                    - (self.group2()[1] * other.group4()[3])
                    + (self.group2()[2] * other.group1()[0])
                    + (self.group3()[0] * other.group4()[2])
                    + (self.group3()[1] * other.group1()[3])
                    - (self.group3()[2] * other.group4()[0])),
                ((other.group0()[1] * self.group1()[2]) + (other.group2()[0] * self.group1()[1]) + (other.group2()[2] * self.group4()[3]) + (other.group3()[0] * self.group4()[1])
                    - (other.group3()[1] * self.group4()[0])
                    + (other.group3()[2] * self.group1()[3])
                    + (self.group2()[0] * other.group1()[1])
                    - (self.group2()[2] * other.group4()[3])
                    - (self.group3()[0] * other.group4()[1])
                    + (self.group3()[1] * other.group4()[0])
                    + (self.group3()[2] * other.group1()[3])),
                (-(other.group3()[1] * self.group1()[1]) - (other.group3()[2] * self.group1()[2]) - (self.group3()[2] * other.group1()[2])),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group4())
                + (Simd32x4::from(self.group0()[0]) * other.group4())
                - (Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group3()[0]]) * swizzle!(other.group1(), 0, 1, 2, 0))
                - (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[1]]) * swizzle!(other.group1(), 1, 2, 0, 1))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      176      208        0
    //    simd3        0        2        0
    // Totals...
    // yes simd      176      210        0
    //  no simd      176      214        0
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
    //      f32      194      234        0
    //    simd3        2        4        0
    // Totals...
    // yes simd      196      238        0
    //  no simd      200      246        0
    fn sandwich(self, other: Plane) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (self.group4()[3] * other.group0()[3] * -1.0),
                ((self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2]) + (self.group1()[3] * other.group0()[3])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group3()[0] * other.group0()[3]),
                (self.group3()[1] * other.group0()[3]),
                (self.group3()[2] * other.group0()[3]),
                (-(self.group0()[1] * other.group0()[3])
                    - (self.group3()[0] * other.group0()[0])
                    - (self.group3()[1] * other.group0()[1])
                    - (self.group3()[2] * other.group0()[2])),
            ]),
            // e41, e42, e43
            (Simd32x3::from([
                ((self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1])),
                (-(self.group1()[0] * other.group0()[2]) + (self.group1()[2] * other.group0()[0])),
                ((self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0])),
            ]) - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))),
            // e23, e31, e12
            (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((self.group0()[0] * other.group0()[0]) - (self.group2()[0] * other.group0()[3]) - (self.group3()[1] * other.group0()[2]) + (self.group3()[2] * other.group0()[1])),
                ((self.group0()[0] * other.group0()[1]) - (self.group2()[1] * other.group0()[3]) + (self.group3()[0] * other.group0()[2]) - (self.group3()[2] * other.group0()[0])),
                ((self.group0()[0] * other.group0()[2]) - (self.group2()[2] * other.group0()[3]) - (self.group3()[0] * other.group0()[1]) + (self.group3()[1] * other.group0()[0])),
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
    //      f32      191      237        0
    //    simd3        3        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd      198      244        0
    //  no simd      216      262        0
    fn sandwich(self, other: Point) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                ((self.group1()[0] * other.group0()[0]) + (self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2])),
                (-(self.group4()[0] * other.group0()[0])
                    - (self.group4()[1] * other.group0()[1])
                    - (self.group4()[2] * other.group0()[2])
                    - (self.group4()[3] * other.group0()[3])),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group3()[1] * other.group0()[2]) * -1.0),
                ((self.group3()[2] * other.group0()[0]) * -1.0),
                ((self.group3()[0] * other.group0()[1]) * -1.0),
                ((self.group2()[1] * other.group0()[1]) + (self.group2()[2] * other.group0()[2])),
            ]) + (Simd32x4::from(self.group0()[0]) * other.group0())
                + (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
            // e41, e42, e43
            (Simd32x3::from([
                (-(self.group4()[1] * other.group0()[2]) + (self.group4()[2] * other.group0()[1])),
                ((self.group4()[0] * other.group0()[2]) - (self.group4()[2] * other.group0()[0])),
                (-(self.group4()[0] * other.group0()[1]) + (self.group4()[1] * other.group0()[0])),
            ]) + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))),
            // e23, e31, e12
            (Simd32x3::from([
                ((self.group1()[1] * other.group0()[2]) - (self.group1()[2] * other.group0()[1])),
                (-(self.group1()[0] * other.group0()[2]) + (self.group1()[2] * other.group0()[0])),
                ((self.group1()[0] * other.group0()[1]) - (self.group1()[1] * other.group0()[0])),
            ]) - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((self.group2()[1] * other.group0()[2]) + (self.group3()[0] * other.group0()[3])),
                ((self.group2()[2] * other.group0()[0]) + (self.group3()[1] * other.group0()[3])),
                ((self.group2()[0] * other.group0()[1]) + (self.group3()[2] * other.group0()[3])),
                ((self.group3()[2] * other.group0()[2]) * -1.0),
            ]) - (Simd32x4::from([self.group0()[1], self.group0()[1], self.group0()[1], self.group3()[0]]) * swizzle!(other.group0(), 0, 1, 2, 0))
                - (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[1]]) * swizzle!(other.group0(), 1, 2, 0, 1))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      176      202        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd      176      207        0
    //  no simd      176      218        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from(other[scalar]) * self.group0()),
            // e1, e2, e3, e4
            (Simd32x4::from(other[scalar]) * self.group1()),
            // e41, e42, e43
            (Simd32x3::from(other[scalar]) * self.group2()),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group3()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[scalar]) * self.group4()),
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
            Simd32x4::from([(other.group1()[0] * self[e4]), (other.group1()[1] * self[e4]), (other.group1()[2] * self[e4]), 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * self[e4])]),
            // e423, e431, e412, e321
            Simd32x4::from([(other.group1()[0] * self[e4]), (other.group1()[1] * self[e4]), (other.group1()[2] * self[e4]), 0.0]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       17        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0       18        0
    //  no simd        0       20        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, (other.group4()[3] * self[e4])]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * self[e4])]),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([(other.group3()[0] * self[e4]), (other.group3()[1] * self[e4]), (other.group3()[2] * self[e4]), 0.0]),
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
    //      add/sub      mul      div
    // f32        0        6        0
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
    //      f32       12       29        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       30        0
    //  no simd       12       33        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * self.group0()[3])]),
            // e423, e431, e412, e321
            (Simd32x4::from(other.group0()[0]) * self.group0()),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       18       36        0
    //  no simd       24       48        0
    fn sandwich(self, other: Flector) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[2]) + (other.group1()[3] * self.group0()[0])),
                ((other.group0()[2] * self.group0()[0]) + (other.group1()[3] * self.group0()[1])),
                ((other.group0()[0] * self.group0()[1]) + (other.group1()[3] * self.group0()[2])),
                (-(other.group0()[2] * self.group0()[2]) - (other.group0()[3] * self.group0()[3])),
            ]) - (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[1]]) * swizzle!(self.group0(), 3, 3, 3, 1))
                - (swizzle!(other.group0(), 2, 0, 1, 0) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       29        0
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
    //      add/sub      mul      div
    // f32       20       43        0
    fn sandwich(self, other: Line) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group1()[0] * self.group0()[3]),
                (other.group1()[1] * self.group0()[3]),
                (other.group1()[2] * self.group0()[3]),
                (-(other.group1()[0] * self.group0()[0]) - (other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((other.group0()[0] * self.group0()[3]) + (other.group1()[1] * self.group0()[2]) - (other.group1()[2] * self.group0()[1])),
                ((other.group0()[1] * self.group0()[3]) - (other.group1()[0] * self.group0()[2]) + (other.group1()[2] * self.group0()[0])),
                ((other.group0()[2] * self.group0()[3]) + (other.group1()[0] * self.group0()[1]) - (other.group1()[1] * self.group0()[0])),
                0.0,
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       48        0
    fn sandwich(self, other: Motor) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group1()[0] * self.group0()[3]),
                (other.group1()[1] * self.group0()[3]),
                (other.group1()[2] * self.group0()[3]),
                ((other.group0()[3] * self.group0()[3]) - (other.group1()[0] * self.group0()[0]) - (other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2])),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((other.group0()[0] * self.group0()[3]) + (other.group1()[1] * self.group0()[2]) - (other.group1()[2] * self.group0()[1]) + (other.group1()[3] * self.group0()[0])),
                ((other.group0()[1] * self.group0()[3]) - (other.group1()[0] * self.group0()[2]) + (other.group1()[2] * self.group0()[0]) + (other.group1()[3] * self.group0()[1])),
                ((other.group0()[2] * self.group0()[3]) + (other.group1()[0] * self.group0()[1]) - (other.group1()[1] * self.group0()[0]) + (other.group1()[3] * self.group0()[2])),
                (other.group1()[3] * self.group0()[3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       80        0
    //    simd3        2        4        0
    // Totals...
    // yes simd       44       84        0
    //  no simd       48       92        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (other.group4()[3] * self.group0()[3] * -1.0),
                (-(other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2])
                    - (other.group1()[3] * self.group0()[3])),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group3()[0] * self.group0()[3]),
                (other.group3()[1] * self.group0()[3]),
                (other.group3()[2] * self.group0()[3]),
                ((other.group0()[1] * self.group0()[3]) - (other.group3()[0] * self.group0()[0]) - (other.group3()[1] * self.group0()[1]) - (other.group3()[2] * self.group0()[2])),
            ]),
            // e41, e42, e43
            (Simd32x3::from([
                ((other.group1()[1] * self.group0()[2]) - (other.group1()[2] * self.group0()[1])),
                (-(other.group1()[0] * self.group0()[2]) + (other.group1()[2] * self.group0()[0])),
                ((other.group1()[0] * self.group0()[1]) - (other.group1()[1] * self.group0()[0])),
            ]) + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))),
            // e23, e31, e12
            (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            Simd32x4::from([
                ((other.group0()[0] * self.group0()[0]) + (other.group2()[0] * self.group0()[3]) + (other.group3()[1] * self.group0()[2]) - (other.group3()[2] * self.group0()[1])),
                ((other.group0()[0] * self.group0()[1]) + (other.group2()[1] * self.group0()[3]) - (other.group3()[0] * self.group0()[2]) + (other.group3()[2] * self.group0()[0])),
                ((other.group0()[0] * self.group0()[2]) + (other.group2()[2] * self.group0()[3]) + (other.group3()[0] * self.group0()[1]) - (other.group3()[1] * self.group0()[0])),
                (other.group0()[0] * self.group0()[3]),
            ]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Origin> for Plane {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_product = AntiScalar::from_groups(/* e1234 */ (self.group0()[3] * other[e4] * -1.0));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       32        0
    fn sandwich(self, other: Plane) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (-(other.group0()[0] * self.group0()[3]) + (other.group0()[3] * self.group0()[0])),
                (-(other.group0()[1] * self.group0()[3]) + (other.group0()[3] * self.group0()[1])),
                (-(other.group0()[2] * self.group0()[3]) + (other.group0()[3] * self.group0()[2])),
                0.0,
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * self.group0()[3] * -1.0)]),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       36        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       15       37        0
    //  no simd       18       40        0
    fn sandwich(self, other: Point) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                (self.group0()[2] * other.group0()[1]),
                (self.group0()[0] * other.group0()[2]),
                (self.group0()[1] * other.group0()[0]),
                (-(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[3] * other.group0()[3])),
            ]) - (swizzle!(self.group0(), 1, 2, 0, 0) * swizzle!(other.group0(), 2, 0, 1, 0))),
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
    //      f32        3       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3       13        0
    //  no simd        3       16        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x4::from(other[scalar]) * self.group0()));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl InfixSandwich for Point {}
impl Sandwich<AntiScalar> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       19        0
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
    //      f32       20       31        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       20       32        0
    //  no simd       20       35        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other.group0()[0]) * self.group0()),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group0()[1] * self.group0()[0]),
                (other.group0()[1] * self.group0()[1]),
                (other.group0()[1] * self.group0()[2]),
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
    //      f32       28       44        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       31       47        0
    //  no simd       40       56        0
    fn sandwich(self, other: Flector) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                (-(other.group0()[3] * self.group0()[0]) - (other.group1()[1] * self.group0()[2])),
                (-(other.group0()[3] * self.group0()[1]) - (other.group1()[2] * self.group0()[0])),
                (-(other.group0()[3] * self.group0()[2]) - (other.group1()[0] * self.group0()[1])),
                ((other.group1()[2] * self.group0()[2]) + (other.group1()[3] * self.group0()[3])),
            ]) + (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]) * swizzle!(self.group0(), 3, 3, 3, 0))
                + (swizzle!(other.group1(), 2, 0, 1, 1) * swizzle!(self.group0(), 1, 2, 0, 1))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                (-(other.group0()[1] * self.group0()[2]) - (other.group1()[3] * self.group0()[0])),
                (-(other.group0()[2] * self.group0()[0]) - (other.group1()[3] * self.group0()[1])),
                (-(other.group0()[0] * self.group0()[1]) - (other.group1()[3] * self.group0()[2])),
                ((other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2])),
            ]) + (swizzle!(other.group0(), 2, 0, 1, 0) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Horizon> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       20       35        0
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
    //      f32       25       41        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       27       43        0
    //  no simd       33       49        0
    fn sandwich(self, other: Line) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                (other.group1()[1] * self.group0()[2]),
                (other.group1()[2] * self.group0()[0]),
                (other.group1()[0] * self.group0()[1]),
                (-(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[2]) + (other.group1()[0] * self.group0()[3])),
                ((other.group0()[2] * self.group0()[0]) + (other.group1()[1] * self.group0()[3])),
                ((other.group0()[0] * self.group0()[1]) + (other.group1()[2] * self.group0()[3])),
                (-(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Motor> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       31       47        0
    //  no simd       40       56        0
    fn sandwich(self, other: Motor) -> Self::Output {
        let geometric_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                (other.group1()[3] * self.group0()[0]),
                (other.group1()[3] * self.group0()[1]),
                (other.group1()[3] * self.group0()[2]),
                (-(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))
                + (swizzle!(other.group1(), 1, 2, 0, 3) * swizzle!(self.group0(), 2, 0, 1, 3))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[2]) + (other.group0()[3] * self.group0()[0]) + (other.group1()[0] * self.group0()[3])),
                ((other.group0()[2] * self.group0()[0]) + (other.group0()[3] * self.group0()[1]) + (other.group1()[1] * self.group0()[3])),
                ((other.group0()[0] * self.group0()[1]) + (other.group0()[3] * self.group0()[2]) + (other.group1()[2] * self.group0()[3])),
                (-(other.group1()[1] * self.group0()[1]) - (other.group1()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       54       85        0
    //    simd2        3        3        0
    //    simd3        3        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       63       94        0
    //  no simd       81      112        0
    fn sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([0.0, (other.group4()[3] * self.group0()[3])])
                + (Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group1()[0], other.group4()[0]]))
                + (Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group1()[1], other.group4()[1]]))
                + (Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group1()[2], other.group4()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                (other.group3()[1] * self.group0()[2]),
                (other.group3()[2] * self.group0()[0]),
                (other.group3()[0] * self.group0()[1]),
                (-(other.group2()[1] * self.group0()[1]) - (other.group2()[2] * self.group0()[2])),
            ]) + (Simd32x4::from(other.group0()[0]) * self.group0())
                - (Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e41, e42, e43
            (Simd32x3::from([
                (-(other.group4()[1] * self.group0()[2]) + (other.group4()[2] * self.group0()[1])),
                ((other.group4()[0] * self.group0()[2]) - (other.group4()[2] * self.group0()[0])),
                (-(other.group4()[0] * self.group0()[1]) + (other.group4()[1] * self.group0()[0])),
            ]) - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))),
            // e23, e31, e12
            (Simd32x3::from([
                (-(other.group1()[1] * self.group0()[2]) + (other.group1()[2] * self.group0()[1])),
                ((other.group1()[0] * self.group0()[2]) - (other.group1()[2] * self.group0()[0])),
                (-(other.group1()[0] * self.group0()[1]) + (other.group1()[1] * self.group0()[0])),
            ]) - (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[0]) + (other.group2()[1] * self.group0()[2]) + (other.group3()[0] * self.group0()[3])),
                ((other.group0()[1] * self.group0()[1]) + (other.group2()[2] * self.group0()[0]) + (other.group3()[1] * self.group0()[3])),
                ((other.group0()[1] * self.group0()[2]) + (other.group2()[0] * self.group0()[1]) + (other.group3()[2] * self.group0()[3])),
                (-(other.group3()[1] * self.group0()[1]) - (other.group3()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
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
    //      f32       22       43        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       23       44        0
    //  no simd       26       47        0
    fn sandwich(self, other: Plane) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[2]) * -1.0),
                ((other.group0()[2] * self.group0()[0]) * -1.0),
                ((other.group0()[0] * self.group0()[1]) * -1.0),
                ((other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]) + (other.group0()[3] * self.group0()[3])),
            ]) + (swizzle!(other.group0(), 2, 0, 1, 0) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group0()[3] * self.group0()[0] * -1.0),
                (other.group0()[3] * self.group0()[1] * -1.0),
                (other.group0()[3] * self.group0()[2] * -1.0),
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
    //      f32       24       42        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       43        0
    //  no simd       28       46        0
    fn sandwich(self, other: Point) -> Self::Output {
        let geometric_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                ((other.group0()[0] * self.group0()[3]) - (other.group0()[3] * self.group0()[0])),
                ((other.group0()[1] * self.group0()[3]) - (other.group0()[3] * self.group0()[1])),
                ((other.group0()[2] * self.group0()[3]) - (other.group0()[3] * self.group0()[2])),
                0.0,
            ]),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[2]) * -1.0),
                ((other.group0()[2] * self.group0()[0]) * -1.0),
                ((other.group0()[0] * self.group0()[1]) * -1.0),
                ((other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2])),
            ]) + (swizzle!(other.group0(), 2, 0, 1, 0) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Scalar> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       15        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       16        0
    //  no simd        8       19        0
    fn sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x4::from(other[scalar]) * self.group0()));
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
        let geometric_product = AntiScalar::from_groups(/* e1234 */ (other[e1234] * self[scalar]));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<DualNum> for Scalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_product = DualNum::from_groups(/* scalar, e1234 */ (Simd32x2::from(self[scalar]) * other.group0()));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       16        0
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
        let geometric_product = Horizon::from_groups(/* e321 */ (other[e321] * self[scalar]));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
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
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       16        0
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
    //      f32        0       16        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       21        0
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
        let geometric_product = Origin::from_groups(/* e4 */ (other[e4] * self[scalar]));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Plane> for Scalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_product = Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x4::from(self[scalar]) * other.group0()));
        return geometric_product.geometric_product(self.reverse());
    }
}
impl Sandwich<Point> for Scalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
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
        let geometric_product = Scalar::from_groups(/* scalar */ (other[scalar] * self[scalar]));
        return geometric_product.geometric_product(self.reverse());
    }
}
