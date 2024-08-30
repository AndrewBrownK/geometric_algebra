// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 33
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
//
//  No SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
impl Unit for AntiCircleRotor {
    fn unit() -> Self {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(1.0),
            // e15, e25, e35, scalar
            Simd32x4::from(1.0),
        );
    }
}
impl Unit for AntiDipoleInversion {
    fn unit() -> Self {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e235, e315, e125, e4
            Simd32x4::from(1.0),
            // e1, e2, e3, e5
            Simd32x4::from(1.0),
        );
    }
}
impl Unit for AntiDualNum321 {
    fn unit() -> Self {
        return AntiDualNum321::from_groups(/* e45, scalar */ Simd32x2::from(1.0));
    }
}
impl Unit for AntiDualNum4 {
    fn unit() -> Self {
        return AntiDualNum4::from_groups(/* e1234, scalar */ Simd32x2::from(1.0));
    }
}
impl Unit for AntiDualNum5 {
    fn unit() -> Self {
        return AntiDualNum5::from_groups(/* e3215, scalar */ Simd32x2::from(1.0));
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
impl Unit for AntiLine {
    fn unit() -> Self {
        return AntiLine::from_groups(/* e23, e31, e12 */ Simd32x3::from(1.0), /* e15, e25, e35 */ Simd32x3::from(1.0));
    }
}
impl Unit for AntiMotor {
    fn unit() -> Self {
        return AntiMotor::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from(1.0), /* e15, e25, e35, e3215 */ Simd32x4::from(1.0));
    }
}
impl Unit for AntiPlane {
    fn unit() -> Self {
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(1.0));
    }
}
impl Unit for AntiQuadNum {
    fn unit() -> Self {
        return AntiQuadNum::from_groups(/* e1234, e3215, e45, scalar */ Simd32x4::from(1.0));
    }
}
impl Unit for AntiScalar {
    fn unit() -> Self {
        return AntiScalar::from_groups(/* e12345 */ 1.0);
    }
}
impl Unit for AntiTripleNum {
    fn unit() -> Self {
        return AntiTripleNum::from_groups(/* e1234, e3215, scalar */ Simd32x3::from(1.0));
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
impl Unit for CircleRotor {
    fn unit() -> Self {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e235, e315, e125, e12345
            Simd32x4::from(1.0),
        );
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
impl Unit for DipoleInversion {
    fn unit() -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(1.0),
            // e23, e31, e12, e45
            Simd32x4::from(1.0),
            // e15, e25, e35, e1234
            Simd32x4::from(1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(1.0),
        );
    }
}
impl Unit for DualNum321 {
    fn unit() -> Self {
        return DualNum321::from_groups(/* e321, e12345 */ Simd32x2::from(1.0));
    }
}
impl Unit for DualNum4 {
    fn unit() -> Self {
        return DualNum4::from_groups(/* e4, e12345 */ Simd32x2::from(1.0));
    }
}
impl Unit for DualNum5 {
    fn unit() -> Self {
        return DualNum5::from_groups(/* e5, e12345 */ Simd32x2::from(1.0));
    }
}
impl Unit for FlatPoint {
    fn unit() -> Self {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(1.0));
    }
}
impl Unit for Flector {
    fn unit() -> Self {
        return Flector::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(1.0), /* e4235, e4315, e4125, e3215 */ Simd32x4::from(1.0));
    }
}
impl Unit for Line {
    fn unit() -> Self {
        return Line::from_groups(/* e415, e425, e435 */ Simd32x3::from(1.0), /* e235, e315, e125 */ Simd32x3::from(1.0));
    }
}
impl Unit for Motor {
    fn unit() -> Self {
        return Motor::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from(1.0), /* e235, e315, e125, e5 */ Simd32x4::from(1.0));
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
            // e15, e25, e35, e45
            Simd32x4::from(1.0),
            // e41, e42, e43
            Simd32x3::from(1.0),
            // e23, e31, e12
            Simd32x3::from(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(1.0),
            // e423, e431, e412
            Simd32x3::from(1.0),
            // e235, e315, e125
            Simd32x3::from(1.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(1.0),
            // e1234
            1.0,
        );
    }
}
impl Unit for Plane {
    fn unit() -> Self {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(1.0));
    }
}
impl Unit for QuadNum {
    fn unit() -> Self {
        return QuadNum::from_groups(/* e4, e5, e321, e12345 */ Simd32x4::from(1.0));
    }
}
impl Unit for RoundPoint {
    fn unit() -> Self {
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0), /* e5 */ 1.0);
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
impl Unit for TripleNum {
    fn unit() -> Self {
        return TripleNum::from_groups(/* e4, e5, e12345 */ Simd32x3::from(1.0));
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