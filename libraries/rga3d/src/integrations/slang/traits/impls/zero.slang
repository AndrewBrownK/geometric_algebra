extension AntiScalar: Zero {
    func zero() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            0.0
        );
    }
}
extension DualNum: Zero {
    func zero() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(0.0)
        );
    }
}
extension Flector: Zero {
    func zero() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Horizon: Zero {
    func zero() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            0.0
        );
    }
}
extension Line: Zero {
    func zero() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
extension Motor: Zero {
    func zero() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension MultiVector: Zero {
    func zero() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Zero {
    func zero() -> Origin {
        return Origin.from_groups(
            /* e4 */
            0.0
        );
    }
}
extension Plane: Zero {
    func zero() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Point: Zero {
    func zero() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
extension Scalar: Zero {
    func zero() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            0.0
        );
    }
}
