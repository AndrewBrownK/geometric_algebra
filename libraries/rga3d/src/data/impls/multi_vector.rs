use crate::traits::GeometricProduct;
use crate::traits::RightDual;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 56
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       0       0
//  Average:         7      10       0
//  Maximum:       103     118       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       0       0
//  Average:        12      17       0
//  Maximum:       177     192       0
impl std::ops::Add<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self.group0()[0], self.group0()[1] + other[e1234]]),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::AddAssign<AntiScalar> for MultiVector {
    fn add_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self.group0()[0], self.group0()[1] + other[e1234]]),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Add<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        1        0        0
    // no simd        2        0        0
    fn add(self, other: DualNum) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            other.group0() + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::AddAssign<DualNum> for MultiVector {
    fn add_assign(&mut self, other: DualNum) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            other.group0() + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Add<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: Flector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            other.group0() + self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            other.group1() + self.group4(),
        );
    }
}
impl std::ops::AddAssign<Flector> for MultiVector {
    fn add_assign(&mut self, other: Flector) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            other.group0() + self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            other.group1() + self.group4(),
        );
    }
}
impl std::ops::Add<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from([self.group4()[0], self.group4()[1], self.group4()[2], self.group4()[3] + other[e321]]),
        );
    }
}
impl std::ops::AddAssign<Horizon> for MultiVector {
    fn add_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from([self.group4()[0], self.group4()[1], self.group4()[2], self.group4()[3] + other[e321]]),
        );
    }
}
impl std::ops::Add<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: Line) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            other.group0() + self.group2(),
            // e23, e31, e12
            other.group1() + self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::AddAssign<Line> for MultiVector {
    fn add_assign(&mut self, other: Line) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            other.group0() + self.group2(),
            // e23, e31, e12
            other.group1() + self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Add<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        0        0
    //  no simd        8        0        0
    fn add(self, other: Motor) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other.group1()[3], other.group0()[3]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group2(),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::AddAssign<Motor> for MultiVector {
    fn add_assign(&mut self, other: Motor) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other.group1()[3], other.group0()[3]]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) + self.group2(),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) + self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Add<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       16        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            other.group0() + self.group0(),
            // e1, e2, e3, e4
            other.group1() + self.group1(),
            // e41, e42, e43
            other.group2() + self.group2(),
            // e23, e31, e12
            other.group3() + self.group3(),
            // e423, e431, e412, e321
            other.group4() + self.group4(),
        );
    }
}
impl std::ops::AddAssign<MultiVector> for MultiVector {
    fn add_assign(&mut self, other: MultiVector) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            other.group0() + self.group0(),
            // e1, e2, e3, e4
            other.group1() + self.group1(),
            // e41, e42, e43
            other.group2() + self.group2(),
            // e23, e31, e12
            other.group3() + self.group3(),
            // e423, e431, e412, e321
            other.group4() + self.group4(),
        );
    }
}
impl std::ops::Add<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] + other[e4]]),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::AddAssign<Origin> for MultiVector {
    fn add_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] + other[e4]]),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Add<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Plane) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4() + other.group0(),
        );
    }
}
impl std::ops::AddAssign<Plane> for MultiVector {
    fn add_assign(&mut self, other: Plane) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4() + other.group0(),
        );
    }
}
impl std::ops::Add<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: Point) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group0(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::AddAssign<Point> for MultiVector {
    fn add_assign(&mut self, other: Point) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() + other.group0(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Add<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self.group0()[0] + other[scalar], self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::AddAssign<Scalar> for MultiVector {
    fn add_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self.group0()[0] + other[scalar], self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::BitXor<AntiScalar> for MultiVector {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn bitxor(self, other: AntiScalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        3        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        7        0
    //  no simd        1       17        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<DualNum> for MultiVector {
    fn bitxor_assign(&mut self, other: DualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       22        0
    //    simd3        1        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       17       27        0
    //  no simd       25       40        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Flector> for MultiVector {
    fn bitxor_assign(&mut self, other: Flector) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn bitxor(self, other: Horizon) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Horizon> for MultiVector {
    fn bitxor_assign(&mut self, other: Horizon) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       14        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       17        0
    //  no simd       13       24        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Line> for MultiVector {
    fn bitxor_assign(&mut self, other: Line) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       17        0
    //    simd3        2        4        0
    //    simd4        2        3        0
    // Totals...
    // yes simd       15       24        0
    //  no simd       25       41        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Motor> for MultiVector {
    fn bitxor_assign(&mut self, other: Motor) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       39        0
    //    simd3        5        6        0
    //    simd4        5        6        0
    // Totals...
    // yes simd       40       51        0
    //  no simd       65       81        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<MultiVector> for MultiVector {
    fn bitxor_assign(&mut self, other: MultiVector) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn bitxor(self, other: Origin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Origin> for MultiVector {
    fn bitxor_assign(&mut self, other: Origin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        5        0
    //  no simd        3        8        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Plane> for MultiVector {
    fn bitxor_assign(&mut self, other: Plane) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        1        2        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       12       22        0
    //  no simd       17       32        0
    fn bitxor(self, other: Point) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Point> for MultiVector {
    fn bitxor_assign(&mut self, other: Point) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for MultiVector {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}

impl From<AntiScalar> for MultiVector {
    fn from(from_anti_scalar: AntiScalar) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([0.0, from_anti_scalar[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}

impl From<DualNum> for MultiVector {
    fn from(from_dual_num: DualNum) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([from_dual_num[scalar], from_dual_num[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}

impl From<Flector> for MultiVector {
    fn from(from_flector: Flector) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([from_flector[e1], from_flector[e2], from_flector[e3], from_flector[e4]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([from_flector[e423], from_flector[e431], from_flector[e412], from_flector[e321]]),
        );
    }
}

impl From<Horizon> for MultiVector {
    fn from(from_horizon: Horizon) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([0.0, 0.0, 0.0, from_horizon[e321]]),
        );
    }
}

impl From<Line> for MultiVector {
    fn from(from_line: Line) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from([from_line[e41], from_line[e42], from_line[e43]]),
            // e23, e31, e12
            Simd32x3::from([from_line[e23], from_line[e31], from_line[e12]]),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}

impl From<Motor> for MultiVector {
    fn from(from_motor: Motor) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([from_motor[scalar], from_motor[e1234]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from([from_motor[e41], from_motor[e42], from_motor[e43]]),
            // e23, e31, e12
            Simd32x3::from([from_motor[e23], from_motor[e31], from_motor[e12]]),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}

impl From<Origin> for MultiVector {
    fn from(from_origin: Origin) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, from_origin[e4]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}

impl From<Plane> for MultiVector {
    fn from(from_plane: Plane) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from([from_plane[e423], from_plane[e431], from_plane[e412], from_plane[e321]]),
        );
    }
}

impl From<Point> for MultiVector {
    fn from(from_point: Point) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([from_point[e1], from_point[e2], from_point[e3], from_point[e4]]),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}

impl From<Scalar> for MultiVector {
    fn from(from_scalar: Scalar) -> Self {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([from_scalar[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
        );
    }
}
impl std::ops::Mul<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<AntiScalar> for MultiVector {
    fn mul_assign(&mut self, other: AntiScalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       15        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        6       18        0
    //  no simd        8       24        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<DualNum> for MultiVector {
    fn mul_assign(&mut self, other: DualNum) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for MultiVector {
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
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Flector> for MultiVector {
    fn mul_assign(&mut self, other: Flector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        2        0
    //    simd3        0        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        9        0
    //  no simd        0       29        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Horizon> for MultiVector {
    fn mul_assign(&mut self, other: Horizon) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for MultiVector {
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
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Line> for MultiVector {
    fn mul_assign(&mut self, other: Line) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for MultiVector {
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
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Motor> for MultiVector {
    fn mul_assign(&mut self, other: Motor) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for MultiVector {
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
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<MultiVector> for MultiVector {
    fn mul_assign(&mut self, other: MultiVector) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Origin> for MultiVector {
    fn mul_assign(&mut self, other: Origin) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       32        0
    //    simd3        2        4        0
    // Totals...
    // yes simd       20       36        0
    //  no simd       24       44        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Plane> for MultiVector {
    fn mul_assign(&mut self, other: Plane) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       35        0
    //    simd3        3        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd       22       42        0
    //  no simd       40       60        0
    fn mul(self, other: Point) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Point> for MultiVector {
    fn mul_assign(&mut self, other: Point) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for MultiVector {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Neg for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       16        0
    fn neg(self) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            self.group1() * Simd32x4::from(-1.0),
            // e41, e42, e43
            self.group2() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group3() * Simd32x3::from(-1.0),
            // e423, e431, e412, e321
            self.group4() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Not for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiScalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self.group0()[0], self.group0()[1] - other[e1234]]),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::SubAssign<AntiScalar> for MultiVector {
    fn sub_assign(&mut self, other: AntiScalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self.group0()[0], self.group0()[1] - other[e1234]]),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Sub<DualNum> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        1        0        0
    // no simd        2        0        0
    fn sub(self, other: DualNum) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0() - other.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::SubAssign<DualNum> for MultiVector {
    fn sub_assign(&mut self, other: DualNum) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0() - other.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Sub<Flector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn sub(self, other: Flector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group0(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4() - other.group1(),
        );
    }
}
impl std::ops::SubAssign<Flector> for MultiVector {
    fn sub_assign(&mut self, other: Flector) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group0(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4() - other.group1(),
        );
    }
}
impl std::ops::Sub<Horizon> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from([self.group4()[0], self.group4()[1], self.group4()[2], self.group4()[3] - other[e321]]),
        );
    }
}
impl std::ops::SubAssign<Horizon> for MultiVector {
    fn sub_assign(&mut self, other: Horizon) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            Simd32x4::from([self.group4()[0], self.group4()[1], self.group4()[2], self.group4()[3] - other[e321]]),
        );
    }
}
impl std::ops::Sub<Line> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn sub(self, other: Line) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2() - other.group0(),
            // e23, e31, e12
            self.group3() - other.group1(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::SubAssign<Line> for MultiVector {
    fn sub_assign(&mut self, other: Line) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2() - other.group0(),
            // e23, e31, e12
            self.group3() - other.group1(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Sub<Motor> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        8        0
    //    simd2        1        0        0
    //    simd3        2        0        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        8        8        0
    fn sub(self, other: Motor) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other.group1()[3] * -1.0, other.group0()[3] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            Simd32x3::from([other.group0()[0] * -1.0, other.group0()[1] * -1.0, other.group0()[2] * -1.0]) + self.group2(),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0] * -1.0, other.group1()[1] * -1.0, other.group1()[2] * -1.0]) + self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::SubAssign<Motor> for MultiVector {
    fn sub_assign(&mut self, other: Motor) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([other.group1()[3] * -1.0, other.group0()[3] * -1.0]) + self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            Simd32x3::from([other.group0()[0] * -1.0, other.group0()[1] * -1.0, other.group0()[2] * -1.0]) + self.group2(),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0] * -1.0, other.group1()[1] * -1.0, other.group1()[2] * -1.0]) + self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Sub<MultiVector> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        1        0        0
    //    simd3        2        0        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        5        0        0
    //  no simd       16        0        0
    fn sub(self, other: MultiVector) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0() - other.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group1(),
            // e41, e42, e43
            self.group2() - other.group2(),
            // e23, e31, e12
            self.group3() - other.group3(),
            // e423, e431, e412, e321
            self.group4() - other.group4(),
        );
    }
}
impl std::ops::SubAssign<MultiVector> for MultiVector {
    fn sub_assign(&mut self, other: MultiVector) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0() - other.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group1(),
            // e41, e42, e43
            self.group2() - other.group2(),
            // e23, e31, e12
            self.group3() - other.group3(),
            // e423, e431, e412, e321
            self.group4() - other.group4(),
        );
    }
}
impl std::ops::Sub<Origin> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] - other[e4]]),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::SubAssign<Origin> for MultiVector {
    fn sub_assign(&mut self, other: Origin) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], self.group1()[3] - other[e4]]),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Sub<Plane> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: Plane) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4() - other.group0(),
        );
    }
}
impl std::ops::SubAssign<Plane> for MultiVector {
    fn sub_assign(&mut self, other: Plane) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4() - other.group0(),
        );
    }
}
impl std::ops::Sub<Point> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn sub(self, other: Point) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group0(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::SubAssign<Point> for MultiVector {
    fn sub_assign(&mut self, other: Point) {
        *self = MultiVector::from_groups(
            // scalar, e1234
            self.group0(),
            // e1, e2, e3, e4
            self.group1() - other.group0(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::Sub<Scalar> for MultiVector {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        1        0        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self.group0()[0] - other[scalar], self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
impl std::ops::SubAssign<Scalar> for MultiVector {
    fn sub_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from([self.group0()[0] - other[scalar], self.group0()[1]]),
            // e1, e2, e3, e4
            self.group1(),
            // e41, e42, e43
            self.group2(),
            // e23, e31, e12
            self.group3(),
            // e423, e431, e412, e321
            self.group4(),
        );
    }
}
