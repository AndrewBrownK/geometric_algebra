use crate::traits::AntiReverse;
use crate::traits::GeometricAntiProduct;
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
impl InfixAntiSandwich for AntiScalar {}
impl AntiSandwich<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ (other[e1234] * self[e1234]));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        4        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(/* scalar, e1234 */ (Simd32x2::from(self[e1234]) * other.group0()));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       16        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * other.group0()),
            // e423, e431, e412, e321
            (Simd32x4::from(self[e1234]) * other.group1()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for AntiScalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Horizon::from_groups(/* e321 */ (self[e1234] * other[e321]));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group0()),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * other.group1()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       16        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e1234]) * other.group0()),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e1234]) * other.group1()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for AntiScalar {
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
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from(self[e1234]) * other.group0()),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e1234]) * other.group1()),
            // e41, e42, e43
            (Simd32x3::from(self[e1234]) * other.group2()),
            // e23, e31, e12
            (Simd32x3::from(self[e1234]) * other.group3()),
            // e423, e431, e412, e321
            (Simd32x4::from(self[e1234]) * other.group4()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for AntiScalar {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Origin::from_groups(/* e4 */ (self[e1234] * other[e4]));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x4::from(self[e1234]) * other.group0()));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for AntiScalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x4::from(self[e1234]) * other.group0()));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for AntiScalar {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Scalar::from_groups(/* scalar */ (self[e1234] * other[scalar]));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl InfixAntiSandwich for DualNum {}
impl AntiSandwich<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        1        5        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(/* scalar, e1234 */ (Simd32x2::from(other[e1234]) * self.group0()));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        let geometric_anti_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([
            ((other.group0()[0] * self.group0()[1]) + (other.group0()[1] * self.group0()[0])),
            (other.group0()[1] * self.group0()[1]),
        ]));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       24        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                ((self.group0()[0] * other.group1()[0]) + (self.group0()[1] * other.group0()[0])),
                ((self.group0()[0] * other.group1()[1]) + (self.group0()[1] * other.group0()[1])),
                ((self.group0()[0] * other.group1()[2]) + (self.group0()[1] * other.group0()[2])),
                (self.group0()[1] * other.group0()[3]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self.group0()[1] * other.group1()[0]),
                (self.group0()[1] * other.group1()[1]),
                (self.group0()[1] * other.group1()[2]),
                ((self.group0()[0] * other.group0()[3]) + (self.group0()[1] * other.group1()[3])),
            ]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for DualNum {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Horizon::from_groups(/* e321 */ (self.group0()[1] * other[e321]));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        6       18        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(self.group0()[1]) * other.group0()),
            // e23, e31, e12
            ((Simd32x3::from(self.group0()[0]) * other.group0()) + (Simd32x3::from(self.group0()[1]) * other.group1())),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       12        0
    //    simd4        1        3        0
    // Totals...
    // yes simd        5       15        0
    //  no simd        8       24        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self.group0()[1]) * other.group0()),
            // e23, e31, e12, scalar
            ((Simd32x4::from(self.group0()[0]) * other.group0()) + (Simd32x4::from(self.group0()[1]) * other.group1())),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       39        0
    //    simd3        1        3        0
    // Totals...
    // yes simd       14       42        0
    //  no simd       16       48        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                ((self.group0()[0] * other.group0()[1]) + (self.group0()[1] * other.group0()[0])),
                (self.group0()[1] * other.group0()[1]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                ((self.group0()[0] * other.group4()[0]) + (self.group0()[1] * other.group1()[0])),
                ((self.group0()[0] * other.group4()[1]) + (self.group0()[1] * other.group1()[1])),
                ((self.group0()[0] * other.group4()[2]) + (self.group0()[1] * other.group1()[2])),
                (self.group0()[1] * other.group1()[3]),
            ]),
            // e41, e42, e43
            (Simd32x3::from(self.group0()[1]) * other.group2()),
            // e23, e31, e12
            ((Simd32x3::from(self.group0()[0]) * other.group2()) + (Simd32x3::from(self.group0()[1]) * other.group3())),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self.group0()[1] * other.group4()[0]),
                (self.group0()[1] * other.group4()[1]),
                (self.group0()[1] * other.group4()[2]),
                ((self.group0()[0] * other.group1()[3]) + (self.group0()[1] * other.group4()[3])),
            ]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       14        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[1] * other[e4])]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[0] * other[e4])]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       15        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       16        0
    //  no simd        4       19        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] * other.group0()[0]),
                (self.group0()[0] * other.group0()[1]),
                (self.group0()[0] * other.group0()[2]),
                0.0,
            ]),
            // e423, e431, e412, e321
            (Simd32x4::from(self.group0()[1]) * other.group0()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       13        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4       14        0
    //  no simd        4       17        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self.group0()[1]) * other.group0()),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[0] * other.group0()[3])]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for DualNum {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Scalar::from_groups(/* scalar */ (self.group0()[1] * other[scalar]));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl InfixAntiSandwich for Flector {}
impl AntiSandwich<AntiScalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       40       54        0
    //  no simd       40       60        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other[e1234]) * self.group0()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * self.group1()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       44       64        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(other.group0()[0] * self.group1()[0]) + (other.group0()[1] * self.group0()[0])),
                (-(other.group0()[0] * self.group1()[1]) + (other.group0()[1] * self.group0()[1])),
                (-(other.group0()[0] * self.group1()[2]) + (other.group0()[1] * self.group0()[2])),
                (other.group0()[1] * self.group0()[3]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group0()[1] * self.group1()[0]),
                (other.group0()[1] * self.group1()[1]),
                (other.group0()[1] * self.group1()[2]),
                (-(other.group0()[0] * self.group0()[3]) + (other.group0()[1] * self.group1()[3])),
            ]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       68        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       56       76        0
    //  no simd       80      100        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                (-(other.group1()[0] * self.group0()[3]) - (other.group1()[2] * self.group1()[1])),
                (-(other.group1()[0] * self.group1()[2]) - (other.group1()[1] * self.group0()[3])),
                (-(other.group1()[1] * self.group1()[0]) - (other.group1()[2] * self.group0()[3])),
                ((other.group1()[1] * self.group1()[1]) + (other.group1()[2] * self.group1()[2])),
            ]) - (Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]))
                + (swizzle!(other.group1(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((other.group0()[2] * self.group1()[1]) - (other.group1()[2] * self.group0()[1])),
                ((other.group0()[0] * self.group1()[2]) - (other.group1()[0] * self.group0()[2])),
                ((other.group0()[1] * self.group1()[0]) - (other.group1()[1] * self.group0()[0])),
                (-(other.group0()[3] * self.group1()[3]) + (other.group1()[3] * self.group0()[3])),
            ]) + (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]) * swizzle!(self.group0(), 3, 3, 3, 0))
                - (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[2]]) * swizzle!(self.group1(), 3, 3, 3, 2))
                - (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[1]]) * swizzle!(other.group0(), 3, 3, 3, 1))
                + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[2]]) * swizzle!(other.group1(), 3, 3, 3, 2))
                - (swizzle!(other.group0(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))
                + (swizzle!(other.group1(), 1, 2, 0, 1) * swizzle!(self.group0(), 2, 0, 1, 1))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       40       53        0
    //  no simd       40       56        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e321]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]])),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       80        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       62       82        0
    //  no simd       68       88        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group0()[0] * self.group1()[3]) - (other.group0()[1] * self.group0()[2]) + (other.group0()[2] * self.group0()[1]) - (other.group1()[0] * self.group0()[3])
                    + (other.group1()[1] * self.group1()[2])),
                ((other.group0()[0] * self.group0()[2]) + (other.group0()[1] * self.group1()[3]) - (other.group0()[2] * self.group0()[0]) - (other.group1()[1] * self.group0()[3])
                    + (other.group1()[2] * self.group1()[0])),
                (-(other.group0()[0] * self.group0()[1])
                    + (other.group0()[1] * self.group0()[0])
                    + (other.group0()[2] * self.group1()[3])
                    + (other.group1()[0] * self.group1()[1])
                    - (other.group1()[2] * self.group0()[3])),
                (-(other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[0] * self.group0()[3]) - (other.group0()[1] * self.group1()[2])),
                ((other.group0()[1] * self.group0()[3]) - (other.group0()[2] * self.group1()[0])),
                (-(other.group0()[0] * self.group1()[1]) + (other.group0()[2] * self.group0()[3])),
                (-(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2])
                    + (other.group1()[1] * self.group1()[1])
                    + (other.group1()[2] * self.group1()[2])),
            ]) + (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       52       68        0
    //    simd4        8        8        0
    // Totals...
    // yes simd       60       76        0
    //  no simd       84      100        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group0()[1] * other.group0()[2]) - (self.group0()[3] * other.group1()[0]) + (self.group1()[2] * other.group1()[1]) + (self.group1()[3] * other.group0()[0])),
                ((self.group0()[2] * other.group0()[0]) - (self.group0()[3] * other.group1()[1]) + (self.group1()[0] * other.group1()[2]) + (self.group1()[3] * other.group0()[1])),
                ((self.group0()[2] * other.group0()[3]) - (self.group0()[3] * other.group1()[2]) + (self.group1()[1] * other.group1()[0]) + (self.group1()[3] * other.group0()[2])),
                0.0,
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[3], other.group0()[2]]) * swizzle!(self.group1(), 1, 2, 2, 2))
                - (Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[1], other.group0()[1]]) * swizzle!(self.group1(), 0, 1, 0, 1))
                + (swizzle!(self.group0(), 0, 1, 0, 3) * swizzle!(other.group0(), 3, 3, 1, 3))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                0.0,
                0.0,
                0.0,
                (-(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[3] * other.group1()[3])
                    + (self.group1()[2] * other.group1()[2])),
            ]) + (Simd32x4::from([self.group0()[3], self.group0()[3], self.group0()[3], self.group1()[3]]) * other.group0())
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))
                + (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[3], other.group1()[1]]) * swizzle!(self.group1(), 1, 2, 2, 1))
                + (Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[1], other.group1()[0]]) * swizzle!(self.group1(), 0, 1, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Flector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      115      151        0
    //    simd2        4        4        0
    //    simd3        6        6        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      130      166        0
    //  no simd      161      197        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                ((self.group0()[3] * other.group4()[3]) - (self.group1()[0] * other.group1()[0]) - (self.group1()[1] * other.group1()[1]) - (self.group1()[2] * other.group1()[2])),
                0.0,
            ]) - (Simd32x2::from(other.group1()[3]) * Simd32x2::from([self.group1()[3], self.group0()[3]]))
                + (Simd32x2::from(other.group4()[0]) * Simd32x2::from([self.group0()[0], self.group1()[0]]))
                + (Simd32x2::from(other.group4()[1]) * Simd32x2::from([self.group0()[1], self.group1()[1]]))
                + (Simd32x2::from(other.group4()[2]) * Simd32x2::from([self.group0()[2], self.group1()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group2()[0] * self.group1()[3]) - (other.group2()[1] * self.group0()[2]) + (other.group2()[2] * self.group0()[1]) - (other.group3()[0] * self.group0()[3])
                    + (other.group3()[1] * self.group1()[2])),
                ((other.group2()[0] * self.group0()[2]) + (other.group2()[1] * self.group1()[3]) - (other.group2()[2] * self.group0()[0]) - (other.group3()[1] * self.group0()[3])
                    + (other.group3()[2] * self.group1()[0])),
                (-(other.group2()[0] * self.group0()[1])
                    + (other.group2()[1] * self.group0()[0])
                    + (other.group2()[2] * self.group1()[3])
                    + (other.group3()[0] * self.group1()[1])
                    - (other.group3()[2] * self.group0()[3])),
                ((other.group2()[2] * self.group1()[2]) * -1.0),
            ]) + (Simd32x4::from(other.group0()[1]) * self.group0())
                - (Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group2()[0]]) * swizzle!(self.group1(), 0, 1, 2, 0))
                - (Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[1]]) * swizzle!(self.group1(), 1, 2, 0, 1))),
            // e41, e42, e43
            (Simd32x3::from([
                (-(self.group1()[1] * other.group4()[2]) + (self.group1()[2] * other.group4()[1])),
                ((self.group1()[0] * other.group4()[2]) - (self.group1()[2] * other.group4()[0])),
                (-(self.group1()[0] * other.group4()[1]) + (self.group1()[1] * other.group4()[0])),
            ]) - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))),
            // e23, e31, e12
            (Simd32x3::from([
                (-(self.group0()[1] * other.group4()[2]) + (self.group0()[2] * other.group4()[1]) + (self.group1()[1] * other.group1()[2])
                    - (self.group1()[2] * other.group1()[1])),
                ((self.group0()[0] * other.group4()[2]) - (self.group0()[2] * other.group4()[0]) - (self.group1()[0] * other.group1()[2]) + (self.group1()[2] * other.group1()[0])),
                (-(self.group0()[0] * other.group4()[1]) + (self.group0()[1] * other.group4()[0]) + (self.group1()[0] * other.group1()[1])
                    - (self.group1()[1] * other.group1()[0])),
            ]) + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group2()[0] * self.group0()[3]) - (other.group2()[1] * self.group1()[2])),
                ((other.group2()[1] * self.group0()[3]) - (other.group2()[2] * self.group1()[0])),
                (-(other.group2()[0] * self.group1()[1]) + (other.group2()[2] * self.group0()[3])),
                (-(other.group0()[0] * self.group0()[3])
                    - (other.group2()[0] * self.group0()[0])
                    - (other.group2()[1] * self.group0()[1])
                    - (other.group2()[2] * self.group0()[2])
                    + (other.group3()[1] * self.group1()[1])
                    + (other.group3()[2] * self.group1()[2])),
            ]) + (Simd32x4::from(other.group0()[1]) * self.group1())
                + (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[0]]) * swizzle!(self.group1(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       52        0
    //    simd4        0        4        0
    // Totals...
    // yes simd       40       56        0
    //  no simd       40       68        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[e4]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]) * Simd32x4::from(-1.0)),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e4]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]) * Simd32x4::from(-1.0)),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       68        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       51       71        0
    //  no simd       60       80        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                (-(self.group0()[3] * other.group0()[0]) - (self.group1()[1] * other.group0()[2])),
                (-(self.group0()[3] * other.group0()[1]) - (self.group1()[2] * other.group0()[0])),
                (-(self.group0()[3] * other.group0()[2]) - (self.group1()[0] * other.group0()[1])),
                ((self.group1()[1] * other.group0()[1]) + (self.group1()[2] * other.group0()[2])),
            ]) + (swizzle!(self.group1(), 2, 0, 1, 0) * swizzle!(other.group0(), 1, 2, 0, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                (-(self.group0()[1] * other.group0()[2]) - (self.group1()[3] * other.group0()[0])),
                (-(self.group0()[2] * other.group0()[0]) - (self.group1()[3] * other.group0()[1])),
                (-(self.group0()[0] * other.group0()[1]) - (self.group1()[3] * other.group0()[2])),
                ((self.group0()[2] * other.group0()[2]) + (self.group0()[3] * other.group0()[3])),
            ]) + (Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[1]]) * swizzle!(other.group0(), 3, 3, 3, 1))
                + (swizzle!(self.group0(), 2, 0, 1, 0) * swizzle!(other.group0(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       46       64        0
    //  no simd       52       76        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]]) * Simd32x4::from(-1.0)),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((self.group0()[3] * other.group0()[0]) + (self.group1()[1] * other.group0()[2])),
                ((self.group0()[3] * other.group0()[1]) + (self.group1()[2] * other.group0()[0])),
                ((self.group0()[3] * other.group0()[2]) + (self.group1()[0] * other.group0()[1])),
                (-(self.group1()[2] * other.group0()[2]) - (self.group1()[3] * other.group0()[3])),
            ]) - (Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[0]]) * swizzle!(other.group0(), 3, 3, 3, 0))
                - (swizzle!(self.group1(), 2, 0, 1, 1) * swizzle!(other.group0(), 1, 2, 0, 1))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       40       60        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group1()[0] * other[scalar] * -1.0),
                (self.group1()[1] * other[scalar] * -1.0),
                (self.group1()[2] * other[scalar] * -1.0),
                0.0,
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * other[scalar] * -1.0)]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl InfixAntiSandwich for Horizon {}
impl AntiSandwich<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e321]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) * Simd32x4::from(-1.0)),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([
            (other.group0()[0] * self[e321]),
            (other.group0()[1] * self[e321]),
            (other.group0()[2] * self[e321]),
            0.0,
        ]));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * self[e321]), (other.group0()[1] * self[e321]), (other.group0()[2] * self[e321]), 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * self[e321])]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       14        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0       16        0
    //  no simd        0       20        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(other.group1()[3] * self[e321] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group2()[0] * self[e321]), (other.group2()[1] * self[e321]), (other.group2()[2] * self[e321]), 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e321]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * self[e321])]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Horizon {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        9        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[e321]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl InfixAntiSandwich for Line {}
