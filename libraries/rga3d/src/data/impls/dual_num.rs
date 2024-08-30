use crate::traits::GeometricProduct;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 50
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       2       0
//  Maximum:         6      18       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       1       0
//  Average:         0       3       0
//  Maximum:         8      24       0
impl std::ops::Add<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([self.group0()[0], (self.group0()[1] + other[e1234])]));
        return addition;
    }
}
impl std::ops::AddAssign<AntiScalar> for DualNum {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        let addition = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([self.group0()[0], (self.group0()[1] + other[e1234])]));
        *self = addition;
    }
}
impl std::ops::Add<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        1        0        0
    // no simd        2        0        0
    fn add(self, other: DualNum) -> Self::Output {
        let addition = DualNum::from_groups(/* scalar, e1234 */ (self.group0() + other.group0()));
        return addition;
    }
}
impl std::ops::AddAssign<DualNum> for DualNum {
    fn add_assign(&mut self, other: DualNum) {
        let addition = DualNum::from_groups(/* scalar, e1234 */ (self.group0() + other.group0()));
        *self = addition;
    }
}
impl std::ops::Add<Flector> for DualNum {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            other.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<Horizon> for DualNum {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
        );
        return addition;
    }
}
impl std::ops::Add<Line> for DualNum {
    type Output = Motor;
    fn add(self, other: Line) -> Self::Output {
        let addition = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], self.group0()[1]]),
            // e23, e31, e12, scalar
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], self.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        0        0
    fn add(self, other: Motor) -> Self::Output {
        let addition = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], (self.group0()[1] + other.group0()[3])]),
            // e23, e31, e12, scalar
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], (self.group0()[0] + other.group1()[3])]),
        );
        return addition;
    }
}
impl std::ops::Add<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        1        0        0
    // no simd        2        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e1234
            (self.group0() + other.group0()),
            // e1, e2, e3, e4
            other.group1(),
            // e41, e42, e43
            other.group2(),
            // e23, e31, e12
            other.group3(),
            // e423, e431, e412, e321
            other.group4(),
        );
        return addition;
    }
}
impl std::ops::Add<Origin> for DualNum {
    type Output = MultiVector;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<Plane> for DualNum {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            other.group0(),
        );
        return addition;
    }
}
impl std::ops::Add<Point> for DualNum {
    type Output = MultiVector;
    fn add(self, other: Point) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            other.group0(),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([(self.group0()[0] + other[scalar]), self.group0()[1]]));
        return addition;
    }
}
impl std::ops::AddAssign<Scalar> for DualNum {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let addition = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([(self.group0()[0] + other[scalar]), self.group0()[1]]));
        *self = addition;
    }
}
impl std::ops::BitXor<AntiScalar> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DualNum> for DualNum {
    fn bitxor_assign(&mut self, other: DualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Horizon> for DualNum {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        6        0
    //  no simd        1        9        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       17        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for DualNum {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Point> for DualNum {
    type Output = Point;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn bitxor(self, other: Point) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for DualNum {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}

impl From<AntiScalar> for DualNum {
    fn from(anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([0.0, anti_scalar[e1234]]));
    }
}

impl From<Scalar> for DualNum {
    fn from(scalar: Scalar) -> Self {
        use crate::elements::*;
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([scalar[scalar], 0.0]));
    }
}
impl std::ops::Mul<AntiScalar> for DualNum {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        3        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DualNum> for DualNum {
    fn mul_assign(&mut self, other: DualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       12        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for DualNum {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        3        0
    // no simd        3        9        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        3        0
    // no simd        4       12        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        6       18        0
    //  no simd        8       24        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for DualNum {
    type Output = Origin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn mul(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        6        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Point> for DualNum {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       10        0
    fn mul(self, other: Point) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for DualNum {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for DualNum {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn neg(self) -> Self {
        let negation = DualNum::from_groups(/* scalar, e1234 */ (self.group0() * Simd32x2::from(-1.0)));
        return negation;
    }
}
impl std::ops::Not for DualNum {
    type Output = AntiScalar;
    fn not(self) -> Self::Output {
        let right_dual = AntiScalar::from_groups(/* e1234 */ self.group0()[0]);
        return right_dual;
    }
}
impl std::ops::Sub<AntiScalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([self.group0()[0], (self.group0()[1] - other[e1234])]));
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiScalar> for DualNum {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        let subtraction = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([self.group0()[0], (self.group0()[1] - other[e1234])]));
        *self = subtraction;
    }
}
impl std::ops::Sub<DualNum> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        1        0        0
    // no simd        2        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        let subtraction = DualNum::from_groups(/* scalar, e1234 */ (self.group0() - other.group0()));
        return subtraction;
    }
}
impl std::ops::SubAssign<DualNum> for DualNum {
    fn sub_assign(&mut self, other: DualNum) {
        let subtraction = DualNum::from_groups(/* scalar, e1234 */ (self.group0() - other.group0()));
        *self = subtraction;
    }
}
impl std::ops::Sub<Flector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: Flector) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (other.group1() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Horizon> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other[e321] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Line> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn sub(self, other: Line) -> Self::Output {
        let subtraction = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), self.group0()[1]]),
            // e23, e31, e12, scalar
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), self.group0()[0]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Motor> for DualNum {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn sub(self, other: Motor) -> Self::Output {
        let subtraction = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (other.group0()[1] * -1.0),
                (other.group0()[2] * -1.0),
                (self.group0()[1] - other.group0()[3]),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (other.group1()[1] * -1.0),
                (other.group1()[2] * -1.0),
                (self.group0()[0] - other.group1()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MultiVector> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        2       14        0
    fn sub(self, other: MultiVector) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e1234
            (self.group0() - other.group0()),
            // e1, e2, e3, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e41, e42, e43
            (other.group2() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group3() * Simd32x3::from(-1.0)),
            // e423, e431, e412, e321
            (other.group4() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Origin> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other[e4] * -1.0)]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            (other.group0() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Point> for DualNum {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Point) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for DualNum {
    type Output = DualNum;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([(self.group0()[0] - other[scalar]), self.group0()[1]]));
        return subtraction;
    }
}
impl std::ops::SubAssign<Scalar> for DualNum {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        let subtraction = DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([(self.group0()[0] - other[scalar]), self.group0()[1]]));
        *self = subtraction;
    }
}

impl TryFrom<Motor> for DualNum {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into DualNum { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([motor[scalar], motor[e1234]])));
    }
}

impl TryFrom<MultiVector> for DualNum {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = multi_vector[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e2: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into DualNum { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from([multi_vector[scalar], multi_vector[e1234]])));
    }
}