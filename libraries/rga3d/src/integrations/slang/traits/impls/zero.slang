impl Zero for AntiScalar {
    fn zero() -> Self {
        return AntiScalar::from_groups(/* e1234 */0.0);
    }
}
impl Zero for DualNum {
    fn zero() -> Self {
        return DualNum::from_groups(/* scalar, e1234 */float2(0.0));
    }
}
impl Zero for Flector {
    fn zero() -> Self {
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(0.0), /* e423, e431, e412, e321 */float4(0.0));
    }
}
impl Zero for Horizon {
    fn zero() -> Self {
        return Horizon::from_groups(/* e321 */0.0);
    }
}
impl Zero for Line {
    fn zero() -> Self {
        return Line::from_groups(/* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0));
    }
}
impl Zero for Motor {
    fn zero() -> Self {
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(0.0), /* e23, e31, e12, scalar */float4(0.0));
    }
}
impl Zero for MultiVector {
    fn zero() -> Self {
        return MultiVector::from_groups(/* scalar, e1234 */float2(0.0), /* e1, e2, e3, e4 */float4(0.0), /* e41, e42, e43 */float3(0.0), /* e23, e31, e12 */float3(0.0), /* e423, e431, e412, e321 */float4(0.0));
    }
}
impl Zero for Origin {
    fn zero() -> Self {
        return Origin::from_groups(/* e4 */0.0);
    }
}
impl Zero for Plane {
    fn zero() -> Self {
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(0.0));
    }
}
impl Zero for Point {
    fn zero() -> Self {
        return Point::from_groups(/* e1, e2, e3, e4 */float4(0.0));
    }
}
impl Zero for Scalar {
    fn zero() -> Self {
        return Scalar::from_groups(/* scalar */0.0);
    }
}
