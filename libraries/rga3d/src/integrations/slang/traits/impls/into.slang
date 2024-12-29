implementing rga3d;
using traits;
public extension DualNum: From<AntiScalar> {
    public static func from(AntiScalar from_anti_scalar) -> This {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(0.0, from_anti_scalar.e1234)
        );
    }
}
public extension AntiScalar: Into<DualNum> {
    public func into() -> DualNum {
        return DualNum.from(this);
    }
}
public extension Motor: From<AntiScalar> {
    public static func from(AntiScalar from_anti_scalar) -> This {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), from_anti_scalar.e1234), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension AntiScalar: Into<Motor> {
    public func into() -> Motor {
        return Motor.from(this);
    }
}
public extension MultiVector: From<AntiScalar> {
    public static func from(AntiScalar from_anti_scalar) -> This {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, from_anti_scalar.e1234), 
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
public extension AntiScalar: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
public extension Motor: From<DualNum> {
    public static func from(DualNum from_dual_num) -> This {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), from_dual_num.e1234), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), from_dual_num.scalar)
        );
    }
}
public extension DualNum: Into<Motor> {
    public func into() -> Motor {
        return Motor.from(this);
    }
}
public extension MultiVector: From<DualNum> {
    public static func from(DualNum from_dual_num) -> This {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            from_dual_num.group0.xy, 
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
public extension DualNum: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
public extension MultiVector: From<Flector> {
    public static func from(Flector from_flector) -> This {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            from_flector.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            from_flector.group1
        );
    }
}
public extension Flector: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
public extension Flector: From<Horizon> {
    public static func from(Horizon from_horizon) -> This {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(float3(0.0), from_horizon.e321)
        );
    }
}
public extension Horizon: Into<Flector> {
    public func into() -> Flector {
        return Flector.from(this);
    }
}
public extension MultiVector: From<Horizon> {
    public static func from(Horizon from_horizon) -> This {
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
            float4(float3(0.0), from_horizon.e321)
        );
    }
}
public extension Horizon: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
public extension Plane: From<Horizon> {
    public static func from(Horizon from_horizon) -> This {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(float3(0.0), from_horizon.e321)
        );
    }
}
public extension Horizon: Into<Plane> {
    public func into() -> Plane {
        return Plane.from(this);
    }
}
public extension Motor: From<Line> {
    public static func from(Line from_line) -> This {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(from_line.group0.xyz, 0.0), 
            /* e23, e31, e12, scalar */
            float4(from_line.group1.xyz, 0.0)
        );
    }
}
public extension Line: Into<Motor> {
    public func into() -> Motor {
        return Motor.from(this);
    }
}
public extension MultiVector: From<Line> {
    public static func from(Line from_line) -> This {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            from_line.group0.xyz, 
            /* e23, e31, e12 */
            from_line.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Line: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
public extension MultiVector: From<Motor> {
    public static func from(Motor from_motor) -> This {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(from_motor.scalar, from_motor.e1234), 
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e41, e42, e43 */
            from_motor.group0.xyz, 
            /* e23, e31, e12 */
            from_motor.group1.xyz, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Motor: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
public extension Flector: From<Origin> {
    public static func from(Origin from_origin) -> This {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), from_origin.e4), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin: Into<Flector> {
    public func into() -> Flector {
        return Flector.from(this);
    }
}
public extension MultiVector: From<Origin> {
    public static func from(Origin from_origin) -> This {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            float4(float3(0.0), from_origin.e4), 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Origin: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
public extension Point: From<Origin> {
    public static func from(Origin from_origin) -> This {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(float3(0.0), from_origin.e4)
        );
    }
}
public extension Origin: Into<Point> {
    public func into() -> Point {
        return Point.from(this);
    }
}
public extension Flector: From<Plane> {
    public static func from(Plane from_plane) -> This {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            from_plane.group0
        );
    }
}
public extension Plane: Into<Flector> {
    public func into() -> Flector {
        return Flector.from(this);
    }
}
public extension MultiVector: From<Plane> {
    public static func from(Plane from_plane) -> This {
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
            from_plane.group0
        );
    }
}
public extension Plane: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
public extension Flector: From<Point> {
    public static func from(Point from_point) -> This {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            from_point.group0, 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point: Into<Flector> {
    public func into() -> Flector {
        return Flector.from(this);
    }
}
public extension MultiVector: From<Point> {
    public static func from(Point from_point) -> This {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0), 
            /* e1, e2, e3, e4 */
            from_point.group0, 
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
public extension DualNum: From<Scalar> {
    public static func from(Scalar from_scalar) -> This {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(from_scalar.scalar, 0.0)
        );
    }
}
public extension Scalar: Into<DualNum> {
    public func into() -> DualNum {
        return DualNum.from(this);
    }
}
public extension Motor: From<Scalar> {
    public static func from(Scalar from_scalar) -> This {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), from_scalar.scalar)
        );
    }
}
public extension Scalar: Into<Motor> {
    public func into() -> Motor {
        return Motor.from(this);
    }
}
public extension MultiVector: From<Scalar> {
    public static func from(Scalar from_scalar) -> This {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(from_scalar.scalar, 0.0), 
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
public extension Scalar: Into<MultiVector> {
    public func into() -> MultiVector {
        return MultiVector.from(this);
    }
}