impl AntiSandwich<AntiScalar> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       33        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       19       35        0
    //  no simd       19       39        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group0()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * self.group1()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       33        0
    //    simd3        1        3        0
    // Totals...
    // yes simd       20       36        0
    //  no simd       22       42        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            (Simd32x3::from(other.group0()[1]) * self.group0()),
            // e23, e31, e12
            ((Simd32x3::from(other.group0()[0]) * self.group0()) + (Simd32x3::from(other.group0()[1]) * self.group1())),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       70        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       50       72        0
    //  no simd       56       78        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group0()[0] * other.group1()[3]) + (self.group0()[1] * other.group0()[2]) - (self.group0()[2] * other.group0()[1])
                    + (self.group1()[0] * other.group0()[3])
                    + (self.group1()[1] * other.group1()[2])),
                (-(self.group0()[0] * other.group0()[2])
                    + (self.group0()[1] * other.group1()[3])
                    + (self.group0()[2] * other.group0()[0])
                    + (self.group1()[1] * other.group0()[3])
                    + (self.group1()[2] * other.group1()[0])),
                ((self.group0()[0] * other.group0()[1]) - (self.group0()[1] * other.group0()[0])
                    + (self.group0()[2] * other.group1()[3])
                    + (self.group1()[0] * other.group1()[1])
                    + (self.group1()[2] * other.group0()[3])),
                (-(self.group0()[1] * other.group1()[1]) - (self.group0()[2] * other.group1()[2])),
            ]) - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(other.group1(), 1, 2, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((self.group0()[0] * other.group0()[3]) + (self.group0()[1] * other.group1()[2])),
                ((self.group0()[1] * other.group0()[3]) + (self.group0()[2] * other.group1()[0])),
                ((self.group0()[0] * other.group1()[1]) + (self.group0()[2] * other.group0()[3])),
                (-(self.group0()[0] * other.group0()[0])
                    - (self.group0()[1] * other.group0()[1])
                    - (self.group0()[2] * other.group0()[2])
                    - (self.group1()[1] * other.group1()[1])
                    - (self.group1()[2] * other.group1()[2])),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group1(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       24        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([
            (self.group0()[0] * other[e321]),
            (self.group0()[1] * other[e321]),
            (self.group0()[2] * other[e321]),
            0.0,
        ]));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       47       69        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (-(other.group0()[1] * self.group0()[2]) + (other.group0()[2] * self.group0()[1])),
                ((other.group0()[0] * self.group0()[2]) - (other.group0()[2] * self.group0()[0])),
                (-(other.group0()[0] * self.group0()[1]) + (other.group0()[1] * self.group0()[0])),
                (-(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2])),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (-(other.group0()[1] * self.group1()[2]) + (other.group0()[2] * self.group1()[1]) - (other.group1()[1] * self.group0()[2])
                    + (other.group1()[2] * self.group0()[1])),
                ((other.group0()[0] * self.group1()[2]) - (other.group0()[2] * self.group1()[0]) + (other.group1()[0] * self.group0()[2]) - (other.group1()[2] * self.group0()[0])),
                (-(other.group0()[0] * self.group1()[1]) + (other.group0()[1] * self.group1()[0]) - (other.group1()[0] * self.group0()[1])
                    + (other.group1()[1] * self.group0()[0])),
                (-(other.group0()[0] * self.group1()[0])
                    - (other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2])),
            ]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       66        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       47       69        0
    //  no simd       56       78        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((self.group0()[0] * other.group0()[3]) + (self.group0()[1] * other.group0()[2])),
                ((self.group0()[1] * other.group0()[3]) + (self.group0()[2] * other.group0()[0])),
                ((self.group0()[0] * other.group0()[1]) + (self.group0()[2] * other.group0()[3])),
                (-(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2])),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group0()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((self.group0()[0] * other.group1()[3]) + (self.group0()[1] * other.group1()[2]) + (self.group1()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2])),
                ((self.group0()[1] * other.group1()[3]) + (self.group0()[2] * other.group1()[0]) + (self.group1()[1] * other.group0()[3]) + (self.group1()[2] * other.group0()[0])),
                ((self.group0()[0] * other.group1()[1]) + (self.group0()[2] * other.group1()[3]) + (self.group1()[0] * other.group0()[1]) + (self.group1()[2] * other.group0()[3])),
                (-(self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group0()[1])
                    - (self.group1()[2] * other.group0()[2])),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group0()[0]]) * swizzle!(other.group1(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group1()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Line {
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
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                (-(self.group1()[0] * other.group2()[0]) - (self.group1()[1] * other.group2()[1]) - (self.group1()[2] * other.group2()[2])),
                0.0,
            ]) - (Simd32x2::from(self.group0()[0]) * Simd32x2::from([other.group3()[0], other.group2()[0]]))
                - (Simd32x2::from(self.group0()[1]) * Simd32x2::from([other.group3()[1], other.group2()[1]]))
                - (Simd32x2::from(self.group0()[2]) * Simd32x2::from([other.group3()[2], other.group2()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group0()[0] * other.group4()[3]) + (self.group0()[1] * other.group1()[2]) - (self.group0()[2] * other.group1()[1])
                    + (self.group1()[0] * other.group1()[3])
                    + (self.group1()[1] * other.group4()[2])),
                (-(self.group0()[0] * other.group1()[2])
                    + (self.group0()[1] * other.group4()[3])
                    + (self.group0()[2] * other.group1()[0])
                    + (self.group1()[1] * other.group1()[3])
                    + (self.group1()[2] * other.group4()[0])),
                ((self.group0()[0] * other.group1()[1]) - (self.group0()[1] * other.group1()[0])
                    + (self.group0()[2] * other.group4()[3])
                    + (self.group1()[0] * other.group4()[1])
                    + (self.group1()[2] * other.group1()[3])),
                (-(self.group0()[1] * other.group4()[1]) - (self.group0()[2] * other.group4()[2])),
            ]) - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(other.group4(), 1, 2, 0, 0))),
            // e41, e42, e43
            ((Simd32x3::from(other.group0()[1]) * self.group0()) + (swizzle!(self.group0(), 1, 2, 0) * swizzle!(other.group2(), 2, 0, 1))
                - (swizzle!(self.group0(), 2, 0, 1) * swizzle!(other.group2(), 1, 2, 0))),
            // e23, e31, e12
            ((Simd32x3::from(other.group0()[0]) * self.group0())
                + (Simd32x3::from(other.group0()[1]) * self.group1())
                + (swizzle!(self.group0(), 1, 2, 0) * swizzle!(other.group3(), 2, 0, 1))
                - (swizzle!(self.group0(), 2, 0, 1) * swizzle!(other.group3(), 1, 2, 0))
                + (swizzle!(self.group1(), 1, 2, 0) * swizzle!(other.group2(), 2, 0, 1))
                - (swizzle!(self.group1(), 2, 0, 1) * swizzle!(other.group2(), 1, 2, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((self.group0()[0] * other.group1()[3]) + (self.group0()[1] * other.group4()[2])),
                ((self.group0()[1] * other.group1()[3]) + (self.group0()[2] * other.group4()[0])),
                ((self.group0()[0] * other.group4()[1]) + (self.group0()[2] * other.group1()[3])),
                (-(self.group0()[0] * other.group1()[0])
                    - (self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[1] * other.group4()[1])
                    - (self.group1()[2] * other.group4()[2])),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group4(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       28       48        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([(self.group1()[0] * other[e4]), (self.group1()[1] * other[e4]), (self.group1()[2] * other[e4]), 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([(self.group0()[0] * other[e4]), (self.group0()[1] * other[e4]), (self.group0()[2] * other[e4]), 0.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       55        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       35       57        0
    //  no simd       41       63        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group0()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2])),
                ((self.group0()[1] * other.group0()[3]) + (self.group1()[2] * other.group0()[0])),
                ((self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[1])),
                (-(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2])),
            ]) - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                (self.group0()[1] * other.group0()[2]),
                (self.group0()[2] * other.group0()[0]),
                (self.group0()[0] * other.group0()[1]),
                (-(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2])),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       36       57        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                ((self.group0()[1] * other.group0()[2]) - (self.group0()[2] * other.group0()[1]) + (self.group1()[0] * other.group0()[3])),
                (-(self.group0()[0] * other.group0()[2]) + (self.group0()[2] * other.group0()[0]) + (self.group1()[1] * other.group0()[3])),
                ((self.group0()[0] * other.group0()[1]) - (self.group0()[1] * other.group0()[0]) + (self.group1()[2] * other.group0()[3])),
                0.0,
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]),
                (self.group0()[1] * other.group0()[3]),
                (self.group0()[2] * other.group0()[3]),
                (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2])),
            ]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       33        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       19       34        0
    //  no simd       19       36        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ (Simd32x3::from(other[scalar]) * self.group0()));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl InfixAntiSandwich for Motor {}
