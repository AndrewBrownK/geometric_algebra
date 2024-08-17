// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
//
// Total Implementations: 6
//
// Yes SIMD:   add/sub     mul     div
//  Minimum:         0       0       0
//   Median:         0       0       0
//  Average:         0       0       0
//  Maximum:         0       0       0
//
//  No SIMD:   add/sub    mul    div
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
            Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
        );
    }
}
impl AntiOne for DualNum {
    fn anti_one() -> Self {
        return DualNum::from_groups(/* e5, e12345 */ Simd32x2::from([0.0, 1.0]));
    }
}
impl AntiOne for Motor {
    fn anti_one() -> Self {
        return Motor::from_groups(
            // e415, e425, e435, e12345
            Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
        );
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
            // e15, e25, e35, e45
            Simd32x4::from(0.0),
            // e41, e42, e43
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
            // e12, e31, e23
            Simd32x3::from(0.0),
        );
    }
}
impl AntiOne for VersorEven {
    fn anti_one() -> Self {
        return VersorEven::from_groups(
            // e423, e431, e412, e12345
            Simd32x4::from([0.0, 0.0, 0.0, 1.0]),
            // e415, e425, e435, e321
            Simd32x4::from(0.0),
            // e235, e315, e125, e5
            Simd32x4::from(0.0),
            // e1, e2, e3, e4
            Simd32x4::from(0.0),
        );
    }
}
