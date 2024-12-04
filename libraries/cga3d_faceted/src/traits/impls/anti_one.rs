// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 16
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
impl AntiOne for AntiScalar {
    fn anti_one() -> Self {
        return AntiScalar::from_groups(/* e12345 */ 1.0);
    }
}
impl AntiOne for CircleRotor {
    fn anti_one() -> Self {
        return CircleRotor::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x3::from(0.0).with_w(1.0),
        );
    }
}
impl AntiOne for CircleRotorAligningOrigin {
    fn anti_one() -> Self {
        return CircleRotorAligningOrigin::from_groups(
            // e423, e431, e412
            Simd32x3::from(0.0),
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x3::from(0.0).with_w(1.0),
        );
    }
}
impl AntiOne for CircleRotorAligningOriginAtInfinity {
    fn anti_one() -> Self {
        return CircleRotorAligningOriginAtInfinity::from_groups(
            // e415, e425, e435
            Simd32x3::from(0.0),
            // e235, e315, e125, e12345
            Simd32x3::from(0.0).with_w(1.0),
        );
    }
}
impl AntiOne for CircleRotorAtInfinity {
    fn anti_one() -> Self {
        return CircleRotorAtInfinity::from_groups(
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e12345
            Simd32x3::from(0.0).with_w(1.0),
        );
    }
}
impl AntiOne for CircleRotorOnOrigin {
    fn anti_one() -> Self {
        return CircleRotorOnOrigin::from_groups(/* e423, e431, e412, e12345 */ Simd32x3::from(0.0).with_w(1.0), /* e415, e425, e435 */ Simd32x3::from(0.0));
    }
}
impl AntiOne for DualNum {
    fn anti_one() -> Self {
        return DualNum::from_groups(/* e4, e12345 */ Simd32x2::from([0.0, 1.0]));
    }
}
impl AntiOne for Motor {
    fn anti_one() -> Self {
        return Motor::from_groups(/* e415, e425, e435, e12345 */ Simd32x3::from(0.0).with_w(1.0), /* e235, e315, e125, e5 */ Simd32x4::from(0.0));
    }
}
impl AntiOne for MotorOnOrigin {
    fn anti_one() -> Self {
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x3::from(0.0).with_w(1.0));
    }
}
impl AntiOne for MultiVector {
    fn anti_one() -> Self {
        return MultiVector::from_groups(
            // scalar, e12345
            Simd32x2::from([0.0, 1.0]),
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
impl AntiOne for MysteryCircleRotor {
    fn anti_one() -> Self {
        return MysteryCircleRotor::from_groups(/* e415, e425, e435, e321 */ Simd32x4::from(0.0), /* e12345 */ 1.0);
    }
}
impl AntiOne for MysteryVersorEven {
    fn anti_one() -> Self {
        return MysteryVersorEven::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([1.0, 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
        );
    }
}
impl AntiOne for VersorEven {
    fn anti_one() -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(1.0),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
impl AntiOne for VersorEvenAligningOrigin {
    fn anti_one() -> Self {
        return VersorEvenAligningOrigin::from_groups(
            // e423, e431, e412, e12345
            Simd32x3::from(0.0).with_w(1.0),
            // e415, e425, e435, e4
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}
impl AntiOne for VersorEvenAtInfinity {
    fn anti_one() -> Self {
        return VersorEvenAtInfinity::from_groups(
            // e12345, e1, e2, e3
            Simd32x4::from([1.0, 0.0, 0.0, 0.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
    }
}
impl AntiOne for VersorEvenOnOrigin {
    fn anti_one() -> Self {
        return VersorEvenOnOrigin::from_groups(/* e423, e431, e412, e12345 */ Simd32x3::from(0.0).with_w(1.0), /* e415, e425, e435, e4 */ Simd32x4::from(0.0));
    }
}