impl AntiSandwich<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       54        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       40       56        0
    //  no simd       40       62        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other[e1234]) * self.group0()),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[e1234]) * self.group1()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       54        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       41       57        0
    //  no simd       44       66        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(other.group0()[1]) * self.group0()),
            // e23, e31, e12, scalar
            ((Simd32x4::from(other.group0()[0]) * self.group0()) + (Simd32x4::from(other.group0()[1]) * self.group1())),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       79        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       62       85        0
    //  no simd       80      103        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group0()[2] * self.group0()[1])
                    + (other.group0()[3] * self.group1()[0])
                    + (other.group1()[0] * self.group1()[3])
                    + (other.group1()[2] * self.group1()[1])
                    + (other.group1()[3] * self.group0()[0])),
                ((other.group0()[1] * self.group0()[3])
                    + (other.group0()[3] * self.group1()[1])
                    + (other.group1()[0] * self.group1()[2])
                    + (other.group1()[1] * self.group1()[3])
                    + (other.group1()[3] * self.group0()[1])),
                ((other.group0()[2] * self.group0()[3])
                    + (other.group0()[3] * self.group1()[2])
                    + (other.group1()[1] * self.group1()[0])
                    + (other.group1()[2] * self.group1()[3])
                    + (other.group1()[3] * self.group0()[2])),
                ((other.group1()[2] * self.group0()[2]) * -1.0),
            ]) - (Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group1()[0]]) * swizzle!(self.group0(), 2, 0, 1, 0))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[1]]) * swizzle!(other.group1(), 1, 2, 0, 1))
                + (swizzle!(other.group0(), 0, 0, 1, 3) * swizzle!(self.group0(), 3, 2, 0, 3))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                (other.group1()[2] * self.group0()[1]),
                (other.group1()[1] * self.group0()[3]),
                (other.group1()[2] * self.group0()[3]),
                (-(other.group0()[1] * self.group0()[1])
                    - (other.group0()[2] * self.group0()[2])
                    - (other.group1()[0] * self.group1()[0])
                    - (other.group1()[1] * self.group1()[1])
                    - (other.group1()[2] * self.group1()[2])),
            ]) + (Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]]))
                - (Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[0], other.group0()[0]]) * swizzle!(self.group0(), 2, 0, 1, 0))
                + (swizzle!(other.group1(), 0, 0, 1, 3) * swizzle!(self.group0(), 3, 2, 0, 3))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       40       58        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([(self.group0()[0] * other[e321]), (self.group0()[1] * other[e321]), (self.group0()[2] * other[e321]), 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * other[e321])]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       78        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       59       81        0
    //  no simd       68       90        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((other.group0()[0] * self.group0()[3]) + (other.group0()[2] * self.group0()[1])),
                ((other.group0()[0] * self.group0()[2]) + (other.group0()[1] * self.group0()[3])),
                ((other.group0()[1] * self.group0()[0]) + (other.group0()[2] * self.group0()[3])),
                (-(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group0()[0]]) * swizzle!(self.group0(), 2, 0, 1, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((other.group0()[0] * self.group1()[3]) + (other.group0()[2] * self.group1()[1]) + (other.group1()[0] * self.group0()[3]) + (other.group1()[2] * self.group0()[1])),
                ((other.group0()[0] * self.group1()[2]) + (other.group0()[1] * self.group1()[3]) + (other.group1()[0] * self.group0()[2]) + (other.group1()[1] * self.group0()[3])),
                ((other.group0()[1] * self.group1()[0]) + (other.group0()[2] * self.group1()[3]) + (other.group1()[1] * self.group0()[0]) + (other.group1()[2] * self.group0()[3])),
                (-(other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[0], other.group0()[0]]) * swizzle!(self.group1(), 2, 0, 1, 0))
                - (Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[0], other.group1()[0]]) * swizzle!(self.group0(), 2, 0, 1, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       78        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       62       84        0
    //  no simd       80      102        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((other.group0()[2] * self.group0()[1]) + (other.group0()[3] * self.group0()[0])),
                ((other.group0()[1] * self.group0()[3]) + (other.group0()[3] * self.group0()[1])),
                ((other.group0()[2] * self.group0()[3]) + (other.group0()[3] * self.group0()[2])),
                (-(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2])),
            ]) + (swizzle!(other.group0(), 0, 0, 1, 3) * swizzle!(self.group0(), 3, 2, 0, 3))
                - (swizzle!(other.group0(), 1, 2, 0, 0) * swizzle!(self.group0(), 2, 0, 1, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((other.group0()[2] * self.group1()[1]) + (other.group0()[3] * self.group1()[0]) + (other.group1()[2] * self.group0()[1]) + (other.group1()[3] * self.group0()[0])),
                ((other.group0()[1] * self.group1()[3]) + (other.group0()[3] * self.group1()[1]) + (other.group1()[1] * self.group0()[3]) + (other.group1()[3] * self.group0()[1])),
                ((other.group0()[2] * self.group1()[3]) + (other.group0()[3] * self.group1()[2]) + (other.group1()[2] * self.group0()[3]) + (other.group1()[3] * self.group0()[2])),
                (-(other.group0()[1] * self.group1()[1])
                    - (other.group0()[2] * self.group1()[2])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2])),
            ]) + (swizzle!(other.group0(), 0, 0, 1, 3) * swizzle!(self.group1(), 3, 2, 0, 3))
                - (swizzle!(other.group0(), 1, 2, 0, 0) * swizzle!(self.group1(), 2, 0, 1, 0))
                + (swizzle!(other.group1(), 0, 0, 1, 3) * swizzle!(self.group0(), 3, 2, 0, 3))
                - (swizzle!(other.group1(), 1, 2, 0, 0) * swizzle!(self.group0(), 2, 0, 1, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Motor {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      103      135        0
    //    simd2        4        4        0
    //    simd3       10       12        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      122      156        0
    //  no simd      161      199        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                ((other.group0()[1] * self.group1()[3]) - (other.group3()[0] * self.group0()[0]) - (other.group3()[1] * self.group0()[1]) - (other.group3()[2] * self.group0()[2])),
                0.0,
            ]) - (Simd32x2::from(other.group2()[0]) * Simd32x2::from([self.group1()[0], self.group0()[0]]))
                - (Simd32x2::from(other.group2()[1]) * Simd32x2::from([self.group1()[1], self.group0()[1]]))
                - (Simd32x2::from(other.group2()[2]) * Simd32x2::from([self.group1()[2], self.group0()[2]]))
                + (Simd32x2::from(self.group0()[3]) * other.group0())),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group0()[1] * other.group1()[2])
                    + (self.group0()[3] * other.group1()[0])
                    + (self.group1()[0] * other.group1()[3])
                    + (self.group1()[1] * other.group4()[2])
                    + (self.group1()[3] * other.group4()[0])),
                ((self.group0()[2] * other.group1()[0])
                    + (self.group0()[3] * other.group1()[1])
                    + (self.group1()[1] * other.group1()[3])
                    + (self.group1()[2] * other.group4()[0])
                    + (self.group1()[3] * other.group4()[1])),
                ((self.group0()[2] * other.group4()[3])
                    + (self.group0()[3] * other.group1()[2])
                    + (self.group1()[0] * other.group4()[1])
                    + (self.group1()[2] * other.group1()[3])
                    + (self.group1()[3] * other.group4()[2])),
                ((self.group0()[2] * other.group4()[2]) * -1.0),
            ]) - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[1]]) * swizzle!(other.group4(), 1, 2, 0, 1))
                - (Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[0], other.group4()[0]]) * swizzle!(self.group0(), 2, 0, 1, 0))
                + (Simd32x4::from([other.group4()[3], other.group4()[3], other.group1()[1], other.group1()[3]]) * swizzle!(self.group0(), 0, 1, 0, 3))),
            // e41, e42, e43
            ((Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from([self.group0()[1], self.group0()[3], self.group0()[3]]) * swizzle!(other.group2(), 2, 1, 2))
                - (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * swizzle!(other.group2(), 1, 2, 0))
                + (Simd32x3::from([self.group0()[3], self.group0()[2], self.group0()[0]]) * swizzle!(other.group2(), 0, 0, 1))),
            // e23, e31, e12
            ((Simd32x3::from(other.group0()[0]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(other.group0()[1]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from([self.group0()[1], self.group0()[3], self.group0()[3]]) * swizzle!(other.group3(), 2, 1, 2))
                - (Simd32x3::from([self.group0()[2], self.group0()[0], self.group0()[1]]) * swizzle!(other.group3(), 1, 2, 0))
                + (Simd32x3::from([self.group0()[3], self.group0()[2], self.group0()[0]]) * swizzle!(other.group3(), 0, 0, 1))
                + (Simd32x3::from([self.group1()[1], self.group1()[3], self.group1()[3]]) * swizzle!(other.group2(), 2, 1, 2))
                - (Simd32x3::from([self.group1()[2], self.group1()[0], self.group1()[1]]) * swizzle!(other.group2(), 1, 2, 0))
                + (Simd32x3::from([self.group1()[3], self.group1()[2], self.group1()[0]]) * swizzle!(other.group2(), 0, 0, 1))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((self.group0()[1] * other.group4()[2]) + (self.group0()[3] * other.group4()[0])),
                ((self.group0()[2] * other.group4()[0]) + (self.group0()[3] * other.group4()[1])),
                ((self.group0()[2] * other.group1()[3]) + (self.group0()[3] * other.group4()[2])),
                (-(self.group0()[1] * other.group1()[1])
                    - (self.group0()[2] * other.group1()[2])
                    - (self.group1()[0] * other.group4()[0])
                    - (self.group1()[1] * other.group4()[1])
                    - (self.group1()[2] * other.group4()[2])
                    + (self.group1()[3] * other.group1()[3])),
            ]) + (Simd32x4::from([other.group1()[3], other.group1()[3], other.group4()[1], other.group4()[3]]) * swizzle!(self.group0(), 0, 1, 0, 3))
                - (Simd32x4::from([other.group4()[1], other.group4()[2], other.group4()[0], other.group1()[0]]) * swizzle!(self.group0(), 2, 0, 1, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       54        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       40       56        0
    //  no simd       40       62        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other[e4]) * Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group0()[3]])),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e4]) * Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], self.group1()[3]])),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       70        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       51       73        0
    //  no simd       60       82        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group0()[0] * other.group0()[3]) + (self.group1()[1] * other.group0()[2]) + (self.group1()[3] * other.group0()[0])),
                ((self.group0()[1] * other.group0()[3]) + (self.group1()[2] * other.group0()[0]) + (self.group1()[3] * other.group0()[1])),
                ((self.group0()[2] * other.group0()[3]) + (self.group1()[0] * other.group0()[1]) + (self.group1()[3] * other.group0()[2])),
                (-(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2])),
            ]) - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group0()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                (self.group0()[3] * other.group0()[0]),
                (self.group0()[3] * other.group0()[1]),
                (self.group0()[3] * other.group0()[2]),
                (-(self.group1()[1] * other.group0()[1]) - (self.group1()[2] * other.group0()[2])),
            ]) - (Simd32x4::from([self.group0()[2], self.group0()[0], self.group0()[1], self.group1()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))
                + (swizzle!(self.group0(), 1, 2, 0, 3) * swizzle!(other.group0(), 2, 0, 1, 3))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       52       74        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                ((self.group0()[1] * other.group0()[2]) - (self.group0()[2] * other.group0()[1]) + (self.group0()[3] * other.group0()[0]) + (self.group1()[0] * other.group0()[3])),
                (-(self.group0()[0] * other.group0()[2])
                    + (self.group0()[2] * other.group0()[0])
                    + (self.group0()[3] * other.group0()[1])
                    + (self.group1()[1] * other.group0()[3])),
                ((self.group0()[0] * other.group0()[1]) - (self.group0()[1] * other.group0()[0]) + (self.group0()[3] * other.group0()[2]) + (self.group1()[2] * other.group0()[3])),
                (self.group0()[3] * other.group0()[3]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3]),
                (self.group0()[1] * other.group0()[3]),
                (self.group0()[2] * other.group0()[3]),
                (-(self.group0()[0] * other.group0()[0]) - (self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2])
                    + (self.group1()[3] * other.group0()[3])),
            ]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       40       54        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       40       55        0
    //  no simd       40       58        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[scalar]) * self.group0()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl InfixAntiSandwich for MultiVector {}
