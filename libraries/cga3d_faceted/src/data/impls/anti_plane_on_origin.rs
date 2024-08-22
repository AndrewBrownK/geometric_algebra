use crate::traits::GeometricProduct;
use crate::traits::Wedge;
// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 357
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       3       0
//  Average:         3       7       0
//  Maximum:        46      69       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       6       0
//  Average:         4       9       0
//  Maximum:        64      96       0
impl std::ops::Add<AntiCircleOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
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
impl std::ops::Add<AntiCircleRotor> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
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
impl std::ops::Add<AntiCircleRotorAligningOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group2()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            other.group1(),
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
impl std::ops::Add<AntiCircleRotorAligningOriginAtInfinity> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            other.group0(),
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
impl std::ops::Add<AntiCircleRotorAtInfinity> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group1()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
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
impl std::ops::Add<AntiCircleRotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group1(),
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
impl std::ops::Add<AntiDipoleInversion> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiDipoleInversion) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e4
            other.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group0()[0] + other.group3()[0]),
                (self.group0()[1] + other.group3()[1]),
                (self.group0()[2] + other.group3()[2]),
                other.group3()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionAtInfinity> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125
            other.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group0()[0] + other.group2()[0]),
                (self.group0()[1] + other.group2()[1]),
                (self.group0()[2] + other.group2()[2]),
                other.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let addition = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0(),
            // e4, e1, e2, e3
            Simd32x4::from([
                other.group1()[0],
                (self.group0()[0] + other.group1()[1]),
                (self.group0()[1] + other.group1()[2]),
                (self.group0()[2] + other.group1()[3]),
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleInversionOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversion;
    fn add(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e235, e315, e125, e4
            other.group2(),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDipoleOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn add(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let addition = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0(),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, self.group0()[0], self.group0()[1], self.group0()[2]]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiDualNum> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiDualNum) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[1], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlatOrigin> for AntiPlaneOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn add(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([other[e321], self.group0()[0], self.group0()[1], self.group0()[2]]));
        return addition;
    }
}
impl std::ops::Add<AntiFlatPoint> for AntiPlaneOnOrigin {
    type Output = AntiFlector;
    fn add(self, other: AntiFlatPoint) -> Self::Output {
        let addition = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0(),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<AntiFlector> for AntiPlaneOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiFlector) -> Self::Output {
        let addition = AntiFlector::from_groups(
            // e235, e315, e125, e321
            other.group0(),
            // e1, e2, e3, e5
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
impl std::ops::Add<AntiFlectorOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let addition = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([
            other.group0()[0],
            (self.group0()[0] + other.group0()[1]),
            (self.group0()[1] + other.group0()[2]),
            (self.group0()[2] + other.group0()[3]),
        ]));
        return addition;
    }
}
impl std::ops::Add<AntiLine> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiLine) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            other.group0(),
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
impl std::ops::Add<AntiLineOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiLineOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            other.group0(),
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
impl std::ops::Add<AntiMotor> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMotor) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group1()[3],
        );
        return addition;
    }
}
impl std::ops::Add<AntiMotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiMotorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
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
impl std::ops::Add<AntiPlane> for AntiPlaneOnOrigin {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiPlane) -> Self::Output {
        let addition = AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([
            (self.group0()[0] + other.group0()[0]),
            (self.group0()[1] + other.group0()[1]),
            (self.group0()[2] + other.group0()[2]),
            other.group0()[3],
        ]));
        return addition;
    }
}
impl std::ops::Add<AntiPlaneOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn add(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let addition = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ (self.group0() + other.group0()));
        return addition;
    }
}
impl std::ops::AddAssign<AntiPlaneOnOrigin> for AntiPlaneOnOrigin {
    fn add_assign(&mut self, other: AntiPlaneOnOrigin) {
        let addition = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ (self.group0() + other.group0()));
        *self = addition;
    }
}
impl std::ops::Add<AntiScalar> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other[e12345], self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<AntiSphereOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiSphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: AntiSphereOnOrigin) -> Self::Output {
        let addition = AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([
            (self.group0()[0] + other.group0()[0]),
            (self.group0()[1] + other.group0()[1]),
            (self.group0()[2] + other.group0()[2]),
            other.group0()[3],
        ]));
        return addition;
    }
}
impl std::ops::Add<AntiVersorEvenOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Circle> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversion;
    fn add(self, other: Circle) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e4
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleAligningOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversion;
    fn add(self, other: CircleAligningOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtInfinity> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    fn add(self, other: CircleAtInfinity) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125
            other.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleAtOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn add(self, other: CircleAtOrigin) -> Self::Output {
        let addition = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversion;
    fn add(self, other: CircleOnOrigin) -> Self::Output {
        let addition = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn add(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let addition = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotor> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    fn add(self, other: CircleRotor) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e5
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    fn add(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group2()[0], other.group2()[1], other.group2()[2], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAligningOriginAtInfinity> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other.group1()[3], self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorAtInfinity> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: CircleRotorAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other.group1()[3], self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            other.group0(),
            // e235, e315, e125, e5
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<CircleRotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    fn add(self, other: CircleRotorOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<Dipole> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Dipole) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
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
impl std::ops::Add<DipoleAligningOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Add<DipoleAtInfinity> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
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
impl std::ops::Add<DipoleAtOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Add<DipoleInversion> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversion) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]),
            // e3215
            other.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAligningOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtInfinity> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            other.group1(),
            // e23, e31, e12
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionAtOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group1()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            other.group1(),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<DipoleInversionOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            other.group1(),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<DipoleOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            other.group0(),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Add<DipoleOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            other.group2(),
            // e23, e31, e12
            other.group1(),
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
impl std::ops::Add<DualNum> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: DualNum) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other.group0()[1], self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
        );
        return addition;
    }
}
impl std::ops::Add<FlatOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other[e45]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Add<FlatPoint> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatPoint) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Add<FlatPointAtInfinity> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlatPointAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            other.group0(),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Add<Flector> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Flector) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e3215
            other.group1()[3],
        );
        return addition;
    }
}
impl std::ops::Add<FlectorAtInfinity> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlectorAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<FlectorOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: FlectorOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group0()[0]]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<Horizon> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            other[e3215],
        );
        return addition;
    }
}
impl std::ops::Add<Infinity> for AntiPlaneOnOrigin {
    type Output = AntiPlane;
    fn add(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let addition = AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other[e5]]));
        return addition;
    }
}
impl std::ops::Add<Line> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    fn add(self, other: Line) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e235, e315, e125
            other.group1(),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<LineAtInfinity> for AntiPlaneOnOrigin {
    type Output = AntiFlector;
    fn add(self, other: LineAtInfinity) -> Self::Output {
        let addition = AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<LineOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    fn add(self, other: LineOnOrigin) -> Self::Output {
        let addition = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return addition;
    }
}
impl std::ops::Add<Motor> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: Motor) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other.group0()[3], self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e235, e315, e125, e5
            other.group1(),
        );
        return addition;
    }
}
impl std::ops::Add<MotorAtInfinity> for AntiPlaneOnOrigin {
    type Output = AntiFlector;
    fn add(self, other: MotorAtInfinity) -> Self::Output {
        let addition = AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<MotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    fn add(self, other: MotorOnOrigin) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([other.group0()[3], self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
        return addition;
    }
}
impl std::ops::Add<MultiVector> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            other.group0(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] + other.group1()[0]),
                (self.group0()[1] + other.group1()[1]),
                (self.group0()[2] + other.group1()[2]),
                other.group1()[3],
            ]),
            // e5
            other[e1],
            // e41, e42, e43, e45
            other.group3(),
            // e15, e25, e35
            other.group4(),
            // e23, e31, e12
            other.group5(),
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
impl std::ops::Add<NullCircleAtOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn add(self, other: NullCircleAtOrigin) -> Self::Output {
        let addition = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, self.group0()[0], self.group0()[1], self.group0()[2]]),
        );
        return addition;
    }
}
impl std::ops::Add<NullDipoleAtOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: NullDipoleAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Add<NullDipoleInversionAtOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[3], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<NullSphereAtOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e1234], 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<NullVersorEvenAtOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    fn add(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let addition = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e4, e1, e2, e3
            Simd32x4::from([other.group0()[3], self.group0()[0], self.group0()[1], self.group0()[2]]),
        );
        return addition;
    }
}
impl std::ops::Add<Origin> for AntiPlaneOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn add(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let addition = AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other[e4]]));
        return addition;
    }
}
impl std::ops::Add<Plane> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Plane) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<PlaneOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: PlaneOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<RoundPoint> for AntiPlaneOnOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let addition = RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] + other.group0()[0]),
                (self.group0()[1] + other.group0()[1]),
                (self.group0()[2] + other.group0()[2]),
                other.group0()[3],
            ]),
            // e5
            other[e2],
        );
        return addition;
    }
}
impl std::ops::Add<RoundPointAtOrigin> for AntiPlaneOnOrigin {
    type Output = RoundPoint;
    fn add(self, other: RoundPointAtOrigin) -> Self::Output {
        let addition = RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[0]]),
            // e5
            other.group0()[1],
        );
        return addition;
    }
}
impl std::ops::Add<Scalar> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other[scalar], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Add<Sphere> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other[e4315], other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            other.group0()[3],
        );
        return addition;
    }
}
impl std::ops::Add<SphereAtOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: SphereAtOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[1], 0.0, 0.0, 0.0]),
            // e3215
            other.group0()[0],
        );
        return addition;
    }
}
impl std::ops::Add<SphereOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: SphereOnOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group0()[3], other.group0()[0], other.group0()[1], other.group0()[2]]),
            // e3215
            0.0,
        );
        return addition;
    }
}
impl std::ops::Add<VersorEven> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: VersorEven) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e5
            other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] + other.group3()[0]),
                (self.group0()[1] + other.group3()[1]),
                (self.group0()[2] + other.group3()[2]),
                other.group3()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAligningOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    fn add(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e235, e315, e125, e5
            other.group2(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group1()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtInfinity> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: VersorEvenAtInfinity) -> Self::Output {
        let addition = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                other.group0()[0],
                (self.group0()[0] + other.group0()[1]),
                (self.group0()[1] + other.group0()[2]),
                (self.group0()[2] + other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            other.group1(),
            // e235, e315, e125, e5
            other.group2(),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenAtOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    fn add(self, other: VersorEvenAtOrigin) -> Self::Output {
        let addition = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e235, e315, e125, e5
            other.group1(),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group0()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOnOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    fn add(self, other: VersorEvenOnOrigin) -> Self::Output {
        let addition = VersorEven::from_groups(
            // e423, e431, e412, e12345
            other.group0(),
            // e415, e425, e435, e321
            Simd32x4::from([other.group1()[0], other.group1()[1], other.group1()[2], 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], other.group1()[3]]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorEvenOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        0        0
    fn add(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let addition = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            other.group0(),
            // e235, e315, e125, e5
            other.group1(),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] + other.group2()[0]),
                (self.group0()[1] + other.group2()[1]),
                (self.group0()[2] + other.group2()[2]),
                other.group2()[3],
            ]),
        );
        return addition;
    }
}
impl std::ops::Add<VersorOdd> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOdd) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]),
            // e3215
            other.group3()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddAtInfinity> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOddAtInfinity) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[0], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, other.group1()[3]]),
            // e15, e25, e35
            Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e3215
            other.group2()[3],
        );
        return addition;
    }
}
impl std::ops::Add<VersorOddOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    fn add(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let addition = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([other.group0()[3], 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], 0.0]),
            // e15, e25, e35
            Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]),
            // e23, e31, e12
            Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([other.group2()[3], 0.0, 0.0, 0.0]),
            // e3215
            other.group1()[3],
        );
        return addition;
    }
}
impl std::ops::BitXor<AntiCircleOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotor> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5       18        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       20        0
    //  no simd        8       24        0
    fn bitxor(self, other: AntiCircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       18        0
    fn bitxor(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAligningOriginAtInfinity> for AntiPlaneOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorAtInfinity> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2       12        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        3       14        0
    //  no simd        5       18        0
    fn bitxor(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiCircleRotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversion> for AntiPlaneOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       21        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       23        0
    //  no simd       10       27        0
    fn bitxor(self, other: AntiDipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionAtInfinity> for AntiPlaneOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       15        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        8       16        0
    //  no simd        8       18        0
    fn bitxor(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       15        0
    fn bitxor(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleInversionOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       21        0
    fn bitxor(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDipoleOnOrigin> for AntiPlaneOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiDualNum> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: AntiDualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<AntiDualNum> for AntiPlaneOnOrigin {
    fn bitxor_assign(&mut self, other: AntiDualNum) {
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlatPoint> for AntiPlaneOnOrigin {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiFlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlector> for AntiPlaneOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: AntiFlector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiFlectorOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: AntiFlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLine> for AntiPlaneOnOrigin {
    type Output = AntiFlatPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: AntiLine) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiLineOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiFlatOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotor> for AntiPlaneOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: AntiMotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiMotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: AntiMotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlane> for AntiPlaneOnOrigin {
    type Output = AntiLine;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        3        0
    // no simd        3        9        0
    fn bitxor(self, other: AntiPlane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiPlaneOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiSphereOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiCircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        4        0
    // no simd        3       12        0
    fn bitxor(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<AntiVersorEvenOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Circle> for AntiPlaneOnOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: Circle) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAligningOrigin> for AntiPlaneOnOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: CircleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtInfinity> for AntiPlaneOnOrigin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: CircleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleAtOrigin> for AntiPlaneOnOrigin {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn bitxor(self, other: CircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOnOrigin> for AntiPlaneOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: CircleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = SphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4        6        0
    fn bitxor(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotor> for AntiPlaneOnOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: CircleRotor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOrigin> for AntiPlaneOnOrigin {
    type Output = Sphere;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       12        0
    fn bitxor(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAligningOriginAtInfinity> for AntiPlaneOnOrigin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorAtInfinity> for AntiPlaneOnOrigin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<CircleRotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = SphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Dipole> for AntiPlaneOnOrigin {
    type Output = Circle;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        2        4        0
    // Totals...
    // yes simd        4       13        0
    //  no simd        8       21        0
    fn bitxor(self, other: Dipole) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAligningOrigin> for AntiPlaneOnOrigin {
    type Output = CircleAligningOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        6        0
    // no simd        6       18        0
    fn bitxor(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtInfinity> for AntiPlaneOnOrigin {
    type Output = CircleAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        5       15        0
    fn bitxor(self, other: DipoleAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleAtOrigin> for AntiPlaneOnOrigin {
    type Output = CircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        4        0
    // no simd        6       12        0
    fn bitxor(self, other: DipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversion> for AntiPlaneOnOrigin {
    type Output = CircleRotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        8       20        0
    //  no simd       10       24        0
    fn bitxor(self, other: DipoleInversion) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAligningOrigin> for AntiPlaneOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        4        0
    // Totals...
    // yes simd        6       13        0
    //  no simd        8       21        0
    fn bitxor(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtInfinity> for AntiPlaneOnOrigin {
    type Output = CircleRotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       18        0
    fn bitxor(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionAtOrigin> for AntiPlaneOnOrigin {
    type Output = CircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        2        4        0
    // no simd        6       12        0
    fn bitxor(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOnOrigin> for AntiPlaneOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       15        0
    fn bitxor(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleInversionOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = CircleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        8       15        0
    fn bitxor(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOnOrigin> for AntiPlaneOnOrigin {
    type Output = CircleOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        4        0
    // no simd        3       12        0
    fn bitxor(self, other: DipoleOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DipoleOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = CircleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        6       11        0
    //  no simd        8       15        0
    fn bitxor(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<DualNum> for AntiPlaneOnOrigin {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: DualNum) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatOrigin> for AntiPlaneOnOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPoint> for AntiPlaneOnOrigin {
    type Output = Line;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        4        0
    // no simd        3       12        0
    fn bitxor(self, other: FlatPoint) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlatPointAtInfinity> for AntiPlaneOnOrigin {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Flector> for AntiPlaneOnOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        5        9        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        5       11        0
    //  no simd        5       15        0
    fn bitxor(self, other: Flector) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorAtInfinity> for AntiPlaneOnOrigin {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: FlectorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<FlectorOnOrigin> for AntiPlaneOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        9        0
    fn bitxor(self, other: FlectorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Infinity> for AntiPlaneOnOrigin {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Line> for AntiPlaneOnOrigin {
    type Output = Plane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn bitxor(self, other: Line) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineAtInfinity> for AntiPlaneOnOrigin {
    type Output = Horizon;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: LineAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<LineOnOrigin> for AntiPlaneOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: LineOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Motor> for AntiPlaneOnOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       12        0
    fn bitxor(self, other: Motor) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorAtInfinity> for AntiPlaneOnOrigin {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        6        0
    fn bitxor(self, other: MotorAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: MotorOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<MultiVector> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       33        0
    //    simd3        3        7        0
    // Totals...
    // yes simd       14       40        0
    //  no simd       20       54        0
    fn bitxor(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullCircleAtOrigin> for AntiPlaneOnOrigin {
    type Output = NullSphereAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleAtOrigin> for AntiPlaneOnOrigin {
    type Output = NullCircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullDipoleInversionAtOrigin> for AntiPlaneOnOrigin {
    type Output = NullCircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        2        0
    // no simd        3        6        0
    fn bitxor(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<NullVersorEvenAtOrigin> for AntiPlaneOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        9        0
    fn bitxor(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Origin> for AntiPlaneOnOrigin {
    type Output = NullDipoleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn bitxor(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Plane> for AntiPlaneOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: Plane) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<PlaneOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: PlaneOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPoint> for AntiPlaneOnOrigin {
    type Output = DipoleOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        5        0
    // no simd        3       15        0
    fn bitxor(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXor<RoundPointAtOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn bitxor(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<Scalar> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn bitxor(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.wedge(other);
    }
}
impl std::ops::BitXorAssign<Scalar> for AntiPlaneOnOrigin {
    fn bitxor_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.wedge(other);
    }
}
impl std::ops::BitXor<Sphere> for AntiPlaneOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: Sphere) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<SphereOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiScalar;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        2        3        0
    fn bitxor(self, other: SphereOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEven> for AntiPlaneOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       21        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       23        0
    //  no simd       10       27        0
    fn bitxor(self, other: VersorEven) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAligningOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       21        0
    fn bitxor(self, other: VersorEvenAligningOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtInfinity> for AntiPlaneOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       15        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        8       16        0
    //  no simd        8       18        0
    fn bitxor(self, other: VersorEvenAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenAtOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        4       15        0
    fn bitxor(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOnOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       15        0
    fn bitxor(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorEvenOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        4       15        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        5       17        0
    //  no simd        7       21        0
    fn bitxor(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOdd> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       27        0
    fn bitxor(self, other: VersorOdd) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddAtInfinity> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        7       21        0
    fn bitxor(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::BitXor<VersorOddOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        8       18        0
    fn bitxor(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.wedge(other);
    }
}
impl std::ops::Mul<AntiCircleOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       18        0
    fn mul(self, other: AntiCircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotor> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       11       26        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       13       29        0
    //  no simd       18       36        0
    fn mul(self, other: AntiCircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       14       29        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       15       30        0
    //  no simd       18       33        0
    fn mul(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorAligningOriginAtInfinity> for AntiPlaneOnOrigin {
    type Output = AntiFlector;
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
impl std::ops::Mul<AntiCircleRotorAtInfinity> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        6       17        0
    //    simd3        1        2        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        8       20        0
    //  no simd       13       27        0
    fn mul(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiCircleRotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       21        0
    fn mul(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversion> for AntiPlaneOnOrigin {
    type Output = VersorOdd;
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
impl std::ops::Mul<AntiDipoleInversionAtInfinity> for AntiPlaneOnOrigin {
    type Output = VersorOddAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       13       25        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       15       27        0
    //  no simd       21       33        0
    fn mul(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDipoleInversionOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionAligningOrigin;
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
impl std::ops::Mul<AntiDipoleOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiVersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       15        0
    fn mul(self, other: AntiDipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiDualNum> for AntiPlaneOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        9        0
    fn mul(self, other: AntiDualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatOrigin> for AntiPlaneOnOrigin {
    type Output = AntiLineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlatPoint> for AntiPlaneOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       11        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2       12        0
    //  no simd        5       15        0
    fn mul(self, other: AntiFlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlector> for AntiPlaneOnOrigin {
    type Output = AntiMotor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: AntiFlector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiFlectorOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiMotorOnOrigin;
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
impl std::ops::Mul<AntiLine> for AntiPlaneOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       18        0
    fn mul(self, other: AntiLine) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiLineOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: AntiLineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotor> for AntiPlaneOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: AntiMotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiMotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiFlectorOnOrigin;
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
impl std::ops::Mul<AntiPlane> for AntiPlaneOnOrigin {
    type Output = AntiCircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        5       12        0
    fn mul(self, other: AntiPlane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiPlaneOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiMotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: AntiPlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiScalar> for AntiPlaneOnOrigin {
    type Output = PlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiSphereOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiCircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        5       15        0
    fn mul(self, other: AntiSphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<AntiVersorEvenOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Circle> for AntiPlaneOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd3        1        2        0
    // Totals...
    // yes simd       13       29        0
    //  no simd       15       33        0
    fn mul(self, other: Circle) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAligningOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       27        0
    fn mul(self, other: CircleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtInfinity> for AntiPlaneOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        8       20        0
    //  no simd       10       24        0
    fn mul(self, other: CircleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleAtOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       18        0
    fn mul(self, other: CircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOnOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       18        0
    fn mul(self, other: CircleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       10       24        0
    fn mul(self, other: CircleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotor> for AntiPlaneOnOrigin {
    type Output = DipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       15       30        0
    //    simd3        1        2        0
    // Totals...
    // yes simd       16       32        0
    //  no simd       18       36        0
    fn mul(self, other: CircleRotor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionAligningOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       18       30        0
    fn mul(self, other: CircleRotorAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAligningOriginAtInfinity> for AntiPlaneOnOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       13       21        0
    fn mul(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorAtInfinity> for AntiPlaneOnOrigin {
    type Output = DipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       21        0
    //    simd3        1        2        0
    // Totals...
    // yes simd       11       23        0
    //  no simd       13       27        0
    fn mul(self, other: CircleRotorAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<CircleRotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        9       17        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       13       21        0
    fn mul(self, other: CircleRotorOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Dipole> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       12       27        0
    //    simd3        1        2        0
    // Totals...
    // yes simd       13       29        0
    //  no simd       15       33        0
    fn mul(self, other: Dipole) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAligningOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       10       18        0
    //    simd3        0        2        0
    // Totals...
    // yes simd       10       20        0
    //  no simd       10       24        0
    fn mul(self, other: DipoleAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtInfinity> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        7       18        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        8       20        0
    //  no simd       10       24        0
    fn mul(self, other: DipoleAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleAtOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       18        0
    fn mul(self, other: DipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversion> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       21       37        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       23       39        0
    //  no simd       29       45        0
    fn mul(self, other: DipoleInversion) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAligningOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       24       36        0
    fn mul(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtInfinity> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       21       33        0
    fn mul(self, other: DipoleInversionAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionAtOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: DipoleInversionAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOnOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: DipoleInversionOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleInversionOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       17       29        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       21       33        0
    fn mul(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOnOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5       15        0
    fn mul(self, other: DipoleOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DipoleOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       15       27        0
    fn mul(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<DualNum> for AntiPlaneOnOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn mul(self, other: DualNum) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatOrigin> for AntiPlaneOnOrigin {
    type Output = LineOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPoint> for AntiPlaneOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        1       14        0
    //    simd4        1        1        0
    // Totals...
    // yes simd        2       15        0
    //  no simd        5       18        0
    fn mul(self, other: FlatPoint) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlatPointAtInfinity> for AntiPlaneOnOrigin {
    type Output = MotorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: FlatPointAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Flector> for AntiPlaneOnOrigin {
    type Output = Motor;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: Flector) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<FlectorAtInfinity> for AntiPlaneOnOrigin {
    type Output = MotorAtInfinity;
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
impl std::ops::Mul<FlectorOnOrigin> for AntiPlaneOnOrigin {
    type Output = MotorOnOrigin;
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
impl std::ops::Mul<Horizon> for AntiPlaneOnOrigin {
    type Output = LineAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Infinity> for AntiPlaneOnOrigin {
    type Output = FlatPointAtInfinity;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Line> for AntiPlaneOnOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       10       18        0
    fn mul(self, other: Line) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineAtInfinity> for AntiPlaneOnOrigin {
    type Output = FlectorAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: LineAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<LineOnOrigin> for AntiPlaneOnOrigin {
    type Output = FlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: LineOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Motor> for AntiPlaneOnOrigin {
    type Output = Flector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: Motor) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<MotorAtInfinity> for AntiPlaneOnOrigin {
    type Output = FlectorAtInfinity;
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
impl std::ops::Mul<MotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = FlectorOnOrigin;
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
impl std::ops::Mul<MultiVector> for AntiPlaneOnOrigin {
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
impl std::ops::Mul<NullCircleAtOrigin> for AntiPlaneOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: NullCircleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleAtOrigin> for AntiPlaneOnOrigin {
    type Output = NullVersorEvenAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: NullDipoleAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullDipoleInversionAtOrigin> for AntiPlaneOnOrigin {
    type Output = NullVersorEvenAtOrigin;
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
impl std::ops::Mul<NullSphereAtOrigin> for AntiPlaneOnOrigin {
    type Output = NullCircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<NullVersorEvenAtOrigin> for AntiPlaneOnOrigin {
    type Output = NullDipoleInversionAtOrigin;
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
impl std::ops::Mul<Origin> for AntiPlaneOnOrigin {
    type Output = NullDipoleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn mul(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Plane> for AntiPlaneOnOrigin {
    type Output = CircleRotorAligningOriginAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        5       15        0
    fn mul(self, other: Plane) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<PlaneOnOrigin> for AntiPlaneOnOrigin {
    type Output = MotorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        5        9        0
    fn mul(self, other: PlaneOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPoint> for AntiPlaneOnOrigin {
    type Output = AntiCircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        4        0
    // Totals...
    // yes simd        3       10        0
    //  no simd        5       18        0
    fn mul(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<RoundPointAtOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn mul(self, other: RoundPointAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<Scalar> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn mul(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::MulAssign<Scalar> for AntiPlaneOnOrigin {
    fn mul_assign(&mut self, other: Scalar) {
        use crate::elements::*;
        *self = self.geometric_product(other);
    }
}
impl std::ops::Mul<Sphere> for AntiPlaneOnOrigin {
    type Output = CircleRotorAligningOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        9        0
    //    simd3        1        3        0
    // Totals...
    // yes simd        3       12        0
    //  no simd        5       18        0
    fn mul(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereAtOrigin> for AntiPlaneOnOrigin {
    type Output = CircleAtOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        3        0
    // no simd        0        9        0
    fn mul(self, other: SphereAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<SphereOnOrigin> for AntiPlaneOnOrigin {
    type Output = CircleRotorOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        2        6        0
    //    simd3        1        2        0
    // Totals...
    // yes simd        3        8        0
    //  no simd        5       12        0
    fn mul(self, other: SphereOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEven> for AntiPlaneOnOrigin {
    type Output = VersorOdd;
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
impl std::ops::Mul<VersorEvenAligningOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionAligningOrigin;
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
impl std::ops::Mul<VersorEvenAtInfinity> for AntiPlaneOnOrigin {
    type Output = VersorOddAtInfinity;
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
impl std::ops::Mul<VersorEvenAtOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionAtOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       16       24        0
    fn mul(self, other: VersorEvenAtOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOnOrigin> for AntiPlaneOnOrigin {
    type Output = DipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        8       16        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       10       18        0
    //  no simd       16       24        0
    fn mul(self, other: VersorEvenOnOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorEvenOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = VersorOddOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32       24       36        0
    fn mul(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOdd> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       24       40        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       26       42        0
    //  no simd       32       48        0
    fn mul(self, other: VersorOdd) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddAtInfinity> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       16       28        0
    //    simd4        2        2        0
    // Totals...
    // yes simd       18       30        0
    //  no simd       24       36        0
    fn mul(self, other: VersorOddAtInfinity) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Mul<VersorOddOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32       20       32        0
    //    simd4        1        1        0
    // Totals...
    // yes simd       21       33        0
    //  no simd       24       36        0
    fn mul(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        return self.geometric_product(other);
    }
}
impl std::ops::Neg for AntiPlaneOnOrigin {
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn neg(self) -> Self {
        let negation = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ (self.group0() * Simd32x3::from(-1.0)));
        return negation;
    }
}
impl std::ops::Not for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn not(self) -> Self::Output {
        let dual = self;
        return dual;
    }
}
impl std::ops::Sub<AntiCircleOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: AntiCircleOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
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
impl std::ops::Sub<AntiCircleRotor> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       11        0
    fn sub(self, other: AntiCircleRotor) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group2()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
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
impl std::ops::Sub<AntiCircleRotorAligningOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       10        0
    fn sub(self, other: AntiCircleRotorAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group2()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
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
impl std::ops::Sub<AntiCircleRotorAligningOriginAtInfinity> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn sub(self, other: AntiCircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group1()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
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
impl std::ops::Sub<AntiCircleRotorAtInfinity> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn sub(self, other: AntiCircleRotorAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group1()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
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
impl std::ops::Sub<AntiCircleRotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        7        0
    fn sub(self, other: AntiCircleRotorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
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
impl std::ops::Sub<AntiDipoleInversion> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       12        0
    fn sub(self, other: AntiDipoleInversion) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e4
            (other.group2() * Simd32x4::from(-1.0)),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group0()[0] - other.group3()[0]),
                (self.group0()[1] - other.group3()[1]),
                (self.group0()[2] - other.group3()[2]),
                (other.group3()[3] * -1.0),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionAtInfinity> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        8        0
    fn sub(self, other: AntiDipoleInversionAtInfinity) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1, e2, e3, e5
            Simd32x4::from([
                (self.group0()[0] - other.group2()[0]),
                (self.group0()[1] - other.group2()[1]),
                (self.group0()[2] - other.group2()[2]),
                (other.group2()[3] * -1.0),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        5        0
    fn sub(self, other: AntiDipoleInversionOnOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e4, e1, e2, e3
            Simd32x4::from([
                (other.group1()[0] * -1.0),
                (self.group0()[0] - other.group1()[1]),
                (self.group0()[1] - other.group1()[2]),
                (self.group0()[2] - other.group1()[3]),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleInversionOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       11        0
    fn sub(self, other: AntiDipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e235, e315, e125, e4
            (other.group2() * Simd32x4::from(-1.0)),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDipoleOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiDipoleOnOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, self.group0()[0], self.group0()[1], self.group0()[2]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiDualNum> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: AntiDualNum) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[1] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group0()[0] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatOrigin> for AntiPlaneOnOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiFlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction =
            AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([(other[e321] * -1.0), self.group0()[0], self.group0()[1], self.group0()[2]]));
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlatPoint> for AntiPlaneOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: AntiFlatPoint) -> Self::Output {
        let subtraction = AntiFlector::from_groups(
            // e235, e315, e125, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiFlector> for AntiPlaneOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        3        2        0
    //  no simd        3        5        0
    fn sub(self, other: AntiFlector) -> Self::Output {
        let subtraction = AntiFlector::from_groups(
            // e235, e315, e125, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e1, e2, e3, e5
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
impl std::ops::Sub<AntiFlectorOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiFlectorOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: AntiFlectorOnOrigin) -> Self::Output {
        let subtraction = AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from([
            (other.group0()[0] * -1.0),
            (self.group0()[0] - other.group0()[1]),
            (self.group0()[1] - other.group0()[2]),
            (self.group0()[2] - other.group0()[3]),
        ]));
        return subtraction;
    }
}
impl std::ops::Sub<AntiLine> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        2        0
    // no simd        0        6        0
    fn sub(self, other: AntiLine) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
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
impl std::ops::Sub<AntiLineOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: AntiLineOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (other.group0() * Simd32x3::from(-1.0)),
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
impl std::ops::Sub<AntiMotor> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        8        0
    fn sub(self, other: AntiMotor) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group1()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiMotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: AntiMotorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
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
impl std::ops::Sub<AntiPlane> for AntiPlaneOnOrigin {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: AntiPlane) -> Self::Output {
        let subtraction = AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([
            (self.group0()[0] - other.group0()[0]),
            (self.group0()[1] - other.group0()[1]),
            (self.group0()[2] - other.group0()[2]),
            (other.group0()[3] * -1.0),
        ]));
        return subtraction;
    }
}
impl std::ops::Sub<AntiPlaneOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        1        0        0
    // no simd        3        0        0
    fn sub(self, other: AntiPlaneOnOrigin) -> Self::Output {
        let subtraction = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ (self.group0() - other.group0()));
        return subtraction;
    }
}
impl std::ops::SubAssign<AntiPlaneOnOrigin> for AntiPlaneOnOrigin {
    fn sub_assign(&mut self, other: AntiPlaneOnOrigin) {
        let subtraction = AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ (self.group0() - other.group0()));
        *self = subtraction;
    }
}
impl std::ops::Sub<AntiScalar> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: AntiScalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(other[e12345] * -1.0), self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<AntiSphereOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiSphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        1        0
    fn sub(self, other: AntiSphereOnOrigin) -> Self::Output {
        let subtraction = AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([
            (self.group0()[0] - other.group0()[0]),
            (self.group0()[1] - other.group0()[1]),
            (self.group0()[2] - other.group0()[2]),
            (other.group0()[3] * -1.0),
        ]));
        return subtraction;
    }
}
impl std::ops::Sub<AntiVersorEvenOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: AntiVersorEvenOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group1()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Circle> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       10        0
    fn sub(self, other: Circle) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e4
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAligningOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0        9        0
    fn sub(self, other: CircleAligningOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtInfinity> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: CircleAtInfinity) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleAtOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        6        0
    fn sub(self, other: CircleAtOrigin) -> Self::Output {
        let subtraction = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversion;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: CircleOnOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversion::from_groups(
            // e423, e431, e412
            (other.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn sub(self, other: CircleOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotor> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       11        0
    fn sub(self, other: CircleRotor) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) * Simd32x4::from(-1.0)),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       10        0
    fn sub(self, other: CircleRotorAligningOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group2()[3]]) * Simd32x4::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAligningOriginAtInfinity> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        7        0
    fn sub(self, other: CircleRotorAligningOriginAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(other.group1()[3] * -1.0), self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorAtInfinity> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn sub(self, other: CircleRotorAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(other.group1()[3] * -1.0), self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<CircleRotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        7        0
    fn sub(self, other: CircleRotorOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() * Simd32x4::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Dipole> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        2        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0       10        0
    fn sub(self, other: Dipole) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group2() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
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
impl std::ops::Sub<DipoleAligningOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //    simd3        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        7        0
    fn sub(self, other: DipoleAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Sub<DipoleAtInfinity> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        3        0
    //  no simd        0        7        0
    fn sub(self, other: DipoleAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
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
impl std::ops::Sub<DipoleAtOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: DipoleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Sub<DipoleInversion> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0       15        0
    fn sub(self, other: DipoleInversion) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group3()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAligningOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0       12        0
    fn sub(self, other: DipoleInversionAligningOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group1()[3], other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group2()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtInfinity> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       11        0
    fn sub(self, other: DipoleInversionAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (other.group1() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0)]),
            // e3215
            (other.group2()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionAtOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: DipoleInversionAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group1()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        2        0
    // no simd        0        8        0
    fn sub(self, other: DipoleInversionOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (other.group1() * Simd32x4::from(-1.0)),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleInversionOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        7        0
    //  no simd        0       11        0
    fn sub(self, other: DipoleInversionOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group2()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<DipoleOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: DipoleOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            (other.group0() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Sub<DipoleOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        9        0
    fn sub(self, other: DipoleOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (other.group2() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group1() * Simd32x3::from(-1.0)),
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
impl std::ops::Sub<DualNum> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: DualNum) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(other.group0()[1] * -1.0), self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlatOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: FlatOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other[e45] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Sub<FlatPoint> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: FlatPoint) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Sub<FlatPointAtInfinity> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd3        0        1        0
    // no simd        0        3        0
    fn sub(self, other: FlatPointAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (other.group0() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Sub<Flector> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        5        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0        8        0
    fn sub(self, other: Flector) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0)]),
            // e3215
            (other.group1()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlectorAtInfinity> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        4        0
    fn sub(self, other: FlectorAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<FlectorOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: FlectorOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group0()[0] * -1.0)]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), (other.group0()[3] * -1.0)]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<Horizon> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Horizon) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(0.0),
            // e3215
            (other[e3215] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Infinity> for AntiPlaneOnOrigin {
    type Output = AntiPlane;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Infinity) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other[e5] * -1.0)]));
        return subtraction;
    }
}
impl std::ops::Sub<Line> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        3        0
    //    simd3        0        1        0
    // Totals...
    // yes simd        0        4        0
    //  no simd        0        6        0
    fn sub(self, other: Line) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e235, e315, e125
            (other.group1() * Simd32x3::from(-1.0)),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<LineAtInfinity> for AntiPlaneOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: LineAtInfinity) -> Self::Output {
        let subtraction = AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<LineOnOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: LineOnOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversionAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Motor> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn sub(self, other: Motor) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(other.group0()[3] * -1.0), self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e235, e315, e125, e5
            (other.group1() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorAtInfinity> for AntiPlaneOnOrigin {
    type Output = AntiFlector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: MotorAtInfinity) -> Self::Output {
        let subtraction = AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e1, e2, e3, e5
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MotorOnOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: MotorOnOrigin) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([(other.group0()[3] * -1.0), self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<MultiVector> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        3        0
    //    simd2        0        1        0
    //    simd3        0        4        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3       11        0
    //  no simd        3       29        0
    fn sub(self, other: MultiVector) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            (other.group0() * Simd32x2::from(-1.0)),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] - other.group1()[0]),
                (self.group0()[1] - other.group1()[1]),
                (self.group0()[2] - other.group1()[2]),
                (other.group1()[3] * -1.0),
            ]),
            // e5
            (other[e1] * -1.0),
            // e41, e42, e43, e45
            (other.group3() * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (other.group4() * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (other.group5() * Simd32x3::from(-1.0)),
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
impl std::ops::Sub<NullCircleAtOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: NullCircleAtOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e4, e1, e2, e3
            Simd32x4::from([0.0, self.group0()[0], self.group0()[1], self.group0()[2]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullDipoleAtOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: NullDipoleAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Sub<NullDipoleInversionAtOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: NullDipoleInversionAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullSphereAtOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: NullSphereAtOrigin) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other[e1234] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<NullVersorEvenAtOrigin> for AntiPlaneOnOrigin {
    type Output = AntiDipoleInversionOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: NullVersorEvenAtOrigin) -> Self::Output {
        let subtraction = AntiDipoleInversionOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e4, e1, e2, e3
            Simd32x4::from([(other.group0()[3] * -1.0), self.group0()[0], self.group0()[1], self.group0()[2]]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Origin> for AntiPlaneOnOrigin {
    type Output = AntiSphereOnOrigin;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Origin) -> Self::Output {
        use crate::elements::*;
        let subtraction = AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other[e4] * -1.0)]));
        return subtraction;
    }
}
impl std::ops::Sub<Plane> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        4        0
    fn sub(self, other: Plane) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<PlaneOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        3        0
    fn sub(self, other: PlaneOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0)]),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPoint> for AntiPlaneOnOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        3        2        0
    fn sub(self, other: RoundPoint) -> Self::Output {
        use crate::elements::*;
        let subtraction = RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] - other.group0()[0]),
                (self.group0()[1] - other.group0()[1]),
                (self.group0()[2] - other.group0()[2]),
                (other.group0()[3] * -1.0),
            ]),
            // e5
            (other[e2] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<RoundPointAtOrigin> for AntiPlaneOnOrigin {
    type Output = RoundPoint;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: RoundPointAtOrigin) -> Self::Output {
        let subtraction = RoundPoint::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[0] * -1.0)]),
            // e5
            (other.group0()[1] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<Scalar> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        1        0
    fn sub(self, other: Scalar) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other[scalar] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
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
impl std::ops::Sub<Sphere> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        1        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        2        0
    //  no simd        0        5        0
    fn sub(self, other: Sphere) -> Self::Output {
        use crate::elements::*;
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other[e4315], other.group0()[0], other.group0()[1], other.group0()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group0()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereAtOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //      add/sub      mul      div
    // f32        0        2        0
    fn sub(self, other: SphereAtOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group0()[1] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group0()[0] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<SphereOnOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //          add/sub      mul      div
    //   simd4        0        1        0
    // no simd        0        4        0
    fn sub(self, other: SphereOnOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (swizzle!(other.group0(), 3, 0, 1, 2) * Simd32x4::from(-1.0)),
            // e3215
            0.0,
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEven> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        3        0
    // Totals...
    // yes simd        3        4        0
    //  no simd        3       13        0
    fn sub(self, other: VersorEven) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() * Simd32x4::from(-1.0)),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            (other.group2() * Simd32x4::from(-1.0)),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] - other.group3()[0]),
                (self.group0()[1] - other.group3()[1]),
                (self.group0()[2] - other.group3()[2]),
                (other.group3()[3] * -1.0),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAligningOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       12        0
    fn sub(self, other: VersorEvenAligningOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() * Simd32x4::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e235, e315, e125, e5
            (other.group2() * Simd32x4::from(-1.0)),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[3] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtInfinity> for AntiPlaneOnOrigin {
    type Output = VersorEvenAtInfinity;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        9        0
    fn sub(self, other: VersorEvenAtInfinity) -> Self::Output {
        let subtraction = VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([
                (other.group0()[0] * -1.0),
                (self.group0()[0] - other.group0()[1]),
                (self.group0()[1] - other.group0()[2]),
                (self.group0()[2] - other.group0()[3]),
            ]),
            // e415, e425, e435, e321
            (other.group1() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            (other.group2() * Simd32x4::from(-1.0)),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenAtOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn sub(self, other: VersorEvenAtOrigin) -> Self::Output {
        let subtraction = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e235, e315, e125, e5
            (other.group1() * Simd32x4::from(-1.0)),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group0()[3] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOnOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEven;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        4        0
    //    simd4        0        1        0
    // Totals...
    // yes simd        0        5        0
    //  no simd        0        8        0
    fn sub(self, other: VersorEvenOnOrigin) -> Self::Output {
        let subtraction = VersorEven::from_groups(
            // e423, e431, e412, e12345
            (other.group0() * Simd32x4::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from([(other.group1()[0] * -1.0), (other.group1()[1] * -1.0), (other.group1()[2] * -1.0), 0.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (other.group1()[3] * -1.0)]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorEvenOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = VersorEvenOrthogonalOrigin;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        3        1        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        3        3        0
    //  no simd        3        9        0
    fn sub(self, other: VersorEvenOrthogonalOrigin) -> Self::Output {
        let subtraction = VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            (other.group0() * Simd32x4::from(-1.0)),
            // e235, e315, e125, e5
            (other.group1() * Simd32x4::from(-1.0)),
            // e1, e2, e3, e4
            Simd32x4::from([
                (self.group0()[0] - other.group2()[0]),
                (self.group0()[1] - other.group2()[1]),
                (self.group0()[2] - other.group2()[2]),
                (other.group2()[3] * -1.0),
            ]),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOdd> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        2        0
    //    simd3        0        2        0
    //    simd4        0        2        0
    // Totals...
    // yes simd        0        6        0
    //  no simd        0       16        0
    fn sub(self, other: VersorOdd) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            (Simd32x4::from([other.group0()[0], other.group0()[1], other.group0()[2], other.group1()[3]]) * Simd32x4::from(-1.0)),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            (Simd32x4::from([other.group2()[3], other.group3()[0], other.group3()[1], other.group3()[2]]) * Simd32x4::from(-1.0)),
            // e3215
            (other.group3()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddAtInfinity> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn sub(self, other: VersorOddAtInfinity) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[0] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([0.0, 0.0, 0.0, (other.group1()[3] * -1.0)]),
            // e15, e25, e35
            (Simd32x3::from([other.group0()[1], other.group0()[2], other.group0()[3]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([0.0, (other.group2()[0] * -1.0), (other.group2()[1] * -1.0), (other.group2()[2] * -1.0)]),
            // e3215
            (other.group2()[3] * -1.0),
        );
        return subtraction;
    }
}
impl std::ops::Sub<VersorOddOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Output = MultiVector;
    // Operative Statistics for this implementation:
    //           add/sub      mul      div
    //      f32        0        6        0
    //    simd3        0        2        0
    // Totals...
    // yes simd        0        8        0
    //  no simd        0       12        0
    fn sub(self, other: VersorOddOrthogonalOrigin) -> Self::Output {
        let subtraction = MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(other.group0()[3] * -1.0), 0.0]),
            // e1, e2, e3, e4
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], 0.0]),
            // e5
            0.0,
            // e41, e42, e43, e45
            Simd32x4::from([(other.group0()[0] * -1.0), (other.group0()[1] * -1.0), (other.group0()[2] * -1.0), 0.0]),
            // e15, e25, e35
            (Simd32x3::from([other.group2()[0], other.group2()[1], other.group2()[2]]) * Simd32x3::from(-1.0)),
            // e23, e31, e12
            (Simd32x3::from([other.group1()[0], other.group1()[1], other.group1()[2]]) * Simd32x3::from(-1.0)),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from([(other.group2()[3] * -1.0), 0.0, 0.0, 0.0]),
            // e3215
            (other.group1()[3] * -1.0),
        );
        return subtraction;
    }
}

impl TryFrom<AntiDipoleInversion> for AntiPlaneOnOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion: AntiDipoleInversion) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversion do not fit into AntiPlaneOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from([
            anti_dipole_inversion[e1],
            anti_dipole_inversion[e2],
            anti_dipole_inversion[e3],
        ])));
    }
}

impl TryFrom<AntiDipoleInversionAtInfinity> for AntiPlaneOnOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion_at_infinity: AntiDipoleInversionAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionAtInfinity do not fit into AntiPlaneOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from([
            anti_dipole_inversion_at_infinity[e1],
            anti_dipole_inversion_at_infinity[e2],
            anti_dipole_inversion_at_infinity[e3],
        ])));
    }
}

impl TryFrom<AntiDipoleInversionOnOrigin> for AntiPlaneOnOrigin {
    type Error = String;
    fn try_from(anti_dipole_inversion_on_origin: AntiDipoleInversionOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_dipole_inversion_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_dipole_inversion_on_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiDipoleInversionOnOrigin do not fit into AntiPlaneOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from([
            anti_dipole_inversion_on_origin[e1],
            anti_dipole_inversion_on_origin[e2],
            anti_dipole_inversion_on_origin[e3],
        ])));
    }
}

impl TryFrom<AntiFlector> for AntiPlaneOnOrigin {
    type Error = String;
    fn try_from(anti_flector: AntiFlector) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flector[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = anti_flector[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlector do not fit into AntiPlaneOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from([anti_flector[e1], anti_flector[e2], anti_flector[e3]]),
        ));
    }
}

impl TryFrom<AntiFlectorOnOrigin> for AntiPlaneOnOrigin {
    type Error = String;
    fn try_from(anti_flector_on_origin: AntiFlectorOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_flector_on_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiFlectorOnOrigin do not fit into AntiPlaneOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from([
            anti_flector_on_origin[e1],
            anti_flector_on_origin[e2],
            anti_flector_on_origin[e3],
        ])));
    }
}

impl TryFrom<AntiPlane> for AntiPlaneOnOrigin {
    type Error = String;
    fn try_from(anti_plane: AntiPlane) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_plane[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiPlane do not fit into AntiPlaneOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from([anti_plane[e1], anti_plane[e2], anti_plane[e3]])));
    }
}

impl TryFrom<AntiSphereOnOrigin> for AntiPlaneOnOrigin {
    type Error = String;
    fn try_from(anti_sphere_on_origin: AntiSphereOnOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = anti_sphere_on_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from AntiSphereOnOrigin do not fit into AntiPlaneOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from([
            anti_sphere_on_origin[e1],
            anti_sphere_on_origin[e2],
            anti_sphere_on_origin[e3],
        ])));
    }
}

impl TryFrom<MultiVector> for AntiPlaneOnOrigin {
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
        let el = multi_vector[14];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e23: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e31: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = multi_vector[16];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12: ");
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
            let mut error = "Elements from MultiVector do not fit into AntiPlaneOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from([multi_vector[e1], multi_vector[e2], multi_vector[e3]]),
        ));
    }
}

impl TryFrom<RoundPoint> for AntiPlaneOnOrigin {
    type Error = String;
    fn try_from(round_point: RoundPoint) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = round_point[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = round_point[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from RoundPoint do not fit into AntiPlaneOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from([round_point[e1], round_point[e2], round_point[e3]]),
        ));
    }
}

impl TryFrom<VersorEven> for AntiPlaneOnOrigin {
    type Error = String;
    fn try_from(versor_even: VersorEven) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even[15];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEven do not fit into AntiPlaneOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiPlaneOnOrigin::from_groups(
            // e1, e2, e3
            Simd32x3::from([versor_even[e1], versor_even[e2], versor_even[e3]]),
        ));
    }
}

impl TryFrom<VersorEvenAtInfinity> for AntiPlaneOnOrigin {
    type Error = String;
    fn try_from(versor_even_at_infinity: VersorEvenAtInfinity) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_at_infinity[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e12345: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e415: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e425: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e435: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[8];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[9];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[10];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_at_infinity[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenAtInfinity do not fit into AntiPlaneOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from([
            versor_even_at_infinity[e1],
            versor_even_at_infinity[e2],
            versor_even_at_infinity[e3],
        ])));
    }
}

impl TryFrom<VersorEvenOrthogonalOrigin> for AntiPlaneOnOrigin {
    type Error = String;
    fn try_from(versor_even_orthogonal_origin: VersorEvenOrthogonalOrigin) -> Result<Self, Self::Error> {
        use crate::elements::*;
        let mut error_string = String::new();
        let mut fail = false;
        let el = versor_even_orthogonal_origin[0];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e423: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[1];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e431: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[2];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e412: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[3];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e321: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[4];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e235: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[5];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e315: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[6];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e125: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[7];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e5: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        let el = versor_even_orthogonal_origin[11];
        if el != 0.0 {
            fail = true;
            error_string.push_str("e4: ");
            error_string.push_str(el.to_string().as_str());
            error_string.push_str(", ");
        }
        if fail {
            let mut error = "Elements from VersorEvenOrthogonalOrigin do not fit into AntiPlaneOnOrigin { ".to_string();
            error.push_str(error_string.as_str());
            error.push('}');
            return Err(error);
        }
        return Ok(AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from([
            versor_even_orthogonal_origin[e1],
            versor_even_orthogonal_origin[e2],
            versor_even_orthogonal_origin[e3],
        ])));
    }
}
