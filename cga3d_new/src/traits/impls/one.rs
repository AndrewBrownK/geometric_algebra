impl One for AntiDualNum {
    fn one() -> Self {
        return AntiDualNum::from_groups(/* e3215, scalar */ Simd32x2::from([0.0, 1.0]));
    }
}
impl One for AntiDualNumOnOrigin {
    fn one() -> Self {
        return AntiDualNumOnOrigin::from_groups(/* scalar */ 1.0);
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
