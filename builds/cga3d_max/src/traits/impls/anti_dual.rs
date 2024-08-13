impl AntiDual for AntiCircleOnOrigin {
    type Output = CircleOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return CircleOnOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for AntiDipoleOnOrigin {
    type Output = DipoleOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return DipoleOnOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for AntiDualNum {
    type Output = AntiDualNum;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for AntiFlatOrigin {
    type Output = FlatOrigin;
    fn anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return FlatOrigin::from_groups(/* e45 */ (self[e321] * -1.0));
    }
}
impl AntiDual for AntiFlatPoint {
    type Output = FlatPoint;
    fn anti_dual(self) -> Self::Output {
        return FlatPoint::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for AntiFlector {
    type Output = Flector;
    fn anti_dual(self) -> Self::Output {
        return Flector::from_groups(
            // e15, e25, e35, e45
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
        );
    }
}
impl AntiDual for AntiFlectorOnOrigin {
    type Output = FlectorOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return FlectorOnOrigin::from_groups(
            // e45, e4235, e4315, e4125
            Simd32x4::from([(self.group0()[0] * -1.0), self.group0()[1], self.group0()[2], self.group0()[3]]),
        );
    }
}
impl AntiDual for AntiLine {
    type Output = Line;
    fn anti_dual(self) -> Self::Output {
        return Line::from_groups(
            // e415, e425, e435
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for AntiLineOnOrigin {
    type Output = LineOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ (self.group0() * Simd32x3::from(-1.0)));
    }
}
impl AntiDual for AntiMotor {
    type Output = Motor;
    fn anti_dual(self) -> Self::Output {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e235, e315, e125, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiDual for AntiMotorOnOrigin {
    type Output = MotorOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return MotorOnOrigin::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiDual for AntiPlane {
    type Output = Plane;
    fn anti_dual(self) -> Self::Output {
        return Plane::from_groups(
            // e4235, e4315, e4125, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for AntiPlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for AntiScalar {
    type Output = Scalar;
    fn anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Scalar::from_groups(/* scalar */ (self[e12345] * -1.0));
    }
}
impl AntiDual for AntiSphereOnOrigin {
    type Output = SphereOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return SphereOnOrigin::from_groups(
            // e4235, e4315, e4125, e1234
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for Circle {
    type Output = Dipole;
    fn anti_dual(self) -> Self::Output {
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e23, e31, e12
            self.group1(),
            // e15, e25, e35, e45
            Simd32x4::from([self.group2()[0], self.group2()[1], self.group2()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for CircleAligningOrigin {
    type Output = CircleAligningOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for CircleAtInfinity {
    type Output = DipoleAtInfinity;
    fn anti_dual(self) -> Self::Output {
        return DipoleAtInfinity::from_groups(
            // e23, e31, e12
            Simd32x3::from([self.group0()[1], self.group0()[2], self.group0()[3]]),
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group0()[0] * -1.0)]),
        );
    }
}
impl AntiDual for CircleAtOrigin {
    type Output = CircleAtOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for CircleOnOrigin {
    type Output = CircleOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for CircleOrthogonalOrigin {
    type Output = DipoleAligningOrigin;
    fn anti_dual(self) -> Self::Output {
        return DipoleAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from([self.group0()[0], self.group0()[1], self.group0()[2]]),
            // e15, e25, e35, e45
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for Dipole {
    type Output = Circle;
    fn anti_dual(self) -> Self::Output {
        return Circle::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group2()[3]]),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (Simd32x3::from([self.group2()[0], self.group2()[1], self.group2()[2]]) * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for DipoleAligningOrigin {
    type Output = CircleOrthogonalOrigin;
    fn anti_dual(self) -> Self::Output {
        return CircleOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group1()[3]]),
            // e235, e315, e125
            (Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for DipoleAtInfinity {
    type Output = CircleAtInfinity;
    fn anti_dual(self) -> Self::Output {
        return CircleAtInfinity::from_groups(
            // e321, e415, e425, e435
            Simd32x4::from([self.group1()[3], (self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0)]),
            // e235, e315, e125
            (Simd32x3::from([self.group1()[0], self.group1()[1], self.group1()[2]]) * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for DipoleAtOrigin {
    type Output = CircleAtOrigin;
    fn anti_dual(self) -> Self::Output {
        return CircleAtOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group1() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for DipoleOnOrigin {
    type Output = AntiDipoleOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return AntiDipoleOnOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiDual for DipoleOrthogonalOrigin {
    type Output = CircleAligningOrigin;
    fn anti_dual(self) -> Self::Output {
        return CircleAligningOrigin::from_groups(
            // e423, e431, e412
            (self.group0() * Simd32x3::from(-1.0)),
            // e415, e425, e435
            (self.group1() * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group2() * Simd32x3::from(-1.0)),
        );
    }
}
impl AntiDual for DualNum {
    type Output = AntiDualNum;
    fn anti_dual(self) -> Self::Output {
        return AntiDualNum::from_groups(/* e3215, scalar */ (self.group0() * Simd32x2::from(-1.0)));
    }
}
impl AntiDual for FlatOrigin {
    type Output = FlatOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for FlatPoint {
    type Output = AntiFlatPoint;
    fn anti_dual(self) -> Self::Output {
        return AntiFlatPoint::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiDual for FlatPointAtInfinity {
    type Output = LineAtInfinity;
    fn anti_dual(self) -> Self::Output {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ (self.group0() * Simd32x3::from(-1.0)));
    }
}
impl AntiDual for Flector {
    type Output = AntiFlector;
    fn anti_dual(self) -> Self::Output {
        return AntiFlector::from_groups(
            // e235, e315, e125, e321
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
            // e1, e2, e3, e5
            Simd32x4::from([(self.group1()[0] * -1.0), (self.group1()[1] * -1.0), (self.group1()[2] * -1.0), self.group1()[3]]),
        );
    }
}
impl AntiDual for FlectorAtInfinity {
    type Output = MotorAtInfinity;
    fn anti_dual(self) -> Self::Output {
        return MotorAtInfinity::from_groups(
            // e235, e315, e125, e5
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiDual for FlectorOnOrigin {
    type Output = AntiFlectorOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return AntiFlectorOnOrigin::from_groups(
            // e321, e1, e2, e3
            Simd32x4::from([self.group0()[0], (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for Horizon {
    type Output = Horizon;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for Infinity {
    type Output = Horizon;
    fn anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return Horizon::from_groups(/* e3215 */ (self[e5] * -1.0));
    }
}
impl AntiDual for Line {
    type Output = Line;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for LineAtInfinity {
    type Output = LineAtInfinity;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for LineOnOrigin {
    type Output = LineOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for Motor {
    type Output = AntiMotor;
    fn anti_dual(self) -> Self::Output {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
            // e15, e25, e35, e3215
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
        );
    }
}
impl AntiDual for MotorAtInfinity {
    type Output = FlectorAtInfinity;
    fn anti_dual(self) -> Self::Output {
        return FlectorAtInfinity::from_groups(
            // e15, e25, e35, e3215
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for MotorOnOrigin {
    type Output = AntiMotorOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return AntiMotorOnOrigin::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([self.group0()[0], self.group0()[1], self.group0()[2], (self.group0()[3] * -1.0)]),
        );
    }
}
impl AntiDual for MultiVector {
    type Output = MultiVector;
    fn anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([(self.group0()[1] * -1.0), self.group0()[0]]),
            // e1, e2, e3, e4
            Simd32x4::from([(self.group9()[0] * -1.0), (self.group9()[1] * -1.0), (self.group9()[2] * -1.0), self.group9()[3]]),
            // e5
            self[e45],
            // e41, e42, e43, e45
            Simd32x4::from([self.group7()[0], self.group7()[1], self.group7()[2], (self.group6()[0] * -1.0)]),
            // e15, e25, e35
            self.group8(),
            // e23, e31, e12
            Simd32x3::from([self.group6()[1], self.group6()[2], self.group6()[3]]),
            // e321, e415, e425, e435
            Simd32x4::from([self.group3()[3], (self.group5()[0] * -1.0), (self.group5()[1] * -1.0), (self.group5()[2] * -1.0)]),
            // e423, e431, e412
            (Simd32x3::from([self.group3()[0], self.group3()[1], self.group3()[2]]) * Simd32x3::from(-1.0)),
            // e235, e315, e125
            (self.group4() * Simd32x3::from(-1.0)),
            // e4235, e4315, e4125, e1234
            Simd32x4::from([self.group1()[0], self.group1()[1], self.group1()[2], (self.group1()[3] * -1.0)]),
            // e3215
            (self[e1] * -1.0),
        );
    }
}
impl AntiDual for NullCircleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for NullDipoleAtOrigin {
    type Output = NullCircleAtOrigin;
    fn anti_dual(self) -> Self::Output {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ (self.group0() * Simd32x3::from(-1.0)));
    }
}
impl AntiDual for NullSphereAtOrigin {
    type Output = NullSphereAtOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for Origin {
    type Output = NullSphereAtOrigin;
    fn anti_dual(self) -> Self::Output {
        use crate::elements::*;
        return NullSphereAtOrigin::from_groups(/* e1234 */ (self[e4] * -1.0));
    }
}
impl AntiDual for Plane {
    type Output = AntiPlane;
    fn anti_dual(self) -> Self::Output {
        return AntiPlane::from_groups(
            // e1, e2, e3, e5
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
impl AntiDual for PlaneOnOrigin {
    type Output = AntiPlaneOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ (self.group0() * Simd32x3::from(-1.0)));
    }
}
impl AntiDual for RoundPoint {
    type Output = Sphere;
    fn anti_dual(self) -> Self::Output {
        return Sphere::from_groups(/* e4235, e4315, e4125 */ self.group0(), /* e1234, e3215 */ (self.group1() * Simd32x2::from(-1.0)));
    }
}
impl AntiDual for RoundPointAtOrigin {
    type Output = SphereAtOrigin;
    fn anti_dual(self) -> Self::Output {
        return SphereAtOrigin::from_groups(/* e1234, e3215 */ (self.group0() * Simd32x2::from(-1.0)));
    }
}
impl AntiDual for Scalar {
    type Output = Scalar;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for Sphere {
    type Output = RoundPoint;
    fn anti_dual(self) -> Self::Output {
        return RoundPoint::from_groups(/* e1, e2, e3 */ (self.group0() * Simd32x3::from(-1.0)), /* e4, e5 */ self.group1());
    }
}
impl AntiDual for SphereAtOrigin {
    type Output = SphereAtOrigin;
    fn anti_dual(self) -> Self::Output {
        return self;
    }
}
impl AntiDual for SphereOnOrigin {
    type Output = AntiSphereOnOrigin;
    fn anti_dual(self) -> Self::Output {
        return AntiSphereOnOrigin::from_groups(
            // e1, e2, e3, e4
            Simd32x4::from([(self.group0()[0] * -1.0), (self.group0()[1] * -1.0), (self.group0()[2] * -1.0), self.group0()[3]]),
        );
    }
}