impl AntiSandwich<AntiScalar> for MultiVector {
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
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from(other[e1234]) * self.group0()),
            // e1, e2, e3, e4
            (Simd32x4::from(other[e1234]) * self.group1()),
            // e41, e42, e43
            (Simd32x3::from(other[e1234]) * self.group2()),
            // e23, e31, e12
            (Simd32x3::from(other[e1234]) * self.group3()),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e1234]) * self.group4()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      181      217        0
    //    simd3        1        3        0
    // Totals...
    // yes simd      182      220        0
    //  no simd      184      226        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                ((other.group0()[0] * self.group0()[1]) + (other.group0()[1] * self.group0()[0])),
                (other.group0()[1] * self.group0()[1]),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(other.group0()[0] * self.group4()[0]) + (other.group0()[1] * self.group1()[0])),
                (-(other.group0()[0] * self.group4()[1]) + (other.group0()[1] * self.group1()[1])),
                (-(other.group0()[0] * self.group4()[2]) + (other.group0()[1] * self.group1()[2])),
                (other.group0()[1] * self.group1()[3]),
            ]),
            // e41, e42, e43
            (Simd32x3::from(other.group0()[1]) * self.group2()),
            // e23, e31, e12
            ((Simd32x3::from(other.group0()[0]) * self.group2()) + (Simd32x3::from(other.group0()[1]) * self.group3())),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group0()[1] * self.group4()[0]),
                (other.group0()[1] * self.group4()[1]),
                (other.group0()[1] * self.group4()[2]),
                (-(other.group0()[0] * self.group1()[3]) + (other.group0()[1] * self.group4()[3])),
            ]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      211      252        0
    //    simd2        4        4        0
    //    simd3        6        6        0
    //    simd4        5        5        0
    // Totals...
    // yes simd      226      267        0
    //  no simd      257      298        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                (-(other.group0()[0] * self.group4()[0]) - (other.group0()[1] * self.group4()[1]) - (other.group0()[2] * self.group4()[2])
                    + (other.group1()[3] * self.group1()[3])),
                0.0,
            ]) - (Simd32x2::from(other.group0()[3]) * Simd32x2::from([self.group4()[3], self.group1()[3]]))
                + (Simd32x2::from(other.group1()[0]) * Simd32x2::from([self.group1()[0], self.group4()[0]]))
                + (Simd32x2::from(other.group1()[1]) * Simd32x2::from([self.group1()[1], self.group4()[1]]))
                + (Simd32x2::from(other.group1()[2]) * Simd32x2::from([self.group1()[2], self.group4()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group0()[0] * other.group1()[0]) + (self.group2()[0] * other.group1()[3]) + (self.group2()[1] * other.group0()[2]) - (self.group2()[2] * other.group0()[1])
                    + (self.group3()[0] * other.group0()[3])
                    + (self.group3()[1] * other.group1()[2])),
                ((self.group0()[0] * other.group1()[1]) - (self.group2()[0] * other.group0()[2])
                    + (self.group2()[1] * other.group1()[3])
                    + (self.group2()[2] * other.group0()[0])
                    + (self.group3()[1] * other.group0()[3])
                    + (self.group3()[2] * other.group1()[0])),
                ((self.group0()[0] * other.group1()[2]) + (self.group2()[0] * other.group0()[1]) - (self.group2()[1] * other.group0()[0])
                    + (self.group2()[2] * other.group1()[3])
                    + (self.group3()[0] * other.group1()[1])
                    + (self.group3()[2] * other.group0()[3])),
                (-(self.group2()[1] * other.group1()[1]) - (self.group2()[2] * other.group1()[2])),
            ]) + (Simd32x4::from(self.group0()[1]) * other.group0())
                - (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[0]]) * swizzle!(other.group1(), 1, 2, 0, 0))),
            // e41, e42, e43
            (Simd32x3::from([
                ((other.group1()[1] * self.group4()[2]) - (other.group1()[2] * self.group4()[1])),
                (-(other.group1()[0] * self.group4()[2]) + (other.group1()[2] * self.group4()[0])),
                ((other.group1()[0] * self.group4()[1]) - (other.group1()[1] * self.group4()[0])),
            ]) - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))),
            // e23, e31, e12
            (Simd32x3::from([
                (-(other.group0()[1] * self.group4()[2]) + (other.group0()[2] * self.group4()[1]) + (other.group1()[1] * self.group1()[2])
                    - (other.group1()[2] * self.group1()[1])),
                ((other.group0()[0] * self.group4()[2]) - (other.group0()[2] * self.group4()[0]) - (other.group1()[0] * self.group1()[2]) + (other.group1()[2] * self.group1()[0])),
                (-(other.group0()[0] * self.group4()[1]) + (other.group0()[1] * self.group4()[0]) + (other.group1()[0] * self.group1()[1])
                    - (other.group1()[1] * self.group1()[0])),
            ]) - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                (self.group2()[1] * other.group1()[2]),
                (self.group2()[2] * other.group1()[0]),
                (self.group2()[0] * other.group1()[1]),
                (-(self.group2()[0] * other.group0()[0])
                    - (self.group2()[1] * other.group0()[1])
                    - (self.group2()[2] * other.group0()[2])
                    - (self.group3()[1] * other.group1()[1])
                    - (self.group3()[2] * other.group1()[2])),
            ]) + (Simd32x4::from(self.group0()[1]) * other.group1())
                + (Simd32x4::from(other.group0()[3]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[0]]))
                - (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[0]]) * swizzle!(other.group1(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      176      207        0
    //    simd3        0        1        0
    // Totals...
    // yes simd      176      208        0
    //  no simd      176      210        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self.group1()[3] * other[e321]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(self.group2()[0] * other[e321]), (self.group2()[1] * other[e321]), (self.group2()[2] * other[e321]), 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]])),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[1] * other[e321])]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for MultiVector {
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
    fn anti_sandwich(self, other: Line) -> Self::Output {
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                (-(other.group1()[0] * self.group2()[0]) - (other.group1()[1] * self.group2()[1]) - (other.group1()[2] * self.group2()[2])),
                0.0,
            ]) - (Simd32x2::from(other.group0()[0]) * Simd32x2::from([self.group3()[0], self.group2()[0]]))
                - (Simd32x2::from(other.group0()[1]) * Simd32x2::from([self.group3()[1], self.group2()[1]]))
                - (Simd32x2::from(other.group0()[2]) * Simd32x2::from([self.group3()[2], self.group2()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group0()[0] * self.group4()[3]) - (other.group0()[1] * self.group1()[2]) + (other.group0()[2] * self.group1()[1]) - (other.group1()[0] * self.group1()[3])
                    + (other.group1()[1] * self.group4()[2])),
                ((other.group0()[0] * self.group1()[2]) + (other.group0()[1] * self.group4()[3]) - (other.group0()[2] * self.group1()[0]) - (other.group1()[1] * self.group1()[3])
                    + (other.group1()[2] * self.group4()[0])),
                (-(other.group0()[0] * self.group1()[1])
                    + (other.group0()[1] * self.group1()[0])
                    + (other.group0()[2] * self.group4()[3])
                    + (other.group1()[0] * self.group4()[1])
                    - (other.group1()[2] * self.group1()[3])),
                (-(other.group0()[1] * self.group4()[1]) - (other.group0()[2] * self.group4()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group4(), 1, 2, 0, 0))),
            // e41, e42, e43
            ((Simd32x3::from(self.group0()[1]) * other.group0()) - (swizzle!(other.group0(), 1, 2, 0) * swizzle!(self.group2(), 2, 0, 1))
                + (swizzle!(other.group0(), 2, 0, 1) * swizzle!(self.group2(), 1, 2, 0))),
            // e23, e31, e12
            ((Simd32x3::from(self.group0()[0]) * other.group0()) + (Simd32x3::from(self.group0()[1]) * other.group1())
                - (swizzle!(other.group0(), 1, 2, 0) * swizzle!(self.group3(), 2, 0, 1))
                + (swizzle!(other.group0(), 2, 0, 1) * swizzle!(self.group3(), 1, 2, 0))
                - (swizzle!(other.group1(), 1, 2, 0) * swizzle!(self.group2(), 2, 0, 1))
                + (swizzle!(other.group1(), 2, 0, 1) * swizzle!(self.group2(), 1, 2, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[0] * self.group1()[3]) - (other.group0()[1] * self.group4()[2])),
                ((other.group0()[1] * self.group1()[3]) - (other.group0()[2] * self.group4()[0])),
                (-(other.group0()[0] * self.group4()[1]) + (other.group0()[2] * self.group1()[3])),
                (-(other.group0()[0] * self.group1()[0]) - (other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2])
                    + (other.group1()[1] * self.group4()[1])
                    + (other.group1()[2] * self.group4()[2])),
            ]) + (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group4(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      192      226        0
    //    simd2        4        4        0
    //    simd3       10       12        0
    //    simd4        7        7        0
    // Totals...
    // yes simd      213      249        0
    //  no simd      258      298        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                ((self.group0()[1] * other.group1()[3]) - (self.group3()[0] * other.group0()[0]) - (self.group3()[1] * other.group0()[1]) - (self.group3()[2] * other.group0()[2])),
                0.0,
            ]) - (Simd32x2::from(self.group2()[0]) * Simd32x2::from([other.group1()[0], other.group0()[0]]))
                - (Simd32x2::from(self.group2()[1]) * Simd32x2::from([other.group1()[1], other.group0()[1]]))
                - (Simd32x2::from(self.group2()[2]) * Simd32x2::from([other.group1()[2], other.group0()[2]]))
                + (Simd32x2::from(other.group0()[3]) * self.group0())),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group0()[2] * self.group1()[1]) + (other.group0()[3] * self.group1()[0]) - (other.group1()[0] * self.group1()[3]) + (other.group1()[1] * self.group4()[2])),
                ((other.group0()[1] * self.group4()[3]) + (other.group0()[3] * self.group1()[1]) - (other.group1()[1] * self.group1()[3]) + (other.group1()[2] * self.group4()[0])),
                ((other.group0()[2] * self.group4()[3]) + (other.group0()[3] * self.group1()[2]) + (other.group1()[0] * self.group4()[1]) - (other.group1()[2] * self.group1()[3])),
                0.0,
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[1]]) * swizzle!(self.group4(), 1, 2, 0, 1))
                - (Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[3], other.group0()[2]]) * swizzle!(self.group4(), 0, 1, 2, 2))
                - (Simd32x4::from([self.group1()[2], self.group1()[0], self.group1()[1], self.group4()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))
                + (Simd32x4::from([self.group4()[3], self.group1()[2], self.group1()[0], self.group1()[3]]) * swizzle!(other.group0(), 0, 0, 1, 3))),
            // e41, e42, e43
            ((Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * swizzle!(self.group2(), 2, 0, 1))
                + (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[3]]) * swizzle!(self.group2(), 1, 2, 2))
                + (Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * swizzle!(self.group2(), 0, 1, 0))),
            // e23, e31, e12
            ((Simd32x3::from(self.group0()[0]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + (Simd32x3::from(self.group0()[1]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[0]]) * swizzle!(self.group3(), 2, 0, 1))
                + (Simd32x3::from([other.group0()[2], other.group0()[0], other.group0()[3]]) * swizzle!(self.group3(), 1, 2, 2))
                + (Simd32x3::from([other.group0()[3], other.group0()[3], other.group0()[1]]) * swizzle!(self.group3(), 0, 1, 0))
                - (Simd32x3::from([other.group1()[1], other.group1()[2], other.group1()[0]]) * swizzle!(self.group2(), 2, 0, 1))
                + (Simd32x3::from([other.group1()[2], other.group1()[0], other.group1()[3]]) * swizzle!(self.group2(), 1, 2, 2))
                + (Simd32x3::from([other.group1()[3], other.group1()[3], other.group1()[1]]) * swizzle!(self.group2(), 0, 1, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                (other.group0()[3] * self.group4()[0]),
                (other.group0()[1] * self.group1()[3]),
                (other.group0()[2] * self.group1()[3]),
                (-(other.group0()[1] * self.group1()[1]) - (other.group0()[2] * self.group1()[2])
                    + (other.group1()[1] * self.group4()[1])
                    + (other.group1()[2] * self.group4()[2])
                    - (other.group1()[3] * self.group1()[3])),
            ]) + (Simd32x4::from([other.group0()[2], other.group0()[3], other.group0()[3], other.group1()[0]]) * swizzle!(self.group4(), 1, 1, 2, 0))
                + (Simd32x4::from([self.group1()[3], self.group4()[2], self.group4()[0], self.group4()[3]]) * swizzle!(other.group0(), 0, 0, 1, 3))
                - (Simd32x4::from([self.group4()[2], self.group4()[0], self.group4()[1], self.group1()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for MultiVector {
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
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([
                ((other.group0()[1] * self.group0()[0])
                    - (other.group3()[0] * self.group2()[0])
                    - (other.group3()[1] * self.group2()[1])
                    - (other.group3()[2] * self.group2()[2])
                    - (other.group1()[0] * self.group4()[0])
                    - (other.group1()[1] * self.group4()[1])
                    - (other.group1()[2] * self.group4()[2])
                    + (other.group4()[3] * self.group1()[3])),
                0.0,
            ]) + (Simd32x2::from(self.group0()[1]) * other.group0())
                - (Simd32x2::from(other.group2()[0]) * Simd32x2::from([self.group3()[0], self.group2()[0]]))
                - (Simd32x2::from(other.group2()[1]) * Simd32x2::from([self.group3()[1], self.group2()[1]]))
                - (Simd32x2::from(other.group2()[2]) * Simd32x2::from([self.group3()[2], self.group2()[2]]))
                - (Simd32x2::from(other.group1()[3]) * Simd32x2::from([self.group4()[3], self.group1()[3]]))
                + (Simd32x2::from(other.group4()[0]) * Simd32x2::from([self.group1()[0], self.group4()[0]]))
                + (Simd32x2::from(other.group4()[1]) * Simd32x2::from([self.group1()[1], self.group4()[1]]))
                + (Simd32x2::from(other.group4()[2]) * Simd32x2::from([self.group1()[2], self.group4()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group0()[0] * other.group4()[0]) + (other.group2()[0] * self.group4()[3]) - (other.group2()[1] * self.group1()[2]) + (other.group2()[2] * self.group1()[1])
                    - (other.group3()[0] * self.group1()[3])
                    + (other.group3()[1] * self.group4()[2])
                    + (self.group2()[0] * other.group4()[3])
                    + (self.group2()[1] * other.group1()[2])
                    - (self.group2()[2] * other.group1()[1])
                    + (self.group3()[0] * other.group1()[3])
                    + (self.group3()[1] * other.group4()[2])),
                ((self.group0()[0] * other.group4()[1]) + (other.group2()[0] * self.group1()[2]) + (other.group2()[1] * self.group4()[3])
                    - (other.group2()[2] * self.group1()[0])
                    - (other.group3()[1] * self.group1()[3])
                    + (other.group3()[2] * self.group4()[0])
                    - (self.group2()[0] * other.group1()[2])
                    + (self.group2()[1] * other.group4()[3])
                    + (self.group2()[2] * other.group1()[0])
                    + (self.group3()[1] * other.group1()[3])
                    + (self.group3()[2] * other.group4()[0])),
                ((self.group0()[0] * other.group4()[2]) - (other.group2()[0] * self.group1()[1])
                    + (other.group2()[1] * self.group1()[0])
                    + (other.group2()[2] * self.group4()[3])
                    + (other.group3()[0] * self.group4()[1])
                    - (other.group3()[2] * self.group1()[3])
                    + (self.group2()[0] * other.group1()[1])
                    - (self.group2()[1] * other.group1()[0])
                    + (self.group2()[2] * other.group4()[3])
                    + (self.group3()[0] * other.group4()[1])
                    + (self.group3()[2] * other.group1()[3])),
                (-(other.group2()[2] * self.group4()[2]) - (self.group2()[1] * other.group4()[1]) - (self.group2()[2] * other.group4()[2])),
            ]) + (Simd32x4::from(other.group0()[1]) * self.group1())
                + (Simd32x4::from(self.group0()[1]) * other.group1())
                - (Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group2()[0]]) * swizzle!(self.group4(), 0, 1, 2, 0))
                - (Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[1]]) * swizzle!(self.group4(), 1, 2, 0, 1))
                - (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[0]]) * swizzle!(other.group4(), 1, 2, 0, 0))),
            // e41, e42, e43
            (Simd32x3::from([
                ((other.group4()[1] * self.group4()[2]) - (other.group4()[2] * self.group4()[1])),
                (-(other.group4()[0] * self.group4()[2]) + (other.group4()[2] * self.group4()[0])),
                ((other.group4()[0] * self.group4()[1]) - (other.group4()[1] * self.group4()[0])),
            ]) + (Simd32x3::from(other.group0()[1]) * self.group2())
                + (Simd32x3::from(self.group0()[1]) * other.group2())
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                - (swizzle!(other.group2(), 1, 2, 0) * swizzle!(self.group2(), 2, 0, 1))
                + (swizzle!(other.group2(), 2, 0, 1) * swizzle!(self.group2(), 1, 2, 0))),
            // e23, e31, e12
            (Simd32x3::from([
                (-(other.group1()[1] * self.group4()[2]) + (other.group1()[2] * self.group4()[1]) + (other.group4()[1] * self.group1()[2])
                    - (other.group4()[2] * self.group1()[1])),
                ((other.group1()[0] * self.group4()[2]) - (other.group1()[2] * self.group4()[0]) - (other.group4()[0] * self.group1()[2]) + (other.group4()[2] * self.group1()[0])),
                (-(other.group1()[0] * self.group4()[1]) + (other.group1()[1] * self.group4()[0]) + (other.group4()[0] * self.group1()[1])
                    - (other.group4()[1] * self.group1()[0])),
            ]) + (Simd32x3::from(other.group0()[0]) * self.group2())
                + (Simd32x3::from(other.group0()[1]) * self.group3())
                + (Simd32x3::from(self.group0()[0]) * other.group2())
                + (Simd32x3::from(self.group0()[1]) * other.group3())
                - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))
                + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))
                + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))
                - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))
                - (swizzle!(other.group2(), 1, 2, 0) * swizzle!(self.group3(), 2, 0, 1))
                + (swizzle!(other.group2(), 2, 0, 1) * swizzle!(self.group3(), 1, 2, 0))
                - (swizzle!(other.group3(), 1, 2, 0) * swizzle!(self.group2(), 2, 0, 1))
                + (swizzle!(other.group3(), 2, 0, 1) * swizzle!(self.group2(), 1, 2, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group2()[0] * self.group1()[3]) - (other.group2()[1] * self.group4()[2]) + (self.group2()[1] * other.group4()[2])),
                ((other.group2()[1] * self.group1()[3]) - (other.group2()[2] * self.group4()[0]) + (self.group2()[2] * other.group4()[0])),
                (-(other.group2()[0] * self.group4()[1]) + (other.group2()[2] * self.group1()[3]) + (self.group2()[0] * other.group4()[1])),
                (-(other.group0()[0] * self.group1()[3])
                    - (other.group2()[0] * self.group1()[0])
                    - (other.group2()[1] * self.group1()[1])
                    - (other.group2()[2] * self.group1()[2])
                    + (other.group3()[1] * self.group4()[1])
                    + (other.group3()[2] * self.group4()[2])
                    - (self.group2()[0] * other.group1()[0])
                    - (self.group2()[1] * other.group1()[1])
                    - (self.group2()[2] * other.group1()[2])
                    - (self.group3()[1] * other.group4()[1])
                    - (self.group3()[2] * other.group4()[2])),
            ]) + (Simd32x4::from(other.group0()[1]) * self.group4())
                + (Simd32x4::from(self.group0()[1]) * other.group4())
                + (Simd32x4::from(other.group1()[3]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[0]]))
                + (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[0]]) * swizzle!(self.group4(), 1, 2, 0, 0))
                - (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[0]]) * swizzle!(other.group4(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      176      202        0
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd      176      210        0
    //  no simd      176      226        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from(other[e4]) * Simd32x2::from([self.group4()[3], self.group1()[3]]) * Simd32x2::from(-1.0)),
            // e1, e2, e3, e4
            (Simd32x4::from(other[e4]) * Simd32x4::from([self.group3()[0], self.group3()[1], self.group3()[2], self.group0()[1]])),
            // e41, e42, e43
            (Simd32x3::from(other[e4]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from(other[e4]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            (Simd32x4::from(other[e4]) * Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], self.group0()[0]])),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      190      231        0
    //    simd2        3        3        0
    //    simd3        3        3        0
    //    simd4        3        3        0
    // Totals...
    // yes simd      199      240        0
    //  no simd      217      258        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from([(self.group1()[3] * other.group0()[3]), 0.0])
                + (Simd32x2::from(other.group0()[0]) * Simd32x2::from([self.group1()[0], self.group4()[0]]))
                + (Simd32x2::from(other.group0()[1]) * Simd32x2::from([self.group1()[1], self.group4()[1]]))
                + (Simd32x2::from(other.group0()[2]) * Simd32x2::from([self.group1()[2], self.group4()[2]]))),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((self.group0()[0] * other.group0()[0]) + (self.group2()[0] * other.group0()[3]) + (self.group3()[1] * other.group0()[2])),
                ((self.group0()[0] * other.group0()[1]) + (self.group2()[1] * other.group0()[3]) + (self.group3()[2] * other.group0()[0])),
                ((self.group0()[0] * other.group0()[2]) + (self.group2()[2] * other.group0()[3]) + (self.group3()[0] * other.group0()[1])),
                (-(self.group2()[1] * other.group0()[1]) - (self.group2()[2] * other.group0()[2])),
            ]) - (Simd32x4::from([self.group3()[2], self.group3()[0], self.group3()[1], self.group2()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
            // e41, e42, e43
            (Simd32x3::from([
                (-(self.group4()[1] * other.group0()[2]) + (self.group4()[2] * other.group0()[1])),
                ((self.group4()[0] * other.group0()[2]) - (self.group4()[2] * other.group0()[0])),
                (-(self.group4()[0] * other.group0()[1]) + (self.group4()[1] * other.group0()[0])),
            ]) - (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))),
            // e23, e31, e12
            (Simd32x3::from([
                (-(self.group1()[1] * other.group0()[2]) + (self.group1()[2] * other.group0()[1])),
                ((self.group1()[0] * other.group0()[2]) - (self.group1()[2] * other.group0()[0])),
                (-(self.group1()[0] * other.group0()[1]) + (self.group1()[1] * other.group0()[0])),
            ]) - (Simd32x3::from(self.group4()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                + (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                (self.group2()[1] * other.group0()[2]),
                (self.group2()[2] * other.group0()[0]),
                (self.group2()[0] * other.group0()[1]),
                (-(self.group3()[1] * other.group0()[1]) - (self.group3()[2] * other.group0()[2])),
            ]) + (Simd32x4::from(self.group0()[1]) * other.group0())
                - (Simd32x4::from([self.group2()[2], self.group2()[0], self.group2()[1], self.group3()[0]]) * swizzle!(other.group0(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      194      234        0
    //    simd3        2        4        0
    // Totals...
    // yes simd      196      238        0
    //  no simd      200      246        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (-(self.group4()[0] * other.group0()[0])
                    - (self.group4()[1] * other.group0()[1])
                    - (self.group4()[2] * other.group0()[2])
                    - (self.group4()[3] * other.group0()[3])),
                (self.group1()[3] * other.group0()[3] * -1.0),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                ((self.group0()[1] * other.group0()[0]) + (self.group2()[1] * other.group0()[2]) - (self.group2()[2] * other.group0()[1]) + (self.group3()[0] * other.group0()[3])),
                ((self.group0()[1] * other.group0()[1]) - (self.group2()[0] * other.group0()[2]) + (self.group2()[2] * other.group0()[0]) + (self.group3()[1] * other.group0()[3])),
                ((self.group0()[1] * other.group0()[2]) + (self.group2()[0] * other.group0()[1]) - (self.group2()[1] * other.group0()[0]) + (self.group3()[2] * other.group0()[3])),
                (self.group0()[1] * other.group0()[3]),
            ]),
            // e41, e42, e43
            (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group4()[0], self.group4()[1], self.group4()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([
                ((self.group4()[1] * other.group0()[2]) - (self.group4()[2] * other.group0()[1])),
                (-(self.group4()[0] * other.group0()[2]) + (self.group4()[2] * other.group0()[0])),
                ((self.group4()[0] * other.group0()[1]) - (self.group4()[1] * other.group0()[0])),
            ]) + (Simd32x3::from(self.group1()[3]) * Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]))
                - (Simd32x3::from(other.group0()[3]) * Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]))),
            // e423, e431, e412, e321
            Simd32x4::from([
                (self.group2()[0] * other.group0()[3]),
                (self.group2()[1] * other.group0()[3]),
                (self.group2()[2] * other.group0()[3]),
                ((self.group0()[0] * other.group0()[3]) - (self.group2()[0] * other.group0()[0]) - (self.group2()[1] * other.group0()[1]) - (self.group2()[2] * other.group0()[2])),
            ]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      176      211        0
    //    simd3        0        1        0
    // Totals...
    // yes simd      176      212        0
    //  no simd      176      214        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(self.group0()[1] * other[scalar]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group4()[0] * other[scalar] * -1.0),
                (self.group4()[1] * other[scalar] * -1.0),
                (self.group4()[2] * other[scalar] * -1.0),
                0.0,
            ]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group2()),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (self.group1()[3] * other[scalar] * -1.0)]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl InfixAntiSandwich for Origin {}
