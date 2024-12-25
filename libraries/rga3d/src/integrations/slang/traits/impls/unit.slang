impl Unit for AntiScalar {
    fn unit() -> Self {
        return AntiScalar::from_groups(/* e1234 */1.0);
    }
}
impl Unit for DualNum {
    fn unit() -> Self {
        return DualNum::from_groups(/* scalar, e1234 */float2(1.0));
    }
}
impl Unit for Flector {
    fn unit() -> Self {
        return Flector::from_groups(/* e1, e2, e3, e4 */float4(1.0), /* e423, e431, e412, e321 */float4(1.0));
    }
}
impl Unit for Horizon {
    fn unit() -> Self {
        return Horizon::from_groups(/* e321 */1.0);
    }
}
impl Unit for Line {
    fn unit() -> Self {
        return Line::from_groups(/* e41, e42, e43 */float3(1.0), /* e23, e31, e12 */float3(1.0));
    }
}
impl Unit for Motor {
    fn unit() -> Self {
        return Motor::from_groups(/* e41, e42, e43, e1234 */float4(1.0), /* e23, e31, e12, scalar */float4(1.0));
    }
}
impl Unit for MultiVector {
    fn unit() -> Self {
        return MultiVector::from_groups(/* scalar, e1234 */float2(1.0), /* e1, e2, e3, e4 */float4(1.0), /* e41, e42, e43 */float3(1.0), /* e23, e31, e12 */float3(1.0), /* e423, e431, e412, e321 */float4(1.0));
    }
}
impl Unit for Origin {
    fn unit() -> Self {
        return Origin::from_groups(/* e4 */1.0);
    }
}
impl Unit for Plane {
    fn unit() -> Self {
        return Plane::from_groups(/* e423, e431, e412, e321 */float4(1.0));
    }
}
impl Unit for Point {
    fn unit() -> Self {
        return Point::from_groups(/* e1, e2, e3, e4 */float4(1.0));
    }
}
impl Unit for Scalar {
    fn unit() -> Self {
        return Scalar::from_groups(/* scalar */1.0);
    }
}
