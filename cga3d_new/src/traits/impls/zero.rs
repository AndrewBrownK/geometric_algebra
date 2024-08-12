impl Zero for AntiCircleOnOrigin {
    fn zero() -> Self {
        return AntiCircleOnOrigin::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e23, e31, e12 */ Simd32x3::from(0.0));
    }
}
impl Zero for AntiDipoleOnOrigin {
    fn zero() -> Self {
        return AntiDipoleOnOrigin::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(0.0));
    }
}
impl Zero for AntiFlatOrigin {
    fn zero() -> Self {
        return AntiFlatOrigin::from_groups(/* e321 */ 0.0);
    }
}
impl Zero for AntiFlatPoint {
    fn zero() -> Self {
        return AntiFlatPoint::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(0.0));
    }
}
impl Zero for AntiFlector {
    fn zero() -> Self {
        return AntiFlector::from_groups(/* e235, e315, e125, e321 */ Simd32x4::from(0.0), /* e1, e2, e3, e5 */ Simd32x4::from(0.0));
    }
}
impl Zero for AntiFlectorOnOrigin {
    fn zero() -> Self {
        return AntiFlectorOnOrigin::from_groups(/* e321, e1, e2, e3 */ Simd32x4::from(0.0));
    }
}
impl Zero for AntiLine {
    fn zero() -> Self {
        return AntiLine::from_groups(/* e23, e31, e12 */ Simd32x3::from(0.0), /* e15, e25, e35 */ Simd32x3::from(0.0));
    }
}
impl Zero for AntiLineOnOrigin {
    fn zero() -> Self {
        return AntiLineOnOrigin::from_groups(/* e23, e31, e12 */ Simd32x3::from(0.0));
    }
}
impl Zero for AntiMotor {
    fn zero() -> Self {
        return AntiMotor::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from(0.0), /* e15, e25, e35, e3215 */ Simd32x4::from(0.0));
    }
}
impl Zero for AntiMotorOnOrigin {
    fn zero() -> Self {
        return AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from(0.0));
    }
}
impl Zero for AntiPlane {
    fn zero() -> Self {
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(0.0));
    }
}
impl Zero for AntiPlaneOnOrigin {
    fn zero() -> Self {
        return AntiPlaneOnOrigin::from_groups(/* e1, e2, e3 */ Simd32x3::from(0.0));
    }
}
impl Zero for AntiScalar {
    fn zero() -> Self {
        return AntiScalar::from_groups(/* e12345 */ 0.0);
    }
}
impl Zero for AntiSphereOnOrigin {
    fn zero() -> Self {
        return AntiSphereOnOrigin::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(0.0));
    }
}
impl Zero for Circle {
    fn zero() -> Self {
        return Circle::from_groups(
            // e423, e431, e412, e321
            Simd32x4::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
        );
    }
}
impl Zero for CircleAligningOrigin {
    fn zero() -> Self {
        return CircleAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
        );
    }
}
impl Zero for CircleAtInfinity {
    fn zero() -> Self {
        return CircleAtInfinity::from_groups(/* e321, e415, e425, e435 */ Simd32x4::from(0.0), /* e235, e315, e125 */ Simd32x3::from(0.0));
    }
}
impl Zero for CircleAtOrigin {
    fn zero() -> Self {
        return CircleAtOrigin::from_groups(/* e423, e431, e412 */ Simd32x3::from(0.0), /* e235, e315, e125 */ Simd32x3::from(0.0));
    }
}
impl Zero for CircleOnOrigin {
    fn zero() -> Self {
        return CircleOnOrigin::from_groups(/* e423, e431, e412 */ Simd32x3::from(0.0), /* e415, e425, e435 */ Simd32x3::from(0.0));
    }
}
impl Zero for CircleOrthogonalOrigin {
    fn zero() -> Self {
        return CircleOrthogonalOrigin::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(0.0), /* e235, e315, e125 */ Simd32x3::from(0.0));
    }
}
impl Zero for Dipole {
    fn zero() -> Self {
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
        );
    }
}
impl Zero for DipoleAligningOrigin {
    fn zero() -> Self {
        return DipoleAligningOrigin::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e15, e25, e35, e45 */ Simd32x4::from(0.0));
    }
}
impl Zero for DipoleAtInfinity {
    fn zero() -> Self {
        return DipoleAtInfinity::from_groups(/* e23, e31, e12 */ Simd32x3::from(0.0), /* e15, e25, e35, e45 */ Simd32x4::from(0.0));
    }
}
impl Zero for DipoleAtOrigin {
    fn zero() -> Self {
        return DipoleAtOrigin::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0), /* e15, e25, e35 */ Simd32x3::from(0.0));
    }
}
impl Zero for DipoleOnOrigin {
    fn zero() -> Self {
        return DipoleOnOrigin::from_groups(/* e41, e42, e43, e45 */ Simd32x4::from(0.0));
    }
}
impl Zero for DipoleOrthogonalOrigin {
    fn zero() -> Self {
        return DipoleOrthogonalOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl Zero for DualNum {
    fn zero() -> Self {
        return DualNum::from_groups(/* scalar, e12345 */ Simd32x2::from(0.0));
    }
}
impl Zero for FlatOrigin {
    fn zero() -> Self {
        return FlatOrigin::from_groups(/* e45 */ 0.0);
    }
}
impl Zero for FlatPoint {
    fn zero() -> Self {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(0.0));
    }
}
impl Zero for FlatPointAtInfinity {
    fn zero() -> Self {
        return FlatPointAtInfinity::from_groups(/* e15, e25, e35 */ Simd32x3::from(0.0));
    }
}
impl Zero for Flector {
    fn zero() -> Self {
        return Flector::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(0.0), /* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0));
    }
}
impl Zero for FlectorAtInfinity {
    fn zero() -> Self {
        return FlectorAtInfinity::from_groups(/* e15, e25, e35, e3215 */ Simd32x4::from(0.0));
    }
}
impl Zero for FlectorOnOrigin {
    fn zero() -> Self {
        return FlectorOnOrigin::from_groups(/* e45, e4235, e4315, e4125 */ Simd32x4::from(0.0));
    }
}
impl Zero for Horizon {
    fn zero() -> Self {
        return Horizon::from_groups(/* e3215 */ 0.0);
    }
}
impl Zero for Infinity {
    fn zero() -> Self {
        return Infinity::from_groups(/* e5 */ 0.0);
    }
}
impl Zero for Line {
    fn zero() -> Self {
        return Line::from_groups(/* e415, e425, e435 */ Simd32x3::from(0.0), /* e235, e315, e125 */ Simd32x3::from(0.0));
    }
}
impl Zero for LineAtInfinity {
    fn zero() -> Self {
        return LineAtInfinity::from_groups(/* e235, e315, e125 */ Simd32x3::from(0.0));
    }
}
impl Zero for LineOnOrigin {
    fn zero() -> Self {
        return LineOnOrigin::from_groups(/* e415, e425, e435 */ Simd32x3::from(0.0));
    }
}
impl Zero for Motor {
    fn zero() -> Self {
        return Motor::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from(0.0), /* e235, e315, e125, e5 */ Simd32x4::from(0.0));
    }
}
impl Zero for MotorAtInfinity {
    fn zero() -> Self {
        return MotorAtInfinity::from_groups(/* e235, e315, e125, e5 */ Simd32x4::from(0.0));
    }
}
impl Zero for MotorOnOrigin {
    fn zero() -> Self {
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from(0.0));
    }
}
impl Zero for MultiVector {
    fn zero() -> Self {
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
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e321, e415, e425, e435
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e1234
            Simd32x4::from(0.0),
            // e3215
            0.0,
        );
    }
}
impl Zero for NullCircleAtOrigin {
    fn zero() -> Self {
        return NullCircleAtOrigin::from_groups(/* e423, e431, e412 */ Simd32x3::from(0.0));
    }
}
impl Zero for NullDipoleAtOrigin {
    fn zero() -> Self {
        return NullDipoleAtOrigin::from_groups(/* e41, e42, e43 */ Simd32x3::from(0.0));
    }
}
impl Zero for NullSphereAtOrigin {
    fn zero() -> Self {
        return NullSphereAtOrigin::from_groups(/* e1234 */ 0.0);
    }
}
impl Zero for Origin {
    fn zero() -> Self {
        return Origin::from_groups(/* e4 */ 0.0);
    }
}
impl Zero for Plane {
    fn zero() -> Self {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0));
    }
}
impl Zero for PlaneOnOrigin {
    fn zero() -> Self {
        return PlaneOnOrigin::from_groups(/* e4235, e4315, e4125 */ Simd32x3::from(0.0));
    }
}
impl Zero for RoundPoint {
    fn zero() -> Self {
        return RoundPoint::from_groups(/* e1, e2, e3 */ Simd32x3::from(0.0), /* e4, e5 */ Simd32x2::from(0.0));
    }
}
impl Zero for RoundPointAtOrigin {
    fn zero() -> Self {
        return RoundPointAtOrigin::from_groups(/* e4, e5 */ Simd32x2::from(0.0));
    }
}
impl Zero for Scalar {
    fn zero() -> Self {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl Zero for Sphere {
    fn zero() -> Self {
        return Sphere::from_groups(/* e4235, e4315, e4125 */ Simd32x3::from(0.0), /* e1234, e3215 */ Simd32x2::from(0.0));
    }
}
impl Zero for SphereAtOrigin {
    fn zero() -> Self {
        return SphereAtOrigin::from_groups(/* e1234, e3215 */ Simd32x2::from(0.0));
    }
}
impl Zero for SphereOnOrigin {
    fn zero() -> Self {
        return SphereOnOrigin::from_groups(/* e4235, e4315, e4125, e1234 */ Simd32x4::from(0.0));
    }
}
