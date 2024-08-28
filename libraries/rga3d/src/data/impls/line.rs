use crate::traits::GeometricProduct;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 45
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       2       0
//  Average:         3       5       0
//  Maximum:        34      45       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       3       0
//  Average:         5       8       0
//  Maximum:        57      72       0
impl std::ops::Add<AntiScalar> for Line {
    type Output = Motor;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other[e1234]]),
            // e23, e31, e12, scalar
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<DualNum> for Line {
    type Output = Motor;
    fn add(self, other: DualNum) -> Self::Output {
        let addition = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[1]]),
            // e23, e31, e12, scalar
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<Flector> for Line {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e423, e431, e412, e321
            other.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<Horizon> for Line {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
        );
        return addition;
    }
}
impl std::ops::Add<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: Line) -> Self::Output {
        let addition = Line::from_groups(/* e41, e42, e43 */ (self.group0() + other.group0()), /* e23, e31, e12 */ (self.group1() + other.group1()));
        return addition;
    }
}
impl std::ops::AddAssign<Line> for Line {
    fn add_assign(&mut self, other: Line) {
        let addition = Line::from_groups(/* e41, e42, e43 */ (self.group0() + other.group0()), /* e23, e31, e12 */ (self.group1() + other.group1()));
        *self = addition;
    }
}
impl std::ops::Add<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        0        0
    fn add(self, other: Motor) -> Self::Output {
        let addition = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self.group1()[0] + other.group1()[0]),
                (self.group1()[1] + other.group1()[1]),
                (self.group1()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e1234
            other.group0(),
            // e1, e2, e3, e4
            other.group1(),
            // e41, e42, e43
            (self.group0() + other.group2()),
            // e23, e31, e12
            (self.group1() + other.group3()),
            // e423, e431, e412, e321
            other.group4(),
        );
        return addition;
    }
}
impl std::ops::Add<Origin> for Line {
    type Output = MultiVector;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<Plane> for Line {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e423, e431, e412, e321
            other.group0(),
        );
        return addition;
    }
}
impl std::ops::Add<Point> for Line {
    type Output = MultiVector;
    fn add(self, other: Point) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<Scalar> for Line {
    type Output = Motor;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], other[scalar]]),
        );
        return addition;
    }
}
impl std::ops::BitXor<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DualNum> for Line {
    fn bitxor_assign(&mut self, other: DualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for Line {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       13       24        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn bitxor(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Point> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bitxor(self, other: Point) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for Line {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::Mul<AntiScalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiScalar> for Line {
    fn mul_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        3        0
    // no simd        3        9        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DualNum> for Line {
    fn mul_assign(&mut self, other: DualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       28        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       22       30        0
    //  no simd       28       36        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       19       27        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       24        0
    //    simd4        3        3        0
    // Totals...
    // yes simd       19       27        0
    //  no simd       28       36        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for Line {
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
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for Line {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn mul(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       15        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Point> for Line {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       18        0
    //  no simd       13       24        0
    fn mul(self, other: Point) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for Line {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for Line {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn neg(self) -> Self {
        let negation = Line::from_groups(
            // e41, e42, e43
            (self.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group1() * Simd32x3::from(-1.0)),
        );
        return negation;
    }
}
impl std::ops::Not for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn not(self) -> Self::Output {
        let right_dual = Line::from_groups(/* e41, e42, e43 */ (self.group1() * Simd32x3::from(-1.0)), /* e23, e31, e12 */ Simd32x3::from(0.0));
        return right_dual;
    }
}
impl std::ops::Sub<AntiScalar> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other[e1234] * -1.0)]),
            // e23, e31, e12, scalar
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DualNum> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        let subtraction = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] * -1.0)]),
            // e23, e31, e12, scalar
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other.group0()[0] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Flector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: Flector) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e423, e431, e412, e321
            (other.group1() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Horizon> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other[e321] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Line> for Line {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        let subtraction = Line::from_groups(/* e41, e42, e43 */ (self.group0() - other.group0()), /* e23, e31, e12 */ (self.group1() - other.group1()));
        return subtraction;
    }
}
impl std::ops::SubAssign<Line> for Line {
    fn sub_assign(&mut self, other: Line) {
        let subtraction = Line::from_groups(/* e41, e42, e43 */ (self.group0() - other.group0()), /* e23, e31, e12 */ (self.group1() - other.group1()));
        *self = subtraction;
    }
}
impl std::ops::Sub<Motor> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        6        2        0
    fn sub(self, other: Motor) -> Self::Output {
        let subtraction = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self.group1()[0] - other.group1()[0]),
                (self.group1()[1] - other.group1()[1]),
                (self.group1()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MultiVector> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        2        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        6       10        0
    fn sub(self, other: MultiVector) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e1234
            (other.group0() * Simd32x2::from(-1.0)),
            // e1, e2, e3, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e41, e42, e43
            (self.group0() - other.group2()),
            // e23, e31, e12
            (self.group1() - other.group3()),
            // e423, e431, e412, e321
            (other.group4() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Origin> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other[e4] * -1.0)]),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e423, e431, e412, e321
            (other.group0() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Point> for Line {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Point) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e41, e42, e43
            self.group0(),
            // e23, e31, e12
            self.group1(),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for Line {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = Motor::from_groups(
            // e41, e42, e43, e1234
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e23, e31, e12, scalar
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (other[scalar] * -1.0)]),
        );
        return subtraction;
    }
}

impl TryFrom<Motor> for Line {
    type Error = String;
    fn try_from(motor: Motor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = motor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Motor do not fit into Line { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Line::from_groups(
            // e41, e42, e43
            Simd32x3::from([motor[e41], motor[e42], motor[e43]]),
            // e23, e31, e12
            Simd32x3::from([motor[e23], motor[e31], motor[e12]]),
        ));
    }
}

impl TryFrom<MultiVector> for Line {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = multi_vector[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
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
            let mut error = "Elements from MultiVector do not fit into Line { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(Line::from_groups(
            // e41, e42, e43
            Simd32x3::from([multi_vector[e41], multi_vector[e42], multi_vector[e43]]),
            // e23, e31, e12
            Simd32x3::from([multi_vector[e23], multi_vector[e31], multi_vector[e12]]),
        ));
    }
}
