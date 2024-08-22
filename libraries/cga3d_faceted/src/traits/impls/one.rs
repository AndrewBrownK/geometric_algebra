// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 14
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
impl One for AntiCircleRotor {
    fn one() -> Self {
        return AntiCircleRotor::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
    }
}
impl One for AntiCircleRotorAligningOrigin {
    fn one() -> Self {
        return AntiCircleRotorAligningOrigin::from_groups(
            // e41, e42, e43
            Simd32x3::from(0.0),
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
    }
}
impl One for AntiCircleRotorAligningOriginAtInfinity {
    fn one() -> Self {
        return AntiCircleRotorAligningOriginAtInfinity::from_groups(
            // e23, e31, e12
            Simd32x3::from(0.0),
            // e15, e25, e35, scalar
            Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
    }
}
impl One for AntiCircleRotorAtInfinity {
    fn one() -> Self {
        return AntiCircleRotorAtInfinity::from_groups(/* e23, e31, e12, e45 */ Simd32x4::from(0.0), /* e15, e25, e35, scalar */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]));
    }
}
impl One for AntiCircleRotorOnOrigin {
    fn one() -> Self {
        return AntiCircleRotorOnOrigin::from_groups(/* e41, e42, e43, scalar */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]), /* e23, e31, e12 */ Simd32x3::from(0.0));
    }
}
impl One for AntiDualNum {
    fn one() -> Self {
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([0.0, 1.0]));
    }
}
impl One for AntiMotor {
    fn one() -> Self {
        return AntiMotor::from_groups(
            // e23, e31, e12, scalar
            Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            // e15, e25, e35, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl One for AntiMotorOnOrigin {
    fn one() -> Self {
        return AntiMotorOnOrigin::from_groups(/* e23, e31, e12, scalar */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]));
    }
}
impl One for AntiVersorEvenOnOrigin {
    fn one() -> Self {
        return AntiVersorEvenOnOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            // e23, e31, e12, e1234
            Simd32x4::from(0.0),
        );
    }
}
impl One for MultiVector {
    fn one() -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([1.0, 0.0]),
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
impl One for Scalar {
    fn one() -> Self {
        return Scalar::from_groups(/* scalar */ 1.0);
    }
}
impl One for VersorOdd {
    fn one() -> Self {
        return VersorOdd::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl One for VersorOddAtInfinity {
    fn one() -> Self {
        return VersorOddAtInfinity::from_groups(
            // scalar, e15, e25, e35
            Simd32x4::from([1.0, 0.0, 0.0, 0.0]),
            // e23, e31, e12, e45
            Simd32x4::from(0.0),
            // e4235, e4315, e4125, e3215
            Simd32x4::from(0.0),
        );
    }
}
impl One for VersorOddOrthogonalOrigin {
    fn one() -> Self {
        return VersorOddOrthogonalOrigin::from_groups(
            // e41, e42, e43, scalar
            Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            // e23, e31, e12, e3215
            Simd32x4::from(0.0),
            // e15, e25, e35, e1234
            Simd32x4::from(0.0),
        );
    }
}
