impl AntiOne for AntiScalar {
    fn anti_one() -> Self {
        return AntiScalar::from_groups(/* e12345 */ 1.0);
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
impl AntiOne for MotorOnOrigin {
    fn anti_one() -> Self {
        return MotorOnOrigin::from_groups(/* e415, e425, e435, e12345 */ Simd32x4::from([0.0, 0.0, 0.0, 1.0]));
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
