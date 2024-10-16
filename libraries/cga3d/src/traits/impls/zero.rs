// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 25
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
impl Zero for AntiCircleRotor {
    fn zero() -> Self {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, scalar
            Simd32x4::from(0.0),
        );
    }
}
impl Zero for AntiDipoleInversion {
    fn zero() -> Self {
        return AntiDipoleInversion::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e4
            Simd32x4::from(0.0),
            // e1, e2, e3, e5
            Simd32x4::from(0.0),
        );
    }
}
impl Zero for AntiDualNum {
    fn zero() -> Self {
        return AntiDualNum::from_groups(/* e1234, scalar */ Simd32x2::from(0.0));
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
impl Zero for AntiLine {
    fn zero() -> Self {
        return AntiLine::from_groups(/* e23, e31, e12 */ Simd32x3::from(0.0), /* e15, e25, e35 */ Simd32x3::from(0.0));
    }
}
impl Zero for AntiMotor {
    fn zero() -> Self {
        return AntiMotor::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from(0.0), /* e15, e25, e35, e3215 */ Simd32x4::from(0.0));
    }
}
impl Zero for AntiPlane {
    fn zero() -> Self {
        return AntiPlane::from_groups(/* e1, e2, e3, e5 */ Simd32x4::from(0.0));
    }
}
impl Zero for AntiScalar {
    fn zero() -> Self {
        return AntiScalar::from_groups(/* e12345 */ 0.0);
    }
}
impl Zero for Circle {
    fn zero() -> Self {
        return Circle::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
        );
    }
}
impl Zero for CircleRotor {
    fn zero() -> Self {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x4::from(0.0),
        );
    }
}
impl Zero for Dipole {
    fn zero() -> Self {
        return Dipole::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35
            Simd32x3::from(0.0),
        );
    }
}
impl Zero for DipoleInversion {
    fn zero() -> Self {
        return DipoleInversion::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl Zero for DualNum {
    fn zero() -> Self {
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from(0.0));
    }
}
impl Zero for FlatPoint {
    fn zero() -> Self {
        return FlatPoint::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(0.0));
    }
}
impl Zero for Flector {
    fn zero() -> Self {
        return Flector::from_groups(/* e15, e25, e35, e45 */ Simd32x4::from(0.0), /* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0));
    }
}
impl Zero for Line {
    fn zero() -> Self {
        return Line::from_groups(/* e415, e425, e435 */ Simd32x3::from(0.0), /* e235, e315, e125 */ Simd32x3::from(0.0));
    }
}
impl Zero for Motor {
    fn zero() -> Self {
        return Motor::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from(0.0), /* e235, e315, e125, e5 */ Simd32x4::from(0.0));
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
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e235, e315, e125
            Simd32x3::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
            // e1234
            0.0,
        );
    }
}
impl Zero for Plane {
    fn zero() -> Self {
        return Plane::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0));
    }
}
impl Zero for RoundPoint {
    fn zero() -> Self {
        return RoundPoint::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(0.0), /* e5 */ 0.0);
    }
}
impl Zero for Scalar {
    fn zero() -> Self {
        return Scalar::from_groups(/* scalar */ 0.0);
    }
}
impl Zero for Sphere {
    fn zero() -> Self {
        return Sphere::from_groups(/* e4235, e4315, e4125, e3215 */ Simd32x4::from(0.0), /* e1234 */ 0.0);
    }
}
impl Zero for VersorEven {
    fn zero() -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl Zero for VersorOdd {
    fn zero() -> Self {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
