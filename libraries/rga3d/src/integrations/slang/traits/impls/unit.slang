extension AntiScalar: Unit {
    func unit() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            1.0
        );
    }
}
extension DualNum: Unit {
    func unit() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(1.0)
        );
    }
}
extension Flector: Unit {
    func unit() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0), 
            /* e423, e431, e412, e321 */
            float4(1.0)
        );
    }
}
extension Horizon: Unit {
    func unit() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            1.0
        );
    }
}
extension Line: Unit {
    func unit() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(1.0), 
            /* e23, e31, e12 */
            float3(1.0)
        );
    }
}
extension Motor: Unit {
    func unit() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(1.0), 
            /* e23, e31, e12, scalar */
            float4(1.0)
        );
    }
}
extension MultiVector: Unit {
    func unit() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0), 
            /* e1, e2, e3, e4 */
            float4(1.0), 
            /* e41, e42, e43 */
            float3(1.0), 
            /* e23, e31, e12 */
            float3(1.0), 
            /* e423, e431, e412, e321 */
            float4(1.0)
        );
    }
}
extension Origin: Unit {
    func unit() -> Origin {
        return Origin.from_groups(
            /* e4 */
            1.0
        );
    }
}
extension Plane: Unit {
    func unit() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0)
        );
    }
}
extension Point: Unit {
    func unit() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0)
        );
    }
}
extension Scalar: Unit {
    func unit() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            1.0
        );
    }
}
