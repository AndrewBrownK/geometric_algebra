// Note on Operative Statistics:
// Operative Statistics are not a precise predictor of performance or performance comparisons.
// This is due to varying hardware capabilities and compiler optimizations.
// As always, where performance is a concern, there is no substitute for
// real measurements on real work-loads on real hardware.
// Disclaimer aside, enjoy the fun information =)
impl Unit for AntiScalar {
    fn unit() -> Self {
        return AntiScalar::from_groups(/* e1234 */ 1.0);
    }
}
impl Unit for DualNum {
    fn unit() -> Self {
        return DualNum::from_groups(/* scalar, e1234 */ Simd32x2::from(1.0));
    }
}
impl Unit for Flector {
    fn unit() -> Self {
        return Flector::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0), /* e423, e431, e412, e321 */ Simd32x4::from(1.0));
    }
}
impl Unit for Horizon {
    fn unit() -> Self {
        return Horizon::from_groups(/* e321 */ 1.0);
    }
}
impl Unit for Line {
    fn unit() -> Self {
        return Line::from_groups(/* e41, e42, e43 */ Simd32x3::from(1.0), /* e23, e31, e12 */ Simd32x3::from(1.0));
    }
}
impl Unit for Motor {
    fn unit() -> Self {
        return Motor::from_groups(/* e41, e42, e43, e1234 */ Simd32x4::from(1.0), /* e23, e31, e12, scalar */ Simd32x4::from(1.0));
    }
}
impl Unit for MultiVector {
    fn unit() -> Self {
        return MultiVector::from_groups(
            // scalar, e1234
            Simd32x2::from(1.0),
            // e1, e2, e3, e4
            Simd32x4::from(1.0),
            // e41, e42, e43
            Simd32x3::from(1.0),
            // e23, e31, e12
            Simd32x3::from(1.0),
            // e423, e431, e412, e321
            Simd32x4::from(1.0),
        );
    }
}
impl Unit for Origin {
    fn unit() -> Self {
        return Origin::from_groups(/* e4 */ 1.0);
    }
}
impl Unit for Plane {
    fn unit() -> Self {
        return Plane::from_groups(/* e423, e431, e412, e321 */ Simd32x4::from(1.0));
    }
}
impl Unit for Point {
    fn unit() -> Self {
        return Point::from_groups(/* e1, e2, e3, e4 */ Simd32x4::from(1.0));
    }
}
impl Unit for Scalar {
    fn unit() -> Self {
        return Scalar::from_groups(/* scalar */ 1.0);
    }
}
