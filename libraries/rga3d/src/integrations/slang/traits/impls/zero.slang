implementing rga3d;
using traits;
public extension AntiScalar: Zero {
    public typedef AntiScalar ZeroOutput;
    public func zero() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            0.0
        );
    }
}
public extension DualNum: Zero {
    public typedef DualNum ZeroOutput;
    public func zero() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(0.0)
        );
    }
}
public extension Flector: Zero {
    public typedef Flector ZeroOutput;
    public func zero() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Horizon: Zero {
    public typedef Horizon ZeroOutput;
    public func zero() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            0.0
        );
    }
}
public extension Line: Zero {
    public typedef Line ZeroOutput;
    public func zero() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Motor: Zero {
    public typedef Motor ZeroOutput;
    public func zero() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension MultiVector: Zero {
    public typedef MultiVector ZeroOutput;
    public func zero() -> MultiVector {
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
public extension Origin: Zero {
    public typedef Origin ZeroOutput;
    public func zero() -> Origin {
        return Origin.from_groups(
            /* e4 */
            0.0
        );
    }
}
public extension Plane: Zero {
    public typedef Plane ZeroOutput;
    public func zero() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point: Zero {
    public typedef Point ZeroOutput;
    public func zero() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension Scalar: Zero {
    public typedef Scalar ZeroOutput;
    public func zero() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            0.0
        );
    }
}
