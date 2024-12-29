implementing rga3d;
using traits;
public extension AntiScalar {
    public AntiScalar zero() {
        return AntiScalar.from_groups(
            /* e1234 */
            0.0
        );
    }
}
public extension DualNum {
    public DualNum zero() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(0.0)
        );
    }
}
public extension Flector {
    public Flector zero() {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0), 
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Horizon {
    public Horizon zero() {
        return Horizon.from_groups(
            /* e321 */
            0.0
        );
    }
}
public extension Line {
    public Line zero() {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(0.0), 
            /* e23, e31, e12 */
            float3(0.0)
        );
    }
}
public extension Motor {
    public Motor zero() {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension MultiVector {
    public MultiVector zero() {
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
public extension Origin {
    public Origin zero() {
        return Origin.from_groups(
            /* e4 */
            0.0
        );
    }
}
public extension Plane {
    public Plane zero() {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(0.0)
        );
    }
}
public extension Point {
    public Point zero() {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(0.0)
        );
    }
}
public extension Scalar {
    public Scalar zero() {
        return Scalar.from_groups(
            /* scalar */
            0.0
        );
    }
}
