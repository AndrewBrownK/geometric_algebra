implementing rga3d;
using traits;
public extension AntiScalar {
    public AntiScalar unit() {
        return AntiScalar.from_groups(
            /* e1234 */
            1.0
        );
    }
}
public extension DualNum {
    public DualNum unit() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(1.0)
        );
    }
}
public extension Flector {
    public Flector unit() {
        return Flector.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0), 
            /* e423, e431, e412, e321 */
            float4(1.0)
        );
    }
}
public extension Horizon {
    public Horizon unit() {
        return Horizon.from_groups(
            /* e321 */
            1.0
        );
    }
}
public extension Line {
    public Line unit() {
        return Line.from_groups(
            /* e41, e42, e43 */
            float3(1.0), 
            /* e23, e31, e12 */
            float3(1.0)
        );
    }
}
public extension Motor {
    public Motor unit() {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(1.0), 
            /* e23, e31, e12, scalar */
            float4(1.0)
        );
    }
}
public extension MultiVector {
    public MultiVector unit() {
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
public extension Origin {
    public Origin unit() {
        return Origin.from_groups(
            /* e4 */
            1.0
        );
    }
}
public extension Plane {
    public Plane unit() {
        return Plane.from_groups(
            /* e423, e431, e412, e321 */
            float4(1.0)
        );
    }
}
public extension Point {
    public Point unit() {
        return Point.from_groups(
            /* e1, e2, e3, e4 */
            float4(1.0)
        );
    }
}
public extension Scalar {
    public Scalar unit() {
        return Scalar.from_groups(
            /* scalar */
            1.0
        );
    }
}
