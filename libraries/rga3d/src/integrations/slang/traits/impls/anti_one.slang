public extension AntiScalar: AntiOne {
    public typedef AntiScalar AntiOneOutput;
    public func anti_one() -> AntiScalar {
        return AntiScalar.from_groups(
            /* e1234 */
            1.0
        );
    }
}
public extension DualNum: AntiOne {
    public typedef DualNum AntiOneOutput;
    public func anti_one() -> DualNum {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(0.0, 1.0)
        );
    }
}
public extension Motor: AntiOne {
    public typedef Motor AntiOneOutput;
    public func anti_one() -> Motor {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension MultiVector: AntiOne {
    public typedef MultiVector AntiOneOutput;
    public func anti_one() -> MultiVector {
        return MultiVector.from_groups(
            /* scalar, e1234 */
            float2(0.0, 1.0), 
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
