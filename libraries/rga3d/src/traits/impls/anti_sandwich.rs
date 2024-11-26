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
//   Median:        17      31       0
//  Average:        34      48       0
//  Maximum:       260     293       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       2       0
//   Median:        19      40       0
//  Average:        44      65       0
//  Maximum:       354     394       0
impl std::ops::Div<anti_sandwich> for AntiScalar {
    type Output = anti_sandwich_partial<AntiScalar>;
    fn div(self, _rhs: anti_sandwich) -> Self::Output {
        anti_sandwich_partial(self)
    }
}
impl AntiSandwich<AntiScalar> for AntiScalar {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ other[e1234] * self[e1234]);
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for AntiScalar {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        2        0
    // no simd        0        4        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(self[e1234]) * other.group0());
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for AntiScalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group1(),
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
        let geometric_anti_product = Horizon::from_groups(/* e321 */ self[e1234] * other[e321]);
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for AntiScalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        4        0
    // no simd        0       12        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            Simd32x3::from(self[e1234]) * other.group1(),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for AntiScalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e1234]) * other.group1(),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for AntiScalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        2        0
    //    simd3        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       32        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        let geometric_anti_product = Origin::from_groups(/* e4 */ self[e1234] * other[e4]);
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for AntiScalar {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(self[e1234]) * other.group0());
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for AntiScalar {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(self[e1234]) * other.group0());
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
        let geometric_anti_product = Scalar::from_groups(/* scalar */ self[e1234] * other[scalar]);
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<anti_sandwich> for DualNum {
    type Output = anti_sandwich_partial<DualNum>;
    fn div(self, _rhs: anti_sandwich) -> Self::Output {
        anti_sandwich_partial(self)
    }
}
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
        let geometric_anti_product = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(other[e1234]) * self.group0());
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = DualNum::from_groups(
            // scalar, e1234
            Simd32x2::from([(other[scalar] * self[e1234]) + (other[e1234] * self[scalar]), other[e1234] * self[e1234]]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       24        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
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
        let geometric_anti_product = Horizon::from_groups(/* e321 */ self[e1234] * other[e321]);
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        6        0
    // no simd        6       18        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(self[e1234]) * other.group0(),
            // e23, e31, e12
            (Simd32x3::from(self[scalar]) * other.group0()) + (Simd32x3::from(self[e1234]) * other.group1()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        6        0
    // no simd        8       24        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e1234]) * other.group0(),
            // e23, e31, e12, scalar
            (Simd32x4::from(self[scalar]) * other.group0()) + (Simd32x4::from(self[e1234]) * other.group1()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       30        0
    //    simd3        2        6        0
    // Totals...
    // yes simd       12       36        0
    //  no simd       16       48        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       14        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       16        0
    //  no simd        4       22        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, self[e1234] * other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, self[scalar] * other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       15        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       17        0
    //  no simd        4       23        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[scalar] * other[e423], self[scalar] * other[e431], self[scalar] * other[e412], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e1234]) * other.group0(),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       13        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        4       15        0
    //  no simd        4       21        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e1234]) * other.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, self[scalar] * other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
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
        let geometric_anti_product = Scalar::from_groups(/* scalar */ self[e1234] * other[scalar]);
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<anti_sandwich> for Flector {
    type Output = anti_sandwich_partial<Flector>;
    fn div(self, _rhs: anti_sandwich) -> Self::Output {
        anti_sandwich_partial(self)
    }
}
impl AntiSandwich<AntiScalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       31       42        0
    //  no simd       40       60        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group1(),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       35       52        0
    //  no simd       44       64        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        6        7        0
    // Totals...
    // yes simd       62       79        0
    //  no simd       80      100        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       31       41        0
    //  no simd       40       56        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e321]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       59       76        0
    //  no simd       68       88        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       65       82        0
    //  no simd       80      100        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Flector {
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
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        8        0
    // Totals...
    // yes simd       31       44        0
    //  no simd       40       68        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e4]) * Simd32x4::from([self[e423], self[e431], self[e412], self[e4]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e4]) * Simd32x4::from([self[e1], self[e2], self[e3], self[e321]]) * Simd32x4::from(-1.0),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4        4        5        0
    // Totals...
    // yes simd       48       65        0
    //  no simd       60       80        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Flector {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       48        0
    //    simd4        4        7        0
    // Totals...
    // yes simd       40       55        0
    //  no simd       52       76        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Flector {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       40        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       31       46        0
    //  no simd       40       64        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e423] * other[scalar], self[e431] * other[scalar], self[e412] * other[scalar], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, self[e4] * other[scalar]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<anti_sandwich> for Horizon {
    type Output = anti_sandwich_partial<Horizon>;
    fn div(self, _rhs: anti_sandwich) -> Self::Output {
        anti_sandwich_partial(self)
    }
}
impl AntiSandwich<Flector> for Horizon {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       20        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e321]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]]) * Simd32x4::from(-1.0),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Horizon {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321] * other[e41], self[e321] * other[e42], self[e321] * other[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Horizon {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       16        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e321] * other[e41], self[e321] * other[e42], self[e321] * other[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, self[e321] * other[e1234]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Horizon {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd2        0        2        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       19        0
    //  no simd        0       39        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Horizon {
    type Output = Point;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       13        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(self[e321]) * Simd32x3::from([other[e423], other[e431], other[e412]]) * Simd32x3::from(-1.0),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<anti_sandwich> for Line {
    type Output = anti_sandwich_partial<Line>;
    fn div(self, _rhs: anti_sandwich) -> Self::Output {
        anti_sandwich_partial(self)
    }
}
impl AntiSandwich<AntiScalar> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        0
    //    simd3        0        4        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       19       39        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            Simd32x3::from(other[e1234]) * self.group1(),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        0
    //    simd3        1        5        0
    // Totals...
    // yes simd       20       32        0
    //  no simd       22       42        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Line::from_groups(
            // e41, e42, e43
            Simd32x3::from(other[e1234]) * self.group0(),
            // e23, e31, e12
            (Simd32x3::from(other[scalar]) * self.group0()) + (Simd32x3::from(other[e1234]) * self.group1()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       56       74        0
    //  no simd       56       78        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       18        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        8       21        0
    //  no simd        8       28        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e321] * self[e41], other[e321] * self[e42], other[e321] * self[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       47       63        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       47       65        0
    //  no simd       47       69        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       56       74        0
    //  no simd       56       78        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       84      114        0
    //    simd2        6        6        0
    //    simd3        6        8        0
    // Totals...
    // yes simd       96      128        0
    //  no simd      114      150        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       42        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       28       46        0
    //  no simd       28       56        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e23] * other[e4], self[e31] * other[e4], self[e12] * other[e4], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([self[e41] * other[e4], self[e42] * other[e4], self[e43] * other[e4], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       41       57        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       41       59        0
    //  no simd       41       63        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       51        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       36       53        0
    //  no simd       36       57        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       27        0
    //    simd3        0        3        0
    // Totals...
    // yes simd       19       30        0
    //  no simd       19       36        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(other[scalar]) * self.group0());
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<anti_sandwich> for Motor {
    type Output = anti_sandwich_partial<Motor>;
    fn div(self, _rhs: anti_sandwich) -> Self::Output {
        anti_sandwich_partial(self)
    }
}
impl AntiSandwich<AntiScalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        7        0
    // Totals...
    // yes simd       31       43        0
    //  no simd       40       64        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            Simd32x4::from(other[e1234]) * self.group1(),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        4        8        0
    // Totals...
    // yes simd       32       44        0
    //  no simd       44       68        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(other[e1234]) * self.group0(),
            // e23, e31, e12, scalar
            (Simd32x4::from(other[scalar]) * self.group0()) + (Simd32x4::from(other[e1234]) * self.group1()),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       60       76        0
    //    simd4        5        7        0
    // Totals...
    // yes simd       65       83        0
    //  no simd       80      104        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       44        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       34       50        0
    //  no simd       40       68        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e321] * self[e41], other[e321] * self[e42], other[e321] * self[e43], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e321] * self[e1234]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       59       77        0
    //  no simd       68       92        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       56       72        0
    //    simd4        6        8        0
    // Totals...
    // yes simd       62       80        0
    //  no simd       80      104        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Motor {
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
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       40        0
    //    simd4        2        6        0
    // Totals...
    // yes simd       34       46        0
    //  no simd       40       64        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e4]) * Simd32x4::from([self[e23], self[e31], self[e12], self[e1234]]),
            // e423, e431, e412, e321
            Simd32x4::from(other[e4]) * Simd32x4::from([self[e41], self[e42], self[e43], self[scalar]]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       48       64        0
    //    simd4        3        5        0
    // Totals...
    // yes simd       51       69        0
    //  no simd       60       84        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Motor {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       44       60        0
    //    simd4        2        4        0
    // Totals...
    // yes simd       46       64        0
    //  no simd       52       76        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Motor {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       36        0
    //    simd4        3        6        0
    // Totals...
    // yes simd       31       42        0
    //  no simd       40       60        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(other[scalar]) * self.group0(),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<anti_sandwich> for MultiVector {
    type Output = anti_sandwich_partial<MultiVector>;
    fn div(self, _rhs: anti_sandwich) -> Self::Output {
        anti_sandwich_partial(self)
    }
}
impl AntiSandwich<AntiScalar> for MultiVector {
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
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      110      135        0
    //    simd2        8        8        0
    //    simd3       13       17        0
    //    simd4        5        6        0
    // Totals...
    // yes simd      136      166        0
    //  no simd      185      226        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for MultiVector {
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
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      105      125        0
    //    simd2        8        9        0
    //    simd3       12       15        0
    //    simd4        5        8        0
    // Totals...
    // yes simd      130      157        0
    //  no simd      177      220        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for MultiVector {
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
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for MultiVector {
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
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for MultiVector {
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
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      105      120        0
    //    simd2        8       10        0
    //    simd3       12       18        0
    //    simd4        5        8        0
    // Totals...
    // yes simd      130      156        0
    //  no simd      177      226        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      127      157        0
    //    simd2       11       11        0
    //    simd3       15       17        0
    //    simd4        6        7        0
    // Totals...
    // yes simd      159      192        0
    //  no simd      218      258        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      123      151        0
    //    simd2        8        9        0
    //    simd3       14       18        0
    //    simd4        5        6        0
    // Totals...
    // yes simd      150      184        0
    //  no simd      201      247        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32      105      125        0
    //    simd2        8        9        0
    //    simd3       12       15        0
    //    simd4        5        8        0
    // Totals...
    // yes simd      130      157        0
    //  no simd      177      220        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<anti_sandwich> for Origin {
    type Output = anti_sandwich_partial<Origin>;
    fn div(self, _rhs: anti_sandwich) -> Self::Output {
        anti_sandwich_partial(self)
    }
}
impl AntiSandwich<AntiScalar> for Origin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Origin::from_groups(/* e4 */ other[e1234] * self[e4]);
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       27        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([1.0, 1.0, 1.0, other[e1234] * self[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[scalar] * self[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        5        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       21        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(self[e4]) * Simd32x4::from([other[e423], other[e431], other[e412], other[e4]]) * Simd32x4::from(-1.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[e4]) * Simd32x4::from([other[e1], other[e2], other[e3], other[e321]]),
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
        let geometric_anti_product = Scalar::from_groups(/* scalar */ other[e321] * self[e4]);
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        7        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        0       13        0
    //  no simd        0       31        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e23] * self[e4], other[e31] * self[e4], other[e12] * self[e4], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([other[e41] * self[e4], other[e42] * self[e4], other[e43] * self[e4], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Origin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        8        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       33        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(self[e4]) * Simd32x4::from([other[e23], other[e31], other[e12], other[e1234]]) * Simd32x4::from([-1.0, -1.0, -1.0, 1.0]),
            // e423, e431, e412, e321
            Simd32x4::from(self[e4]) * Simd32x4::from([other[e41], other[e42], other[e43], other[scalar]]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Origin {
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
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        let geometric_anti_product = AntiScalar::from_groups(/* e1234 */ other[e4] * self[e4] * -1.0);
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       21        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self[e4] * other[e423], self[e4] * other[e431], self[e4] * other[e412], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([1.0, 1.0, 1.0, self[e4] * other[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Origin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       21        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([1.0, 1.0, 1.0, self[e4] * other[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([self[e4] * other[e1], self[e4] * other[e2], self[e4] * other[e3], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
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
        let geometric_anti_product = Horizon::from_groups(/* e321 */ self[e4] * other[scalar] * -1.0);
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<anti_sandwich> for Plane {
    type Output = anti_sandwich_partial<Plane>;
    fn div(self, _rhs: anti_sandwich) -> Self::Output {
        anti_sandwich_partial(self)
    }
}
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
        let geometric_anti_product = Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(other[e1234]) * self.group0());
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       27        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       17       30        0
    //  no simd       20       39        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[scalar] * self[e423], other[scalar] * self[e431], other[scalar] * self[e412], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from(other[e1234]) * self.group0(),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       34       50        0
    //  no simd       40       56        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
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
            Simd32x3::from(other[e321]) * Simd32x3::from([self[e423], self[e431], self[e412]]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       29       45        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       30       46        0
    //  no simd       33       49        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       48        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       34       50        0
    //  no simd       40       56        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Plane {
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
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        1        3        0
    // Totals...
    // yes simd       17       31        0
    //  no simd       20       40        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([other[e4] * self[e423], other[e4] * self[e431], other[e4] * self[e412], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([1.0, 1.0, 1.0, other[e4] * self[e321]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       39        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       25       40        0
    //  no simd       28       43        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Plane {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       37        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       26       45        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Plane {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       16        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        6       18        0
    //  no simd        6       24        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self[e423] * other[scalar], self[e431] * other[scalar], self[e412] * other[scalar], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<anti_sandwich> for Point {
    type Output = anti_sandwich_partial<Point>;
    fn div(self, _rhs: anti_sandwich) -> Self::Output {
        anti_sandwich_partial(self)
    }
}
impl AntiSandwich<AntiScalar> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        7        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        3       19        0
    fn anti_sandwich(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(other[e1234]) * self.group0());
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<DualNum> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       13        0
    //    simd4        1        6        0
    // Totals...
    // yes simd        9       19        0
    //  no simd       12       37        0
    fn anti_sandwich(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from(other[e1234]) * self.group0(),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[scalar] * self[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Flector> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       21       36        0
    //  no simd       24       48        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Horizon> for Point {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn anti_sandwich(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Scalar::from_groups(/* scalar */ other[e321] * self[e4]);
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       27        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       17       31        0
    //  no simd       20       43        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Point {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       21       36        0
    //  no simd       24       48        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Point {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       62        0
    //    simd2        0        2        0
    //    simd3        4        8        0
    //    simd4        0        1        0
    // Totals...
    // yes simd       40       73        0
    //  no simd       48       94        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Origin> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       24        0
    //    simd4        0        3        0
    // Totals...
    // yes simd       12       27        0
    //  no simd       12       36        0
    fn anti_sandwich(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([1.0, 1.0, 1.0, other[e4] * self[e4]]) * Simd32x4::from([0.0, 0.0, 0.0, -1.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([other[e4] * self[e1], other[e4] * self[e2], other[e4] * self[e3], 1.0]) * Simd32x4::from([-1.0, -1.0, -1.0, 0.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       33        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       18       35        0
    //  no simd       18       41        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Point> for Point {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd4        0        2        0
    // Totals...
    // yes simd       15       29        0
    //  no simd       15       35        0
    fn anti_sandwich(self, other: Point) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Scalar> for Point {
    type Output = Scalar;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn anti_sandwich(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Horizon::from_groups(/* e321 */ self[e4] * other[scalar] * -1.0);
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl std::ops::Div<anti_sandwich> for Scalar {
    type Output = anti_sandwich_partial<Scalar>;
    fn div(self, _rhs: anti_sandwich) -> Self::Output {
        anti_sandwich_partial(self)
    }
}
impl AntiSandwich<Flector> for Scalar {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       12        0
    //  no simd        0       24        0
    fn anti_sandwich(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Flector::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e423] * self[scalar], other[e431] * self[scalar], other[e412] * self[scalar], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
            // e423, e431, e412, e321
            Simd32x4::from([1.0, 1.0, 1.0, other[e4] * self[scalar]]) * Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Line> for Scalar {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn anti_sandwich(self, other: Line) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(self[scalar]) * other.group0());
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Motor> for Scalar {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn anti_sandwich(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from(0.0),
            // e23, e31, e12, scalar
            Simd32x4::from(self[scalar]) * other.group0(),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<MultiVector> for Scalar {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       10        0
    //    simd2        0        2        0
    //    simd3        0        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0       18        0
    //  no simd        0       36        0
    fn anti_sandwich(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = MultiVector::from_groups(
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
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
impl AntiSandwich<Plane> for Scalar {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        9        0
    fn anti_sandwich(self, other: Plane) -> Self::Output {
        use crate::elements::*;
        let geometric_anti_product = Point::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([other[e423] * self[scalar], other[e431] * self[scalar], other[e412] * self[scalar], 1.0]) * Simd32x4::from([1.0, 1.0, 1.0, 0.0]),
        );
        return geometric_anti_product.geometric_anti_product(self.anti_reverse());
    }
}
