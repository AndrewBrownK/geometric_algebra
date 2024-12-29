implementing rga3d;
using traits;
public extension DualNum: One {
    public typedef DualNum OneOutput;
    public func one() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(1.0, 0.0)
        );
    }
}
public extension Motor: One {
    public typedef Motor OneOutput;
    public func one() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(0.0), 
            /* e23, e31, e12, scalar */
            float4(float3(0.0), 1.0)
        );
    }
}
public extension MultiVector: One {
    public typedef MultiVector OneOutput;
    public func one() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(1.0, 0.0), 
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
public extension Scalar: One {
    public typedef Scalar OneOutput;
    public func one() -> Scalar {
        return Scalar.from_groups(
            /* scalar */
            1.0
        );
    }
}
