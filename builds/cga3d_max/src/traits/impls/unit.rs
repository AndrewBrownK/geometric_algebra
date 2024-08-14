// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
impl Unit for AntiCircleOnOrigin {
    fn unit() -> Self {
        return AntiCircleOnOrigin::from_groups(/* e41, e42, e43 */ Simd32x3::from(1.0), /* e23, e31, e12 */ Simd32x3::from(1.0));
    }
}
impl Unit for AntiDipoleOnOrigin {
    fn unit() -> Self {
        return AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(1.0));
    }
}
impl Unit for AntiDualNum {
    fn unit() -> Self {
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from(1.0));
    }
}
impl Unit for AntiFlatOrigin {
    fn unit() -> Self {
        return AntiFlatOrigin::from_groups(/* e321 */ 1.0);
    }
}
impl Unit for AntiFlatPoint {
    fn unit() -> Self {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(1.0));
    }
}
impl Unit for AntiFlector {
    fn unit() -> Self {
        return AntiFlector::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(1.0), /* e1, e2, e3, e5 */ Simd32x4::from(1.0));
    }
}
impl Unit for AntiFlectorOnOrigin {
    fn unit() -> Self {
        return AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from(1.0));
    }
}
impl Unit for AntiLine {
    fn unit() -> Self {
        return AntiLine::from_groups(/* e23, e31, e12 */ Simd32x3::from(1.0), /* e15, e25, e35 */ Simd32x3::from(1.0));
    }
}
impl Unit for AntiLineOnOrigin {
    fn unit() -> Self {
        return AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from(1.0));
    }
}
impl Unit for AntiMotor {
    fn unit() -> Self {
        return AntiMotor::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from(1.0), /* e15, e25, e35, e3215 */ Simd32x4::from(1.0));
    }
}
impl Unit for AntiMotorOnOrigin {
    fn unit() -> Self {
        return AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from(1.0));
    }
}
impl Unit for AntiPlane {
    fn unit() -> Self {
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(1.0));
    }
}
impl Unit for AntiPlaneOnOrigin {
    fn unit() -> Self {
        return AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from(1.0));
    }
}
impl Unit for AntiScalar {
    fn unit() -> Self {
        return AntiScalar::from_groups(/* e12345 */ 1.0);
    }
}
impl Unit for AntiSphereOnOrigin {
    fn unit() -> Self {
        return AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0));
    }
}
impl Unit for AntiVersorEvenOnOrigin {
    fn unit() -> Self {
        return AntiVersorEvenOnOrigin::from_groups(/* e41, e42, e43, scalar */ Simd32x4::from(1.0), /* e23, e31, e12, e1234 */ Simd32x4::from(1.0));
    }
}
impl Unit for AntiVersorOddOnOrigin {
    fn unit() -> Self {
        return AntiVersorOddOnOrigin::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(1.0), /* e4, e1, e2, e3 */ Simd32x4::from(1.0));
    }
}
impl Unit for Circle {
    fn unit() -> Self {
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e235, e315, e125
            Simd32x3::from(1.0),
        );
    }
}
impl Unit for CircleAligningOrigin {
    fn unit() -> Self {
        return CircleAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(1.0),
            // e415, e425, e435
            Simd32x3::from(1.0),
            // e235, e315, e125
            Simd32x3::from(1.0),
        );
    }
}
impl Unit for CircleAtInfinity {
    fn unit() -> Self {
        return CircleAtInfinity::from_groups(/* e415, e425, e435, e321 */ Simd32x4::from(1.0), /* e235, e315, e125 */ Simd32x3::from(1.0));
    }
}
impl Unit for CircleAtOrigin {
    fn unit() -> Self {
        return CircleAtOrigin::from_groups(/* e423, e431, e412 */ Simd32x3::from(1.0), /* e235, e315, e125 */ Simd32x3::from(1.0));
    }
}
impl Unit for CircleOnOrigin {
    fn unit() -> Self {
        return CircleOnOrigin::from_groups(/* e423, e431, e412 */ Simd32x3::from(1.0), /* e415, e425, e435 */ Simd32x3::from(1.0));
    }
}
impl Unit for CircleOrthogonalOrigin {
    fn unit() -> Self {
        return CircleOrthogonalOrigin::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(1.0), /* e235, e315, e125 */ Simd32x3::from(1.0));
    }
}
impl Unit for Dipole {
    fn unit() -> Self {
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(1.0),
            // e15, e25, e35
            Simd32x3::from(1.0),
        );
    }
}
impl Unit for DipoleAligningOrigin {
    fn unit() -> Self {
        return DipoleAligningOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from(1.0), /* e15, e25, e35 */ Simd32x3::from(1.0));
    }
}
impl Unit for DipoleAtInfinity {
    fn unit() -> Self {
        return DipoleAtInfinity::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(1.0), /* e15, e25, e35 */ Simd32x3::from(1.0));
    }
}
impl Unit for DipoleAtOrigin {
    fn unit() -> Self {
        return DipoleAtOrigin::from_groups(/* e41, e42, e43 */ Simd32x3::from(1.0), /* e15, e25, e35 */ Simd32x3::from(1.0));
    }
}
impl Unit for DipoleOnOrigin {
    fn unit() -> Self {
        return DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from(1.0));
    }
}
impl Unit for DipoleOrthogonalOrigin {
    fn unit() -> Self {
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(1.0),
            // e23, e31, e12
            Simd32x3::from(1.0),
            // e15, e25, e35
            Simd32x3::from(1.0),
        );
    }
}
impl Unit for DualNum {
    fn unit() -> Self {
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from(1.0));
    }
}
impl Unit for FlatOrigin {
    fn unit() -> Self {
        return FlatOrigin::from_groups(/* e45 */ 1.0);
    }
}
impl Unit for FlatPoint {
    fn unit() -> Self {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(1.0));
    }
}
impl Unit for FlatPointAtInfinity {
    fn unit() -> Self {
        return FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ Simd32x3::from(1.0));
    }
}
impl Unit for Flector {
    fn unit() -> Self {
        return Flector::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(1.0), /* e4235, e4315, e4125, e3215 */ Simd32x4::from(1.0));
    }
}
impl Unit for FlectorAtInfinity {
    fn unit() -> Self {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from(1.0));
    }
}
impl Unit for FlectorOnOrigin {
    fn unit() -> Self {
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from(1.0));
    }
}
impl Unit for Horizon {
    fn unit() -> Self {
        return Horizon::from_groups(/* e3215 */ 1.0);
    }
}
impl Unit for Infinity {
    fn unit() -> Self {
        return Infinity::from_groups(/* e5 */ 1.0);
    }
}
impl Unit for Line {
    fn unit() -> Self {
        return Line::from_groups(/* e415, e425, e435 */ Simd32x3::from(1.0), /* e235, e315, e125 */ Simd32x3::from(1.0));
    }
}
impl Unit for LineAtInfinity {
    fn unit() -> Self {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ Simd32x3::from(1.0));
    }
}
impl Unit for LineOnOrigin {
    fn unit() -> Self {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ Simd32x3::from(1.0));
    }
}
impl Unit for Motor {
    fn unit() -> Self {
        return Motor::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from(1.0), /* e235, e315, e125, e5 */ Simd32x4::from(1.0));
    }
}
impl Unit for MotorAtInfinity {
    fn unit() -> Self {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from(1.0));
    }
}
impl Unit for MotorOnOrigin {
    fn unit() -> Self {
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from(1.0));
    }
}
impl Unit for MultiVector {
    fn unit() -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from(1.0),
            // e1, e2, e3, e4
            Simd32x4::from(1.0),
            // e5
            1.0,
            // e41, e42, e43, e45
            Simd32x4::from(1.0),
            // e15, e25, e35
            Simd32x3::from(1.0),
            // e23, e31, e12
            Simd32x3::from(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e423, e431, e412
            Simd32x3::from(1.0),
            // e235, e315, e125
            Simd32x3::from(1.0),
            // e1234, e4235, e4315, e4125
            Simd32x4::from(1.0),
            // e3215
            1.0,
        );
    }
}
impl Unit for NullCircleAtOrigin {
    fn unit() -> Self {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ Simd32x3::from(1.0));
    }
}
impl Unit for NullDipoleAtOrigin {
    fn unit() -> Self {
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ Simd32x3::from(1.0));
    }
}
impl Unit for NullSphereAtOrigin {
    fn unit() -> Self {
        return NullSphereAtOrigin::from_groups(/* e1234 */ 1.0);
    }
}
impl Unit for NullVersorEvenAtOrigin {
    fn unit() -> Self {
        return NullVersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from(1.0));
    }
}
impl Unit for NullVersorOddAtOrigin {
    fn unit() -> Self {
        return NullVersorOddAtOrigin::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from(1.0));
    }
}
impl Unit for Origin {
    fn unit() -> Self {
        return Origin::from_groups(/* e4 */ 1.0);
    }
}
impl Unit for Plane {
    fn unit() -> Self {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(1.0));
    }
}
impl Unit for PlaneOnOrigin {
    fn unit() -> Self {
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ Simd32x3::from(1.0));
    }
}
impl Unit for RoundPoint {
    fn unit() -> Self {
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0), /* e5 */ 1.0);
    }
}
impl Unit for RoundPointAtOrigin {
    fn unit() -> Self {
        return RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from(1.0));
    }
}
impl Unit for Scalar {
    fn unit() -> Self {
        return Scalar::from_groups(/* scalar */ 1.0);
    }
}
impl Unit for Sphere {
    fn unit() -> Self {
        return Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(1.0), /* e1234 */ 1.0);
    }
}
impl Unit for SphereAtOrigin {
    fn unit() -> Self {
        return SphereAtOrigin::from_groups(/* e3215, e1234 */ Simd32x2::from(1.0));
    }
}
impl Unit for SphereOnOrigin {
    fn unit() -> Self {
        return SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ Simd32x4::from(1.0));
    }
}
impl Unit for VersorEven {
    fn unit() -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e235, e315, e125, e5
            Simd32x4::from(1.0),
            // e1, e2, e3, e4
            Simd32x4::from(1.0),
        );
    }
}
impl Unit for VersorEvenAligningOrigin {
    fn unit() -> Self {
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(1.0),
            // e415, e425, e435, e4
            Simd32x4::from(1.0),
            // e235, e315, e125, e5
            Simd32x4::from(1.0),
        );
    }
}
impl Unit for VersorEvenAtInfinity {
    fn unit() -> Self {
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e235, e315, e125, e5
            Simd32x4::from(1.0),
        );
    }
}
impl Unit for VersorEvenAtOrigin {
    fn unit() -> Self {
        return VersorEvenAtOrigin::from_groups(/* e423, e431, e412, e4 */ Simd32x4::from(1.0), /* e235, e315, e125, e5 */ Simd32x4::from(1.0));
    }
}
impl Unit for VersorEvenOnOrigin {
    fn unit() -> Self {
        return VersorEvenOnOrigin::from_groups(/* e423, e431, e412, e12345 */ Simd32x4::from(1.0), /* e415, e425, e435, e4 */ Simd32x4::from(1.0));
    }
}
impl Unit for VersorEvenOrthogonalOrigin {
    fn unit() -> Self {
        return VersorEvenOrthogonalOrigin::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(1.0),
            // e235, e315, e125, e5
            Simd32x4::from(1.0),
            // e1, e2, e3, e4
            Simd32x4::from(1.0),
        );
    }
}
impl Unit for VersorOdd {
    fn unit() -> Self {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(1.0),
            // e15, e25, e35, e1234
            Simd32x4::from(1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(1.0),
        );
    }
}
impl Unit for VersorOddAligningOrigin {
    fn unit() -> Self {
        return VersorOddAligningOrigin::from_groups(
            // e41, e42, e43, e45
            Simd32x4::from(1.0),
            // e15, e25, e35, e1234
            Simd32x4::from(1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(1.0),
        );
    }
}
impl Unit for VersorOddAtInfinity {
    fn unit() -> Self {
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(1.0),
        );
    }
}
impl Unit for VersorOddAtOrigin {
    fn unit() -> Self {
        return VersorOddAtOrigin::from_groups(/* e41, e42, e43, e3215 */ Simd32x4::from(1.0), /* e15, e25, e35, e1234 */ Simd32x4::from(1.0));
    }
}
impl Unit for VersorOddOnOrigin {
    fn unit() -> Self {
        return VersorOddOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from(1.0), /* e1234, e4235, e4315, e4125 */ Simd32x4::from(1.0));
    }
}
impl Unit for VersorOddOrthogonalOrigin {
    fn unit() -> Self {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(1.0),
            // e23, e31, e12, e3215
            Simd32x4::from(1.0),
            // e15, e25, e35, e1234
            Simd32x4::from(1.0),
        );
    }
}
