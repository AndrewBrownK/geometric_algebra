public extension AntiScalar: Unit {
    public typedef AntiScalar UnitOutput;
    public func unit() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            1.0
        );
    }
}
public extension DualNum: Unit {
    public typedef DualNum UnitOutput;
    public func unit() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(1.0)
        );
    }
}
public extension Flector: Unit {
    public typedef Flector UnitOutput;
    public func unit() -> Flector {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0), 
            /* e423, e431, e412, e321 */
            float4(1.0)
        );
    }
}
public extension Horizon: Unit {
    public typedef Horizon UnitOutput;
    public func unit() -> Horizon {
        return Horizon.from_groups(
            /* e321 */
            1.0
        );
    }
}
public extension Line: Unit {
    public typedef Line UnitOutput;
    public func unit() -> Line {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(1.0), 
            /* e23, e31, e12 */
            float3(1.0)
        );
    }
}
public extension Motor: Unit {
    public typedef Motor UnitOutput;
    public func unit() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(1.0), 
            /* e23, e31, e12, scalar */
            float4(1.0)
        );
    }
}
public extension MultiVector: Unit {
    public typedef MultiVector UnitOutput;
    public func unit() -> MultiVector {
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
public extension Origin: Unit {
    public typedef Origin UnitOutput;
    public func unit() -> Origin {
        return Origin.from_groups(
            /* e4 */
            1.0
        );
    }
}
public extension Plane: Unit {
    public typedef Plane UnitOutput;
    public func unit() -> Plane {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0)
        );
    }
}
public extension Point: Unit {
    public typedef Point UnitOutput;
    public func unit() -> Point {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0)
        );
    }
}
public extension Scalar: Unit {
    public typedef Scalar UnitOutput;
    public func unit() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            1.0
        );
    }
}