impl AntiSandwich<AntiScalar> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Origin::from_groups(/* e4 */ (other[e1234] * self[e4]));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       20        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * self[e4])]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * self[e4] * -1.0)]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        9        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       21        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self[e4]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) * Simd32x4::from(-1.0)),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[e4]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]])),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Origin {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Scalar::from_groups(/* scalar */ (other[e321] * self[e4]));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       26        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([(other.group1()[0] * self[e4] * -1.0), (other.group1()[1] * self[e4] * -1.0), (other.group1()[2] * self[e4] * -1.0), 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([(other.group0()[0] * self[e4]), (other.group0()[1] * self[e4]), (other.group0()[2] * self[e4]), 0.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       17        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       21        0
    //  no simd        0       33        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(self[e4]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])),
            // e423, e431, e412, e321
            (Simd32x4::from(self[e4]) * Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Origin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       25        0
    //    simd2        0        2        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       34        0
    //  no simd        0       54        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            (Simd32x2::from(self[e4]) * Simd32x2::from([other.group4()[3], other.group1()[3]]) * Simd32x2::from([1.0, -1.0])),
            // e1, e2, e3, e4
            (Simd32x4::from(self[e4]) * Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group0()[1]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0])),
            // e41, e42, e43
            (Simd32x3::from(self[e4]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from(self[e4]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]])),
            // e423, e431, e412, e321
            (Simd32x4::from(self[e4]) * Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], other.group0()[0]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0])),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Origin {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ (other[e4] * self[e4] * -1.0));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       16        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([(other.group0()[0] * self[e4] * -1.0), (other.group0()[1] * self[e4] * -1.0), (other.group0()[2] * self[e4] * -1.0), 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * self[e4])]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       14        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * self[e4] * -1.0)]),
            // e23, e31, e12, scalar
            Simd32x4::from([(other.group0()[0] * self[e4]), (other.group0()[1] * self[e4]), (other.group0()[2] * self[e4]), 0.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Origin {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Horizon::from_groups(/* e321 */ (self[e4] * other[scalar] * -1.0));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl InfixAntiSandwich for Plane {}
impl AntiSandwich<AntiScalar> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       15        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       16        0
    //  no simd        8       19        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Plane::from_groups(/* e423, e431, e412, e321 */ (Simd32x4::from(other[e1234]) * self.group0()));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       34        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       20       35        0
    //  no simd       20       38        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (other.group0()[0] * self.group0()[0] * -1.0),
                (other.group0()[0] * self.group0()[1] * -1.0),
                (other.group0()[0] * self.group0()[2] * -1.0),
                0.0,
            ]),
            // e423, e431, e412, e321
            (Simd32x4::from(other.group0()[1]) * self.group0()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       31       47        0
    //  no simd       40       56        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                (-(other.group0()[3] * self.group0()[0]) - (other.group1()[2] * self.group0()[1])),
                (-(other.group0()[3] * self.group0()[1]) - (other.group1()[0] * self.group0()[2])),
                (-(other.group0()[3] * self.group0()[2]) - (other.group1()[1] * self.group0()[0])),
                ((other.group1()[1] * self.group0()[1]) + (other.group1()[2] * self.group0()[2])),
            ]) + (swizzle!(other.group1(), 1, 2, 0, 0) * swizzle!(self.group0(), 2, 0, 1, 0))),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((other.group0()[2] * self.group0()[1]) + (other.group1()[3] * self.group0()[0])),
                ((other.group0()[0] * self.group0()[2]) + (other.group1()[3] * self.group0()[1])),
                ((other.group0()[1] * self.group0()[0]) + (other.group1()[3] * self.group0()[2])),
                (-(other.group0()[2] * self.group0()[2]) - (other.group0()[3] * self.group0()[3])),
            ]) - (Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[1]]) * swizzle!(self.group0(), 3, 3, 3, 1))
                - (swizzle!(other.group0(), 1, 2, 0, 0) * swizzle!(self.group0(), 2, 0, 1, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       21        0
    //    simd3        0        1        0
    // Totals...
    // yes simd       13       22        0
    //  no simd       13       24        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(other[e321]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]])),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       44        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       27       46        0
    //  no simd       33       52        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group0()[0] * self.group0()[3]) + (other.group1()[1] * self.group0()[2])),
                ((other.group0()[1] * self.group0()[3]) + (other.group1()[2] * self.group0()[0])),
                ((other.group0()[2] * self.group0()[3]) + (other.group1()[0] * self.group0()[1])),
                (-(other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2])),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[2]) * -1.0),
                ((other.group0()[2] * self.group0()[0]) * -1.0),
                ((other.group0()[0] * self.group0()[1]) * -1.0),
                ((other.group1()[1] * self.group0()[1]) + (other.group1()[2] * self.group0()[2])),
            ]) + (Simd32x4::from([other.group0()[2], other.group0()[0], other.group0()[1], other.group1()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       44        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       28       48        0
    //  no simd       40       60        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group0()[0] * self.group0()[3]) + (other.group1()[1] * self.group0()[2])),
                ((other.group0()[1] * self.group0()[3]) + (other.group1()[2] * self.group0()[0])),
                ((other.group0()[2] * self.group0()[3]) + (other.group1()[0] * self.group0()[1])),
                ((other.group0()[2] * self.group0()[2]) * -1.0),
            ]) - (Simd32x4::from([other.group1()[2], other.group1()[0], other.group1()[1], other.group0()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))
                - (Simd32x4::from([other.group1()[3], other.group1()[3], other.group1()[3], other.group0()[1]]) * swizzle!(self.group0(), 0, 1, 2, 1))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group0()[1] * self.group0()[2]) * -1.0),
                ((other.group0()[2] * self.group0()[0]) * -1.0),
                ((other.group0()[0] * self.group0()[1]) * -1.0),
                ((other.group1()[1] * self.group0()[1]) + (other.group1()[2] * self.group0()[2])),
            ]) + (Simd32x4::from([other.group0()[3], other.group0()[3], other.group0()[3], other.group1()[0]]) * swizzle!(self.group0(), 0, 1, 2, 0))
                + (swizzle!(other.group0(), 2, 0, 1, 3) * swizzle!(self.group0(), 1, 2, 0, 3))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Plane {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       55       91        0
    //    simd3        3        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       62       98        0
    //  no simd       80      116        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                (-(other.group1()[0] * self.group0()[0])
                    - (other.group1()[1] * self.group0()[1])
                    - (other.group1()[2] * self.group0()[2])
                    - (other.group1()[3] * self.group0()[3])),
                ((other.group4()[0] * self.group0()[0]) + (other.group4()[1] * self.group0()[1]) + (other.group4()[2] * self.group0()[2])),
            ]),
            // e1, e2, e3, e4
            (Simd32x4::from([
                ((other.group2()[0] * self.group0()[3]) + (other.group3()[1] * self.group0()[2])),
                ((other.group2()[1] * self.group0()[3]) + (other.group3()[2] * self.group0()[0])),
                ((other.group2()[2] * self.group0()[3]) + (other.group3()[0] * self.group0()[1])),
                ((other.group2()[2] * self.group0()[2]) * -1.0),
            ]) - (Simd32x4::from([other.group0()[0], other.group0()[0], other.group0()[0], other.group2()[0]]) * swizzle!(self.group0(), 0, 1, 2, 0))
                - (Simd32x4::from([other.group3()[2], other.group3()[0], other.group3()[1], other.group2()[1]]) * swizzle!(self.group0(), 1, 2, 0, 1))),
            // e41, e42, e43
            (Simd32x3::from([
                ((other.group4()[1] * self.group0()[2]) - (other.group4()[2] * self.group0()[1])),
                (-(other.group4()[0] * self.group0()[2]) + (other.group4()[2] * self.group0()[0])),
                ((other.group4()[0] * self.group0()[1]) - (other.group4()[1] * self.group0()[0])),
            ]) - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))),
            // e23, e31, e12
            (Simd32x3::from([
                (-(other.group1()[1] * self.group0()[2]) + (other.group1()[2] * self.group0()[1])),
                ((other.group1()[0] * self.group0()[2]) - (other.group1()[2] * self.group0()[0])),
                (-(other.group1()[0] * self.group0()[1]) + (other.group1()[1] * self.group0()[0])),
            ]) + (Simd32x3::from(other.group4()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                - (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]))),
            // e423, e431, e412, e321
            (Simd32x4::from([
                ((other.group2()[1] * self.group0()[2]) * -1.0),
                ((other.group2()[2] * self.group0()[0]) * -1.0),
                ((other.group2()[0] * self.group0()[1]) * -1.0),
                ((other.group3()[1] * self.group0()[1]) + (other.group3()[2] * self.group0()[2])),
            ]) + (Simd32x4::from(other.group0()[1]) * self.group0())
                + (Simd32x4::from([other.group2()[2], other.group2()[0], other.group2()[1], other.group3()[0]]) * swizzle!(self.group0(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       20       36        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([(self.group0()[0] * other[e4] * -1.0), (self.group0()[1] * other[e4] * -1.0), (self.group0()[2] * other[e4] * -1.0), 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * other[e4] * -1.0)]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       42        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       43        0
    //  no simd       28       46        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from([
                ((other.group0()[2] * self.group0()[1]) * -1.0),
                ((other.group0()[0] * self.group0()[2]) * -1.0),
                ((other.group0()[1] * self.group0()[0]) * -1.0),
                ((other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2])),
            ]) + (swizzle!(other.group0(), 1, 2, 0, 0) * swizzle!(self.group0(), 2, 0, 1, 0))),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (-(other.group0()[0] * self.group0()[3]) + (other.group0()[3] * self.group0()[0])),
                (-(other.group0()[1] * self.group0()[3]) + (other.group0()[3] * self.group0()[1])),
                (-(other.group0()[2] * self.group0()[3]) + (other.group0()[3] * self.group0()[2])),
                0.0,
            ]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       40        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       23       41        0
    //  no simd       26       44        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self.group0()[0] * other.group0()[3] * -1.0),
                (self.group0()[1] * other.group0()[3] * -1.0),
                (self.group0()[2] * other.group0()[3] * -1.0),
                0.0,
            ]),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                (self.group0()[1] * other.group0()[2]),
                (self.group0()[2] * other.group0()[0]),
                (self.group0()[0] * other.group0()[1]),
                (-(self.group0()[1] * other.group0()[1]) - (self.group0()[2] * other.group0()[2]) - (self.group0()[3] * other.group0()[3])),
            ]) - (swizzle!(self.group0(), 2, 0, 1, 0) * swizzle!(other.group0(), 1, 2, 0, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6       22        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([
            (self.group0()[0] * other[scalar] * -1.0),
            (self.group0()[1] * other[scalar] * -1.0),
            (self.group0()[2] * other[scalar] * -1.0),
            0.0,
        ]));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl InfixAntiSandwich for Point {}
impl AntiSandwich<AntiScalar> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       12        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3       13        0
    //  no simd        3       16        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ (Simd32x4::from(other[e1234]) * self.group0()));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       30        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       12       31        0
    //  no simd       12       34        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            (Simd32x4::from(other.group0()[1]) * self.group0()),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * self.group0()[3] * -1.0)]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       32        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       18       36        0
    //  no simd       24       48        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            (Simd32x4::from(self.group0()[3]) * Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], other.group0()[3]]) * Simd32x4::from(-1.0)),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                (-(other.group0()[3] * self.group0()[0]) - (other.group1()[2] * self.group0()[1])),
                (-(other.group0()[3] * self.group0()[1]) - (other.group1()[0] * self.group0()[2])),
                (-(other.group0()[3] * self.group0()[2]) - (other.group1()[1] * self.group0()[0])),
                ((other.group1()[2] * self.group0()[2]) + (other.group1()[3] * self.group0()[3])),
            ]) + (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[0]]) * swizzle!(self.group0(), 3, 3, 3, 0))
                + (swizzle!(other.group1(), 1, 2, 0, 1) * swizzle!(self.group0(), 2, 0, 1, 1))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Point {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Scalar::from_groups(/* scalar */ (self.group0()[3] * other[e321]));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       20       43        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(other.group0()[1] * self.group0()[2]) + (other.group0()[2] * self.group0()[1]) - (other.group1()[0] * self.group0()[3])),
                ((other.group0()[0] * self.group0()[2]) - (other.group0()[2] * self.group0()[0]) - (other.group1()[1] * self.group0()[3])),
                (-(other.group0()[0] * self.group0()[1]) + (other.group0()[1] * self.group0()[0]) - (other.group1()[2] * self.group0()[3])),
                0.0,
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group0()[0] * self.group0()[3]),
                (other.group0()[1] * self.group0()[3]),
                (other.group0()[2] * self.group0()[3]),
                (-(other.group0()[0] * self.group0()[0]) - (other.group0()[1] * self.group0()[1]) - (other.group0()[2] * self.group0()[2])),
            ]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       48        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (-(other.group0()[1] * self.group0()[2]) + (other.group0()[2] * self.group0()[1]) + (other.group0()[3] * self.group0()[0])
                    - (other.group1()[0] * self.group0()[3])),
                ((other.group0()[0] * self.group0()[2]) - (other.group0()[2] * self.group0()[0]) + (other.group0()[3] * self.group0()[1]) - (other.group1()[1] * self.group0()[3])),
                (-(other.group0()[0] * self.group0()[1]) + (other.group0()[1] * self.group0()[0]) + (other.group0()[3] * self.group0()[2])
                    - (other.group1()[2] * self.group0()[3])),
                (other.group0()[3] * self.group0()[3]),
            ]),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group0()[0] * self.group0()[3]),
                (other.group0()[1] * self.group0()[3]),
                (other.group0()[2] * self.group0()[3]),
                (-(other.group0()[0] * self.group0()[0])
                    - (other.group0()[1] * self.group0()[1])
                    - (other.group0()[2] * self.group0()[2])
                    - (other.group1()[3] * self.group0()[3])),
            ]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       42       80        0
    //    simd3        2        4        0
    // Totals...
    // yes simd       44       84        0
    //  no simd       48       92        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([
                ((other.group4()[0] * self.group0()[0]) + (other.group4()[1] * self.group0()[1]) + (other.group4()[2] * self.group0()[2]) + (other.group4()[3] * self.group0()[3])),
                (other.group1()[3] * self.group0()[3] * -1.0),
            ]),
            // e1, e2, e3, e4
            Simd32x4::from([
                ((other.group0()[1] * self.group0()[0]) - (other.group2()[1] * self.group0()[2]) + (other.group2()[2] * self.group0()[1]) - (other.group3()[0] * self.group0()[3])),
                ((other.group0()[1] * self.group0()[1]) + (other.group2()[0] * self.group0()[2]) - (other.group2()[2] * self.group0()[0]) - (other.group3()[1] * self.group0()[3])),
                ((other.group0()[1] * self.group0()[2]) - (other.group2()[0] * self.group0()[1]) + (other.group2()[1] * self.group0()[0]) - (other.group3()[2] * self.group0()[3])),
                (other.group0()[1] * self.group0()[3]),
            ]),
            // e41, e42, e43
            (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group4()[0], other.group4()[1], other.group4()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([
                ((other.group4()[1] * self.group0()[2]) - (other.group4()[2] * self.group0()[1])),
                (-(other.group4()[0] * self.group0()[2]) + (other.group4()[2] * self.group0()[0])),
                ((other.group4()[0] * self.group0()[1]) - (other.group4()[1] * self.group0()[0])),
            ]) - (Simd32x3::from(other.group1()[3]) * Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]))
                + (Simd32x3::from(self.group0()[3]) * Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]))),
            // e423, e431, e412, e321
            Simd32x4::from([
                (other.group2()[0] * self.group0()[3]),
                (other.group2()[1] * self.group0()[3]),
                (other.group2()[2] * self.group0()[3]),
                (-(other.group0()[0] * self.group0()[3])
                    - (other.group2()[0] * self.group0()[0])
                    - (other.group2()[1] * self.group0()[1])
                    - (other.group2()[2] * self.group0()[2])),
            ]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       12       32        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (self.group0()[3] * other[e4] * -1.0)]),
            // e23, e31, e12, scalar
            Simd32x4::from([(self.group0()[0] * other[e4] * -1.0), (self.group0()[1] * other[e4] * -1.0), (self.group0()[2] * other[e4] * -1.0), 0.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       39        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       15       40        0
    //  no simd       18       43        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other.group0()[0] * self.group0()[3] * -1.0),
                (other.group0()[1] * self.group0()[3] * -1.0),
                (other.group0()[2] * self.group0()[3] * -1.0),
                0.0,
            ]),
            // e23, e31, e12, scalar
            (Simd32x4::from([
                ((other.group0()[2] * self.group0()[1]) * -1.0),
                ((other.group0()[0] * self.group0()[2]) * -1.0),
                ((other.group0()[1] * self.group0()[0]) * -1.0),
                ((other.group0()[1] * self.group0()[1]) + (other.group0()[2] * self.group0()[2]) + (other.group0()[3] * self.group0()[3])),
            ]) + (swizzle!(other.group0(), 1, 2, 0, 0) * swizzle!(self.group0(), 2, 0, 1, 0))),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       32        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * self.group0()[3] * -1.0)]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                ((other.group0()[0] * self.group0()[3]) - (other.group0()[3] * self.group0()[0])),
                ((other.group0()[1] * self.group0()[3]) - (other.group0()[3] * self.group0()[1])),
                ((other.group0()[2] * self.group0()[3]) - (other.group0()[3] * self.group0()[2])),
                0.0,
            ]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Point {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        8        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Horizon::from_groups(/* e321 */ (self.group0()[3] * other[scalar] * -1.0));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl InfixAntiSandwich for Scalar {}
impl AntiSandwich<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0       12        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([(other.group1()[0] * self[scalar]), (other.group1()[1] * self[scalar]), (other.group1()[2] * self[scalar]), 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * self[scalar])]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ (Simd32x3::from(self[scalar]) * other.group0()));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[scalar]) * other.group0()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       17        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0       18        0
    //  no simd        0       20        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([(other.group0()[1] * self[scalar]), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group4()[0] * self[scalar]), (other.group4()[1] * self[scalar]), (other.group4()[2] * self[scalar]), 0.0]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group2()),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * self[scalar])]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        5        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([
            (other.group0()[0] * self[scalar]),
            (other.group0()[1] * self[scalar]),
            (other.group0()[2] * self[scalar]),
            0.0,
        ]));
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
