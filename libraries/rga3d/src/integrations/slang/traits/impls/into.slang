extension DualNum: From<AntiScalar> {
    associatedtype Self = DualNum:
    static func from(from_anti_scalar: AntiScalar) -> Self {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, from_anti_scalar.e1234), 0.0, 0.0)
        );
    }
}
extension AntiScalar: Into<DualNum> {
    public func into() -> DualNum {
        return DualNum.from(this);
    }
}
extension Motor: From<AntiScalar> {
    associatedtype Self = Motor:
    static func from(from_anti_scalar: AntiScalar) -> Self {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), from_anti_scalar.e1234), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
extension AntiScalar: Into<Motor> {
    public func into() -> Motor {
        return Motor.from(this);
    }
}
extension MultiVector: From<AntiScalar> {
    associatedtype Self = MultiVector:
    static func from(from_anti_scalar: AntiScalar) -> Self {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(0.0, from_anti_scalar.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension AntiScalar: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension Motor: From<DualNum> {
    associatedtype Self = Motor:
    static func from(from_dual_num: DualNum) -> Self {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), from_dual_num.e1234), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), from_dual_num.scalar)
        );
    }
}
extension DualNum: Into<Motor> {
    public func into() -> Motor {
        return Motor.from(this);
    }
}
extension MultiVector: From<DualNum> {
    associatedtype Self = MultiVector:
    static func from(from_dual_num: DualNum) -> Self {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(from_dual_num.group0.xy, 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension DualNum: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension MultiVector: From<Flector> {
    associatedtype Self = MultiVector:
    static func from(from_flector: Flector) -> Self {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            from_flector.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            from_flector.group1
        );
    }
}
extension Flector: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension Flector: From<Horizon> {
    associatedtype Self = Flector:
    static func from(from_horizon: Horizon) -> Self {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), from_horizon.e321)
        );
    }
}
extension Horizon: Into<Flector> {
    public func into() -> Flector {
        return Flector.from(this);
    }
}
extension MultiVector: From<Horizon> {
    associatedtype Self = MultiVector:
    static func from(from_horizon: Horizon) -> Self {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), from_horizon.e321)
        );
    }
}
extension Horizon: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension Plane: From<Horizon> {
    associatedtype Self = Plane:
    static func from(from_horizon: Horizon) -> Self {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(float3(0.0), from_horizon.e321)
        );
    }
}
extension Horizon: Into<Plane> {
    public func into() -> Plane {
        return Plane.from(this);
    }
}
extension Motor: From<Line> {
    associatedtype Self = Motor:
    static func from(from_line: Line) -> Self {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(from_line.group0.xyz, 0.0), 
            /* e23, e31, e12, scalar */
            float4(from_line.group1.xyz, 0.0)
        );
    }
}
extension Line: Into<Motor> {
    public func into() -> Motor {
        return Motor.from(this);
    }
}
extension MultiVector: From<Line> {
    associatedtype Self = MultiVector:
    static func from(from_line: Line) -> Self {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(from_line.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(from_line.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Line: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension MultiVector: From<Motor> {
    associatedtype Self = MultiVector:
    static func from(from_motor: Motor) -> Self {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(from_motor.scalar, from_motor.e1234), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(from_motor.group0.xyz, 0.0), 
            /* e23, e31, e12, 0 */
            float4(from_motor.group1.xyz, 0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Motor: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension Flector: From<Origin> {
    associatedtype Self = Flector:
    static func from(from_origin: Origin) -> Self {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), from_origin.e4), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Into<Flector> {
    public func into() -> Flector {
        return Flector.from(this);
    }
}
extension MultiVector: From<Origin> {
    associatedtype Self = MultiVector:
    static func from(from_origin: Origin) -> Self {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), from_origin.e4), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Origin: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension Point: From<Origin> {
    associatedtype Self = Point:
    static func from(from_origin: Origin) -> Self {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), from_origin.e4)
        );
    }
}
extension Origin: Into<Point> {
    public func into() -> Point {
        return Point.from(this);
    }
}
extension Flector: From<Plane> {
    associatedtype Self = Flector:
    static func from(from_plane: Plane) -> Self {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            from_plane.group0
        );
    }
}
extension Plane: Into<Flector> {
    public func into() -> Flector {
        return Flector.from(this);
    }
}
extension MultiVector: From<Plane> {
    associatedtype Self = MultiVector:
    static func from(from_plane: Plane) -> Self {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            from_plane.group0
        );
    }
}
extension Plane: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension Flector: From<Point> {
    associatedtype Self = Flector:
    static func from(from_point: Point) -> Self {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            from_point.group0, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Point: Into<Flector> {
    public func into() -> Flector {
        return Flector.from(this);
    }
}
extension MultiVector: From<Point> {
    associatedtype Self = MultiVector:
    static func from(from_point: Point) -> Self {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(0.0), 
            /* e1, e2, e3, e4 */
            from_point.group0, 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Point: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
extension DualNum: From<Scalar> {
    associatedtype Self = DualNum:
    static func from(from_scalar: Scalar) -> Self {
        return DualNum.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(from_scalar.scalar, 0.0), 0.0, 0.0)
        );
    }
}
extension Scalar: Into<DualNum> {
    public func into() -> DualNum {
        return DualNum.from(this);
    }
}
extension Motor: From<Scalar> {
    associatedtype Self = Motor:
    static func from(from_scalar: Scalar) -> Self {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), from_scalar.scalar)
        );
    }
}
extension Scalar: Into<Motor> {
    public func into() -> Motor {
        return Motor.from(this);
    }
}
extension MultiVector: From<Scalar> {
    associatedtype Self = MultiVector:
    static func from(from_scalar: Scalar) -> Self {
        return MultiVector.from_groups(
            /* scalar, e1234, 0, 0 */
            float4(float2(from_scalar.scalar, 0.0), 0.0, 0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43, 0 */
            float4(0.0), 
            /* e23, e31, e12, 0 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
extension Scalar: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
