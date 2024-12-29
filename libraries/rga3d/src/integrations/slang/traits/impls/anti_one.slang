implementing rga3d;
using traits;
public extension AntiScalar {
    public AntiScalar anti_one() {
        return AntiScalar.from_groups(
            /* e1234 */
            1.0
        );
    }
}
public extension DualNum {
    public DualNum anti_one() {
        return DualNum.from_groups(
            /* scalar, e1234 */
            float2(0.0, 1.0)
        );
    }
}
public extension Motor {
    public Motor anti_one() {
        return Motor.from_groups(
            /* e41, e42, e43, e1234 */
            float4(float3(0.0), 1.0), 
            /* e23, e31, e12, scalar */
            float4(0.0)
        );
    }
}
public extension MultiVector {
    public MultiVector anti_one() {
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
