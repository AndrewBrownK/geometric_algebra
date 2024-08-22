use crate::traits::GeometricProduct;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 354
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       3       0
//  Average:         3       6       0
//  Maximum:        46      69       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       4       0
//  Average:         3       7       0
//  Maximum:        64      96       0
impl std::ops::Add<AntiCircleOnOrigin> for AntiLineOnOrigin {
    type Output = AntiCircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        let addition = AntiCircleOnOrigin::from_groups(/* e41, e42, e43 */ other.group0(), /* e23, e31, e12 */ (self.group0() + other.group1()));
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotor> for AntiLineOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        let addition = AntiCircleRotor::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, scalar
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOrigin> for AntiLineOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let addition = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            (self.group0() + other.group1()),
            // e15, e25, e35, scalar
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for AntiLineOnOrigin {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition =
            AntiCircleRotorAligningOriginAtInfinity::from_groups(/* e23, e31, e12 */ (self.group0() + other.group0()), /* e15, e25, e35, scalar */ other.group1());
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorAtInfinity> for AntiLineOnOrigin {
    type Output = AntiCircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let addition = AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35, scalar
            other.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiCircleRotorOnOrigin> for AntiLineOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let addition = AntiCircleRotorOnOrigin::from_groups(/* e41, e42, e43, scalar */ other.group0(), /* e23, e31, e12 */ (self.group0() + other.group1()));
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversion> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]),
            // e5
            other.group3()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[3], other.group1()[0]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group2()[3]]),
            // e5
            other.group0()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiDualNum> for AntiLineOnOrigin {
    type Output = AntiMotor;
    fn add(self, other: AntiDualNum) -> Self::Output {
        let addition = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[1]]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other[e321]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlector> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiFlector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlectorOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<AntiLine> for AntiLineOnOrigin {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiLine) -> Self::Output {
        let addition = AntiLine::from_groups(/* e23, e31, e12 */ (self.group0() + other.group0()), /* e15, e25, e35 */ other.group1());
        return addition;
    }
}
impl std::ops::Add<AntiLineOnOrigin> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        let addition = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ (self.group0() + other.group0()));
        return addition;
    }
}
impl std::ops::AddAssign<AntiLineOnOrigin> for AntiLineOnOrigin {
    fn add_assign(&mut self, other: AntiLineOnOrigin) {
        let addition = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ (self.group0() + other.group0()));
        *self = addition;
    }
}
impl std::ops::Add<AntiMotor> for AntiLineOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiMotor) -> Self::Output {
        let addition = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35, e3215
            other.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for AntiLineOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        let addition = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([
            (self.group0()[0] + other.group0()[0]),
            (self.group0()[1] + other.group0()[1]),
            (self.group0()[2] + other.group0()[2]),
            other.group0()[3],
        ]));
        return addition;
    }
}
impl std::ops::Add<AntiPlane> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiPlane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e5
            other.group0()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<AntiScalar> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other[e12345]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for AntiLineOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let addition = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e1234
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<Circle> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Circle) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<CircleAligningOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            other.group2(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtInfinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<CircleAtOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<CircleOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotor> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group2()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            other.group0(),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group1()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group1()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group0(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Dipole> for AntiLineOnOrigin {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: Dipole) -> Self::Output {
        let addition = Dipole::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAligningOrigin> for AntiLineOnOrigin {
    type Output = Dipole;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        let addition = Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e15, e25, e35
            other.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAtInfinity> for AntiLineOnOrigin {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        let addition = DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35
            other.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleAtOrigin> for AntiLineOnOrigin {
    type Output = DipoleOrthogonalOrigin;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        let addition =
            DipoleOrthogonalOrigin::from_groups(/* e41, e42, e43 */ other.group0(), /* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ other.group1());
        return addition;
    }
}
impl std::ops::Add<DipoleInversion> for AntiLineOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleInversion) -> Self::Output {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            other.group2(),
            // e4235, e4315, e4125, e3215
            other.group3(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversion;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e15, e25, e35, e1234
            other.group1(),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for AntiLineOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let addition = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e15, e25, e35
            other.group1(),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let addition = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            other.group0(),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, e1234
            other.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversion;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[0]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group1()[1], other.group1()[2], other.group1()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            other.group0(),
            // e23, e31, e12
            (self.group0() + other.group1()),
            // e15, e25, e35, e1234
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOnOrigin> for AntiLineOnOrigin {
    type Output = Dipole;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        let addition = Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = DipoleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let addition = DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            other.group0(),
            // e23, e31, e12
            (self.group0() + other.group1()),
            // e15, e25, e35
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<DualNum> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DualNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[1]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group0()[0],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<FlatOrigin> for AntiLineOnOrigin {
    type Output = DipoleAtInfinity;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<FlatPoint> for AntiLineOnOrigin {
    type Output = DipoleAtInfinity;
    fn add(self, other: FlatPoint) -> Self::Output {
        let addition = DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
        );
        return addition;
    }
}
impl std::ops::Add<FlatPointAtInfinity> for AntiLineOnOrigin {
    type Output = AntiLine;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        let addition = AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ other.group0());
        return addition;
    }
}
impl std::ops::Add<Flector> for AntiLineOnOrigin {
    type Output = DipoleInversionAtInfinity;
    fn add(self, other: Flector) -> Self::Output {
        let addition = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e4235, e4315, e4125, e3215
            other.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<FlectorAtInfinity> for AntiLineOnOrigin {
    type Output = AntiMotor;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        let addition = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e3215
            other.group0(),
        );
        return addition;
    }
}
impl std::ops::Add<FlectorOnOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionAtInfinity;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        let addition = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[0]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<Horizon> for AntiLineOnOrigin {
    type Output = AntiMotor;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let addition = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other[e3215]]),
        );
        return addition;
    }
}
impl std::ops::Add<Infinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other[e5],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<Line> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Line) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            other.group1(),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<LineAtInfinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: LineAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<LineOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Motor> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Motor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<MotorAtInfinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            other.group0()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<MotorOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<MultiVector> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            other.group1(),
            // e5
            other[e1],
            // e41, e42, e43, e45
            other.group3(),
            // e15, e25, e35
            other.group4(),
            // e23, e31, e12
            (self.group0() + other.group5()),
            // e415, e425, e435, e321
            other.group6(),
            // e423, e431, e412
            other.group7(),
            // e235, e315, e125
            other.group8(),
            // e1234, e4235, e4315, e4125
            other.group9(),
            // e3215
            other[e45],
        );
        return addition;
    }
}
impl std::ops::Add<NullCircleAtOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for AntiLineOnOrigin {
    type Output = AntiCircleOnOrigin;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        let addition = AntiCircleOnOrigin::from_groups(/* e41, e42, e43 */ other.group0(), /* e23, e31, e12 */ self.group0());
        return addition;
    }
}
impl std::ops::Add<NullDipoleInversionAtOrigin> for AntiLineOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let addition = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e23, e31, e12, e1234
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<NullSphereAtOrigin> for AntiLineOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e1234
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other[e1234]]),
        );
        return addition;
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Origin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other[e4]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<Plane> for AntiLineOnOrigin {
    type Output = DipoleInversionAtInfinity;
    fn add(self, other: Plane) -> Self::Output {
        let addition = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            other.group0(),
        );
        return addition;
    }
}
impl std::ops::Add<PlaneOnOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionAtInfinity;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        let addition = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<RoundPoint> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group0(),
            // e5
            other[e2],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<RoundPointAtOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e5
            other.group0()[1],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return addition;
    }
}
impl std::ops::Add<Scalar> for AntiLineOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other[scalar]]));
        return addition;
    }
}
impl std::ops::Add<Sphere> for AntiLineOnOrigin {
    type Output = DipoleInversion;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other[e4315]]),
            // e4235, e4315, e4125, e3215
            other.group0(),
        );
        return addition;
    }
}
impl std::ops::Add<SphereAtOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        let addition = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[1]]),
        );
        return addition;
    }
}
impl std::ops::Add<SphereOnOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversion;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        let addition = DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEven> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEven) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[3]]),
            // e1, e2, e3, e4
            other.group3(),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([other.group0()[1], other.group0()[2], other.group0()[3], 0.0]),
            // e5
            other.group2()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, other.group0()[3]]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            other.group2(),
            // e5
            other.group1()[3],
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e235, e315, e125
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorOdd> for AntiLineOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: VersorOdd) -> Self::Output {
        let addition = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            other.group2(),
            // e4235, e4315, e4125, e3215
            other.group3(),
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddAtInfinity> for AntiLineOnOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        let addition = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            other.group0(),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e4235, e4315, e4125, e3215
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let addition = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            other.group0(),
            // e23, e31, e12, e3215
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e15, e25, e35, e1234
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for AntiLineOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for AntiLineOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       12        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4        9        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for AntiLineOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for AntiLineOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        9        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for AntiLineOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for AntiLineOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       12        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for AntiLineOnOrigin {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        9        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for AntiLineOnOrigin {
    type Output = AntiDipoleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2        9        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for AntiLineOnOrigin {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for AntiLineOnOrigin {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for AntiLineOnOrigin {
    type Output = AntiFlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for AntiLineOnOrigin {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for AntiLineOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiMotorOnOrigin> for AntiLineOnOrigin {
    fn bitxor_assign(&mut self, other: AntiMotorOnOrigin) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for AntiLineOnOrigin {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for AntiLineOnOrigin {
    type Output = AntiFlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for AntiLineOnOrigin {
    type Output = AntiDipoleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for AntiLineOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for AntiLineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for AntiLineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for AntiLineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for AntiLineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for AntiLineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for AntiLineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for AntiLineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAtInfinity> for AntiLineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for AntiLineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for AntiLineOnOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        9        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for AntiLineOnOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        9        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for AntiLineOnOrigin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for AntiLineOnOrigin {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        2        3        0
    // no simd        4        6        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for AntiLineOnOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        9        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for AntiLineOnOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        9        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for AntiLineOnOrigin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for AntiLineOnOrigin {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        2        3        0
    // no simd        4        6        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for AntiLineOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        2        3        0
    // no simd        4        6        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for AntiLineOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd2        2        3        0
    // no simd        4        6        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for AntiLineOnOrigin {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for AntiLineOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for AntiLineOnOrigin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for AntiLineOnOrigin {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for AntiLineOnOrigin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for AntiLineOnOrigin {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorOnOrigin> for AntiLineOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: FlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for AntiLineOnOrigin {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for AntiLineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineOnOrigin> for AntiLineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: LineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for AntiLineOnOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for AntiLineOnOrigin {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorOnOrigin> for AntiLineOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: MotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       15        0
    //    simd3        0        3        0
    // Totals...
    // yes simd        8       18        0
    //  no simd        8       24        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for AntiLineOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for AntiLineOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for AntiLineOnOrigin {
    type Output = NullCircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for AntiLineOnOrigin {
    type Output = NullCircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for AntiLineOnOrigin {
    type Output = CircleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2        9        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for AntiLineOnOrigin {
    type Output = CircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for AntiLineOnOrigin {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for AntiLineOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        9        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4       10        0
    //  no simd        4       12        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for AntiLineOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2        9        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for AntiLineOnOrigin {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        9        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for AntiLineOnOrigin {
    type Output = CircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for AntiLineOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = CircleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        2        7        0
    //  no simd        2        9        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for AntiLineOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       12        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for AntiLineOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        9        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        4        7        0
    //  no simd        4        9        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for AntiLineOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       18        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiLineOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       33        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for AntiLineOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       30        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for AntiLineOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       21        0
    fn mul(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAtInfinity> for AntiLineOnOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       24        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for AntiLineOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       21        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiLineOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       25       41        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       26       42        0
    //  no simd       29       45        0
    fn mul(self, other: AntiDipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for AntiLineOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       21       33        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for AntiLineOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       21       33        0
    fn mul(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleOnOrigin> for AntiLineOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for AntiLineOnOrigin {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for AntiLineOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiLineOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiLineOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for AntiLineOnOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn mul(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLine> for AntiLineOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       18        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for AntiLineOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for AntiLineOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for AntiLineOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn mul(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlane> for AntiLineOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for AntiLineOnOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for AntiLineOnOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for AntiLineOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for AntiLineOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for AntiLineOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       30        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for AntiLineOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       27        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for AntiLineOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       21        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for AntiLineOnOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       18        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for AntiLineOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       18        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       21        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiLineOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       29        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       15       30        0
    //  no simd       18       33        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for AntiLineOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       26        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       15       27        0
    //  no simd       18       30        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for AntiLineOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       21        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for AntiLineOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       20        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       21        0
    //  no simd       13       24        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for AntiLineOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       21        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiLineOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       30        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       21        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for AntiLineOnOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       21        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       18        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for AntiLineOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       29       45        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for AntiLineOnOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       21       33        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       21       33        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       27        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for AntiLineOnOrigin {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for AntiLineOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiLineOnOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for AntiLineOnOrigin {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for AntiLineOnOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for AntiLineOnOrigin {
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
impl std::ops::Mul<FlectorOnOrigin> for AntiLineOnOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn mul(self, other: FlectorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Horizon> for AntiLineOnOrigin {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for AntiLineOnOrigin {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiLineOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       18        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for AntiLineOnOrigin {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for AntiLineOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiLineOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for AntiLineOnOrigin {
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
impl std::ops::Mul<MotorOnOrigin> for AntiLineOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn mul(self, other: MotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MultiVector> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       36       54        0
    //    simd2        2        3        0
    //    simd3        8       12        0
    // Totals...
    // yes simd       46       69        0
    //  no simd       64       96        0
    fn mul(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullCircleAtOrigin> for AntiLineOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for AntiLineOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for AntiLineOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn mul(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullSphereAtOrigin> for AntiLineOnOrigin {
    type Output = NullDipoleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for AntiLineOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4        8        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        5        9        0
    //  no simd        8       12        0
    fn mul(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Origin> for AntiLineOnOrigin {
    type Output = NullCircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for AntiLineOnOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for AntiLineOnOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiLineOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       15        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for AntiLineOnOrigin {
    type Output = CircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for AntiLineOnOrigin {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiLineOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       15        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for AntiLineOnOrigin {
    type Output = DipoleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiLineOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       28       44        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       29       45        0
    //  no simd       32       48        0
    fn mul(self, other: VersorEven) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAligningOrigin> for AntiLineOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       24       36        0
    fn mul(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtInfinity> for AntiLineOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       24       36        0
    fn mul(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenAtOrigin> for AntiLineOnOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for AntiLineOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       24       36        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiLineOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       32       48        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for AntiLineOnOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for AntiLineOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn neg(self) -> Self {
        let negation = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ (self.group0() * Simd32x3::from(-1.0)));
        return negation;
    }
}
impl std::ops::Not for AntiLineOnOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn not(self) -> Self::Output {
        let dual = LineOnOrigin::from_groups(/* e415, e425, e435 */ (self.group0() * Simd32x3::from(-1.0)));
        return dual;
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for AntiLineOnOrigin {
    type Output = AntiCircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        1        0
    // no simd        3        3        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        let subtraction = AntiCircleOnOrigin::from_groups(
            // e41, e42, e43
            (other.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group0() - other.group1()),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotor> for AntiLineOnOrigin {
    type Output = AntiCircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        8        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        let subtraction = AntiCircleRotor::from_groups(
            // e41, e42, e43
            (other.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, scalar
            (other.group2() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for AntiLineOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        7        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let subtraction = AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            (other.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group0() - other.group1()),
            // e15, e25, e35, scalar
            (other.group2() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for AntiLineOnOrigin {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        3        4        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            (self.group0() - other.group0()),
            // e15, e25, e35, scalar
            (other.group1() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorAtInfinity> for AntiLineOnOrigin {
    type Output = AntiCircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        5        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let subtraction = AntiCircleRotorAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35, scalar
            (other.group1() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiCircleRotorOnOrigin> for AntiLineOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        1        1        0
    //  no simd        3        4        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let subtraction = AntiCircleRotorOnOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0() * Simd32x4::from(-1.0)),
            // e23, e31, e12
            (self.group0() - other.group1()),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversion> for AntiLineOnOrigin {
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
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (Simd32x4::from([other.group3()[0], other.group3()[1], other.group3()[2], other.group2()[3]]) * Simd32x4::from(-1.0)),
            // e5
            (other.group3()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       11        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), 0.0]),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for AntiLineOnOrigin {
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
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (swizzle!(other.group1(), 1, 2, 3, 0) * Simd32x4::from(-1.0)),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       11        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group2()[3] * -1.0)]),
            // e5
            (other.group0()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDualNum> for AntiLineOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[1] * -1.0)]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other[e321] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlector> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlectorOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLine> for AntiLineOnOrigin {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        1        0
    // no simd        3        3        0
    fn sub(self, other: AntiLine) -> Self::Output {
        let subtraction = AntiLine::from_groups(
            // e23, e31, e12
            (self.group0() - other.group0()),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiLineOnOrigin> for AntiLineOnOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        let subtraction = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ (self.group0() - other.group0()));
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiLineOnOrigin> for AntiLineOnOrigin {
    fn sub_assign(&mut self, other: AntiLineOnOrigin) {
        let subtraction = AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ (self.group0() - other.group0()));
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiMotor> for AntiLineOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        5        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35, e3215
            (other.group1() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for AntiLineOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        let subtraction = AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([
            (self.group0()[0] - other.group0()[0]),
            (self.group0()[1] - other.group0()[1]),
            (self.group0()[2] - other.group0()[2]),
            (other.group0()[3] * -1.0),
        ]));
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlane> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e5
            (other.group0()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return subtraction;
    }
}
impl std::ops::Sub<AntiScalar> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other[e12345] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return subtraction;
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return subtraction;
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for AntiLineOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        5        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0() * Simd32x4::from(-1.0)),
            // e23, e31, e12, e1234
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Circle> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Circle) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group2() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAligningOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        9        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group2() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtInfinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group2()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       10        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group2()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group1()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group1()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Dipole> for AntiLineOnOrigin {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        7        0
    fn sub(self, other: Dipole) -> Self::Output {
        let subtraction = Dipole::from_groups(
            // e41, e42, e43
            (other.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35
            (other.group2() * Simd32x3::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAligningOrigin> for AntiLineOnOrigin {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        let subtraction = Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAtInfinity> for AntiLineOnOrigin {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        4        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        let subtraction = DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleAtOrigin> for AntiLineOnOrigin {
    type Output = DipoleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        let subtraction = DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            (other.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversion> for AntiLineOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       12        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            (other.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            (other.group2() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (other.group3() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35, e1234
            (other.group1() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (other.group2() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for AntiLineOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        8        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let subtraction = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (other.group2() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let subtraction = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            (other.group0() * Simd32x4::from(-1.0)),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, e1234
            (other.group1() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[0] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(other.group1()[1] * -1.0), (other.group1()[2] * -1.0), (other.group1()[3] * -1.0), 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        1        0        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        1        2        0
    //  no simd        3        8        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            (other.group0() * Simd32x4::from(-1.0)),
            // e23, e31, e12
            (self.group0() - other.group1()),
            // e15, e25, e35, e1234
            (other.group2() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOnOrigin> for AntiLineOnOrigin {
    type Output = Dipole;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        let subtraction = Dipole::from_groups(
            // e41, e42, e43
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = DipoleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let subtraction = DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            (other.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group0() - other.group1()),
            // e15, e25, e35
            (other.group2() * Simd32x3::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DualNum> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[1] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group0()[0] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return subtraction;
    }
}
impl std::ops::Sub<FlatOrigin> for AntiLineOnOrigin {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other[e45] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPoint> for AntiLineOnOrigin {
    type Output = DipoleAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        let subtraction = DipoleAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatPointAtInfinity> for AntiLineOnOrigin {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        let subtraction = AntiLine::from_groups(/* e23, e31, e12 */ self.group0(), /* e15, e25, e35 */ (other.group0() * Simd32x3::from(-1.0)));
        return subtraction;
    }
}
impl std::ops::Sub<Flector> for AntiLineOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        8        0
    fn sub(self, other: Flector) -> Self::Output {
        let subtraction = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (other.group1() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlectorAtInfinity> for AntiLineOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e3215
            (other.group0() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlectorOnOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        let subtraction = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[0] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Horizon> for AntiLineOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e3215
            Simd32x4::from([0.0, 0.0, 0.0, (other[e3215] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Infinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other[e5] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return subtraction;
    }
}
impl std::ops::Sub<Line> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: Line) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<LineAtInfinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (other.group0() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<LineOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Motor> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: Motor) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorAtInfinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            (other.group0()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<MultiVector> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd2        0        1        0
    //    simd3        1        3        0
    //    simd4        0        4        0
    // Totals...
    // yes simd        1       10        0
    //  no simd        3       29        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            (other.group0() * Simd32x2::from(-1.0)),
            // e1, e2, e3, e4
            (other.group1() * Simd32x4::from(-1.0)),
            // e5
            (other[e1] * -1.0),
            // e41, e42, e43, e45
            (other.group3() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group4() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (self.group0() - other.group5()),
            // e415, e425, e435, e321
            (other.group6() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (other.group7() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (other.group8() * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            (other.group9() * Simd32x4::from(-1.0)),
            // e3215
            (other[e45] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullCircleAtOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for AntiLineOnOrigin {
    type Output = AntiCircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        let subtraction = AntiCircleOnOrigin::from_groups(/* e41, e42, e43 */ (other.group0() * Simd32x3::from(-1.0)), /* e23, e31, e12 */ self.group0());
        return subtraction;
    }
}
impl std::ops::Sub<NullDipoleInversionAtOrigin> for AntiLineOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e23, e31, e12, e1234
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for AntiLineOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e1234
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other[e1234] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Origin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other[e4] * -1.0)]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for AntiLineOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            (other.group0() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<PlaneOnOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        let subtraction = DipoleInversionAtInfinity::from_groups(
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for AntiLineOnOrigin {
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
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group0() * Simd32x4::from(-1.0)),
            // e5
            (other[e2] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return subtraction;
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e5
            (other.group0()[1] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
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
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for AntiLineOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other[scalar] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Sphere> for AntiLineOnOrigin {
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
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other[e4315] * -1.0)]),
            // e4235, e4315, e4125, e3215
            (other.group0() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereAtOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        let subtraction = DipoleInversionOrthogonalOrigin::from_groups(
            // e41, e42, e43, e3215
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e23, e31, e12
            self.group0(),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[1] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereOnOrigin> for AntiLineOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        let subtraction = DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e15, e25, e35, e1234
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEven> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       16        0
    fn sub(self, other: VersorEven) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            (other.group3() * Simd32x4::from(-1.0)),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       12        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[0] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([(other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0), 0.0]),
            // e5
            (other.group2()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for AntiLineOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, (other.group0()[3] * -1.0)]),
            // e1, e2, e3, e4
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for AntiLineOnOrigin {
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
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            (other.group2() * Simd32x4::from(-1.0)),
            // e5
            (other.group1()[3] * -1.0),
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            self.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for AntiLineOnOrigin {
    type Output = VersorOdd;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       13        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        let subtraction = VersorOdd::from_groups(
            // e41, e42, e43, scalar
            (other.group0() * Simd32x4::from(-1.0)),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            (other.group2() * Simd32x4::from(-1.0)),
            // e4235, e4315, e4125, e3215
            (other.group3() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for AntiLineOnOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        9        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        let subtraction = VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            (other.group0() * Simd32x4::from(-1.0)),
            // e23, e31, e12, e45
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e4235, e4315, e4125, e3215
            (other.group2() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for AntiLineOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        9        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            (other.group0() * Simd32x4::from(-1.0)),
            // e23, e31, e12, e3215
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e15, e25, e35, e1234
            (other.group2() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}

impl TryFrom<AntiCircleOnOrigin> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(anti_circle_on_origin: AntiCircleOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
            let mut error = "Elements from AntiCircleOnOrigin do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([
            anti_circle_on_origin[e23],
            anti_circle_on_origin[e31],
            anti_circle_on_origin[e12],
        ])));
    }
}

impl TryFrom<AntiCircleRotor> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor: AntiCircleRotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
        let el = anti_circle_rotor[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from AntiCircleRotor do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(
            // e23, e31, e12
            Simd32x3::from([anti_circle_rotor[e23], anti_circle_rotor[e31], anti_circle_rotor[e12]]),
        ));
    }
}

impl TryFrom<AntiCircleRotorAligningOrigin> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin: AntiCircleRotorAligningOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
        let el = anti_circle_rotor_aligning_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from AntiCircleRotorAligningOrigin do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([
            anti_circle_rotor_aligning_origin[e23],
            anti_circle_rotor_aligning_origin[e31],
            anti_circle_rotor_aligning_origin[e12],
        ])));
    }
}

impl TryFrom<AntiCircleRotorAligningOriginAtInfinity> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor_aligning_origin_at_infinity: AntiCircleRotorAligningOriginAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_aligning_origin_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_aligning_origin_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiCircleRotorAligningOriginAtInfinity do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([
            anti_circle_rotor_aligning_origin_at_infinity[e23],
            anti_circle_rotor_aligning_origin_at_infinity[e31],
            anti_circle_rotor_aligning_origin_at_infinity[e12],
        ])));
    }
}

impl TryFrom<AntiCircleRotorAtInfinity> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor_at_infinity: AntiCircleRotorAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_circle_rotor_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_circle_rotor_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from AntiCircleRotorAtInfinity do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([
            anti_circle_rotor_at_infinity[e23],
            anti_circle_rotor_at_infinity[e31],
            anti_circle_rotor_at_infinity[e12],
        ])));
    }
}

impl TryFrom<AntiCircleRotorOnOrigin> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(anti_circle_rotor_on_origin: AntiCircleRotorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
            let mut error = "Elements from AntiCircleRotorOnOrigin do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([
            anti_circle_rotor_on_origin[e23],
            anti_circle_rotor_on_origin[e31],
            anti_circle_rotor_on_origin[e12],
        ])));
    }
}

impl TryFrom<AntiLine> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(anti_line: AntiLine) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_line[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_line[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_line[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiLine do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([anti_line[e23], anti_line[e31], anti_line[e12]])));
    }
}

impl TryFrom<AntiMotor> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(anti_motor: AntiMotor) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_motor[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("scalar: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_motor[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from AntiMotor do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(
            // e23, e31, e12
            Simd32x3::from([anti_motor[e23], anti_motor[e31], anti_motor[e12]]),
        ));
    }
}

impl TryFrom<AntiMotorOnOrigin> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(anti_motor_on_origin: AntiMotorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
            let mut error = "Elements from AntiMotorOnOrigin do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([
            anti_motor_on_origin[e23],
            anti_motor_on_origin[e31],
            anti_motor_on_origin[e12],
        ])));
    }
}

impl TryFrom<AntiVersorEvenOnOrigin> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(anti_versor_even_on_origin: AntiVersorEvenOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
            let mut error = "Elements from AntiVersorEvenOnOrigin do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([
            anti_versor_even_on_origin[e23],
            anti_versor_even_on_origin[e31],
            anti_versor_even_on_origin[e12],
        ])));
    }
}

impl TryFrom<Dipole> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(dipole: Dipole) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
        let el = dipole[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from Dipole do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([dipole[e23], dipole[e31], dipole[e12]])));
    }
}

impl TryFrom<DipoleAtInfinity> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(dipole_at_infinity: DipoleAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleAtInfinity do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([
            dipole_at_infinity[e23],
            dipole_at_infinity[e31],
            dipole_at_infinity[e12],
        ])));
    }
}

impl TryFrom<DipoleInversion> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(dipole_inversion: DipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
        let el = dipole_inversion[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from DipoleInversion do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(
            // e23, e31, e12
            Simd32x3::from([dipole_inversion[e23], dipole_inversion[e31], dipole_inversion[e12]]),
        ));
    }
}

impl TryFrom<DipoleInversionAtInfinity> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(dipole_inversion_at_infinity: DipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = dipole_inversion_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e45: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from DipoleInversionAtInfinity do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([
            dipole_inversion_at_infinity[e23],
            dipole_inversion_at_infinity[e31],
            dipole_inversion_at_infinity[e12],
        ])));
    }
}

impl TryFrom<DipoleInversionOrthogonalOrigin> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(dipole_inversion_orthogonal_origin: DipoleInversionOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
        let el = dipole_inversion_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_inversion_orthogonal_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from DipoleInversionOrthogonalOrigin do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([
            dipole_inversion_orthogonal_origin[e23],
            dipole_inversion_orthogonal_origin[e31],
            dipole_inversion_orthogonal_origin[e12],
        ])));
    }
}

impl TryFrom<DipoleOrthogonalOrigin> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(dipole_orthogonal_origin: DipoleOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
        let el = dipole_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = dipole_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from DipoleOrthogonalOrigin do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([
            dipole_orthogonal_origin[e23],
            dipole_orthogonal_origin[e31],
            dipole_orthogonal_origin[e12],
        ])));
    }
}

impl TryFrom<MultiVector> for AntiLineOnOrigin {
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
        let el = multi_vector[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[12];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[13];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from MultiVector do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(
            // e23, e31, e12
            Simd32x3::from([multi_vector[e23], multi_vector[e31], multi_vector[e12]]),
        ));
    }
}

impl TryFrom<VersorOdd> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(versor_odd: VersorOdd) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
        let el = versor_odd[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from VersorOdd do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(
            // e23, e31, e12
            Simd32x3::from([versor_odd[e23], versor_odd[e31], versor_odd[e12]]),
        ));
    }
}

impl TryFrom<VersorOddAtInfinity> for AntiLineOnOrigin {
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
        let el = versor_odd_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from VersorOddAtInfinity do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([
            versor_odd_at_infinity[e23],
            versor_odd_at_infinity[e31],
            versor_odd_at_infinity[e12],
        ])));
    }
}

impl TryFrom<VersorOddOrthogonalOrigin> for AntiLineOnOrigin {
    type Error = String;
    fn try_from(versor_odd_orthogonal_origin: VersorOddOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
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
        let el = versor_odd_orthogonal_origin[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e15: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e25: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_odd_orthogonal_origin[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e35: ");
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
            let mut error = "Elements from VersorOddOrthogonalOrigin do not fit into AntiLineOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from([
            versor_odd_orthogonal_origin[e23],
            versor_odd_orthogonal_origin[e31],
            versor_odd_orthogonal_origin[e12],
        ])));
    }
}
