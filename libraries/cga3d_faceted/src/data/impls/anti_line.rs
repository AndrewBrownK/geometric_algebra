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
// Total Implementations: 403
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         1       2       0
//  Average:         4       8       0
//  Maximum:        77     106       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         2       6       0
//  Average:         8      13       0
//  Maximum:       161     194       0
impl std::ops::Add<AntiCircleOnOrigin> for AntiLine {
    type Output = DipoleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1() + self.group0(),
            // e15, e25, e35
            self.group1(),
        );
    }
}
impl std::ops::Add<AntiCircleRotor> for AntiLine {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]) + self.group0().with_w(other[e45]),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]) + self.group1().with_w(other[scalar]),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for AntiLine {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            other.group1() + self.group0(),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]) + self.group1().with_w(other[scalar]),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for AntiLine {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            other.group0() + self.group0(),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]) + self.group1().with_w(other[scalar]),
        );
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for AntiLine {
    type Output = AntiCircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]) + self.group0().with_w(other[e45]),
            // e15, e25, e35, scalar
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]) + self.group1().with_w(other[scalar]),
        );
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for AntiLine {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            other.group0().xyz(),
            // e23, e31, e12
            other.group1() + self.group0(),
            // e15, e25, e35, scalar
            self.group1().with_w(other[scalar]),
        );
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], other[e4]]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group1().yzwx(),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiDualNum> for AntiLine {
    type Output = VersorOddOrthogonalOrigin;
    fn add(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(other[scalar]),
            // e23, e31, e12, e3215
            self.group0().with_w(0.0),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<AntiFlatOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlector> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiLine> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        return AntiLine::from_groups(/* e23, e31, e12 */ other.group0() + self.group0(), /* e15, e25, e35 */ other.group1() + self.group1());
    }
}
impl std::ops::AddAssign<AntiLine> for AntiLine {
    fn add_assign(&mut self, other: AntiLine) {
        *self = AntiLine::from_groups(/* e23, e31, e12 */ other.group0() + self.group0(), /* e15, e25, e35 */ other.group1() + self.group1());
    }
}
impl std::ops::Add<AntiLineOnOrigin> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        return AntiLine::from_groups(/* e23, e31, e12 */ self.group0() + other.group0(), /* e15, e25, e35 */ self.group1());
    }
}
impl std::ops::AddAssign<AntiLineOnOrigin> for AntiLine {
    fn add_assign(&mut self, other: AntiLineOnOrigin) {
        *self = AntiLine::from_groups(/* e23, e31, e12 */ self.group0() + other.group0(), /* e15, e25, e35 */ self.group1());
    }
}
impl std::ops::Add<AntiMotor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]) + self.group0().with_w(other[scalar]),
            // e15, e25, e35, e3215
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]) + self.group1().with_w(other[e3215]),
        );
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for AntiLine {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() + other.group0().xyz(),
            // e15, e25, e35, scalar
            self.group1().with_w(other[scalar]),
        );
    }
}
impl std::ops::Add<AntiMysteryCircleRotor> for AntiLine {
    type Output = AntiCircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]) + self.group0().with_w(other[e45]),
            // e15, e25, e35, scalar
            self.group1().with_w(other[scalar]),
        );
    }
}
impl std::ops::Add<AntiMysteryDipoleInversion> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group1().with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiPlane> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0().with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiScalar> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for AntiLine {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e3215
            (self.group0() + other.group1().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<Circle> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleAligningOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleAtInfinity> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleAtOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleOnOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotor> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1().with_w(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Dipole> for AntiLine {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]) + self.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group1() + other.group2(),
        );
    }
}
impl std::ops::Add<DipoleAligningOrigin> for AntiLine {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            other.group0().xyz(),
            // e23, e31, e12, e45
            self.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group1() + other.group1(),
        );
    }
}
impl std::ops::Add<DipoleAtInfinity> for AntiLine {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]) + self.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group1() + other.group1(),
        );
    }
}
impl std::ops::Add<DipoleAtOrigin> for AntiLine {
    type Output = DipoleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35
            self.group1() + other.group1(),
        );
    }
}
impl std::ops::Add<DipoleInversion> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]) + self.group0().with_w(other[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]) + self.group1().with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            other.group3(),
        );
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().xyz(),
            // e23, e31, e12, e45
            self.group0().with_w(other[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]) + self.group1().with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]) + self.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group1() + other.group1(),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for AntiLine {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            other.group0(),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]) + self.group1().with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for AntiLine {
    type Output = DipoleInversion;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().xyz(),
            // e23, e31, e12, e45
            self.group0().with_w(other[e45]),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 0.0]),
        );
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for AntiLine {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        0        0
    //  no simd        7        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            other.group0(),
            // e23, e31, e12
            self.group0() + other.group1(),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]) + self.group1().with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<DipoleOnOrigin> for AntiLine {
    type Output = Dipole;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            other.group0().xyz(),
            // e23, e31, e12, e45
            self.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group1(),
        );
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for AntiLine {
    type Output = DipoleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            self.group0() + other.group1(),
            // e15, e25, e35
            self.group1() + other.group2(),
        );
    }
}
impl std::ops::Add<DualNum> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<FlatOrigin> for AntiLine {
    type Output = DipoleAtInfinity;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleAtInfinity::from_groups(/* e23, e31, e12, e45 */ self.group0().with_w(other[e45]), /* e15, e25, e35 */ self.group1());
    }
}
impl std::ops::Add<FlatPoint> for AntiLine {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group1() + other.group0().xyz(),
        );
    }
}
impl std::ops::Add<FlatPointAtInfinity> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        return AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1() + other.group0());
    }
}
impl std::ops::AddAssign<FlatPointAtInfinity> for AntiLine {
    fn add_assign(&mut self, other: FlatPointAtInfinity) {
        *self = AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1() + other.group0());
    }
}
impl std::ops::Add<Flector> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group1() + other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            other.group1(),
        );
    }
}
impl std::ops::Add<FlectorAtInfinity> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0().with_w(0.0),
            // e15, e25, e35, e3215
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]) + self.group1().with_w(other[e3215]),
        );
    }
}
impl std::ops::Add<FlectorOnOrigin> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 0.0]),
        );
    }
}
impl std::ops::Add<Horizon> for AntiLine {
    type Output = AntiMotor;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0().with_w(0.0),
            // e15, e25, e35, e3215
            self.group1().with_w(other[e3215]),
        );
    }
}
impl std::ops::Add<Infinity> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Line> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<LineAtInfinity> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: LineAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<LineOnOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0().with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Motor> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MotorAtInfinity> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MotorOnOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MultiVector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            other.group3(),
            // e15, e25, e35
            self.group1() + other.group4(),
            // e23, e31, e12
            self.group0() + other.group5(),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e3215],
        );
    }
}
impl std::ops::Add<MysteryCircle> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: MysteryCircle) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryCircleRotor> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryDipole> for AntiLine {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]) + self.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group1(),
        );
    }
}
impl std::ops::Add<MysteryDipoleInversion> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]) + self.group0().with_w(other[e45]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            other.group1().with_w(0.0),
        );
    }
}
impl std::ops::Add<MysteryVersorEven> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<MysteryVersorOdd> for AntiLine {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        1        0        0
    // no simd        4        0        0
    fn add(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]) + self.group0().with_w(other[e45]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 0.0]),
        );
    }
}
impl std::ops::Add<NullCircleAtOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for AntiLine {
    type Output = DipoleOrthogonalOrigin;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        return DipoleOrthogonalOrigin::from_groups(/* e41, e42, e43 */ other.group0(), /* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1());
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for AntiLine {
    type Output = DipoleInversionOrthogonalOrigin;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([other[e41], other[e42], other[e43], 0.0]),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<NullSphereAtOrigin> for AntiLine {
    type Output = DipoleInversionOrthogonalOrigin;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Origin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Plane> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    fn add(self, other: Plane) -> Self::Output {
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            other.group0(),
        );
    }
}
impl std::ops::Add<PlaneOnOrigin> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            other.group0().with_w(0.0),
        );
    }
}
impl std::ops::Add<RoundPoint> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<RoundPointAtOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<Scalar> for AntiLine {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOriginAtInfinity::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35, scalar */ self.group1().with_w(other[scalar]));
    }
}
impl std::ops::Add<Sphere> for AntiLine {
    type Output = DipoleInversion;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            self.group0().with_w(0.0),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            other.group0(),
        );
    }
}
impl std::ops::Add<SphereAtOrigin> for AntiLine {
    type Output = DipoleInversionOrthogonalOrigin;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x3::from(0.0).with_w(other[e3215]),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234]),
        );
    }
}
impl std::ops::Add<SphereOnOrigin> for AntiLine {
    type Output = DipoleInversion;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            self.group0().with_w(0.0),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other[e4235], other[e4315], other[e4125], 0.0]),
        );
    }
}
impl std::ops::Add<VersorEven> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from([other[e1], other[e2], other[e3], 0.0]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group2().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other[e415], other[e425], other[e435], 0.0]),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for AntiLine {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group2(),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321]),
            // e423, e431, e412
            other.group0().xyz(),
            // e235, e315, e125
            other.group1().xyz(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Add<VersorOdd> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]) + self.group0().with_w(other[e45]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]) + self.group1().with_w(other[e1234]),
            // e4235, e4315, e4125, e3215
            other.group3(),
        );
    }
}
impl std::ops::Add<VersorOddAtInfinity> for AntiLine {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([0.0, self[e15], other[e25], other[e35]]) + other.group0().xy().with_zw(self[e25], self[e35]),
            // e23, e31, e12, e45
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]) + self.group0().with_w(other[e45]),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for AntiLine {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        2        0        0
    // no simd        8        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([other[e23], other[e31], other[e12], 0.0]) + self.group0().with_w(other[e3215]),
            // e15, e25, e35, e1234
            Simd32x4::from([other[e15], other[e25], other[e35], 0.0]) + self.group1().with_w(other[e1234]),
        );
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        7       12        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       13       34        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        0        1        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        7       17        0
    //  no simd       10       31        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       22        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       25        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        1        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        4       14        0
    //  no simd        7       28        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for AntiLine {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       25        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for AntiLine {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       10       15        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for AntiLine {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        4       12        0
    //  no simd        7       19        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for AntiLine {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       16        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for AntiLine {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for AntiLine {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for AntiLine {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5       10        0
    //  no simd        5       22        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       19        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryCircleRotor> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2       18        0
    fn bitxor(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMysteryDipoleInversion> for AntiLine {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        7       12        0
    fn bitxor(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for AntiLine {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        5        9        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for AntiLine {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        1        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        7        0
    //  no simd        5       16        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        1        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        4       14        0
    //  no simd        7       28        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: CircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOrthogonalOrigin> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAtInfinity> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        6        0
    fn bitxor(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       13       18        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       10       15        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        7        0
    //  no simd        5       10        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        7       12        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd       10       16        0
    //  no simd       13       18        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       10       15        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        7        0
    //  no simd        5       10        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        7       12        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for AntiLine {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       10       15        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for AntiLine {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for AntiLine {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       10       15        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for AntiLine {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for AntiLine {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: FlatOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for AntiLine {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for AntiLine {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorOnOrigin> for AntiLine {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: FlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for AntiLine {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineOnOrigin> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: LineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for AntiLine {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for AntiLine {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorOnOrigin> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: MotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       23        0
    //    simd3        2        6        0
    //    simd4        1        2        0
    // Totals...
    // yes simd       19       31        0
    //  no simd       26       49        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircle> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: MysteryCircle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryCircleRotor> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: MysteryCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipole> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bitxor(self, other: MysteryDipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryDipoleInversion> for AntiLine {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        2        7        0
    fn bitxor(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorEven> for AntiLine {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        9        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        7       12        0
    fn bitxor(self, other: MysteryVersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MysteryVersorOdd> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        8        0
    //  no simd        2       22        0
    fn bitxor(self, other: MysteryVersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for AntiLine {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for AntiLine {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        5        9        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for AntiLine {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        5        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        5        9        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for AntiLine {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        5        0
    //  no simd        2       10        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for AntiLine {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: Origin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for AntiLine {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        2        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        4        8        0
    //  no simd        8       19        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for AntiLine {
    type Output = CircleAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for AntiLine {
    fn bitxor_assign(&mut self, other: Scalar) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for AntiLine {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       25        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for AntiLine {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        5       16        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for AntiLine {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        7       13        0
    //  no simd       10       15        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for AntiLine {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        6        0
    //  no simd        2       13        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for AntiLine {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        5        8        0
    //  no simd        5       13        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for AntiLine {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        0        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        7       15        0
    //  no simd       10       22        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       15        0
    //    simd3        0        1        0
    //    simd4        1        4        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       13       34        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        6        0
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       25        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       12        0
    //    simd3        0        1        0
    //    simd4        1        4        0
    // Totals...
    // yes simd        7       17        0
    //  no simd       10       31        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}

impl From<AntiLineOnOrigin> for AntiLine {
    fn from(from_anti_line_on_origin: AntiLineOnOrigin) -> Self {
        return AntiLine::from_groups(/* e23, e31, e12 */ from_anti_line_on_origin.group0(), /* e15, e25, e35 */ Simd32x3::from(0.0));
    }
}

impl From<FlatPointAtInfinity> for AntiLine {
    fn from(from_flat_point_at_infinity: FlatPointAtInfinity) -> Self {
        return AntiLine::from_groups(/* e23, e31, e12 */ Simd32x3::from(0.0), /* e15, e25, e35 */ from_flat_point_at_infinity.group0());
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       31        0
    //    simd3        0        5        0
    //    simd4        5        0        0
    // Totals...
    // yes simd       14       36        0
    //  no simd       29       46        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       46        0
    //    simd3        0        7        0
    //    simd4        7        0        0
    // Totals...
    // yes simd       30       53        0
    //  no simd       51       67        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       44        0
    //    simd3        0        6        0
    //    simd4        6        0        0
    // Totals...
    // yes simd       26       50        0
    //  no simd       44       62        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       24        0
    //    simd3        0        3        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       16       27        0
    //  no simd       25       33        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for AntiLine {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       15        0
    //    simd3        0        3        0
    //    simd4        5        4        0
    // Totals...
    // yes simd       16       22        0
    //  no simd       31       40        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       36        0
    //    simd3        0        4        0
    //    simd4        5        1        0
    // Totals...
    // yes simd       20       41        0
    //  no simd       35       52        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       30       54        0
    //    simd3        0       11        0
    //    simd4       12        1        0
    // Totals...
    // yes simd       42       66        0
    //  no simd       78       91        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for AntiLine {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       22       38        0
    //    simd3        0        4        0
    //    simd4        5        1        0
    // Totals...
    // yes simd       27       43        0
    //  no simd       42       54        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       30        0
    //    simd3        0        2        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       24       38        0
    //  no simd       48       60        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       19       39        0
    //    simd3        0        4        0
    //    simd4        8        4        0
    // Totals...
    // yes simd       27       47        0
    //  no simd       51       67        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       21        0
    //    simd4        3        6        0
    // Totals...
    // yes simd        9       27        0
    //  no simd       18       45        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        3        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        1        9        0
    //  no simd        3       33        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for AntiLine {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiLine {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd       10       16        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiLine {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       27        0
    //    simd3        0        2        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       19       30        0
    //  no simd       28       37        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for AntiLine {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        0        3        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       19       27        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       12        0
    //    simd3        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4       14        0
    //  no simd       10       18        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd3        0        1        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       28       36        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryCircleRotor> for AntiLine {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       11        0
    //    simd3        0        3        0
    //    simd4        4        3        0
    // Totals...
    // yes simd        9       17        0
    //  no simd       21       32        0
    fn mul(self, other: AntiMysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMysteryDipoleInversion> for AntiLine {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       28        0
    //    simd3        0        2        0
    //    simd4        4        2        0
    // Totals...
    // yes simd       18       32        0
    //  no simd       30       42        0
    fn mul(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiLine {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       13        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       15        0
    //  no simd       13       21        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for AntiLine {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       12        0
    //    simd3        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4       14        0
    //  no simd       10       18        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for AntiLine {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for AntiLine {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd3        1        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        5       14        0
    //  no simd       13       29        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       45        0
    //    simd3        0        4        0
    //    simd4        5        1        0
    // Totals...
    // yes simd       29       50        0
    //  no simd       44       61        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       42        0
    //    simd3        0        6        0
    //    simd4        6        0        0
    // Totals...
    // yes simd       26       48        0
    //  no simd       44       60        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       36        0
    //    simd3        0        6        0
    //    simd4        6        0        0
    // Totals...
    // yes simd       20       42        0
    //  no simd       38       54        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for AntiLine {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       21        0
    //    simd3        1        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd       14       25        0
    //  no simd       22       33        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       27        0
    //    simd3        0        4        0
    //    simd4        4        0        0
    // Totals...
    // yes simd        8       31        0
    //  no simd       20       39        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       30        0
    //    simd3        0        5        0
    //    simd4        5        0        0
    // Totals...
    // yes simd       14       35        0
    //  no simd       29       45        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       30        0
    //    simd3        0        1        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       14       34        0
    //  no simd       26       45        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       23       42        0
    //    simd3        0        4        0
    //    simd4        7        3        0
    // Totals...
    // yes simd       30       49        0
    //  no simd       51       66        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       36        0
    //    simd3        0        4        0
    //    simd4        7        3        0
    // Totals...
    // yes simd       24       43        0
    //  no simd       45       60        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for AntiLine {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       23        0
    //    simd3        0        2        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       16       26        0
    //  no simd       25       33        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for AntiLine {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       23        0
    //    simd3        0        4        0
    //    simd4        4        1        0
    // Totals...
    // yes simd       18       28        0
    //  no simd       30       39        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       29        0
    //    simd3        0        2        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       18       35        0
    //  no simd       36       51        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       40        0
    //    simd3        0        7        0
    //    simd4        7        0        0
    // Totals...
    // yes simd       24       47        0
    //  no simd       45       61        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       25        0
    //    simd3        0        2        0
    //    simd4        5        3        0
    // Totals...
    // yes simd       11       30        0
    //  no simd       26       43        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for AntiLine {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       12        0
    //    simd3        1        2        0
    //    simd4        3        4        0
    // Totals...
    // yes simd       12       18        0
    //  no simd       23       34        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       24        0
    //    simd3        0        4        0
    //    simd4        4        0        0
    // Totals...
    // yes simd        8       28        0
    //  no simd       20       36        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       31       52        0
    //    simd3        0        6        0
    //    simd4       11        5        0
    // Totals...
    // yes simd       42       63        0
    //  no simd       75       90        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       37        0
    //    simd3        0        4        0
    //    simd4       10        6        0
    // Totals...
    // yes simd       27       47        0
    //  no simd       57       73        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for AntiLine {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       18       28        0
    //    simd3        0        2        0
    //    simd4        7        5        0
    // Totals...
    // yes simd       25       35        0
    //  no simd       46       54        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       34        0
    //    simd3        0        2        0
    //    simd4        4        2        0
    // Totals...
    // yes simd       20       38        0
    //  no simd       32       48        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       34        0
    //    simd3        0        1        0
    //    simd4        7        6        0
    // Totals...
    // yes simd       24       41        0
    //  no simd       45       61        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       26       49        0
    //    simd3        0        5        0
    //    simd4        6        1        0
    // Totals...
    // yes simd       32       55        0
    //  no simd       50       68        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       13        0
    //    simd3        0        3        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        8       20        0
    //  no simd       20       38        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       38        0
    //    simd3        0        6        0
    //    simd4        6        0        0
    // Totals...
    // yes simd       20       44        0
    //  no simd       38       56        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for AntiLine {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        3        0
    //    simd4        0        6        0
    // Totals...
    // yes simd        1        9        0
    //  no simd        3       33        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for AntiLine {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        4        0
    // no simd        0       16        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiLine {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        3        0
    //    simd3        0        3        0
    //    simd4        2        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd       10       16        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for AntiLine {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        5        9        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for AntiLine {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       14        0
    //    simd3        0        2        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       14       20        0
    //  no simd       32       36        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for AntiLine {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn mul(self, other: FlectorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorOnOrigin> for AntiLine {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0       12        0
    //    simd4        4        4        0
    // Totals...
    // yes simd        4       16        0
    //  no simd       16       28        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for AntiLine {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: Horizon) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for AntiLine {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: Infinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiLine {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        0        3        0
    //    simd4        3        0        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       19       27        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for AntiLine {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1        6        0
    //    simd3        0        1        0
    //    simd4        1        0        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        5        9        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for AntiLine {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       12        0
    //    simd3        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4       14        0
    //  no simd       10       18        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiLine {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       25        0
    //    simd3        0        1        0
    //    simd4        3        2        0
    // Totals...
    // yes simd       19       28        0
    //  no simd       28       36        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for AntiLine {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn mul(self, other: MotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorOnOrigin> for AntiLine {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       38       62        0
    //    simd2        5        6        0
    //    simd3       23       32        0
    //    simd4       11        6        0
    // Totals...
    // yes simd       77      106        0
    //  no simd      161      194        0
    fn mul(self, other: MultiVector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircle> for AntiLine {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       14        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       17        0
    //  no simd       13       24        0
    fn mul(self, other: MysteryCircle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryCircleRotor> for AntiLine {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd3        0        3        0
    //    simd4        3        1        0
    // Totals...
    // yes simd       12       21        0
    //  no simd       21       30        0
    fn mul(self, other: MysteryCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipole> for AntiLine {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd3        1        2        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        5       13        0
    //  no simd       13       26        0
    fn mul(self, other: MysteryDipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryDipoleInversion> for AntiLine {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       21        0
    //    simd3        0        2        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       15       27        0
    //  no simd       33       43        0
    fn mul(self, other: MysteryDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorEven> for AntiLine {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       33        0
    //    simd3        0        1        0
    //    simd4        4        3        0
    // Totals...
    // yes simd       24       37        0
    //  no simd       36       48        0
    fn mul(self, other: MysteryVersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MysteryVersorOdd> for AntiLine {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       27        0
    //    simd3        0        2        0
    //    simd4        6        4        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       39       49        0
    fn mul(self, other: MysteryVersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3       21        0
    //    simd3        0        3        0
    //    simd4        3        0        0
    // Totals...
    // yes simd        6       24        0
    //  no simd       15       30        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       15        0
    //    simd3        1        4        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        7       19        0
    //  no simd       15       27        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       19        0
    //    simd3        0        3        0
    //    simd4        4        2        0
    // Totals...
    // yes simd       14       24        0
    //  no simd       26       36        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       20        0
    //    simd4        5        5        0
    // Totals...
    // yes simd        9       25        0
    //  no simd       24       40        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for AntiLine {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       19        0
    fn mul(self, other: Origin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for AntiLine {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd        7       18        0
    //  no simd       13       24        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for AntiLine {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       15        0
    //    simd3        0        2        0
    //    simd4        2        0        0
    // Totals...
    // yes simd        4       17        0
    //  no simd       10       21        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiLine {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        8        0
    //    simd3        0        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd        6       15        0
    //  no simd       18       32        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for AntiLine {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        9        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       39        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: Scalar) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for AntiLine {
    fn mul_assign(&mut self, other: Scalar) {
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        0
    //    simd3        0        4        0
    //    simd4        4        3        0
    // Totals...
    // yes simd        6       17        0
    //  no simd       18       34        0
    fn mul(self, other: Sphere) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        9        0
    // Totals...
    // yes simd        0       10        0
    //  no simd        0       39        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       10        0
    //    simd3        1        3        0
    //    simd4        2        3        0
    // Totals...
    // yes simd        5       16        0
    //  no simd       13       31        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       33       55        0
    //    simd3        0        6        0
    //    simd4       12        6        0
    // Totals...
    // yes simd       45       67        0
    //  no simd       81       97        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       43        0
    //    simd3        0        2        0
    //    simd4        8        6        0
    // Totals...
    // yes simd       33       51        0
    //  no simd       57       73        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for AntiLine {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       43        0
    //    simd3        0        3        0
    //    simd4        5        2        0
    // Totals...
    // yes simd       33       48        0
    //  no simd       48       60        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       28        0
    //    simd4        6        6        0
    // Totals...
    // yes simd       14       34        0
    //  no simd       32       52        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       34        0
    //    simd3        0        1        0
    //    simd4        7        6        0
    // Totals...
    // yes simd       24       41        0
    //  no simd       45       61        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for AntiLine {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       39        0
    //    simd3        0        3        0
    //    simd4        9        6        0
    // Totals...
    // yes simd       33       48        0
    //  no simd       60       72        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       37       58        0
    //    simd3        0        6        0
    //    simd4       11        5        0
    // Totals...
    // yes simd       48       69        0
    //  no simd       81       96        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for AntiLine {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       34        0
    //    simd3        0        2        0
    //    simd4        7        5        0
    // Totals...
    // yes simd       31       41        0
    //  no simd       52       60        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       32       55        0
    //    simd3        0        5        0
    //    simd4        6        1        0
    // Totals...
    // yes simd       38       61        0
    //  no simd       56       74        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn neg(self) -> Self::Output {
        return AntiLine::from_groups(
            // e23, e31, e12
            self.group0() * Simd32x3::from(-1.0),
            // e15, e25, e35
            self.group1() * Simd32x3::from(-1.0),
        );
    }
}
impl std::ops::Not for AntiLine {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn not(self) -> Self::Output {
        return self.right_dual();
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for AntiLine {
    type Output = DipoleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        1        0
    // no simd        3        3        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group0() - other.group1(),
            // e15, e25, e35
            self.group1(),
        );
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiLine {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        6       11        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            (self.group0() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            (self.group1() - other.group2().xyz()).with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for AntiLine {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        6        7        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group0() - other.group1(),
            // e15, e25, e35, scalar
            (self.group1() - other.group2().xyz()).with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for AntiLine {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        6        4        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() - other.group0(),
            // e15, e25, e35, scalar
            (self.group1() - other.group1().xyz()).with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for AntiLine {
    type Output = AntiCircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        6        8        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            (self.group0() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            (self.group1() - other.group1().xyz()).with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for AntiLine {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        4        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group0() - other.group1(),
            // e15, e25, e35, scalar
            self.group1().with_w(other[scalar] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiDipoleInversion> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group3().xyz().with_w(other[e4]) * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group2().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group1().yzwx() * Simd32x4::from(-1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       11        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiDualNum> for AntiLine {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x3::from(0.0).with_w(other[scalar] * -1.0),
            // e23, e31, e12, e3215
            self.group0().with_w(0.0),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiFlatOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group0().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiLine> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        0        0
    // no simd        6        0        0
    fn sub(self, other: AntiLine) -> Self::Output {
        return AntiLine::from_groups(/* e23, e31, e12 */ self.group0() - other.group0(), /* e15, e25, e35 */ self.group1() - other.group1());
    }
}
impl std::ops::SubAssign<AntiLine> for AntiLine {
    fn sub_assign(&mut self, other: AntiLine) {
        *self = AntiLine::from_groups(/* e23, e31, e12 */ self.group0() - other.group0(), /* e15, e25, e35 */ self.group1() - other.group1());
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        return AntiLine::from_groups(/* e23, e31, e12 */ self.group0() - other.group0(), /* e15, e25, e35 */ self.group1());
    }
}
impl std::ops::SubAssign<AntiLineOnOrigin> for AntiLine {
    fn sub_assign(&mut self, other: AntiLineOnOrigin) {
        *self = AntiLine::from_groups(/* e23, e31, e12 */ self.group0() - other.group0(), /* e15, e25, e35 */ self.group1());
    }
}
impl std::ops::Sub<AntiMotor> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        6        8        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            (self.group0() - other.group0().xyz()).with_w(other[scalar]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e3215
            (self.group1() - other.group1().xyz()).with_w(other[e3215]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for AntiLine {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        3        1        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0() - other.group0().xyz(),
            // e15, e25, e35, scalar
            self.group1().with_w(other[scalar] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiMysteryCircleRotor> for AntiLine {
    type Output = AntiCircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        5        0
    fn sub(self, other: AntiMysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            (self.group0() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, scalar
            self.group1().with_w(other[scalar] * -1.0),
        );
    }
}
impl std::ops::Sub<AntiMysteryDipoleInversion> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: AntiMysteryDipoleInversion) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiPlane> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiScalar> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        0        1        0
    // no simd        0        2        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for AntiLine {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        5        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e3215
            (self.group0() - other.group1().xyz()).with_w(0.0),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::Sub<Circle> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Circle) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleAligningOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleAtInfinity> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleAtOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleOnOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotor> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Dipole> for AntiLine {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        6        7        0
    fn sub(self, other: Dipole) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            (self.group0() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1() - other.group2(),
        );
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for AntiLine {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        4        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group0().with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group1() - other.group1(),
        );
    }
}
impl std::ops::Sub<DipoleAtInfinity> for AntiLine {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        2        1        0
    //  no simd        6        4        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            (self.group0() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1() - other.group1(),
        );
    }
}
impl std::ops::Sub<DipoleAtOrigin> for AntiLine {
    type Output = DipoleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        1        0
    // no simd        3        3        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35
            self.group1() - other.group1(),
        );
    }
}
impl std::ops::Sub<DipoleInversion> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        6       15        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            (self.group0() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            (self.group1() - other.group2().xyz()).with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        4        0
    //  no simd        3       12        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group0().with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            (self.group1() - other.group1().xyz()).with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        6        8        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            (self.group0() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1() - other.group1(),
            // e4235, e4315, e4125, e3215
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for AntiLine {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        8        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, e1234
            (self.group1() - other.group1().xyz()).with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group0().with_w(other[e45] * -1.0),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            (other.group1().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
        );
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for AntiLine {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        2        2        0
    //  no simd        6        8        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            other.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12
            self.group0() - other.group1(),
            // e15, e25, e35, e1234
            (self.group1() - other.group2().xyz()).with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<DipoleOnOrigin> for AntiLine {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        use crate::elements::*;
        return Dipole::from_groups(
            // e41, e42, e43
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e23, e31, e12, e45
            self.group0().with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group1(),
        );
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for AntiLine {
    type Output = DipoleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        1        0
    // no simd        6        3        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group0() - other.group1(),
            // e15, e25, e35
            self.group1() - other.group2(),
        );
    }
}
impl std::ops::Sub<DualNum> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        3        0
    fn sub(self, other: DualNum) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<FlatOrigin> for AntiLine {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleAtInfinity::from_groups(/* e23, e31, e12, e45 */ self.group0().with_w(other[e45] * -1.0), /* e15, e25, e35 */ self.group1());
    }
}
impl std::ops::Sub<FlatPoint> for AntiLine {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        3        1        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        use crate::elements::*;
        return DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group1() - other.group0().xyz(),
        );
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for AntiLine {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        return AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1() - other.group0());
    }
}
impl std::ops::SubAssign<FlatPointAtInfinity> for AntiLine {
    fn sub_assign(&mut self, other: FlatPointAtInfinity) {
        *self = AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ self.group1() - other.group0());
    }
}
impl std::ops::Sub<Flector> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        5        0
    fn sub(self, other: Flector) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group1() - other.group0().xyz(),
            // e4235, e4315, e4125, e3215
            other.group1() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<FlectorAtInfinity> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        3        4        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0().with_w(0.0),
            // e15, e25, e35, e3215
            (self.group1() - other.group0().xyz()).with_w(other[e3215]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}
impl std::ops::Sub<FlectorOnOrigin> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().with_w(other[e45] * -1.0),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            (other.group0().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
        );
    }
}
impl std::ops::Sub<Horizon> for AntiLine {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            self.group0().with_w(0.0),
            // e15, e25, e35, e3215
            self.group1().with_w(other[e3215] * -1.0),
        );
    }
}
impl std::ops::Sub<Infinity> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Line> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: Line) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<LineAtInfinity> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<LineOnOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Motor> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn sub(self, other: Motor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MotorAtInfinity> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MotorOnOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MultiVector> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        2        2        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        9        0
    //  no simd        6       26        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            other.group0() * Simd32x2::from(-1.0),
            // e1, e2, e3, e4
            other.group1() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            other.group3() * Simd32x4::from(-1.0),
            // e15, e25, e35
            self.group1() - other.group4(),
            // e23, e31, e12
            self.group0() - other.group5(),
            // e415, e425, e435, e321
            other.group6() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group7() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group8() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            other.group9() * Simd32x4::from(-1.0),
            // e3215
            other[e3215] * -1.0,
        );
    }
}
impl std::ops::Sub<MysteryCircle> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: MysteryCircle) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryCircleRotor> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        6        0
    fn sub(self, other: MysteryCircleRotor) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryDipole> for AntiLine {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        3        4        0
    fn sub(self, other: MysteryDipole) -> Self::Output {
        use crate::elements::*;
        return DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            (self.group0() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1(),
        );
    }
}
impl std::ops::Sub<MysteryDipoleInversion> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        7        0
    fn sub(self, other: MysteryDipoleInversion) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            (self.group0() - other.group0().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            (other.group1() * Simd32x3::from(-1.0)).with_w(0.0),
        );
    }
}
impl std::ops::Sub<MysteryVersorEven> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd2        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        9        0
    fn sub(self, other: MysteryVersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            (other.group0().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<MysteryVersorOdd> for AntiLine {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        3        0
    //  no simd        3       11        0
    fn sub(self, other: MysteryVersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15], self[e25], self[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            (self.group0() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            (other.group0().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
        );
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for AntiLine {
    type Output = DipoleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            other.group0() * Simd32x3::from(-1.0),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35
            self.group1(),
        );
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for AntiLine {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for AntiLine {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Origin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Plane> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<PlaneOnOrigin> for AntiLine {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        return DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            self.group0().with_w(0.0),
            // e15, e25, e35
            self.group1(),
            // e4235, e4315, e4125, e3215
            (other.group0() * Simd32x3::from(-1.0)).with_w(0.0),
        );
    }
}
impl std::ops::Sub<RoundPoint> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<Scalar> for AntiLine {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, scalar
            self.group1().with_w(other[scalar] * -1.0),
        );
    }
}
impl std::ops::Sub<Sphere> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            self.group0().with_w(0.0),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            other.group0() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<SphereAtOrigin> for AntiLine {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x3::from(0.0).with_w(other[e3215] * -1.0),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234] * -1.0),
        );
    }
}
impl std::ops::Sub<SphereOnOrigin> for AntiLine {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        use crate::elements::*;
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            self.group0().with_w(0.0),
            // e15, e25, e35, e1234
            self.group1().with_w(other[e1234] * -1.0),
            // e4235, e4315, e4125, e3215
            (other.group0().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
        );
    }
}
impl std::ops::Sub<VersorEven> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       17        0
    fn sub(self, other: VersorEven) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            other.group3() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       13        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       13        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            (other.group0().yzw() * Simd32x3::from(-1.0)).with_w(0.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1() * Simd32x4::from(-1.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group2().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd2        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        9        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, other[e12345]]) * Simd32x2::from([0.0, -1.0]),
            // e1, e2, e3, e4
            Simd32x3::from(0.0).with_w(other[e4] * -1.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group1().xyz() * Simd32x3::from(-1.0)).with_w(0.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for AntiLine {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       12        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group2() * Simd32x4::from(-1.0),
            // e5
            other[e5] * -1.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            self.group1(),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x3::from(0.0).with_w(other[e321] * -1.0),
            // e423, e431, e412
            other.group0().xyz() * Simd32x3::from(-1.0),
            // e235, e315, e125
            other.group1().xyz() * Simd32x3::from(-1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl std::ops::Sub<VersorOdd> for AntiLine {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        2        4        0
    //  no simd        6       16        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        use crate::elements::*;
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e45
            (self.group0() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            (self.group1() - other.group2().xyz()).with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group3() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for AntiLine {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        0        0
    //    simd3        1        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        4        3        0
    //  no simd        6       12        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        use crate::elements::*;
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([other[scalar], self[e15] - other[e15], self[e25] - other[e25], self[e35] - other[e35]]) * Simd32x4::from([-1.0, 1.0, 1.0, 1.0]),
            // e23, e31, e12, e45
            (self.group0() - other.group1().xyz()).with_w(other[e45]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e4235, e4315, e4125, e3215
            other.group2() * Simd32x4::from(-1.0),
        );
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for AntiLine {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        2        0        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        2        3        0
    //  no simd        6       12        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        use crate::elements::*;
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            other.group0() * Simd32x4::from(-1.0),
            // e23, e31, e12, e3215
            (self.group0() - other.group1().xyz()).with_w(other[e3215]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
            // e15, e25, e35, e1234
            (self.group1() - other.group2().xyz()).with_w(other[e1234]) * Simd32x4::from([1.0, 1.0, 1.0, -1.0]),
        );
    }
}

impl TryFrom<AntiCircleOnOrigin> for AntiLine {
    type Error = String;
    fn try_from(anti_circle_on_origin: AntiCircleOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleOnOrigin do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            anti_circle_on_origin.group1(),
            // e15, e25, e35
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<AntiCircleRotor> for AntiLine {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotor do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            anti_circle_rotor.group1().xyz(),
            // e15, e25, e35
            anti_circle_rotor.group2().xyz(),
        ));
    }
}

impl TryFrom<AntiCircleRotorAligningOrigin> for AntiLine {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOrigin do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            anti_circle_rotor_aligning_origin.group1(),
            // e15, e25, e35
            anti_circle_rotor_aligning_origin.group2().xyz(),
        ));
    }
}

impl TryFrom<AntiCircleRotorAligningOriginAtInfinity> for AntiLine {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_aligning_origin_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOriginAtInfinity do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            anti_circle_rotor_aligning_origin_at_infinity.group0(),
            // e15, e25, e35
            anti_circle_rotor_aligning_origin_at_infinity.group1().xyz(),
        ));
    }
}

impl TryFrom<AntiCircleRotorAtInfinity> for AntiLine {
    type Error = String;
    fn try_from(anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAtInfinity do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            anti_circle_rotor_at_infinity.group0().xyz(),
            // e15, e25, e35
            anti_circle_rotor_at_infinity.group1().xyz(),
        ));
    }
}

impl TryFrom<AntiCircleRotorOnOrigin> for AntiLine {
    type Error = String;
    fn try_from(anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorOnOrigin do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            anti_circle_rotor_on_origin.group1(),
            // e15, e25, e35
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<AntiMotor> for AntiLine {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotor do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            anti_motor.group0().xyz(),
            // e15, e25, e35
            anti_motor.group1().xyz(),
        ));
    }
}

impl TryFrom<AntiMotorOnOrigin> for AntiLine {
    type Error = String;
    fn try_from(anti_motor_on_origin: AntiMotorOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMotorOnOrigin do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            anti_motor_on_origin.group0().xyz(),
            // e15, e25, e35
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<AntiMysteryCircleRotor> for AntiLine {
    type Error = String;
    fn try_from(anti_mystery_circle_rotor: AntiMysteryCircleRotor) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_mystery_circle_rotor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_mystery_circle_rotor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiMysteryCircleRotor do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            anti_mystery_circle_rotor.group0().xyz(),
            // e15, e25, e35
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<AntiVersorEvenOnOrigin> for AntiLine {
    type Error = String;
    fn try_from(anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_versor_even_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_versor_even_on_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiVersorEvenOnOrigin do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            anti_versor_even_on_origin.group1().xyz(),
            // e15, e25, e35
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<Dipole> for AntiLine {
    type Error = String;
    fn try_from(dipole: Dipole) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Dipole do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(/* e23, e31, e12 */ dipole.group1().xyz(), /* e15, e25, e35 */ dipole.group2()));
    }
}

impl TryFrom<DipoleAligningOrigin> for AntiLine {
    type Error = String;
    fn try_from(dipole_aligning_origin: DipoleAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAligningOrigin do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35
            dipole_aligning_origin.group1(),
        ));
    }
}

impl TryFrom<DipoleAtInfinity> for AntiLine {
    type Error = String;
    fn try_from(dipole_at_infinity: DipoleAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtInfinity do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            dipole_at_infinity.group0().xyz(),
            // e15, e25, e35
            dipole_at_infinity.group1(),
        ));
    }
}

impl TryFrom<DipoleAtOrigin> for AntiLine {
    type Error = String;
    fn try_from(dipole_at_origin: DipoleAtOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtOrigin do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(/* e23, e31, e12 */ Simd32x3::from(0.0), /* e15, e25, e35 */ dipole_at_origin.group1()));
    }
}

impl TryFrom<DipoleInversion> for AntiLine {
    type Error = String;
    fn try_from(dipole_inversion: DipoleInversion) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversion do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            dipole_inversion.group1().xyz(),
            // e15, e25, e35
            dipole_inversion.group2().xyz(),
        ));
    }
}

impl TryFrom<DipoleInversionAligningOrigin> for AntiLine {
    type Error = String;
    fn try_from(dipole_inversion_aligning_origin: DipoleInversionAligningOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_aligning_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_aligning_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAligningOrigin do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35
            dipole_inversion_aligning_origin.group1().xyz(),
        ));
    }
}

impl TryFrom<DipoleInversionAtInfinity> for AntiLine {
    type Error = String;
    fn try_from(dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAtInfinity do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            dipole_inversion_at_infinity.group0().xyz(),
            // e15, e25, e35
            dipole_inversion_at_infinity.group1(),
        ));
    }
}

impl TryFrom<DipoleInversionAtOrigin> for AntiLine {
    type Error = String;
    fn try_from(dipole_inversion_at_origin: DipoleInversionAtOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_at_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionAtOrigin do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35
            dipole_inversion_at_origin.group1().xyz(),
        ));
    }
}

impl TryFrom<DipoleInversionOrthogonalOrigin> for AntiLine {
    type Error = String;
    fn try_from(dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleInversionOrthogonalOrigin do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            dipole_inversion_orthogonal_origin.group1(),
            // e15, e25, e35
            dipole_inversion_orthogonal_origin.group2().xyz(),
        ));
    }
}

impl TryFrom<DipoleOrthogonalOrigin> for AntiLine {
    type Error = String;
    fn try_from(dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleOrthogonalOrigin do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            dipole_orthogonal_origin.group1(),
            // e15, e25, e35
            dipole_orthogonal_origin.group2(),
        ));
    }
}

impl TryFrom<FlatPoint> for AntiLine {
    type Error = String;
    fn try_from(flat_point: FlatPoint) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = flat_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from FlatPoint do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(/* e23, e31, e12 */ Simd32x3::from(0.0), /* e15, e25, e35 */ flat_point.group0().xyz()));
    }
}

impl TryFrom<Flector> for AntiLine {
    type Error = String;
    fn try_from(flector: Flector) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = flector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = flector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Flector do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(/* e23, e31, e12 */ Simd32x3::from(0.0), /* e15, e25, e35 */ flector.group0().xyz()));
    }
}

impl TryFrom<FlectorAtInfinity> for AntiLine {
    type Error = String;
    fn try_from(flector_at_infinity: FlectorAtInfinity) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = flector_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from FlectorAtInfinity do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35
            flector_at_infinity.group0().xyz(),
        ));
    }
}

impl TryFrom<MultiVector> for AntiLine {
    type Error = String;
    fn try_from(multi_vector: MultiVector) -> Result<Self, Self::Error> {
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
            error_string.push_str("e12345: ");
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
        let el = multi_vector[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[17];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[18];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[19];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[20];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[21];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[22];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[23];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[24];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[25];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[26];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[27];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[28];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[29];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[30];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[31];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MultiVector do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(/* e23, e31, e12 */ multi_vector.group5(), /* e15, e25, e35 */ multi_vector.group4()));
    }
}

impl TryFrom<MysteryDipole> for AntiLine {
    type Error = String;
    fn try_from(mystery_dipole: MysteryDipole) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_dipole[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryDipole do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            mystery_dipole.group0().xyz(),
            // e15, e25, e35
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<MysteryDipoleInversion> for AntiLine {
    type Error = String;
    fn try_from(mystery_dipole_inversion: MysteryDipoleInversion) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryDipoleInversion do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            mystery_dipole_inversion.group0().xyz(),
            // e15, e25, e35
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<MysteryVersorOdd> for AntiLine {
    type Error = String;
    fn try_from(mystery_versor_odd: MysteryVersorOdd) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = mystery_versor_odd[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = mystery_versor_odd[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from MysteryVersorOdd do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            mystery_versor_odd.group1().xyz(),
            // e15, e25, e35
            Simd32x3::from(0.0),
        ));
    }
}

impl TryFrom<VersorOdd> for AntiLine {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOdd do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            versor_odd.group1().xyz(),
            // e15, e25, e35
            versor_odd.group2().xyz(),
        ));
    }
}

impl TryFrom<VersorOddAtInfinity> for AntiLine {
    type Error = String;
    fn try_from(versor_odd_at_infinity: VersorOddAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddAtInfinity do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            versor_odd_at_infinity.group1().xyz(),
            // e15, e25, e35
            Simd32x3::from([versor_odd_at_infinity[e15], versor_odd_at_infinity[e25], versor_odd_at_infinity[e35]]),
        ));
    }
}

impl TryFrom<VersorOddOrthogonalOrigin> for AntiLine {
    type Error = String;
    fn try_from(versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Result<Self, Self::Error> {
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_odd_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e41: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e42: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e43: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e3215: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e1234: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorOddOrthogonalOrigin do not fit into AntiLine { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLine::from_groups(
            // e23, e31, e12
            versor_odd_orthogonal_origin.group1().xyz(),
            // e15, e25, e35
            versor_odd_orthogonal_origin.group2().xyz(),
        ));
    }
}
